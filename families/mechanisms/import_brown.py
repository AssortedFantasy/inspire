"""Replay curated public-domain Brown passages; downloads stay outside families/."""

import argparse
import hashlib
import json
import re
import urllib.request
from collections import defaultdict
from html.parser import HTMLParser
from pathlib import Path

ROOT = Path(__file__).resolve().parent


class Paragraphs(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.active = False
        self.current = []
        self.paragraphs = []

    def handle_starttag(self, tag, attrs):
        if tag == "p":
            self.active = True
            self.current = []

    def handle_endtag(self, tag):
        if tag == "p" and self.active:
            self.paragraphs.append(" ".join("".join(self.current).split()))
            self.active = False

    def handle_data(self, data):
        if self.active:
            self.current.append(data)


def produce(cache, download=False):
    groups = defaultdict(list)
    selection = json.loads((ROOT / "brown_selection.json").read_text(encoding="utf-8"))
    for entry in selection:
        source = cache / entry["file"]
        if not source.exists() and download:
            source.parent.mkdir(parents=True, exist_ok=True)
            source.write_bytes(urllib.request.urlopen(entry["url"], timeout=40).read())
        raw = source.read_bytes()
        if hashlib.sha256(raw).hexdigest() != entry["sha256"]:
            raise ValueError(f"Source checksum mismatch: {source}")
        parser = Paragraphs()
        parser.feed(raw.decode("utf-8"))
        number = entry["number"]
        matches = [p for p in parser.paragraphs if p.startswith(f"{number}. ")]
        if len(matches) != 1:
            raise ValueError(f"Expected one original paragraph for movement {number}")
        text = re.sub(r"^\d+\.\s*", "", matches[0])
        if entry["start"]:
            text = text[text.index(entry["start"]):]
        if entry["end"]:
            text = text[:text.index(entry["end"])]
        text = text.rstrip(" ;")
        if text[-1] not in ".?!":
            text += "."
        groups[entry["path"]].append({
            "id": f"brown-1908-movement-{number:03}",
            "text": f"Henry T. Brown (1908), movement {number}:\n{text}",
            "provenance": (
                "Henry T. Brown, Five Hundred and Seven Mechanical Movements, "
                f"21st edition (1908), movement {number}; public-domain excerpt. "
                f"Transcription: {entry['url']}."
            ),
        })
    return groups


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-dir", type=Path, required=True)
    parser.add_argument("--download", action="store_true")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    groups = produce(args.source_dir, args.download)
    for group, records in sorted(groups.items()):
        destination = ROOT / group / "records.json"
        serialized = json.dumps(records, ensure_ascii=False, indent=2) + "\n"
        if args.check:
            if destination.read_text(encoding="utf-8") != serialized:
                raise ValueError(f"Corpus differs from pinned replay: {destination}")
        else:
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_text(serialized, encoding="utf-8", newline="\n")
    print(f"{'Checked' if args.check else 'Imported'} {sum(map(len, groups.values()))} passages in {len(groups)} leaves")


if __name__ == "__main__":
    main()
