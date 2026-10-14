"""Import named animal species from the CC BY 4.0 GBIF Backbone snapshot.

Raw responses live in a temporary cache, never in the sampling tree. Every
response is pinned by SHA-256 in the leaf's sources.json. --replay reads only
those exact cached responses and rejects changed bytes.
"""

import argparse
import gzip
import hashlib
import json
import re
import tempfile
import time
import urllib.parse
import urllib.request
from collections import Counter, defaultdict
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

DATASET = "d7dddbf4-2cf0-4f39-9b2a-bb099caae36c"
API = "https://api.gbif.org/v1/"
CACHE = Path(tempfile.gettempdir()) / "inspire-animal-taxonomy"


def retrieve(url, sources, replay=False):
    CACHE.mkdir(parents=True, exist_ok=True)
    target = CACHE / (hashlib.sha256(url.encode()).hexdigest() + ".json")
    expected = sources.get(url, {}).get("sha256")
    if replay and not expected:
        raise ValueError(f"Input URL is absent from the pinned manifest: {url}")
    if replay and not target.exists():
        raise ValueError(f"Missing pinned input: {target}")
    if not target.exists():
        for attempt in range(4):
            try:
                request = urllib.request.Request(url, headers={"User-Agent": "Inspire-offline-corpus-import/1.0", "Accept-Encoding": "gzip"})
                with urllib.request.urlopen(request, timeout=90) as response:
                    content = response.read()
                    if response.headers.get("Content-Encoding", "").lower() == "gzip":
                        content = gzip.decompress(content)
                target.write_bytes(content)
                break
            except Exception:
                if attempt == 3:
                    raise
                time.sleep(2 ** attempt)
    content = target.read_bytes()
    digest = hashlib.sha256(content).hexdigest()
    if expected and digest != expected:
        raise ValueError(f"Pinned input changed: {url}")
    sources[url] = {"url": url, "sha256": digest, "bytes": len(content), "cache_file": target.name}
    return json.loads(content)


def common_names(row):
    names = []
    latin_parts = set(row.get("canonicalName", "").casefold().split())
    latin_parts.update(row.get("basionym", "").casefold().split()[:2])
    for value in row.get("vernacularNames", []):
        name = " ".join(value.get("vernacularName", "").split()).rstrip(".")
        if value.get("language") != "eng" or not name or len(name) > 100:
            continue
        if name == row.get("canonicalName") or re.search(r"[0-9<>\[\]{}]", name):
            continue
        if (len(name) < 3 and name.casefold() != "ox") or (name.isupper() and len(name) < 7):
            continue
        # Reject recognizable isolated scientific-name components, while keeping
        # legitimate lowercase common names such as wombat, lobster and emu.
        if len(name.split()) == 1 and name.casefold() in latin_parts:
            continue
        names.append(name)
    counts = Counter(names)
    return sorted(counts, key=lambda name: (-counts[name], name.casefold()))


def make_record(row):
    names = common_names(row)
    if not names:
        return None
    canonical = row.get("canonicalName", "")
    if row.get("rank") != "SPECIES" or row.get("taxonomicStatus") != "ACCEPTED":
        return None
    if row.get("extinct") or row.get("kingdom") != "Animalia" or len(canonical.split()) != 2:
        return None
    lineage = [(rank, row[rank]) for rank in ("class", "order", "family") if row.get(rank)]
    if not row.get("family") or len(lineage) < 2:
        return None
    text = f"{names[0]} ({canonical}). "
    text += "; ".join(f"{rank.capitalize()}: {value}" for rank, value in lineage) + "."
    habitats = sorted(set(row.get("habitats", [])) - {"UNKNOWN"})
    if habitats:
        text += " Recorded habitat categories: " + ", ".join(h.lower().replace("_", " ") for h in habitats) + "."
    return {"id": f"gbif-{row['key']}", "text": text, "provenance":
            f"GBIF Secretariat: GBIF Backbone Taxonomy, dataset {DATASET}; "
            f"https://www.gbif.org/species/{row['key']}; snapshot retrieved 2026-10-13. "
            "CC BY 4.0, https://creativecommons.org/licenses/by/4.0/. "
            "Selected factual nomenclature and classification; formatted and filtered by Inspire. "
            "Taxonomy is the pinned backbone snapshot, not a claim of current consensus. "
            "Exact API inputs and SHA-256 hashes are in the adjacent sources.json."}


