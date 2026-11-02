"""Replay pinned craft excerpts: python -B families/procedures/craft/import_craft.py --check."""

import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import re
from urllib.request import urlopen

from bs4 import BeautifulSoup


ROOT = Path(__file__).resolve().parent
MANIFEST = json.loads((ROOT / "source_manifest.json").read_text(encoding="utf-8"))
DEFAULT_CACHE = Path("target/imports/craft")


def source_paragraphs(raw):
    soup = BeautifulSoup(raw.decode("utf-8"), "html.parser")
    for node in soup.select(".pagenum,.pageno,.caption,figure,.figcenter,.figleft,.figright"):
        node.decompose()
    return [re.sub(r"\s+", " ", p.get_text(" ", strip=True)).strip()
            for p in soup.find_all("p")]


def extract(parts, selection):
    matches = [i for i, p in enumerate(parts) if p.startswith(selection["start"])]
    if len(matches) != 1:
        raise ValueError(f"Missing or ambiguous craft excerpt: {selection['id']}")
    first = matches[0]
    blocks = parts[first:first + selection["paragraphs"]]
    if len(blocks) != selection["paragraphs"] or any(not p for p in blocks):
        raise ValueError(f"Incomplete paragraph selection: {selection['id']}")
    text = "\n\n".join(blocks)
    for key in ("begin", "stop"):
        if key in selection:
            boundary = selection[key]
            if text.count(boundary) != 1:
                raise ValueError(f"Missing or ambiguous {key}: {selection['id']}")
            position = text.index(boundary)
            text = text[position:] if key == "begin" else text[:position]
    for omission in selection.get("omit", []):
        if text.count(omission) != 1:
            raise ValueError(f"Missing or ambiguous omission: {selection['id']}")
        text = text.replace(omission, "")
    # These references only navigate to illustrations or page numbers. Selected
    # operations are fully described in words; no illustration labels are used.
    text = re.sub(r"\s*\(\s*(?:see )?(?:fig\.|page)\s*\d+(?:\s*,\s*[A-Z])?\s*\)", "", text,
                  flags=re.IGNORECASE)
    text = re.sub(r", as shown (?:in|at) fig\.\s*\d+\s*\.", ".", text, flags=re.IGNORECASE)
    text = re.sub(r"[ \t]+", " ", text).strip()
    if re.search(r"\bfig\.|\b(?:chapter [IVX]+|see page)\b", text, flags=re.IGNORECASE):
        raise ValueError(f"Unresolved navigation: {selection['id']}")
    return text


def imported(cache=DEFAULT_CACHE):
    parts = {}
    for key, source in MANIFEST["sources"].items():
        raw = (cache / source["file"]).read_bytes()
        digest = hashlib.sha256(raw).hexdigest()
        if digest != source["sha256"]:
            raise ValueError(f"{source['file']}: SHA256 {digest}, expected {source['sha256']}")
        parts[key] = source_paragraphs(raw)
    output = {}
    for selection in MANIFEST["selections"]:
        source = MANIFEST["sources"][selection["source"]]
        body = extract(parts[selection["source"]], selection)
        omissions = ""
        if selection.get("omit"):
            omissions = " Omitted cross-reference sentence: " + " ".join(selection["omit"])
        row = {
            "id": f"{selection['source']}-{selection['id']}",
            "text": body + "\n\n" + source["credit"],
            "provenance": (
                f"Verbatim public-domain primary text, {source['credit']} "
                f"Paragraph beginning {selection['start']!r}; "
                f"{selection['paragraphs']} source paragraph(s). "
                f"{source['url']}; SHA256 {source['sha256']}. "
                "HTML and page numbers removed; whitespace normalized. "
                "Figure/page navigation omitted. Where specified in the manifest, "
                "excerpt boundaries fall between complete sentences. No illustrations reproduced."
                + omissions
            ),
        }
        output.setdefault(selection["path"], []).append(row)
    return output


def corpus_weights(output):
    weights = Counter()
    for path, rows in output.items():
        for length in range(1, len(path.split("/")) + 1):
            weights["/".join(path.split("/")[:length])] += len(rows)
    return weights


def main():
    parser = argparse.ArgumentParser(__doc__)
    parser.add_argument("--source-dir", type=Path, default=DEFAULT_CACHE)
    parser.add_argument("--download", action="store_true", help="Fetch missing pinned source files")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    if args.download:
        args.source_dir.mkdir(parents=True, exist_ok=True)
        for source in MANIFEST["sources"].values():
            destination = args.source_dir / source["file"]
            if not destination.exists():
                with urlopen(source["url"], timeout=60) as response:
                    raw = response.read()
                if hashlib.sha256(raw).hexdigest() != source["sha256"]:
                    raise ValueError(f"Upstream source changed: {source['file']}")
                destination.write_bytes(raw)
    output = imported(args.source_dir)
    payloads = {ROOT / path / "records.json": json.dumps(rows, ensure_ascii=False, indent=2) + "\n"
                for path, rows in output.items()}
    payloads.update({ROOT / path / "node.toml": f"weight = {weight}\n"
                     for path, weight in corpus_weights(output).items()})
    for destination, payload in payloads.items():
        if args.check:
            if destination.read_text(encoding="utf-8") != payload:
                raise ValueError(f"Replay differs: {destination}")
        else:
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_text(payload, encoding="utf-8", newline="\n")
    print(f"{'Verified' if args.check else 'Wrote'} {sum(map(len, output.values()))} records in {len(output)} leaves")


if __name__ == "__main__":
    main()
