"""Attach exact-name AVONET traits and retain the original named bird species.

Run the shared GBIF importer first, then this script. --replay verifies pinned
cached input bytes and requires no network. Requires openpyxl at import time.
"""

import argparse
import hashlib
import importlib.util
import json
import re
import sys
import urllib.parse
import urllib.request
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
CACHE = HERE.parents[4] / "target" / "import-cache" / "birds"
URL = "https://ndownloader.figshare.com/files/34480856"
SHA256 = "eb645e83dddb40f1654a3e8d721998dbca76eff540231b7b809267c1e96f8d3e"
ARTICLE = "https://api.figshare.com/v2/articles/16586228/versions/7"


def main():
    import openpyxl

    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cache", type=Path, default=CACHE)
    parser.add_argument("--replay", action="store_true")
    args = parser.parse_args()
    if args.cache.resolve().is_relative_to(HERE.parents[3]):
        raise ValueError("Raw downloads must remain outside the families sampling tree")
    traits_path = HERE / "traits-sources.json"
    prior = json.loads(traits_path.read_text(encoding="utf-8")) if traits_path.exists() else {}
    args.cache.mkdir(parents=True, exist_ok=True)
    workbook = args.cache / "avonet-v7.xlsx"
    if not workbook.exists():
        if args.replay:
            raise ValueError("Pinned AVONET workbook missing from cache")
        workbook.write_bytes(urllib.request.urlopen(URL, timeout=120).read())
    if hashlib.sha256(workbook.read_bytes()).hexdigest() != SHA256:
        raise ValueError("AVONET workbook checksum mismatch")
    article_path = args.cache / "article.json"
    if not article_path.exists():
        if args.replay:
            raise ValueError("Pinned article metadata missing from cache")
        article_path.write_bytes(urllib.request.urlopen(ARTICLE, timeout=90).read())
    article = json.loads(article_path.read_text(encoding="utf-8"))
    article_digest = hashlib.sha256(article_path.read_bytes()).hexdigest()
    if prior and article_digest != prior["metadata_input"]["sha256"]:
        raise ValueError("Pinned AVONET article metadata checksum mismatch")
    assert article["version"] == 7 and article["license"]["name"] == "CC BY 4.0"

    spec = importlib.util.spec_from_file_location("animal_import", HERE.parents[1] / "import_data.py")
    helper = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(helper)
    manifest = json.loads((HERE / "sources.json").read_text(encoding="utf-8"))
    input_rows = {}
    base_sources = {entry["url"]: entry for entry in manifest["inputs"]}
    for url in base_sources:
        response = helper.retrieve(url, base_sources, replay=True)
        input_rows.update({row["key"]: row for row in response.get("results", [])})
    records = json.loads((HERE / "records.json").read_text(encoding="utf-8"))
    by_id = {record["id"]: record for record in records}
    present = {name.casefold() for key, row in input_rows.items()
               if f"gbif-{key}" in by_id for name in helper.common_names(row)}
    absent = sorted(set(manifest["retained_seed_names"]) - present)
    retention_sources = {entry["url"]: entry for entry in prior.get("retention_inputs", [])}

    def find_original(name):
        params = {"datasetKey": helper.DATASET, "highertaxonKey": 212,
                  "rank": "SPECIES", "status": "ACCEPTED", "limit": 100,
                  "q": name, "qField": "VERNACULAR"}
        url = helper.API + "species/search?" + urllib.parse.urlencode(params)
        response = helper.retrieve(url, retention_sources, replay=args.replay)
        return name, [row for row in response.get("results", [])
                      if name in {value.casefold() for value in helper.common_names(row)}]

    omissions = []
    with ThreadPoolExecutor(max_workers=4) as pool:
        for name, rows in pool.map(find_original, absent):
            usable = [helper.make_record(row) for row in rows]
            usable = [row for row in usable if row]
            if not usable:
                omissions.append(name)
            for record in usable:
                by_id[record["id"]] = record
    records = sorted(by_id.values(), key=lambda record: record["id"])

    book = openpyxl.load_workbook(workbook, read_only=True, data_only=True)
    metadata = {row[0]: {"description": row[1], "units": row[3]}
                for row in book["Metadata"].iter_rows(min_row=2, values_only=True) if row[0]}
    assert metadata["Mass"]["units"] == "gram"
    assert metadata["Wing.Length"]["units"] == metadata["Beak.Length_Culmen"]["units"] == "mm"
    sheet = book["AVONET1_BirdLife"]
    iterator = sheet.iter_rows(values_only=True)
    headers = next(iterator)
    traits = {row[1]: dict(zip(headers, row)) for row in iterator}
    enriched = 0
    inferred_omitted = 0
    unmatched = []
    for record in records:
        # Makes enrichment idempotent without altering the original GBIF clauses.
        record["text"] = record["text"].split(" AVONET species profile:")[0]
        for marker in ("\nMean body mass:", "\nMean beak culmen length:", "\nMean unflattened wing length:", "\nPrimary habitat:", "\nDiet:", "\nMigration:"):
            record["text"] = record["text"].split(marker)[0]
        record["provenance"] = record["provenance"].split(" AVONET traits:")[0]
        name = re.search(r"\(([^()]+)\)$", record["text"].splitlines()[0]).group(1)
        row = traits.get(name)
        if row is None:
            unmatched.append(name)
            continue
        if row["Inference"] != "NO":
            inferred_omitted += 1
            continue
        parts = []
        for key, label, unit in (("Mass", "body mass", "g"),
                                 ("Beak.Length_Culmen", "beak culmen length", "mm"),
                                 ("Wing.Length", "unflattened wing length", "mm")):
            value = row[key]
            if isinstance(value, (float, int)) and value > 0:
                rounded = float(format(value, ".3g"))
                parts.append(f"Mean {label}: {rounded:g} {unit}")
        for key, label in (("Habitat", "Primary habitat"), ("Trophic.Niche", "Diet")):
            if row[key] and row[key] != "NA":
                parts.append(f"{label}: {row[key].lower()}")
        migration = {1: "sedentary", 2: "partially migratory", 3: "migratory"}.get(row["Migration"])
        if migration:
            parts.append(f"Migration: {migration}")
        if parts:
            record["text"] += "\n" + "\n".join(parts)
            record["provenance"] += (
                " AVONET traits: Tobias et al. (2022), Ecology Letters, doi:10.1111/ele.13898; "
                "dataset doi:10.6084/m9.figshare.16586228.v7; CC BY 4.0, "
                "https://creativecommons.org/licenses/by/4.0/. Exact binomial join to "
                "AVONET1_BirdLife; only Inference=NO rows. Measurements are species averages, "
                "not ranges or measurements of a particular individual. Source workbook "
                "and its variable definitions are pinned in traits-sources.json.")
            enriched += 1
    book.close()
    (HERE / "records.json").write_text(json.dumps(records, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    (HERE / "node.toml").write_text(f"weight = {len(records)}\n", encoding="utf-8")
    trait_manifest = {"dataset": "AVONET", "version": 7, "license": article["license"],
                      "citation": "Tobias et al. (2022), Ecology Letters, doi:10.1111/ele.13898",
                      "dataset_doi": article["doi"], "sheet": "AVONET1_BirdLife",
                      "input": {"url": URL, "sha256": SHA256, "bytes": workbook.stat().st_size},
                      "metadata_input": {"url": ARTICLE, "sha256": article_digest},
                      "join": "Exact scientific binomial; Inference=NO; no synonyms or cross-species inference",
                      "record_count": len(records), "trait_profiles": enriched,
                      "inference_rows_omitted": inferred_omitted, "unmatched_binomials": sorted(unmatched),
                      "original_names_omitted": omissions,
                      "variable_definitions": {key: metadata[key] for key in (
                          "Mass", "Beak.Length_Culmen", "Wing.Length", "Habitat", "Trophic.Niche", "Migration", "Inference")},
                      "retention_inputs": sorted(retention_sources.values(), key=lambda entry: entry["url"])}
    traits_path.write_text(json.dumps(trait_manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"records": len(records), "trait_profiles": enriched, "original_names_omitted": omissions,
                      "unmatched": len(unmatched), "inferred_omitted": inferred_omitted}))


if __name__ == "__main__":
    main()
