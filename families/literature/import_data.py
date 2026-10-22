"""Import complete literary units from checksum-pinned development downloads.

Development dependency: beautifulsoup4==4.12.3. No runtime dependencies.
"""

import argparse
import hashlib
import json
from collections import defaultdict
from pathlib import Path
import re
import unicodedata
import urllib.request

from bs4 import BeautifulSoup

ROOT = Path(__file__).parent
MANIFEST = ROOT / "sources.json"


def clean(node, verse=False):
    node = BeautifulSoup(str(node), "html.parser")
    for artifact in node.select(".pagenum,.pageno,.footnote,.note,.caption,.fig,.mynote,.comment,sup"):
        artifact.decompose()
    for initial in node.select("span.letra img[alt]"):
        initial.replace_with(initial["alt"])
    for link in node.select('a[href^="#Footnote"],a[href^="#FN"],a[href^="#fn"]'):
        link.decompose()
    if verse:
        for br in node.find_all("br"):
            br.replace_with("\n")
        lines = node.get_text().replace("\r", "").splitlines()
        return "\n".join(re.sub(r"[ \t\xa0]+", " ", x).strip() for x in lines if x.strip())
    # Inline drop capitals must stay joined to the rest of their word.
    return re.sub(r"\s+", " ", node.get_text().replace("\xa0", " ")).strip()


def heading(node):
    text = re.sub(r"\[\s*\d+\s*\]", "", node.get_text(" ", strip=True))
    return re.sub(r"\s+", " ", text).strip()


def record(source, locator, body, excerpt=False):
    credit = source["author"]
    if source.get("translator"):
        credit += "; English " + source.get("translation_kind", "translation") + " by " + source["translator"]
    # Keep printed chapter and quatrain references when they aid readers, while
    # hiding source HTML anchors and import-time ordinals from sampled text.
    display_locator = None
    if match := re.fullmatch(r"quatrain-([IVXLCDM]+)", locator, re.I):
        display_locator = f"Quatrain {match.group(1)}"
    elif match := re.fullmatch(r"(.+); paragraphs-\d+-\d+", locator):
        display_locator = match.group(1)
    elif match := re.search(r"\bchapter\s+([IVXLCDM]+|\d+)\b", locator, re.I):
        display_locator = f"Chapter {match.group(1)}"
    citation = f"{credit}, {source['title']} ({source['edition']})"
    if display_locator:
        citation += f", {display_locator}"
    return {
        "id": f"pg-{source['id']}-{locator}",
        "text": f"{body}\n- {citation}",
        "provenance": f"{source['url']}; downloaded {source.get('downloaded', '2026-10-13')}; SHA-256 {source['sha256']}. "
        f"Exact source locator: {locator}. {source['rights']} "
        "Underlying public-domain literary text; Project Gutenberg metadata, trademark statements, "
        "license boilerplate, introductions by other writers, notes and illustrations are excluded. "
        "HTML layout whitespace and page-number artifacts removed; verse line order preserved. "
        "Fictional speakers and historical attitudes belong to the cited work.",
    }


def body_text(text):
    """Return the stimulus before its final generated author/work credit."""
    body, delimiter, _credit = text.rpartition("\n- ")
    return body if delimiter else text


def body_key(text):
    return re.sub(r"\s+", " ", unicodedata.normalize("NFKC", body_text(text)).casefold())


def between(head):
    for sibling in head.next_siblings:
        if getattr(sibling, "name", None) in ("h1", "h2", "h3", "h4", "h5", "footer"):
            break
        if getattr(sibling, "name", None):
            yield sibling


def numbered(source, soup):
    for h in soup.find_all("h2"):
        number = heading(h).rstrip(".")
        if not number.isdigit():
            continue
        body = "\n\n".join(clean(p) for block in between(h) for p in
                           ([block] if block.name == "p" else block.find_all("p")))
        if 45 <= len(body) <= 2200:
            yield source["path"], record(source, "poem-" + number, body)


def aesop(source, soup):
    for h in soup.find_all("h2"):
        anchor = h.find("a", id=re.compile(r"^chap\d+$"))
        if not anchor:
            continue
        paragraphs = [clean(p) for block in between(h) for p in
                      ([block] if block.name == "p" else block.find_all("p"))]
        body = "\n\n".join(p for p in paragraphs if p)
        if 100 <= len(body) <= 2000:
            yield source["path"], record(source, anchor["id"], heading(h) + "\n\n" + body)


