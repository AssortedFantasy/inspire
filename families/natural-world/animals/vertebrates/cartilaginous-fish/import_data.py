"""Import exact-name cartilaginous fish with compiled species-level life histories.

Run from the repository root; --replay verifies every pinned cached input first.
"""

import argparse
import csv
import hashlib
import importlib.util
import io
import json
import math
import os
import subprocess
import sys
import urllib.parse
from collections import Counter, defaultdict
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[4]
CACHE = ROOT / "target/import-cache/cartilaginous-fish"
SOURCE = "https://doi.org/10.6084/m9.figshare.3517361.v1"
API = "https://api.gbif.org/v1/"
DATASET = "d7dddbf4-2cf0-4f39-9b2a-bb099caae36c"
TAXA = {"Elasmobranchii": 121, "Holocephali": 120}
PLAUSIBILITY_SOURCE = "https://www.ices.dk/sites/pub/CM%20Doccuments/2001/J/J1101.pdf"
COLS = {"Maximum weight (kg)": ("maximum weight", "kg", 0, 100000),
        "interbirth interval (years)": ("interbirth interval", "years", 0, 100),
        "age at maturity (years)": ("age at maturity", "years", 0, 200)}
PREFERRED_NAMES = {"Leucoraja erinacea": "little skate", "Dalatias licha": "kitefin shark",
                   "Trygonoptera mucosa": "Western Stingaree",
                   "Scyliorhinus canicula": "small-spotted catshark"}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def run():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--replay", action="store_true")
    args = parser.parse_args()
    CACHE.mkdir(parents=True, exist_ok=True)
    previous_path = HERE / "sources.json"
    previous = json.loads(previous_path.read_text()) if previous_path.exists() else {}
    sources = {x["url"]: x for x in previous.get("inputs", [])}
    # Check the complete input set before generating or mutating any output.
    if args.replay:
        if not sources:
            raise ValueError("No pinned source manifest")
        for item in sources.values():
            if digest((CACHE / item["cache_file"]).read_bytes()) != item["sha256"]:
                raise ValueError(f"Pinned source changed: {item['url']}")

    def fetch(url, filename=None):
        old = sources.get(url)
        filename = old["cache_file"] if old else filename or digest(url.encode()) + ".json"
        path = CACHE / filename
        if args.replay and not old:
            raise ValueError(f"Unpinned source: {url}")
        if not path.exists():
            if args.replay:
                raise ValueError(f"Missing pinned source: {url}")
            env = {**os.environ, "INSPIRE_SOURCE_URL": url, "INSPIRE_SOURCE_DEST": str(path)}
            subprocess.run(["pwsh", "-NoProfile", "-Command",
                            "Invoke-WebRequest -Uri $env:INSPIRE_SOURCE_URL -OutFile $env:INSPIRE_SOURCE_DEST"],
                           env=env, check=True, timeout=100)
        data = path.read_bytes()
        if old and digest(data) != old["sha256"]:
            raise ValueError(f"Pinned source changed: {url}")
        sources[url] = {"url": url, "cache_file": filename, "sha256": digest(data), "bytes": len(data)}
        return data

    metadata = json.loads(fetch("https://api.figshare.com/v2/articles/3517361", "lifehistory.json"))
    if metadata["license"]["name"] != "CC0" or metadata["version"] != 1:
        raise ValueError("Unexpected source version or license")
    if "Maximum weight (kg)" not in metadata["description"] or "Age at maturity (yr)" not in metadata["description"]:
        raise ValueError("Source trait definitions changed")
    raw = fetch("https://ndownloader.figshare.com/files/5585351", "lifehistory.tsv")
    file = next(x for x in metadata["files"] if x["id"] == 5585351)
    if hashlib.md5(raw).hexdigest() != file["computed_md5"]:
        raise ValueError("Publisher checksum mismatch")
    if not fetch(PLAUSIBILITY_SOURCE, "ices-deepwater-lifehistory.pdf").startswith(b"%PDF"):
        raise ValueError("Primary deepwater-life-history review changed format")
    traits = defaultdict(list)
    for row in csv.DictReader(io.StringIO(raw.decode("utf-8")), delimiter="\t"):
        if row["class"] == "Chondrichthyes":
            traits[row["species"].replace("_", " ")].append(row)
    spec = importlib.util.spec_from_file_location("animal_import", HERE.parents[1] / "import_data.py")
    helper = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(helper)
    taxonomy_metadata = json.loads(fetch(API + "dataset/" + DATASET))
    if taxonomy_metadata.get("license") != "http://creativecommons.org/licenses/by/4.0/legalcode":
        raise ValueError("Unexpected GBIF taxonomy license")
    rows = {}
    retrievals = []
    for taxon, key in TAXA.items():
        detail = json.loads(fetch(API + f"species/{key}"))
        if (detail.get("canonicalName") != taxon or detail.get("rank") != "CLASS"
                or detail.get("taxonomicStatus") != "ACCEPTED" or detail.get("deleted")
                or detail.get("kingdom") != "Animalia"):
            raise ValueError(f"Unaccepted taxon: {taxon}")
        def url(page):
            return API + "species/search?" + urllib.parse.urlencode({"datasetKey": DATASET,
                "highertaxonKey": key, "rank": "SPECIES", "status": "ACCEPTED",
                "limit": 1000, "offset": page * 1000})
        first = json.loads(fetch(url(0)))
        pages = min(5, max(1, (first["count"] + 999) // 1000))
        with ThreadPoolExecutor(max_workers=4) as pool:
            responses = [first, *pool.map(lambda page: json.loads(fetch(url(page))), range(1, pages))]
        for response in responses:
            for row in response["results"]:
                if (row.get("classKey") == key and not row.get("deleted")
                        and row.get("canonicalName") in traits):
                    rows[row["key"]] = row
        retrievals.append({"taxon": taxon, "key": key, "pages": pages,
                           "fetched": sum(len(x["results"]) for x in responses), "available": first["count"]})
    candidates = defaultdict(list)
    rejected = Counter()
    for row in rows.values():
        record = helper.make_record(row)
        if not record:
            rejected["unnamed_or_unaccepted"] += 1
            continue
        preferred = PREFERRED_NAMES.get(row["canonicalName"])
        if preferred:
            if preferred not in helper.common_names(row):
                raise ValueError(f"Preferred common name absent from source: {preferred}")
            record["text"] = preferred + " (" + record["text"].split(" (", 1)[1]
        summaries = []
        for column, (label, unit, lower, upper) in COLS.items():
            values = {x[column] for x in traits[row["canonicalName"]] if x[column] != "NA"}
            # Repeated species rows sometimes disagree. Retain only unambiguous
            # published values; do not choose one, average, or invent a range.
            if len(values) != 1:
                rejected["missing_or_conflicting_" + label] += 1
                continue
            value = next(iter(values))
            number = float(value)
            if not math.isfinite(number) or not lower < number <= upper:
                raise ValueError(f"Implausible source value: {row['canonicalName']} {column}={value}")
            unit = "year" if unit == "years" and number == 1 else unit
            summaries.append(f"{label} {value} {unit}")
        if not summaries:
            rejected["no_unambiguous_traits"] += 1
            continue
        record["text"] += " Compiled species-level life-history values: " + "; ".join(summaries) + "."
        record["text"] += " These are compiled species-level values; population and sex were not supplied."
        record["provenance"] += " Exact-binomial joined to Hutchings, Myers, Garcia, Lucifora and Kuparinen, 2012, supplement 1, " + SOURCE + "; CC0, https://creativecommons.org/publicdomain/zero/1.0/. Only unambiguous repeated-row values are emitted; no imputation or synonym matching."
        candidates[row["family"]].append((len(summaries), record))
    records = []
    for family in sorted(candidates):
        selected = sorted(candidates[family], key=lambda x: (-x[0], digest(x[1]["id"].encode())))[:3]
        records.extend(x[1] for x in selected)
    records.sort(key=lambda x: x["id"])
    if not records or len({x["id"] for x in records}) != len(records):
        raise ValueError("Invalid record set")
    manifest = {"dataset": DATASET, "trait_source": SOURCE, "trait_license": "CC0",
        "taxonomy_license": "CC BY 4.0", "retrieved": "2026-10-13", "per_family_cap": 3,
        "record_count": len(records), "family_count": len(candidates), "retrievals": retrievals,
        "join": "exact accepted canonical binomials; trait-matched species only",
        "selection": "prefer more unambiguous traits, then stable SHA-256 ordering within family",
        "preferred_common_names": PREFERRED_NAMES,
        "review": "ICES CM 2001/J:11 Table 1 records female leafscale-gulper-shark maturity at 44 years; late maturity is biologically plausible. Figshare's separate compiled 46-year value is retained with source qualification.",
        "rejected_traits_or_rows": dict(sorted(rejected.items())),
        "inputs": sorted(sources.values(), key=lambda x: x["url"])}
    output = {"records.json": json.dumps(records, ensure_ascii=False, indent=2) + "\n",
              "sources.json": json.dumps(manifest, indent=2) + "\n",
              "node.toml": f"weight = {len(records)}\n"}
    for name, text in output.items():
        path = HERE / name
        if args.replay and path.read_text(encoding="utf-8") != text:
            raise ValueError(f"Replay differs: {name}")
    if not args.replay:
        for name, text in output.items():
            (HERE / name).write_text(text, encoding="utf-8")
    print(json.dumps({"records": len(records), "families": len(candidates), "replay": args.replay}))


if __name__ == "__main__":
    run()
