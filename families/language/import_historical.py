"""Rebuild curated, verbatim language extracts from two public-domain books.

Run from the repository root: python -B families/language/import_historical.py
The downloaded source snapshots live under target/imports/language.
"""

import argparse
import hashlib
from html.parser import HTMLParser
import json
from pathlib import Path
import re
import urllib.request

ROOT = Path(__file__).resolve().parent
CACHE = ROOT.parents[1] / "target/imports/language"
NOTICE = """# Historical language sources

The selected text from these books is in the public domain in the United States:

- Thomas Preston (compiler), *A Dictionary of English Proverbs and Proverbial
  Phrases*, Whittaker & Co., nineteenth century. Project Gutenberg ebook 39281,
  produced by Delphine Lettau, Lisa Reigel, and the Online Distributed Proofreading
  Team. <https://www.gutenberg.org/ebooks/39281>
- Ernest Weekley, *The Romance of Words*, fourth edition, 1922. Project Gutenberg
  ebook 23958, produced by Jonathan Ingram, Stephen Blundell, and the Online
  Distributed Proofreading Team. <https://www.gutenberg.org/ebooks/23958>

The importer omits the Project Gutenberg headers, footers, trademarks, and license
boilerplate from sampled text. Source URLs, snapshot checksums, and extract
checksums are retained in `historical-selections.json` and record provenance.
Typography is rendered as plain text and whitespace is normalized. The extracts
retain historical wording and scholarship, without silently modernizing it.

The separately licensed WordNet data retain their full notice in
`lexicon/LICENSE`.

## Source ebook redistribution terms

The complete source ebook terms follow for transparency.

"""


class Element:
    def __init__(self, tag, attrs=(), parent=None):
        self.tag = tag
        self.attrs = dict(attrs)
        self.parent = parent
        self.children = []

    def walk(self, tag=None):
        for child in self.children:
            if isinstance(child, Element):
                if tag is None or child.tag == tag:
                    yield child
                yield from child.walk(tag)

    def text(self, keyword=False):
        classes = self.attrs.get("class", "").split()
        if (set(classes) & {"pagenum", "sidenote"}
                or keyword and "smcap" in classes
                or self.tag == "a" and self.attrs.get("href", "").startswith(
                    ("#Footnote", "#FNanchor"))):
            return ""
        return "".join(child.text(keyword) if isinstance(child, Element) else child
                       for child in self.children)

    def inside(self, classes):
        parent = self.parent
        while parent:
            if set(parent.attrs.get("class", "").split()) & classes:
                return True
            parent = parent.parent
        return False


class Book(HTMLParser):
    def __init__(self, source):
        super().__init__(convert_charrefs=True)
        self.root = Element("root")
        self.current = self.root
        self.feed(source)

    def handle_starttag(self, tag, attrs):
        node = Element(tag, attrs, self.current)
        self.current.children.append(node)
        if tag not in {"area", "base", "br", "col", "embed", "hr", "img", "input",
                       "link", "meta", "param", "source", "track", "wbr"}:
            self.current = node

    def handle_startendtag(self, tag, attrs):
        self.current.children.append(Element(tag, attrs, self.current))

    def handle_endtag(self, tag):
        node = self.current
        while node.parent:
            if node.tag == tag:
                self.current = node.parent
                break
            node = node.parent

    def handle_data(self, data):
        self.current.children.append(data)


def normalize(text):
    return re.sub(r"\s+", " ", text).strip()


def digest(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def extract_preston(book):
    entries = {}
    for p in book.root.walk("p"):
        anchors = [a.attrs.get("id", "") for a in p.walk("a")]
        ids = [int(a.removeprefix("proverb_")) for a in anchors
               if re.fullmatch(r"proverb_\d+", a)]
        if not ids:
            continue
        text = re.sub(r"^\d+\.\s*", "", normalize(p.text(keyword=True)))
        if text:
            entries[ids[0]] = text
    return entries


def extract_weekley(book):
    entries = {}
    for p in book.root.walk("p"):
        if p.attrs.get("class") or p.inside(
                {"footnote", "footnotes", "poem", "poetry", "pg-header", "pg-footer", "figcenter"}):
            continue
        text = normalize(p.text())
        if 100 <= len(text) <= 1800:
            entries[digest(text)] = text
    return entries


def write(relative, records, check):
    destination = ROOT / relative
    assert records and len({r["id"] for r in records}) == len(records)
    assert len({r["text"] for r in records}) == len(records)
    save(destination, json.dumps(records, ensure_ascii=False, indent=2) + "\n", check)
    print(f"{relative}: {len(records)}")


def save(path, text, check):
    if check:
        if path.read_text(encoding="utf-8") != text:
            raise ValueError(f"Generated file is stale: {path}")
    else:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8", newline="\n")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cache", type=Path, default=CACHE)
    parser.add_argument("--check", action="store_true", help="verify generated files without writing")
    args = parser.parse_args()
    selections = json.loads((ROOT / "historical-selections.json").read_text(encoding="utf-8"))
    books = {}
    for name, source in selections["sources"].items():
        path = args.cache / source["filename"]
        if not path.exists():
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(urllib.request.urlopen(source["url"]).read())
        raw = path.read_bytes()
        if hashlib.sha256(raw).hexdigest() != source["sha256"]:
            raise ValueError(f"Source snapshot changed: {path}; review before updating its SHA256")
        books[name] = Book(raw.decode("utf-8"))

    sayings = extract_preston(books["preston"])
    records = []
    for entry in selections["preston"]:
        text = sayings[entry["number"]]
        assert digest(text) == entry["sha256"], entry
        records.append({
            "id": f"preston-{entry['number']}",
            "text": text + "\n— Thomas Preston (compiler), A Dictionary of English Proverbs and Proverbial Phrases",
            "provenance": (
                f"Thomas Preston (compiler), A Dictionary of English Proverbs and Proverbial Phrases, "
                f"entry {entry['number']}; {selections['sources']['preston']['url']}. "
                "Public-domain source text. Exact entry wording with whitespace normalized; "
                "printed entry number, index keyword and page furniture omitted. "
                "No invented definitions, interpretations or examples. "
                "Selection and source checksums: families/language/historical-selections.json."
            ),
        })
    write("phraseology/sayings/records.json", records, args.check)

    histories = extract_weekley(books["weekley"])
    records = []
    for sha in selections["weekley"]:
        text = histories[sha]
        records.append({
            "id": "weekley-" + sha[:12],
            "text": text + "\n— Ernest Weekley, The Romance of Words (1922)",
            "provenance": (
                "Ernest Weekley, The Romance of Words, fourth edition (1922); "
                f"{selections['sources']['weekley']['url']}. Public-domain historical source text. "
                "Exact paragraph wording with whitespace normalized; page furniture and footnote "
                "reference markers omitted. Historical scholarship retained as written. "
                "Selection and source checksums: families/language/historical-selections.json."
            ),
        })
    write("word-histories/records.json", records, args.check)
    footer = next(node for node in books["preston"].root.walk()
                  if node.attrs.get("id") == "pg-footer")
    terms = "\n".join(line.rstrip() for line in footer.text().splitlines()).strip()
    save(ROOT / "LICENSE.md", NOTICE + terms + "\n", args.check)


if __name__ == "__main__":
    main()