def gulistan(source, soup):
    """Keep complete numbered anecdotes; never cut their concluding verse."""
    chapter = None
    for tag in soup.find_all(["h2", "h4"]):
        if tag.name == "h2":
            chapter = heading(tag) if re.fullmatch(r"CHAPTER [IVX]+", heading(tag)) else None
            continue
        if not chapter or not re.fullmatch(r"[IVXLCDM]+\.?", heading(tag)):
            continue
        parts = [clean(p) for block in between(tag) for p in
                 ([block] if block.name == "p" else block.find_all("p"))]
        body = "\n\n".join(p for p in parts if p)
        if 90 <= len(body) <= 2400 and "*" not in body:
            locator = f"{chapter}; anecdote {heading(tag).rstrip('.')}"
            row = record(source, locator, body)
            row["id"] = f"pg-{source['id']}-{tag['id']}"
            row["text"] += f", anecdote {heading(tag).rstrip('.')}"
            yield source["path"], row


def quatrains(source, soup):
    active = False
    for tag in soup.find_all(["h2", "pre"]):
        if tag.name == "h2":
            active = heading(tag) == "First Edition"
        elif active:
            lines = clean(tag, True).splitlines()
            number = clean(tag.find_previous("p"))
            if len(lines) == 4 and re.fullmatch(r"[IVXLCDM]+\.?", number):
                yield source["path"], record(source, "quatrain-" + number.rstrip("."), "\n".join(lines))


def kabir(source, soup):
    active = False
    for h in soup.find_all(["h3", "h4"]):
        if h.get("id") == "id00038":
            active = True
        if not active:
            continue
        parts = [p for b in between(h) for p in ([b] if b.name == "p" else b.find_all("p"))]
        if len(parts) < 2:
            continue
        label = clean(parts[0])
        body = "\n\n".join(clean(p, True) for p in parts[1:])
        if 70 <= len(body) <= 2400:
            yield source["path"], record(source, h["id"], label + "\n\n" + body)


def chinese(source, soup):
    for index, poem in enumerate(soup.select("div.poem"), 1):
        h = poem.find_previous(["h4", "h5"])
        if not h or "title" not in h.get("class", []):
            continue  # Exclude poems quoted inside the translator's introductions.
        if poem.find_parent(class_="comment"):
            continue
        body = clean(poem, True)
        body = re.sub(r"\[\d+\]", "", body)
        if not 70 <= len(body) <= 2200:
            continue
        context = heading(h)
        if re.fullmatch(r"\(\d+\)", context):
            parent = h.find_previous("h4")
            context = heading(parent) + " " + context
        comments = []
        for previous in h.next_siblings:
            if previous is poem or getattr(previous, "name", None) in ["h4", "h5"]:
                break
            if getattr(previous, "name", None) and "comment" in previous.get("class", []):
                text = clean(previous.find("p"))
                if text.startswith("By "):
                    # Keep printed attribution, without the translator's biographical essay.
                    # Date abbreviations are not sentence boundaries.
                    comments.append(re.split(r"(?<=\))[,.;] |(?<!A\.D)(?<!B\.C)\. ", text, 1)[0])
        prefix = context + ("\n" + "\n".join(comments) if comments else "")
        path = source["path"] + ("/bai-juyi" if poem.find_previous("h2") and
                                   heading(poem.find_previous("h2")).startswith("PART II") else "/early-and-various")
        credited = dict(source)
        if path.endswith("/bai-juyi"):
            credited["author"] = "Bai Juyi (Po Chü-i)"
        yield path, record(credited, f"verse-block-{index}", prefix + "\n\n" + body)


def dunbar(source, soup):
    for index, poem in enumerate(soup.select("div.poem"), 1):
        h = poem.find("h4")
        if not h:
            continue
        book = poem.find_previous("h2")
        if not book or heading(book) not in source["collections"]:
            continue
        body = "\n\n".join("\n".join(clean(x) for x in stanza.select("div[class^=i]"))
                            for stanza in poem.select(".stanza"))
        if 70 <= len(body) <= 2100:
            path = source["path"] + "/" + source["collections"][heading(book)]
            yield path, record(source, f"poem-{index}", heading(h) + "\n\n" + body)


def pre_poems(source, soup):
    active = False
    serial = 0
    for tag in soup.find_all(["h2", "h3", "h4", "pre"]):
        if heading(tag) == source["start"]:
            active = True
        if not active or tag.name != "pre":
            continue
        serial += 1
        h = tag.find_previous(["h2", "h4"])
        body = clean(tag, True)
        if 70 <= len(body) <= 2400:
            anchor = h.find("a", id=True)
            title = heading(h)
            if source["id"] == 7241 and not anchor:
                continue  # Dedications and prefatory poems are not fables.
            locator = anchor["id"] if anchor else f"verse-block-{serial}"
            yield source["path"], record(source, locator, title + "\n\n" + body)


