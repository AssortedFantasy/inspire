"""Import a bounded, country-balanced selection of public-domain OurAirports data."""

import argparse
from collections import defaultdict, deque
import csv
import hashlib
import io
import json
from pathlib import Path
import re
import urllib.request


ROOT = Path(__file__).resolve().parent
CACHE = ROOT.parents[1] / "target/imports/ourairports"
REVISION = "b529623d2a0b09a9be78d37fc7a7a5e1e16e3451"
BASE = f"https://raw.githubusercontent.com/davidmegginson/ourairports-data/{REVISION}/"
SOURCES = {
    "airports.csv": "0499dbd88de2fc735ac5c09f59796ccc3af544e17ffda807f793c0c6fe6c3d75",
    "countries.csv": "2a9dbee691125b0cdb8ceb5fe227c48c903f99c488963b8e53e2ab366521c639",
    "runways.csv": "eff294dc5c055e7bb48ef43cb3b564399f2741bbfbba0c18c085aea7b9ab988b",
}
TYPES = {
    "large_airport": "large",
    "medium_airport": "medium",
    "small_airport": "small",
    "heliport": "heliports",
    "seaplane_base": "seaplane-bases",
    "balloonport": "balloonports",
}
CAP = 240


def clean(value):
    return " ".join(value.split())


def integer(value):
    return int(value) if re.fullmatch(r"-?\d+", value) else None


def load(cache, name, download):
    path = cache / name
    if not path.exists() and download:
        cache.mkdir(parents=True, exist_ok=True)
        with urllib.request.urlopen(BASE + name, timeout=120) as response:
            path.write_bytes(response.read())
    data = path.read_bytes()
    if hashlib.sha256(data).hexdigest() != SOURCES[name]:
        raise ValueError(f"Source checksum mismatch: {path}")
    return list(csv.DictReader(io.StringIO(data.decode("utf-8-sig"), newline="")))


def balanced(rows, cap):
    """Take one site per country per round; alternate high, low and hashed sites."""
    buckets = defaultdict(list)
    for row in rows:
        buckets[row["iso_country"]].append(row)
    queues = {}
    for country, sites in buckets.items():
        sites.sort(key=lambda row: hashlib.sha256(row["id"].encode()).digest())
        # Rotate an ordinary site and both extremes, including in partial rounds.
        high = max(sites, key=lambda row: int(row["elevation_ft"]))
        low = min(sites, key=lambda row: int(row["elevation_ft"]))
        candidates = [sites[0], high, low]
        offset = hashlib.sha256(country.encode()).digest()[0] % 3
        first, used = [], set()
        for row in candidates[offset:] + candidates[:offset]:
            if row["id"] not in used:
                first.append(row)
                used.add(row["id"])
        queues[country] = deque(first + [row for row in sites if row["id"] not in used])
    countries = sorted(queues, key=lambda code: hashlib.sha256(code.encode()).digest())
    selected = []
    while countries and len(selected) < cap:
        next_round = []
        for country in countries:
            if len(selected) == cap:
                break
            selected.append(queues[country].popleft())
            if queues[country]:
                next_round.append(country)
        countries = next_round
    return selected


def surface_line(row, airport_type):
    noun = {"heliport": "pad", "seaplane_base": "waterway",
            "balloonport": "launch area"}.get(airport_type, "runway")
    ends = [clean(row[key]) for key in ("le_ident", "he_ident") if clean(row[key])]
    label = "Longest listed " + noun
    if ends:
        label += " " + "/".join(ends)
    length = integer(row["length_ft"])
    width = integer(row["width_ft"])
    dimensions = f"{length:,}"
    if width is not None and width > 0:
        dimensions += f" × {width:,}"
    material = clean(row["surface"])
    return f"{label}: {dimensions} ft" + (f" ({material})" if material else "")


def build(airports, countries, runways):
    country_names = {row["code"]: clean(row["name"]) for row in countries}
    surfaces = defaultdict(list)
    for runway in runways:
        length = integer(runway["length_ft"])
        if runway["closed"] == "0" and length is not None and length > 0:
            surfaces[runway["airport_ref"]].append(runway)
    groups = defaultdict(list)
    ids = set()
    for row in airports:
        if row["id"] in ids:
            raise ValueError(f"Duplicate airport ID: {row['id']}")
        ids.add(row["id"])
        if (row["type"] in TYPES and clean(row["name"])
                and row["iso_country"] in country_names
                and integer(row["elevation_ft"]) is not None
                and not re.search(r"\b(duplicate|duplicated|fictional)\b", row["name"], re.I)):
            groups[row["type"]].append(row)
    files = {}
    counts = {}
    for kind, leaf in TYPES.items():
        records = []
        for row in balanced(groups[kind], CAP):
            codes = []
            for field, label, pattern in (("iata_code", "IATA", r"[A-Z]{3}"),
                                           ("icao_code", "ICAO", r"[A-Z]{4}")):
                if re.fullmatch(pattern, row[field]):
                    codes.append(f"{label} {row[field]}")
            name = clean(row["name"])
            lines = [name + (f" ({', '.join(codes)})" if codes else "")]
            country = country_names[row["iso_country"]]
            town = clean(row["municipality"])
            lines.append(f"Serves {town}, {country}" if town else country)
            lines.append(f"Elevation {int(row['elevation_ft']):,} ft MSL")
            if available := surfaces.get(row["id"]):
                longest = max(available, key=lambda item: (int(item["length_ft"]), -int(item["id"])))
                if longest["airport_ident"] != row["ident"]:
                    raise ValueError(f"Runway join mismatch: {row['ident']}")
                lines.append(surface_line(longest, kind))
            records.append({
                "id": f"ourairports-{row['id']}",
                "text": "\n".join(lines),
                "provenance": (
                    f"OurAirports snapshot {REVISION} (2026-10-16), ID {row['id']}, "
                    f"{kind}; https://ourairports.com/airports/{row['ident']}/; "
                    f"{BASE}airports.csv; public domain: https://ourairports.com/data/. "
                    "Source metadata, country join, longest listed open landing surface. "
                    "Country-balanced bounded selection with high/low elevation contrasts."
                ),
            })
        if not records:
            raise ValueError(f"Empty airport type: {kind}")
        counts[leaf] = len(records)
        path = ROOT / "aviation/airports" / leaf
        files[path / "records.json"] = json.dumps(records, ensure_ascii=False, indent=2) + "\n"
        files[path / "node.toml"] = f"weight = {len(records)}\n"
    return files, counts


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cache", type=Path, default=CACHE)
    parser.add_argument("--download", action="store_true")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    sources = {name: load(args.cache, name, args.download) for name in SOURCES}
    files, counts = build(sources["airports.csv"], sources["countries.csv"], sources["runways.csv"])
    for path, text in files.items():
        if args.check:
            if not path.exists() or path.read_text(encoding="utf-8") != text:
                raise ValueError(f"Import differs: {path}")
        else:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text, encoding="utf-8", newline="\n")
    print(json.dumps(counts, sort_keys=True))


if __name__ == "__main__":
    main()
