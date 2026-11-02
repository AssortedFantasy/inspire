"""Replay pinned public-domain editions. Development: beautifulsoup4==4.12.3."""

import argparse
import hashlib
import json
from pathlib import Path
import re
import urllib.request

from bs4 import BeautifulSoup

ROOT = Path(__file__).resolve().parent
CACHE = ROOT.parents[1] / "target/imports/cultural"
EXCERPTS = json.loads((ROOT / "excerpts.json").read_text(encoding="utf-8"))


def clean(node):
    node = BeautifulSoup(str(node), "html.parser")
    for artifact in node.select(".pagenum,.tag,sup"):
        artifact.decompose()
    if node.find("pre") or node.find("br"):
        for br in node.find_all("br"):
            br.replace_with("\n")
        return "\n".join(
            re.sub(r"[ \t\xa0]+", " ", line).strip()
            for line in node.get_text().replace("\r", "").splitlines()
            if line.strip()
        )
    return re.sub(r"\s+", " ", node.get_text()).strip()


def record(source, locator, body):
    credit = source["author"]
    if source["title"] != credit:
        credit += ", " + source["title"]
    credit += ", " + locator
    if source.get("translator"):
        credit += " (" + source["translator"] + ", " + source["edition"] + ")"
    else:
        credit += " (" + source["edition"] + ")"
    assert body and "\x00" not in body
    return {
        "id": re.sub(r"[^a-z0-9]+", "-", locator.lower()).strip("-"),
        "text": body + "\n\n— " + credit,
        "provenance": f"{source['catalog_url']}. {locator}. "
        f"Pinned HTML SHA-256 {source['sha256']}. {source['rights']}",
    }


def passages(item):
    """Replay reviewed paragraph/sentence spans, preserving every selected word."""
    if item["id"] not in EXCERPTS:
        return [item]
    body, credit = item["text"].rsplit("\n\n— ", 1)
    result = []
    previous_end = 0
    for n, (start, end) in enumerate(EXCERPTS[item["id"]], 1):
        assert previous_end <= start < end <= len(body)
        assert not body[previous_end:start].strip()
        result.append({
            "id": f"{item['id']}-excerpt-{n}",
            "text": body[start:end] + "\n\n— " + credit,
            "provenance": item["provenance"]
            + f" Excerpt {n}, source-body characters {start + 1}–{end}.",
        })
        previous_end = end
    assert not body[previous_end:].strip()
    return result


def tao(source, soup):
    active = False
    chapter = 0
    groups = {}
    for node in soup.find_all(["h2", "p", "pre"]):
        if node.get("id") == "pg-footer-heading":
            break
        text = clean(node)
        if node.name == "h2":
            if text == "PART 1.":
                active = True
            continue
        if not active or not text:
            continue
        # Chapters 1–5 share their numbers with verse numbers. Each begins
        # with an explicit first verse; later chapter numbers are unambiguous.
        expected = chapter + 1
        suffix = r"\s+1\." if expected <= 5 else r"(?:\s|$)"
        if re.match(rf"^(?:Ch\. )?{expected}\.{suffix}", text):
            chapter = expected
            groups[chapter] = []
            text = re.sub(rf"^(?:Ch\. )?{chapter}\.\s*", "", text)
        assert chapter > 0, text
        if text:
            groups[chapter].append(text)
    assert sorted(groups) == list(range(1, 82))
    return [record(source, f"chapter {n}", "\n\n".join(ps)) for n, ps in groups.items()]


def dhammapada(source, soup):
    records = []
    numbers = []
    chapter = None
    for node in soup.find_all(["h2", "p"]):
        if node.get("id") == "pg-footer-heading":
            break
        text = clean(node)
        if node.name == "h2":
            chapter = text if text.startswith("Chapter ") else None
            continue
        if not chapter or not text:
            continue
        match = re.match(r"^(\d+)(?:\s*[-–,]\s*(\d+))?\.", text)
        assert match, text
        first, last = int(match[1]), int(match[2] or match[1])
        numbers.extend(range(first, last + 1))
        span = str(first) if first == last else f"{first}–{last}"
        records.append(record(source, f"{chapter}, verse {span}", text))
    assert numbers == list(range(1, 424)), numbers
    return records


def edda(source, soup):
    records = []
    # Complete printed chapters only. Long chapters are omitted in full.
    for header in soup.find_all("h4"):
        anchor = header.find("a", id=True)
        if not anchor or not re.fullmatch(r"(?:gylfe|brage)_[IVX]+", anchor["id"]):
            continue
        title = None
        paragraphs = []
        for node in header.find_next_siblings():
            if node.name in ["h3", "h4"]:
                break
            if node.name == "h6":
                title = clean(node)
            if node.name == "p" and (text := clean(node)):
                paragraphs.append(text)
        body = "\n\n".join(paragraphs)
        if len(body) > 8000:
            continue
        assert title and 500 <= len(body) <= 8000
        part, chapter = anchor["id"].split("_")
        part = "The Fooling of Gylfe" if part == "gylfe" else "Brage’s Talk"
        locator = f"{part}, chapter {chapter}, {title}"
        item = record(source, locator, body)
        item["provenance"] += f" HTML anchor #{anchor['id']} through next chapter."
        records.append(item)
    assert len(records) == 18, len(records)
    return records


def bulfinch(source, soup):
    records = []
    # Whole named sections in the Greek and Roman chapters, before the Trojan
    # cycle. Preserve Bulfinch's literary quotations and their introductions.
    excluded = {"id00490", "id00673", "id00745"}
    for header in soup.find_all(["h3", "h4"]):
        anchor = header.get("id", "")
        if not re.fullmatch(r"id\d+", anchor) or not 157 <= int(anchor[2:]) <= 903:
            continue
        title = clean(header)
        if title.lower().startswith("chapter") or anchor in excluded:
            continue
        paragraphs = []
        last_id = None
        for node in header.find_next_siblings():
            if node.name in ["h3", "h4"]:
                break
            if node.name == "p" and (text := clean(node)):
                paragraphs.append(text)
                last_id = node.get("id")
        if paragraphs and paragraphs[-1].startswith(
            "The following are other celebrated mythical poets and musicians,"
        ):
            # A bridge to the next named section, not part of this story.
            paragraphs.pop()
        body = "\n\n".join(paragraphs)
        if not 400 <= len(body) <= 9000:
            continue
        item = record(source, title, body)
        item["provenance"] += f" Complete section #{anchor} through #{last_id}."
        records.append(item)
    assert len(records) == 53, len(records)
    return records


PARSERS = {"tao": tao, "dhammapada": dhammapada, "edda": edda, "bulfinch": bulfinch}


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
    all_texts = set()
    for source in sources:
        path = CACHE / f"pg{source['id']}.html"
        if args.download:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(urllib.request.urlopen(source["url"], timeout=60).read())
        raw = path.read_bytes()
        assert hashlib.sha256(raw).hexdigest() == source["sha256"], f"checksum: {path}"
        records = PARSERS[source["parser"]](source, BeautifulSoup(raw, "html.parser"))
        records = [part for item in records for part in passages(item)]
        assert len({item["id"] for item in records}) == len(records)
        for item in records:
            assert item["text"] not in all_texts
            all_texts.add(item["text"])
        emit(ROOT / source["path"] / "records.json",
             json.dumps(records, ensure_ascii=False, indent=2) + "\n", args.check)
        print(f"{source['path']}: {len(records)}")


if __name__ == "__main__":
    main()