def prophet(source, soup):
    for chapter_index, chapter in enumerate(soup.select("div.chapter"), 1):
        paragraphs = chapter.find_all("p", recursive=False)
        if not paragraphs:
            continue
        topic = paragraphs[0].find("b")
        topic = clean(topic) if topic else ("The Coming of the Ship" if chapter_index == 1 else "The Farewell")
        chunk, first = [], 1
        for index, p in enumerate(paragraphs, 1):
            text = clean(p)
            if not text:
                continue
            if chunk and sum(map(len, chunk)) + len(text) > 1400:
                yield source["path"], record(source, f"{topic}; paragraphs-{first}-{index - 1}", "\n\n".join(chunk), True)
                chunk, first = [], index
            chunk.append(text)
        if chunk and len(" ".join(chunk)) >= 120:
            yield source["path"], record(source, f"{topic}; paragraphs-{first}-{len(paragraphs)}", "\n\n".join(chunk), True)


def narrative(source, soup):
    active = False
    section, paragraph = "", 0
    candidates = defaultdict(list)
    for tag in soup.find_all([source.get("heading", "h2"), "p"]):
        if tag.name != "p":
            title = heading(tag)
            if re.search(source["start"], title, re.I):
                active = True
            if active and title:
                section, paragraph = title, 0
            continue
        if not active or not section:
            continue
        paragraph += 1
        if any(set(p.get("class", [])) & {"footnote", "note", "caption", "fig", "poem", "toc", "mynote", "center"}
               or p.name in ("table", "footer", "header") for p in [tag, *tag.parents]):
            continue
        body = clean(tag)
        if not (230 <= len(body) <= 1500) or not re.search(r"[.!?][’'”\"]?$", body):
            continue
        if re.search(r"\b(Project Gutenberg|Illustration|transcriber|copyright)\b", body, re.I):
            continue
        locator = f"section-{section}; paragraph-{paragraph}"
        candidates[section].append(record(source, locator, body, True))
    # Round-robin chapters/stories; select deterministically across each chapter,
    # rather than dumping the opening or letting a long chapter dominate.
    queue = []
    for records in candidates.values():
        count = min(len(records), 8)
        queue.append([records[(i * len(records)) // count] for i in range(count)])
    selected = []
    for i in range(8):
        selected.extend(rows[i] for rows in queue if len(rows) > i)
    for row in selected[:source.get("cap", 140)]:
        # Hash long visible locators only in stable identifiers; preserve them in text/provenance.
        row["id"] = f"pg-{source['id']}-p-" + hashlib.sha256(row["id"].encode()).hexdigest()[:16]
        yield source["path"], row


PARSERS = {f.__name__: f for f in (numbered, aesop, gulistan, quatrains, kabir, chinese, dunbar, pre_poems, narrative, prophet)}


def build(cache):
    outputs, seen = defaultdict(list), set()
    retained_ids = {row["id"] for row in json.loads((ROOT / "baseline_manifest.json").read_text(encoding="utf-8"))}
    # Different citations do not make identical retained paragraphs new stimuli.
    for path in ROOT.rglob("records.json"):
        for row in json.loads(path.read_text(encoding="utf-8")):
            if row["id"] in retained_ids:
                seen.add(body_key(row["text"]))
    for source in json.loads(MANIFEST.read_text(encoding="utf-8")):
        raw = (cache / f"pg{source['id']}.html").read_bytes()
        if hashlib.sha256(raw).hexdigest() != source["sha256"]:
            raise ValueError(f"Source checksum mismatch: {source['id']}")
        soup = BeautifulSoup(raw, "html.parser")
        for node in soup.select("#pg-header,#pg-footer"):
            node.decompose()
        for path, row in PARSERS[source["parser"]](source, soup):
            key = body_key(row["text"])
            if key in seen:
                continue
            seen.add(key)
            outputs[path].append(row)
    return outputs


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-dir", type=Path, default=Path("target/imports/literature"))
    parser.add_argument("--download", action="store_true")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    if args.download:
        args.source_dir.mkdir(parents=True, exist_ok=True)
        for source in json.loads(MANIFEST.read_text(encoding="utf-8")):
            target = args.source_dir / f"pg{source['id']}.html"
            if not target.exists():
                target.write_bytes(urllib.request.urlopen(source["url"], timeout=60).read())
    outputs = build(args.source_dir)
    for path, rows in outputs.items():
        target = ROOT / path / "records.json"
        encoded = json.dumps(rows, ensure_ascii=False, indent=2) + "\n"
        if args.check:
            if not target.exists() or target.read_text(encoding="utf-8") != encoded:
                raise ValueError(f"Generated records differ: {target}")
        else:
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(encoded, encoding="utf-8", newline="\n")
        print(f"{path}: {len(rows)}")
    print(f"Imported {sum(map(len, outputs.values()))} records in {len(outputs)} leaves")


if __name__ == "__main__":
    main()
