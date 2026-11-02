# /// script
# requires-python = ">=3.11"
# dependencies = ["pymupdf==1.28.2"]
# ///
"""Replay reviewed exchanges from the original NASA Apollo 12 transcript."""

import argparse
import hashlib
import json
from pathlib import Path
import urllib.request

import pymupdf


ROOT = Path(__file__).resolve().parent


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def render(source_dir):
    source = json.loads((ROOT / "sources.json").read_text(encoding="utf-8"))
    pdf = source_dir / source["filename"]
    if sha256(pdf.read_bytes()) != source["sha256"]:
        raise ValueError("Apollo 12 source PDF checksum mismatch")
    if pymupdf.VersionBind != source["pymupdf_version"]:
        raise ValueError(f'Use pymupdf=={source["pymupdf_version"]} for pinned extraction')
    selections = json.loads((ROOT / "selections.json").read_text(encoding="utf-8"))
    result = []
    seen_ids, seen_texts = set(), set()
    with pymupdf.open(pdf) as document:
        pages = {}
        for selection in selections:
            lines = []
            locations = []
            for turn in selection["turns"]:
                page = turn.get("page", selection["page"])
                if page not in pages:
                    pages[page] = " ".join(document[page - 1].get_text().split())
                text = pages[page][turn["start"] : turn["end"]]
                if sha256(text.encode()) != turn["sha256"]:
                    raise ValueError(f'Extracted turn changed: {selection["id"]}, page {page}')
                for old, new in turn["corrections"]:
                    if old not in text:
                        raise ValueError(f'OCR correction missing: {selection["id"]}: {old!r}')
                    text = text.replace(old, new)
                if not text.strip() or "\n" in text or "\0" in text:
                    raise ValueError(f'Invalid turn: {selection["id"]}')
                lines.append(f'{turn["speaker"]}: {text}')
                locations.append(page)
            first, last = min(locations), max(locations)
            locator = str(first) if first == last else f"{first}–{last}"
            credit = (
                "— Apollo 12, NASA air-to-ground transcript, November 1969, "
                f'MET {selection["mission_time"]}, PDF p. {locator}'
            )
            text = "\n".join(lines) + "\n\n" + credit
            if selection["id"] in seen_ids or text in seen_texts:
                raise ValueError("Duplicate Apollo selection")
            seen_ids.add(selection["id"])
            seen_texts.add(text)
            result.append(
                {
                    "id": selection["id"],
                    "text": text,
                    "provenance": (
                        f'{source["title"]}; PDF pages {locator}; '
                        f'MET {selection["mission_time"]}. '
                        f'{source["url"]} SHA256 {source["sha256"]}. '
                        "Original NASA mission transcript, public domain in the United States. "
                        "Speaker labels expand CDR=Charles Conrad Jr., CMP=Richard F. Gordon Jr., "
                        "LMP=Alan L. Bean, CC=Houston capsule communicator. "
                        "Whitespace reflowed; manifest lists scan-checked OCR corrections. "
                        "No corrected Journal transcript or editorial commentary is included. "
                        f'Reuse statement: {source["rights_url"]}. See LICENSE.txt.'
                    ),
                }
            )
    return json.dumps(result, ensure_ascii=False, indent=2) + "\n"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-dir", type=Path, required=True)
    parser.add_argument("--download", action="store_true")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    source = json.loads((ROOT / "sources.json").read_text(encoding="utf-8"))
    if args.download:
        args.source_dir.mkdir(parents=True, exist_ok=True)
        path = args.source_dir / source["filename"]
        if not path.exists():
            with urllib.request.urlopen(source["url"], timeout=120) as response:
                data = response.read()
            if sha256(data) != source["sha256"]:
                raise ValueError("Downloaded Apollo PDF checksum mismatch")
            path.write_bytes(data)
    output = render(args.source_dir)
    path = ROOT / "apollo-12" / "records.json"
    if args.check:
        if path.read_text(encoding="utf-8") != output:
            raise SystemExit("Apollo 12 records differ from pinned replay")
    else:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(output, encoding="utf-8")
    print(f'{"Checked" if args.check else "Wrote"} {len(json.loads(output))} Apollo 12 excerpts')


if __name__ == "__main__":
    main()
