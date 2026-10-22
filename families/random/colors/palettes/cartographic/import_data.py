"""Import unmodified official palette rows from a pinned ColorBrewer export.

python import_data.py [--input downloaded-colorbrewer.json]
Raw input is never kept in the family tree; runtime uses records.json only.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re
import urllib.request

REVISION = "7d135fc4e19eda73f2eb1bf55fcdf4a04fe4881f"
URL = f"https://raw.githubusercontent.com/axismaps/colorbrewer/{REVISION}/export/colorbrewer.json"
SHA256 = "729cb527c1edcf3267c4521df29ede5092f3e6171c5a259e2ebdd252335840b9"
KINDS = {"seq": "sequential", "div": "diverging", "qual": "qualitative"}


def transform(raw):
    if hashlib.sha256(raw).hexdigest() != SHA256:
        raise ValueError("Source checksum mismatch")
    data = json.loads(raw)
    result = {kind: [] for kind in KINDS.values()}
    for name, scheme in sorted(data.items()):
        kind = KINDS[scheme["type"]]
        for count in sorted((key for key in scheme if key != "type"), key=int):
            colors = []
            for color in scheme[count]:
                match = re.fullmatch(r"rgb\((\d+),(\d+),(\d+)\)", color)
                if not match:
                    raise ValueError(f"Invalid color: {color}")
                rgb = tuple(map(int, match.groups()))
                if any(channel > 255 for channel in rgb):
                    raise ValueError(f"Invalid channel: {rgb}")
                colors.append("#" + "".join(f"{channel:02x}" for channel in rgb))
            if len(colors) != int(count) or len(set(colors)) != len(colors):
                raise ValueError(f"Invalid palette: {name}/{count}")
            result[kind].append({
                "id": f"colorbrewer-{name.lower()}-{count}",
                "text": f"{', '.join(colors)}\nColorBrewer {name}",
                "provenance": f"ColorBrewer export by Cynthia Brewer, revision {REVISION}; {URL}; SHA-256 {SHA256}; Apache-Style ColorBrewer license in LICENSE.txt. RGB channels and palette ordering preserved; converted to hexadecimal.",
            })
    if len(data) != 35 or {kind: len(rows) for kind, rows in result.items()} != {"sequential": 125, "diverging": 81, "qualitative": 58}:
        raise ValueError("Unexpected source coverage")
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path)
    args = parser.parse_args()
    raw = args.input.read_bytes() if args.input else urllib.request.urlopen(URL).read()
    for kind, rows in transform(raw).items():
        directory = Path(__file__).parent / kind
        directory.mkdir(exist_ok=True)
        (directory / "records.json").write_text(json.dumps(rows, indent=2) + "\n", encoding="utf-8", newline="\n")
        print(f"{kind}: {len(rows)} palettes")


if __name__ == "__main__":
    main()
