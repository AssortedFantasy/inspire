"""Replay the reviewed, checksum-pinned everyday correspondence and journals."""

import argparse
import hashlib
from html.parser import HTMLParser
import json
from pathlib import Path
import urllib.request

ROOT = Path(__file__).resolve().parent
VOID = {"area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"}


class Paragraphs(HTMLParser):
    """Preserve prose; omit inline page labels and footnote reference markup."""

    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.stack = []
        self.entries = []

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        skip = tag in {"script", "style", "sup"} or any(
            c in attrs.get("class", "").split()
            for c in {"pagenum", "tei-pb", "tei-noteref", "fnanchor"}
        )
        skip |= attrs.get("id", "").startswith("Page_")
        skip |= bool(self.stack and self.stack[-1]["skip"])
        self.stack.append({"tag": tag, "attrs": attrs, "parts": [], "skip": skip})
        if tag in VOID:
            self.stack.pop()

    def handle_endtag(self, tag):
        if not any(n["tag"] == tag for n in self.stack):
            return
        while self.stack:
            node = self.stack.pop()
            text = " ".join("".join(node["parts"]).split())
            if node["tag"] in {"p", "h1", "h2", "h3", "h4", "h5", "h6"}:
                self.entries.append((node["tag"], node["attrs"], text))
            if self.stack and not node["skip"]:
                self.stack[-1]["parts"].extend(node["parts"])
            if node["tag"] == tag:
                break

    def handle_data(self, data):
        if self.stack and not self.stack[-1]["skip"]:
            self.stack[-1]["parts"].append(data)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def render(cache, download=False):
    sources = json.loads((ROOT / "sources.json").read_text(encoding="utf-8"))
    books = {}
    for source in sources:
        path = cache / f'{source["id"]}.html'
        if download and not path.exists():
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(urllib.request.urlopen(source["url"], timeout=60).read())
        data = path.read_bytes()
        if digest(data) != source["sha256"]:
            raise ValueError(f'Changed source: {source["id"]}')
        parser = Paragraphs()
        parser.feed(data.decode("utf-8-sig"))
        books[source["id"]] = source, parser.entries
    outputs = {}
    ids, texts = set(), set()
    for item in json.loads((ROOT / "selections.json").read_text(encoding="utf-8")):
        source, entries = books[item["source"]]
        selected = entries[item["paragraph"]:item["paragraph_end"]]
        if not selected or any(tag != "p" or "footnote" in attrs.get("class", "") for tag, attrs, _ in selected):
            raise ValueError(f'Not prose: {item["id"]}')
        prose = "\n".join(text for _, _, text in selected)
        excerpt = prose[item["start"]:item["end"]]
        if not excerpt.strip() or "\0" in excerpt or digest(excerpt.encode()) != item["text_sha256"]:
            raise ValueError(f'Changed excerpt: {item["id"]}')
        if item["id"] in ids or excerpt in texts:
            raise ValueError(f'Duplicate: {item["id"]}')
        ids.add(item["id"])
        texts.add(excerpt)
        record = {
            "id": item["id"],
            "text": f'{excerpt}\n— {source["credit"]}, {item["locator"]}',
            "provenance": f'{source["citation"]}. {item["locator"]}. {source["url"]} '
            f'Public domain in the USA; source SHA-256 {source["sha256"]}. '
            f'Parsed entries {item["paragraph"]}:{item["paragraph_end"]}, characters {item["start"]}:{item["end"]}. '
            'Whitespace normalized; inline page labels and footnote callouts omitted. See LICENSE.txt.',
        }
        outputs.setdefault(source["id"], []).append(record)
    return outputs


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-dir", type=Path, default=Path("target/imports/everyday-voices"))
    parser.add_argument("--download", action="store_true")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    for leaf, records in render(args.source_dir, args.download).items():
        path = ROOT / leaf / "records.json"
        text = json.dumps(records, ensure_ascii=False, indent=2) + "\n"
        if args.check:
            if not path.exists() or path.read_text(encoding="utf-8") != text:
                raise ValueError(f'Stale output: {path}')
        else:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text, encoding="utf-8")
        print(f'{leaf}: {len(records)} records')


if __name__ == "__main__":
    main()
