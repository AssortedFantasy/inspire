"""Replay whole stage-direction blocks. Development: beautifulsoup4==4.12.3."""

import argparse
import hashlib
import json
from pathlib import Path
import re
import urllib.request

from bs4 import BeautifulSoup

ROOT = Path(__file__).resolve().parent
CACHE = ROOT.parents[1] / "target/imports/cultural"


def clean(node):
    return re.sub(r"\s+", " ", node.get_text()).strip()


def extract(source, soup):
    records = []
    act = None
    block = []
    first = last = 0
    ordinal = 0

    def flush():
        nonlocal block
        if not block:
            return
        body = "\n\n".join(block)
        block = []
        # Tiny movement cues and oversize openings are omitted in full.
        if not 120 <= len(body) <= 8000:
            return
        locator = f"{act}, stage block {first}–{last}"
        credit = f"{source['author']}, {source['title']}, {act}"
        if translator := source.get("translator"):
            credit += f" ({translator} translation)"
        else:
            credit += f" ({source['edition']})"
        records.append({
            "id": re.sub(r"[^a-z0-9]+", "-", locator.lower()).strip("-"),
            "text": body + "\n\n— " + credit,
            "provenance": f"{source['catalog_url']}. {locator}. "
            "Ordinals count every source paragraph in the act, including dialogue; "
            "consecutive standalone direction paragraphs form one complete block. "
            f"Pinned HTML SHA-256 {source['sha256']}. {source['rights']}",
        })

    for node in soup.find_all(["h2", "h3", "p"]):
        if node.get("id") == "pg-footer-heading":
            break
        text = clean(node)
        if node.name in ["h2", "h3"]:
            flush()
            if re.fullmatch(r"ACT [IVX]+[.]?", text):
                act = text.rstrip(".")
                ordinal = 0
            else:
                act = None
            continue
        if not act or not text:
            continue
        ordinal += 1
        classes = set(node.get("class", []))
        italic = node.find("i")
        standalone = bool(
            "stage" in classes or "noindent" in classes
            or (italic and clean(italic) == text)
        )
        # This edition uses noindent for one interleaved dialogue passage.
        if re.match(r"^[A-Z][A-Z .’'–-]+\.\s", text):
            standalone = False
        # PG 2446 misclassifies a speech plus two movements as p.stage.
        if source["id"] == 2446 and act == "ACT IV" and ordinal == 38:
            assert "so I will begin" in text
            standalone = False
        if standalone:
            if not block:
                first = ordinal
            last = ordinal
            block.append(text)
        else:
            flush()
    flush()
    assert len({item["id"] for item in records}) == len(records)
    assert records
    return records


def emit(path, text, check):
    if check:
        assert path.read_text(encoding="utf-8") == text, f"stale: {path}"
    else:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8", newline="\n")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--download", action="store_true")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    sources = json.loads((ROOT / "sources.json").read_text(encoding="utf-8"))
    counts = {}
    for source in sources:
        path = CACHE / f"pg{source['id']}.html"
        if args.download:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(urllib.request.urlopen(source["url"], timeout=60).read())
        raw = path.read_bytes()
        assert hashlib.sha256(raw).hexdigest() == source["sha256"], f"checksum: {path}"
        records = extract(source, BeautifulSoup(raw, "html.parser"))
        assert len(records) == {2542: 5, 2446: 12, 3618: 20, 3825: 28}[source["id"]]
        emit(ROOT / source["path"] / "records.json",
             json.dumps(records, ensure_ascii=False, indent=2) + "\n", args.check)
        counts[ROOT / source["path"]] = len(records)
        print(f"{source['path']}: {len(records)}")
    # All stage directions have equal probability within this collection.
    parents = set()
    for path in counts:
        parents.add(path.parent)
        emit(path / "node.toml", f"weight = {counts[path]}\n", args.check)
    for path in parents:
        total = sum(count for leaf, count in counts.items() if leaf.parent == path)
        emit(path / "node.toml", f"weight = {total}\n", args.check)


if __name__ == "__main__":
    main()
