"""Rebuild painting, drawing and print metadata from the pinned CMA catalogue."""

import argparse
from collections import defaultdict, deque
import csv
import hashlib
import html
import json
from pathlib import Path
import re
import urllib.request


ROOT = Path(__file__).resolve().parent
CACHE = ROOT.parents[1] / "target/imports/cleveland/data.csv"
REVISION = "d8dd744bc29e7986df428341133c14e18eb56b3f"
URL = f"https://media.githubusercontent.com/media/ClevelandMuseumArt/openaccess/{REVISION}/data.csv"
SHA256 = "137ddfab3264555d52c0211f804544cab1c62170ce3d753d924c9d464d74e449"
TYPES = {"Painting": "painting", "Drawing": "drawing", "Print": "printmaking"}
CAP = 600
ROLE_LABELS = {
    "published by": "publisher", "printed by": "printer",
    "engraved by": "engraver", "book authored by": "book author",
    "calligraphy by": "calligrapher", "execution supervised by": "execution supervisor",
}


def clean(text):
    return " ".join(html.unescape(re.sub(r"<[^>]*>", "", text)).split())


def creator_credits(text):
    """Keep catalogue credits while removing missing-role serialization noise."""
    if text in {"", "[]", "None"}:
        return []
    credits = []
    for credit in text.split("; "):
        name, separator, role = credit.rpartition(", ")
        if separator and role == "None":
            credit = name
        elif separator and role in ROLE_LABELS:
            credit = f"{name}, {ROLE_LABELS[role]}"
        credits.append(credit)
    return credits


def build(cache):
    if hashlib.sha256(cache.read_bytes()).hexdigest() != SHA256:
        raise ValueError(f"Source checksum mismatch: {cache}")
    csv.field_size_limit(20_000_000)
    groups = defaultdict(lambda: defaultdict(list))
    seen = set()
    with cache.open(encoding="utf-8-sig", newline="") as stream:
        for row in csv.DictReader(stream):
            if row["type"] not in TYPES or row["share_license_status"] != "CC0":
                continue
            row = {field: clean(row[field]) for field in
                   ("id", "title", "type", "creation_date", "culture", "technique", "creators", "url")}
            if any(not row[field] or row[field] in {"[]", "None", "?"}
                   for field in ("title", "creation_date", "culture", "technique")):
                continue
            if re.search(r"\b(?:forgery|fake|fragment)\b", row["title"], re.I):
                continue
            signature = tuple(row[k].casefold() for k in
                              ("title", "creation_date", "culture", "technique", "creators"))
            if signature in seen:
                continue
            seen.add(signature)
            groups[row["type"]][row["culture"].split(",")[0]].append(row)
    files = {}
    for kind, branch in TYPES.items():
        queues = {}
        for culture, rows in groups[kind].items():
            rows.sort(key=lambda row: hashlib.sha256(row["id"].encode()).digest())
            queues[culture] = deque(rows)
        cultures = sorted(queues, key=lambda culture: hashlib.sha256(culture.encode()).digest())
        selected = []
        while cultures and len(selected) < CAP:
            next_round = []
            for culture in cultures:
                if len(selected) == CAP:
                    break
                selected.append(queues[culture].popleft())
                if queues[culture]:
                    next_round.append(culture)
            cultures = next_round
        if len(selected) != CAP:
            raise ValueError(f"Insufficient eligible {kind} records")
        records = []
        for row in selected:
            lines = [f"{row['title']} — {row['creation_date']}"]
            lines += creator_credits(row["creators"])
            culture = row["culture"]
            correction = ""
            if row["id"] == "106525":
                if culture != "Itay, 16th century":
                    raise ValueError("Unexpected culture for CMA work 106525")
                culture = "Italy, 16th century"
                correction = (" Editorial spelling correction: source culture 'Itay, 16th century' "
                              "rendered 'Italy, 16th century'; the museum page identifies its "
                              "collection as Italian Drawings. Original catalogue field retained here.")
            lines += [row["technique"], f"Culture - {culture}"]
            records.append({
                "id": "cma-work-" + row["id"], "text": "\n".join(lines),
                "provenance": f"Cleveland Museum of Art Open Access catalogue, revision {REVISION}; "
                              f"{row['url']}; {URL}; SHA-256 {SHA256}. CC0 1.0 metadata, "
                              f"source type {kind}, source share_license_status CC0. "
                              "Catalogue title, date, creators, technique and culture; HTML/whitespace cleaned. "
                              "Semicolon-delimited creator credits use separate lines; missing None roles "
                              "omitted and predicate roles rendered as equivalent nouns, without inferring roles. "
                              "Culture-balanced bounded selection; no image, essay, attribution "
                              "inference or ownership history imported." + correction,
            })
        files[ROOT / branch / "works/records.json"] = json.dumps(records, ensure_ascii=False, indent=2) + "\n"
    return files


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cache", type=Path, default=CACHE)
    parser.add_argument("--download", action="store_true")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    if not args.cache.exists() and args.download:
        args.cache.parent.mkdir(parents=True, exist_ok=True)
        with urllib.request.urlopen(URL, timeout=120) as response:
            args.cache.write_bytes(response.read())
    for path, text in build(args.cache).items():
        if args.check:
            if not path.exists() or path.read_text(encoding="utf-8") != text:
                raise ValueError(f"Import differs: {path}")
        else:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text, encoding="utf-8", newline="\n")
        print(f"{path.parent.parent.name}: {CAP}")


if __name__ == "__main__":
    main()