def run():
    global CACHE
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--taxon", action="append", required=True)
    parser.add_argument("--taxon-key", action="append", type=int)
    parser.add_argument("--rank", choices=("CLASS", "ORDER", "PHYLUM", "FAMILY"))
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--query", action="append", default=[])
    parser.add_argument("--pages", type=int, default=20)
    parser.add_argument("--per-family", type=int, default=40)
    parser.add_argument("--replay", action="store_true")
    parser.add_argument("--cache-dir", type=Path)
    args = parser.parse_args()
    if args.cache_dir:
        CACHE = args.cache_dir
    tree = next(parent for parent in Path(__file__).resolve().parents if parent.name == "families")
    if CACHE.resolve().is_relative_to(tree):
        parser.error("Keep raw input caches outside the families sampling tree")
    if args.pages < 1 or args.per_family < 1:
        parser.error("--pages and --per-family must be positive")
    if args.taxon_key and len(args.taxon_key) != len(args.taxon):
        parser.error("Supply one --taxon-key for each --taxon")
    previous_path = args.output / "records.json"
    previous = json.loads(previous_path.read_text(encoding="utf-8")) if previous_path.exists() else []
    retained_names = {record["text"].split(" (")[0].strip().casefold() for record in previous
                      if not record["id"].startswith("gbif-")}
    manifest_path = args.output / "sources.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8")) if manifest_path.exists() else {}
    retained_names.update(manifest.get("retained_seed_names", []))
    sources = {entry["url"]: entry for entry in manifest.get("inputs", [])}
    rows = {}
    retrievals = []
    for index, taxon in enumerate(args.taxon):
        if args.taxon_key:
            key = args.taxon_key[index]
            match = retrieve(API + "species/" + str(key), sources, args.replay)
            if (match.get("canonicalName", "").casefold() != taxon.casefold()
                    or match.get("kingdom") != "Animalia"
                    or match.get("taxonomicStatus") != "ACCEPTED"
                    or match.get("deleted")
                    or (args.rank and match.get("rank") != args.rank)):
                raise ValueError(f"Explicit source key failed exact accepted animal validation: {taxon}: {match}")
        else:
            match_params = {"name": taxon, "strict": "true"}
            if args.rank:
                match_params.update(rank=args.rank, kingdom="Animalia")
            match_url = API + "species/match?" + urllib.parse.urlencode(match_params)
            match = retrieve(match_url, sources, args.replay)
            if match.get("matchType") != "EXACT" or match.get("kingdom") != "Animalia":
                raise ValueError(f"Taxon failed exact animal match: {taxon}: {match}")
            key = match["usageKey"]
        for query in args.query or [""]:
            fetched = 0
            def page_url(page):
                params = {"datasetKey": DATASET, "highertaxonKey": key, "rank": "SPECIES",
                          "status": "ACCEPTED", "limit": 1000, "offset": page * 1000}
                if query:
                    params.update(q=query, qField="VERNACULAR")
                return API + "species/search?" + urllib.parse.urlencode(params)
            first = retrieve(page_url(0), sources, args.replay)
            pages = min(args.pages, max(1, (first.get("count", 0) + 999) // 1000))
            def read_page(page):
                return retrieve(page_url(page), sources, args.replay)
            # Independent, checksum-pinned page responses can be fetched together;
            # process them in page order for deterministic selection and manifests.
            with ThreadPoolExecutor(max_workers=4) as pool:
                responses = [first, *pool.map(read_page, range(1, pages))]
            for response in responses:
                fetched += len(response["results"])
                for row in response["results"]:
                    if row.get("kingdom") == "Animalia":
                        rows[row["key"]] = row
                if response.get("endOfRecords"):
                    break
            retrievals.append({"taxon": taxon, "key": key, "query": query, "available": response.get("count"), "fetched": fetched})
    grouped = defaultdict(list)
    for row in rows.values():
        record = make_record(row)
        if record:
            grouped[row["family"]].append(record)
    records = []
    for family in sorted(grouped):
        # Hash ordering spreads coverage through each family without an alphabetic
        # or API-ranking bias. The cap prevents huge families crowding out others.
        candidates = sorted(grouped[family], key=lambda row: hashlib.sha256(row["id"].encode()).hexdigest())
        selected = {record["id"]: record for record in candidates[:args.per_family]}
        for record in candidates:
            source_row = rows[int(record["id"].removeprefix("gbif-"))]
            if retained_names.intersection(name.casefold() for name in common_names(source_row)):
                selected[record["id"]] = record
        records.extend(selected.values())
    records.sort(key=lambda row: row["id"])
    if not records:
        raise ValueError("No usable English-named accepted species not flagged extinct")
    args.output.mkdir(parents=True, exist_ok=True)
    (args.output / "records.json").write_text(json.dumps(records, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    (args.output / "node.toml").write_text(f"weight = {len(records)}\n", encoding="utf-8")
    manifest = {"dataset": DATASET, "dataset_url": "https://www.gbif.org/dataset/" + DATASET,
                "license": "CC BY 4.0", "retrieved": "2026-10-13", "taxa": args.taxon,
                "queries": args.query, "max_pages": args.pages, "per_family_cap": args.per_family,
                "retained_seed_names": sorted(retained_names),
                "retrievals": retrievals, "record_count": len(records),
                "inputs": sorted(sources.values(), key=lambda entry: entry["url"])}
    if args.rank:
        manifest["taxon_rank"] = args.rank
    if args.taxon_key:
        manifest["taxon_keys"] = args.taxon_key
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"output": str(args.output), "records": len(records), "families": len(grouped), "retrievals": retrievals}))


if __name__ == "__main__":
    run()
