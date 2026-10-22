"""Reproduce botanical records from checksum-pinned USDA PLANTS profiles."""

import argparse
from collections import defaultdict
import hashlib
import html
import json
from pathlib import Path
import re
import urllib.request

ROOT = Path(__file__).resolve().parent
DATE = "2026-10-13"
REGIONS = {
    "L48": "Lower 48 U.S. states", "AK": "Alaska", "HI": "Hawaii",
    "CAN": "Canada", "PR": "Puerto Rico", "VI": "U.S. Virgin Islands",
    "NA": "North America", "GL": "Greenland", "SPM": "St. Pierre and Miquelon",
    "NAV": "Navassa Island", "PB": "Pacific Basin excluding Hawaii",
}
STATUSES = {
    "N": "native", "N?": "probably native", "I": "introduced",
    "I?": "probably introduced", "GP": "garden persistent",
    "GP?": "probably garden persistent", "W": "waif", "W?": "probably a waif",
}
SPORE_PATHS = {
    "Fern": "ferns", "Lycopod": "lycophytes", "Quillwort": "lycophytes",
    "Horsetail": "horsetails", "Whisk-fern": "whisk-ferns",
}
NONVASCULAR_PATHS = {"Moss": "mosses", "Liverwort": "liverworts", "Hornwort": "hornworts"}
HABITS = {"Forb/herb": "herbaceous", "Graminoid": "grass-like", "Tree": "tree",
          "Shrub": "shrub", "Subshrub": "subshrub", "Vine": "vine", "Nonvascular": "nonvascular"}


def scientific_name(value):
    match = re.search(r"<i>([^<]+)</i>", value or "")
    if not match:
        raise ValueError(f"Missing scientific-name markup: {value!r}")
    return html.unescape(match.group(1)).strip()


def classify(profile):
    group = profile["Group"]
    if group in NONVASCULAR_PATHS:
        return "nonvascular/" + NONVASCULAR_PATHS[group]
    if group in SPORE_PATHS:
        return "vascular/spore-plants/" + SPORE_PATHS[group]
    if group == "Gymnosperm":
        return "vascular/gymnosperms"
    if group not in {"Monocot", "Dicot"}:
        return None
    habits = set(profile["GrowthHabits"])
    # These are browsing groups, not mutually exclusive biological claims.
    # All source habits remain visible; vines win over woody habits.
    if "Vine" in habits:
        suffix = "climbers"
    elif "Graminoid" in habits:
        suffix = "graminoids"
    elif habits & {"Tree", "Shrub", "Subshrub"}:
        suffix = "woody"
    elif "Forb/herb" in habits:
        suffix = "forbs"
    else:
        return None
    return "vascular/flowering-plants/" + suffix


