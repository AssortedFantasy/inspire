"""Refresh a bounded USDA cache; normal reproduction uses import_data.py pins."""

import argparse
from collections import Counter, defaultdict
from concurrent.futures import ThreadPoolExecutor
import csv
import hashlib
import json
from pathlib import Path
import re
import urllib.request

from import_data import ROOT, classify, record

CHECKLIST_URL = "https://plants.sc.egov.usda.gov/DocumentLibrary/Txt/plantlst.txt"


BASE = "https://plantsservices.sc.egov.usda.gov/api/PlantProfile?symbol="
SPORE_FAMILIES = {
    "Anthocerotaceae", "Notothyladaceae", "Marchantiaceae", "Jungermanniaceae",
    "Porellaceae", "Ricciaceae", "Equisetaceae", "Psilotaceae", "Lycopodiaceae",
    "Selaginellaceae", "Isoetaceae",
}


def select(checklist):
    families = defaultdict(list)
    for row in csv.DictReader(checklist.open(encoding="utf-8-sig")):
        name = row["Scientific Name with Author"]
        tokens = name.split()
        if (row["Synonym Symbol"] or (not row["Common Name"] and row["Family"] not in SPORE_FAMILIES) or len(tokens) < 2
                or not re.fullmatch(r"[a-z][a-z-]+", tokens[1])
                or re.search(r"\b(?:var|subsp|ssp|f)\.", name) or "×" in name):
            continue
        families[row["Family"]].append(row)
    selected = []
    for family, rows in sorted(families.items()):
        genus_counts = Counter()
        for row in sorted(rows, key=lambda r: hashlib.sha256(r["Symbol"].encode()).hexdigest()):
            genus = row["Scientific Name with Author"].split()[0]
            genus_limit = 12 if family in SPORE_FAMILIES else 1
            if genus_counts[genus] >= genus_limit:
                continue
            selected.append(row["Symbol"])
            genus_counts[genus] += 1
            # Extra grass-like genera prevent a two-family sample from hiding
            # the broad diversity of grasses, sedges, and rushes.
            limit = 12 if family in SPORE_FAMILIES else {"Poaceae": 40, "Cyperaceae": 20, "Juncaceae": 10}.get(family, 2)
            if sum(genus_counts.values()) >= limit:
                break
    return sorted(set(selected))


def fetch(symbol, cache):
    target = cache / (symbol + ".json")
    url = BASE + symbol
    if target.exists():
        data = target.read_bytes()
    else:
        data = urllib.request.urlopen(url, timeout=50).read()
        parsed = json.loads(data)
        assert parsed["Symbol"] == symbol, (symbol, parsed.get("Symbol"))
        target.write_bytes(data)
    return {"url": url, "sha256": hashlib.sha256(data).hexdigest(), "symbol": symbol}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--cache", type=Path, required=True)
    args = parser.parse_args()
    args.cache.mkdir(parents=True, exist_ok=True)
    checklist = args.cache / "usda-checklist.txt"
    if not checklist.exists():
        checklist.write_bytes(urllib.request.urlopen(CHECKLIST_URL, timeout=60).read())
    cache = args.cache / "usda-profiles"
    cache.mkdir(exist_ok=True)
    symbols = select(checklist)
    print(f"Downloading {len(symbols)} profiles", flush=True)
    with ThreadPoolExecutor(max_workers=4) as pool:
        manifest = list(pool.map(lambda s: fetch(s, cache), symbols))
    (args.cache / "plant-profile-sources.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    retained = []
    excluded = Counter()
    for source in manifest:
        profile = json.loads((cache / (source["symbol"] + ".json")).read_text(encoding="utf-8"))
        if classify(profile) is None:
            reason = profile["Group"] + ": no mapped growth form" if profile["Group"] in {"Monocot", "Dicot"} else profile["Group"]
            excluded[reason] += 1
        else:
            record(profile, source)
            retained.append(source)
    pinned = {
        "source": "USDA NRCS PLANTS", "downloaded": "2026-10-13",
        "checklist": {"url": CHECKLIST_URL, "sha256": hashlib.sha256(checklist.read_bytes()).hexdigest()},
        "terms": "https://plants.usda.gov/assets/docs/PLANTS_Help_Document_2022.pdf",
        "excluded_profile_counts": dict(excluded), "profiles": retained,
    }
    (ROOT / "sources.json").write_text(json.dumps(pinned, indent=2) + "\n", encoding="utf-8")
    groups = Counter(json.loads(p.read_text(encoding="utf-8"))["Group"] for p in cache.glob("*.json"))
    print(dict(groups), flush=True)


if __name__ == "__main__":
    main()
