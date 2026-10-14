"""Replay pinned primary-source excerpts; all runtime material is embedded JSON."""

import argparse
import hashlib
from html.parser import HTMLParser
import json
from pathlib import Path
import re
import urllib.request

ROOT = Path(__file__).resolve().parent
VOID = {"area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"}


class Paragraphs(HTMLParser):
    """Read prose/headings in document order, excluding inline locator markup."""

    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.stack = []
        self.entries = []

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        skip = tag in {"script", "style", "sup"} or any(
            x in attrs.get("class", "").split()
            for x in {"pagenum", "tei-pb", "tei-noteref", "fnanchor"}
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


def clean(text):
    # Remove print page labels and footnote callouts, never prose or archaic spelling.
    text = re.sub(r"\[(?:Pg|pg)\s+[\divxlc]+\]", "", text)
    text = re.sub(r"\(\d+\)|\[\d+\]|\{\d+\}", "", text)
    text = re.sub(r"\bp\.\s*\d+", "", text)
    return " ".join(text.split())


def passage(text, start, end):
    """Explicit character offsets select contiguous text; no automatic truncation."""
    return text[start:end] if end is not None else text[start:]


def render(source_dir):
    manifest = json.loads((ROOT / "sources.json").read_text(encoding="utf-8"))
    selections = json.loads((ROOT / "selections.json").read_text(encoding="utf-8"))
    books = {}
    for book in manifest:
        data = (source_dir / f'{book["book"]}.html').read_bytes()
        digest = hashlib.sha256(data).hexdigest()
        if digest != book["sha256"]:
            raise ValueError(f'Checksum mismatch for {book["book"]}: {digest}')
        parser = Paragraphs()
        parser.feed(data.decode(book.get("encoding", "utf-8-sig")))
        books[book["book"]] = (book, parser.entries)
    outputs = {}
    seen_ids, seen_texts = set(), set()
    for selection in selections:
        book, entries = books[selection["book"]]
        tag, attrs, raw = entries[selection["paragraph"]]
        if tag != "p" or "footnote" in attrs.get("class", ""):
            raise ValueError(f'Not main prose: {selection}')
        cleaned = clean(raw)
        excerpt = passage(cleaned, selection.get("start", 0), selection.get("end"))
        if not excerpt.strip() or "\0" in excerpt:
            raise ValueError(f'Invalid passage: {selection["id"]}')
        if hashlib.sha256(excerpt.encode()).hexdigest() != selection["text_sha256"]:
            raise ValueError(f'Passage changed: {selection["id"]}')
        text = f'{book["credit"]}, {selection["locator"]}:\n“{excerpt}”'
        record = {
            "id": selection["id"],
            "text": text,
            "provenance": f'Public-domain primary-source excerpt. {book["citation"]}. '
            f'{book["url"]}; source SHA-256 {book["sha256"]}; '
            f'HTML prose entry {selection["paragraph"]}, normalized-text characters '
            f'{selection.get("start", 0)}:{selection.get("end", "end")}. '
            'Whitespace normalized; page labels and footnote callouts omitted. '
            'Historical wording and perspective retained; no modern factual endorsement.',
        }
        if record["id"] in seen_ids or excerpt in seen_texts:
            raise ValueError(f'Duplicate selection: {selection["id"]}')
        seen_ids.add(record["id"])
        seen_texts.add(excerpt)
        outputs.setdefault(selection["leaf"], []).append(record)
    return outputs


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-dir", type=Path, default=Path("target/imports/history"))
    parser.add_argument("--download", action="store_true")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    if args.download:
        args.source_dir.mkdir(parents=True, exist_ok=True)
        for book in json.loads((ROOT / "sources.json").read_text(encoding="utf-8")):
            path = args.source_dir / f'{book["book"]}.html'
            if not path.exists():
                path.write_bytes(urllib.request.urlopen(book["url"], timeout=60).read())
    outputs = render(args.source_dir)
    for leaf, records in sorted(outputs.items()):
        path = ROOT / leaf / "records.json"
        content = json.dumps(records, ensure_ascii=False, indent=2) + "\n"
        if args.check:
            if path.read_bytes() != content.encode("utf-8"):
                raise ValueError(f'Output differs: {path}')
        else:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content, encoding="utf-8", newline="\n")
    print(f'{sum(map(len, outputs.values()))} excerpts in {len(outputs)} leaves')


if __name__ == "__main__":
    main()