def record(profile, source):
    symbol = profile["Symbol"]
    if symbol != source["symbol"] or profile["Rank"] != "Species" or profile["AcceptedId"] != profile["Id"]:
        raise ValueError(f"Unexpected identity or taxonomic rank: {symbol}")
    name = scientific_name(profile["ScientificName"])
    if not re.fullmatch(r"[A-Z][a-z-]+ [a-z][a-z-]+", name):
        raise ValueError(f"Not a binomial species: {name}")
    families = [scientific_name(a["ScientificName"]) for a in profile["Ancestors"] if a["Rank"] == "Family"]
    if len(families) != 1:
        raise ValueError(f"Expected one botanical family: {symbol}: {families}")
    common = html.unescape(profile["CommonName"] or "").strip()
    group = {"Dicot": "flowering plant", "Monocot": "flowering plant",
             "Gymnosperm": "gymnosperm", "Fern": "fern", "Lycopod": "lycophyte",
             "Quillwort": "quillwort", "Horsetail": "horsetail", "Whisk-fern": "whisk fern",
             "Moss": "moss", "Liverwort": "liverwort", "Hornwort": "hornwort"}[profile["Group"]]
    generic_labels = {group, *(HABITS[h] for h in profile["GrowthHabits"])}
    heading = f"{name} ({common})" if common and common.lower() not in generic_labels else name
    detail = [heading, f"Family: {families[0]} ({group})"]
    habits = profile["GrowthHabits"]
    if habits and habits != ["Nonvascular"]:
        detail.append("Growth forms: " + ", ".join(HABITS[h] for h in habits))
    durations = profile["Durations"]
    if durations:
        if not set(durations) <= {"Annual", "Biennial", "Perennial"}:
            raise ValueError(f"Unknown lifecycle: {symbol}")
        label = "Life cycle: " if len(durations) == 1 else "Life cycles: "
        detail.append(label + ", ".join(d.lower() for d in durations))
    statuses = profile["NativeStatuses"]
    if statuses:
        statuses = sorted(statuses, key=lambda s: list(REGIONS).index(s["Region"]))
        # Two explicitly scoped source jurisdictions provide useful detail
        # without turning the stimulus into an exhaustive range list.
        detail.extend(
            f"{REGIONS[s['Region']]} — {STATUSES[s['Status']]}" for s in statuses[:2])
    return {
        "id": "usda-plants-" + symbol.lower(),
        "text": "\n".join(detail),
        "provenance": f"USDA NRCS PLANTS profile {symbol}; {source['url']}; downloaded {DATE}; input SHA256 {source['sha256']}. Basic factual data are not copyrighted and free for any use; PLANTS Help, Conditions of Image and Data Use. Source taxonomy and growth forms retained; no images or guide prose copied.",
    }


def load_profile(source, cache, download):
    target = cache / "usda-profiles" / (source["symbol"] + ".json")
    if not target.exists():
        if not download:
            raise ValueError(f"Missing cached profile: {target}; use --download")
        target.parent.mkdir(parents=True, exist_ok=True)
        payload = urllib.request.urlopen(source["url"], timeout=60).read()
        if hashlib.sha256(payload).hexdigest() != source["sha256"]:
            raise ValueError(f"Live source changed for {source['symbol']}; pinned bytes required")
        target.write_bytes(payload)
    payload = target.read_bytes()
    if hashlib.sha256(payload).hexdigest() != source["sha256"]:
        raise ValueError(f"Checksum mismatch: {target}")
    return json.loads(payload)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cache", type=Path, required=True)
    parser.add_argument("--download", action="store_true")
    parser.add_argument("--check", action="store_true", help="Validate committed output without rewriting")
    args = parser.parse_args()
    manifest = json.loads((ROOT / "sources.json").read_text(encoding="utf-8"))
    datasets = defaultdict(list)
    seen = set()
    for source in manifest["profiles"]:
        profile = load_profile(source, args.cache, args.download)
        path = classify(profile)
        if path is None:
            raise ValueError(f"Unclassified pinned plant: {source['symbol']}")
        item = record(profile, source)
        if item["id"] in seen or not all(item.values()) or "\0" in item["text"]:
            raise ValueError(f"Invalid record: {item['id']}")
        seen.add(item["id"])
        datasets[path].append(item)
    expected_files = set()
    counts = defaultdict(int)
    for path, records in sorted(datasets.items()):
        target = ROOT / path / "records.json"
        expected_files.add(target)
        output = json.dumps(sorted(records, key=lambda r: r["id"]), indent=2, ensure_ascii=False) + "\n"
        if args.check:
            if not target.exists() or target.read_text(encoding="utf-8") != output:
                raise ValueError(f"Reproduction differs: {target}")
        else:
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text(output, encoding="utf-8")
        node = target.parent
        while node != ROOT:
            counts[node] += len(records)
            node = node.parent
    for node, count in counts.items():
        target = node / "node.toml"
        output = f"weight = {count}\n"
        if args.check:
            if not target.exists() or target.read_text(encoding="utf-8") != output:
                raise ValueError(f"Record count weight differs: {target}")
        else:
            target.write_text(output, encoding="utf-8")
    if set(ROOT.rglob("records.json")) != expected_files:
        raise ValueError("Unexpected botanical dataset files remain")
    print(json.dumps({"records": len(seen), "leaves": {k: len(v) for k, v in sorted(datasets.items())}}, indent=2))


if __name__ == "__main__":
    main()
