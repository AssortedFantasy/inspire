"""Import a bounded, diverse set of named insects from pinned GBIF responses."""

import argparse
import hashlib
import importlib.util
import json
import re
import urllib.parse
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("animal_import", HERE.parents[2] / "import_data.py")
animal = importlib.util.module_from_spec(spec)
spec.loader.exec_module(animal)
QUERIES = [
    "beetle", "bee", "wasp", "ant", "fly", "dragonfly", "damselfly",
    "grasshopper", "cricket", "mantis", "butterfly", "moth", "aphid",
    "cicada", "termite", "mayfly", "stonefly", "caddisfly", "earwig", "flea", "bug", "mosquito",
]
# Family-level labels add little beside the scientific classification. These
# exclusions affect selection, never replace a source name with invented text.
GENERIC = {
    "beetle", "wasp", "moth", "fly", "bug", "flea", "mantis", "mayfly",
    "caddisfly", "stonefly", "earwig", "cricket", "grasshopper", "butterfly",
    "parasitoid wasp", "chalcid wasp", "ichneumonid wasp", "megaspilid wasp",
    "fig wasp", "ensign wasp", "longhorn beetle", "bark beetle", "seed bug",
    "burrower bug", "toad bug", "hide beetle", "staph beetle", "microphysid bug",
    "a mammal flea", "a bat flea", "a bird flea", "a flea",
}
# These search rows call a psocid a minute pirate bug and a thrips a leaf-miner
# fly. Neither agrees with the row's own taxonomic order.
EXCLUDED_KEYS = {1031743, 1420243}
COVERAGE = {
    "ants": {"Formicidae"},
    "bees": {"Apidae", "Andrenidae", "Colletidae", "Halictidae", "Megachilidae", "Melittidae"},
    "aphids": {"Aphididae"},
    "cicadas": {"Cicadidae"},
    "damselflies": {"Calopterygidae", "Coenagrionidae", "Lestidae", "Platycnemididae"},
    "mosquitoes": {"Culicidae"},
}


