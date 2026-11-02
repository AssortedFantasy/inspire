"""Rebuild the institutional corpus from checksum-pinned primary documents."""

import argparse
from collections import Counter, defaultdict
import hashlib
import json
from pathlib import Path
import re
import urllib.request


ROOT = Path(__file__).resolve().parents[1]
BRANCH = ROOT / "families/society/institutions"
MANIFEST = BRANCH / "sources.json"


def normalize(text):
    text = re.sub(r"\[Sidenote:.*?\]", "", text, flags=re.S)
    text = re.sub(r"\[\d+\]", "", text)
    return " ".join(text.replace("_", "").split())


def rebuild(download):
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    source = manifest["source"]
    cache = ROOT / "target/imports/institutions" / source["cache"]
    if download and not cache.exists():
        cache.parent.mkdir(parents=True, exist_ok=True)
        with urllib.request.urlopen(source["url"], timeout=120) as response:
            cache.write_bytes(response.read())
    data = cache.read_bytes()
    if hashlib.sha256(data).hexdigest() != source["sha256"]:
        raise ValueError(f"Source checksum mismatch: {cache}")
    original = data.decode("utf-8-sig").replace("\r\n", "\n")
    lines = original.splitlines()
    files, records = {}, defaultdict(list)
    seen = set()
    for entry in manifest["selections"]:
        start, stop = entry["lines"]
        raw = "\n".join(lines[start - 1:stop])
        if not raw.startswith(entry["starts_with"]):
            raise ValueError(f"Selection boundary changed: {entry['id']}")
        if "[Footnote" in raw or "***" in raw:
            raise ValueError(f"Non-document material selected: {entry['id']}")
        body = normalize(raw)
        if not body or body in seen:
            raise ValueError(f"Blank or repeated text: {entry['id']}")
        seen.add(body)
        records[entry["leaf"]].append({
            "id": entry["id"],
            "text": body + "\n— " + entry["citation"],
            "provenance": f"{entry['citation']}. {entry['document']}. "
                          f"In {source['credit']}; {source['url']}; "
                          f"source SHA-256 {source['sha256']}; inclusive source "
                          f"lines {start}–{stop}. Public domain in the USA. "
                          "Complete selected clause or court entry; whitespace normalized, "
                          "italic markers, sidenotes and footnote callouts removed. "
                          "Original editorial brackets and ellipses retained.",
        })
    for leaf, rows in records.items():
        ids = Counter(row["id"] for row in rows)
        if max(ids.values()) != 1:
            raise ValueError(f"Duplicate IDs in {leaf}")
        files[BRANCH / leaf / "records.json"] = json.dumps(
            rows, ensure_ascii=False, indent=2) + "\n"
        files[BRANCH / leaf / "node.toml"] = f"weight = {len(rows)}\n"
    license_start = original.index("THE FULL PROJECT GUTENBERG")
    files[BRANCH / "LICENSE.txt"] = (
        "Selected historical documents from English Economic History: Select Documents, "
        "compiled and edited by A. E. Bland, P. A. Brown and R. H. Tawney (1914). "
        "The Project Gutenberg catalogue identifies ebook 43211 as public domain in "
        "the United States. Reuse elsewhere depends on local copyright law. "
        "Translations and modernized spelling are those of this historical edition; "
        "no modern editorial introduction is sampled. Document and source locators "
        "are retained in sources.json and record provenance. The original download's "
        "Project Gutenberg license follows.\n\n" + original[license_start:].strip() + "\n")
    return files


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--download", action="store_true")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    files = rebuild(args.download)
    stale = []
    for path, content in files.items():
        if args.check:
            if not path.exists() or path.read_text(encoding="utf-8") != content:
                stale.append(str(path.relative_to(ROOT)))
        else:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content, encoding="utf-8", newline="\n")
    if stale:
        raise SystemExit("Stale institution files: " + ", ".join(stale))
    print(f"{'Checked' if args.check else 'Wrote'} {len(files)} institution files")


if __name__ == "__main__":
    main()