def eligible_name(name):
    return len(name.split()) >= 2 and name.casefold() not in GENERIC


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=HERE)
    parser.add_argument("--cache-dir", type=Path, default=animal.CACHE)
    parser.add_argument("--replay", action="store_true")
    args = parser.parse_args()
    animal.CACHE = args.cache_dir
    if args.cache_dir.resolve().is_relative_to(HERE.parents[4]):
        parser.error("Keep raw input caches outside families")
    seed_bytes = (HERE / "legacy-records.json").read_bytes()
    seeds = json.loads(seed_bytes)
    manifest_path = args.output / "sources.json"
    previous = json.loads(manifest_path.read_text(encoding="utf-8")) if manifest_path.exists() else {}
    if args.replay and hashlib.sha256(seed_bytes).hexdigest() != previous.get("legacy_sha256"):
        raise ValueError("Pinned legacy records changed")
    sources = {entry["url"]: entry for entry in previous.get("inputs", [])}
    source_class = animal.retrieve(animal.API + "species/216", sources, args.replay)
    if not (source_class.get("canonicalName") == "Insecta"
            and source_class.get("kingdom") == "Animalia"
            and source_class.get("rank") == "CLASS"
            and source_class.get("taxonomicStatus") == "ACCEPTED"
            and not source_class.get("deleted")):
        raise ValueError("Legacy class key 216 failed accepted animal validation")
    queries = list(dict.fromkeys(QUERIES + [r["text"].split(" (")[0] for r in seeds]))
    rows = {}
    retrievals = []
    for query in queries:
        params = {"datasetKey": animal.DATASET, "highertaxonKey": 216,
                  "rank": "SPECIES", "status": "ACCEPTED", "limit": 1000,
                  "offset": 0, "q": query, "qField": "VERNACULAR"}
        response = animal.retrieve(animal.API + "species/search?" + urllib.parse.urlencode(params), sources, args.replay)
        if not isinstance(response.get("results"), list):
            raise ValueError("Malformed search response")
        rows.update((row["key"], row) for row in response["results"])
        retrievals.append({"query": query, "available": response.get("count"), "fetched": len(response["results"])})
    records = {}
    selected_names = set()
    selected_taxa = set()
    candidates = []
    for row in rows.values():
        if (row.get("class") != "Insecta" or row.get("classKey") != 216
                or not row.get("order") or row.get("deleted") or row["key"] in EXCLUDED_KEYS):
            continue
        names = [name for name in animal.common_names(row) if eligible_name(name)]
        if not names:
            continue
        # Coarse source habitat categories can describe life stages inconsistently;
        # the emitted object is nomenclature and classification only.
        copy = dict(row, habitats=[], vernacularNames=[{"language": "eng", "vernacularName": names[0]}])
        record = animal.make_record(copy)
        if record:
            candidates.append((row, record))
    candidates.sort(key=lambda pair: hashlib.sha256(pair[1]["id"].encode()).hexdigest())
    # Enrich old taxa only when their scientific or English names match the
    # downloaded source. Otherwise keep the original source-grounded record.
    for seed in seeds:
        name = seed["text"].split(" (")[0].casefold()
        canonical = re.search(r"\(([^)]+)\)", seed["text"]).group(1).casefold()
        match = next(((row, record) for row, record in candidates
                      if row["canonicalName"].casefold() == canonical
                      or name in {n.casefold() for n in animal.common_names(row)}), None)
        row, record = match if match else (None, seed)
        if row:
            source_name = next((n for n in animal.common_names(row) if n.casefold() == name), None)
            if source_name:
                record = animal.make_record(dict(row, habitats=[], vernacularNames=[{"language": "eng", "vernacularName": source_name}]))
        records[record["id"]] = record
        selected_taxa.add(row["canonicalName"].casefold() if row else canonical)
        selected_names.add(record["text"].split(" (")[0].casefold())
    grouped = defaultdict(list)
    for row, record in candidates:
        grouped[row["order"]].append((row, record))
    for order in sorted(grouped):
        families = set()
        for row, record in grouped[order]:
            name = record["text"].split(" (")[0].casefold()
            canonical = row["canonicalName"].casefold()
            if canonical in selected_taxa or name in selected_names or row["family"] in families:
                continue
            records[record["id"]] = record
            selected_names.add(name)
            selected_taxa.add(canonical)
            families.add(row["family"])
            if len(families) == 2:
                break
    # Small, recognizable groups must survive an order-level random selection.
    # This is dataset curation by source family, not authorship of new names.
    for label, family_set in COVERAGE.items():
        if any(row["family"] in family_set and record["id"] in records for row, record in candidates):
            continue
        for row, record in candidates:
            name = record["text"].split(" (")[0].casefold()
            if row["family"] in family_set and name not in selected_names and row["canonicalName"].casefold() not in selected_taxa:
                records[record["id"]] = record
                selected_names.add(name)
                selected_taxa.add(row["canonicalName"].casefold())
                break
        else:
            raise ValueError(f"Missing required insect coverage: {label}")
    output = sorted(records.values(), key=lambda r: r["id"])
    if not output:
        raise ValueError("No usable insect records")
    for record in output:
        if set(record) != {"id", "text", "provenance"} or any(not record[k].strip() or "\0" in record[k] for k in record):
            raise ValueError("Invalid record schema")
    manifest = {
        "dataset": animal.DATASET, "dataset_url": "https://www.gbif.org/dataset/" + animal.DATASET,
        "license": "CC BY 4.0", "retrieved": "2026-10-13", "taxa": ["Insecta"],
        "taxon_rank": "CLASS", "taxon_keys": [216], "queries": queries,
        "max_pages": 1, "additional_families_per_order": 2, "record_count": len(output),
        "required_family_groups": {label: sorted(families) for label, families in COVERAGE.items()},
        "legacy_sha256": hashlib.sha256(seed_bytes).hexdigest(), "excluded_keys": sorted(EXCLUDED_KEYS),
        "selection": "Specific English names; two additional distinct families per order by SHA-256 ID order; required ant/bee/aphid/cicada/damselfly/mosquito family coverage; matched legacy taxa enriched; unmatched legacy taxa retained; habitats omitted",
        "retrievals": retrievals, "inputs": sorted(sources.values(), key=lambda e: e["url"]),
    }
    # Finish all pinned retrieval and validation before changing any payload.
    args.output.mkdir(parents=True, exist_ok=True)
    (args.output / "records.json").write_text(json.dumps(output, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    (args.output / "node.toml").write_text(f"weight = {len(output)}\n", encoding="utf-8")
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"records": len(output), "orders": len(grouped), "legacy_enriched": sum(r["id"].startswith("gbif-") for r in output) - (len(output) - len(seeds))}))


if __name__ == "__main__":
    main()
