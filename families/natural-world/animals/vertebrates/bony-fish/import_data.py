"""Import bounded GBIF fish orders and exact-name FISHMORPH traits.

Run from the repository root; --replay uses only checksum-pinned cached inputs.
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
from pathlib import Path

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[4]
CACHE = ROOT / "target/import-cache/bony-fish"
SOURCE = "https://rodare.hzdr.de/record/1316"
URLS = {
    "fishmorph.json": "https://rodare.hzdr.de/api/records/1316",
    "fishmorph.csv": SOURCE + "/files/FISHMORPH_Database.csv?download=1",
    "fishmorph-readme.csv": SOURCE + "/files/FISHMORPH_Readme.csv?download=1",
    "paddlefish-abstract.xml": "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=pubmed&id=31911255&retmode=xml",
    "fishmorph-paper.pdf": "https://aureletoussaint.github.io/homepage/data/2021_Brosse_GEB.pdf",
}
ORDERS = (
    "Acipenseriformes", "Amiiformes", "Anguilliformes", "Atheriniformes",
    "Batrachoidiformes", "Beloniformes", "Ceratodontiformes", "Characiformes",
    "Clupeiformes", "Coelacanthiformes", "Cypriniformes",
    "Cyprinodontiformes", "Elopiformes", "Esociformes", "Gadiformes",
    "Gasterosteiformes", "Gobiesociformes", "Gonorynchiformes", "Gymnotiformes",
    "Lepisosteiformes", "Lophiiformes", "Mugiliformes", "Myctophiformes",
    "Ophidiiformes", "Osmeriformes", "Osteoglossiformes", "Perciformes",
    "Percopsiformes", "Pleuronectiformes", "Polypteriformes", "Salmoniformes", "Scorpaeniformes",
    "Siluriformes", "Synbranchiformes", "Syngnathiformes", "Tetraodontiformes",
)


def digest(content):
    return hashlib.sha256(content).hexdigest()


def make_image_record(row, t, common):
    """A positive dimensionless ratio associated with the source depiction."""
    name = t["Genus species"].strip()
    trait = "BEl" if t["BEl"] != "NA" else "CPt"
    value = float(t[trait])
    if not math.isfinite(value) or value <= 0:
        raise ValueError(f"Not a positive physical image ratio: {name}: {trait}")
    if trait == "BEl":
        label = "standard length/maximum body depth"
        definition = "BEl = standard length (snout to caudal-fin base) divided by maximum body depth"
    else:
        label = "maximum caudal fin depth/minimum caudal peduncle depth"
        definition = "CPt = maximum caudal-fin depth divided by minimum caudal-peduncle depth"
    illustration = t["Type of illustration"].strip().lower()
    reference = t["Reference"].strip()
    if reference.lower() == "unpublished":
        reference += " (source label; image unidentified)"
    reference_sentence = reference + ("" if reference.endswith(".") else ".")
    text = (f"{common} ({name}). Lateral-view image: {label} = {value:.3g}"
            f" (dimensionless), measured from a source-selected {illustration}.")
    provenance = (
        f"FISHMORPH v1.1, FishBase species code {t['SpecCode']}; https://rodare.hzdr.de/record/1316;"
        f" exact scientific-binomial join for {name}. Source value {trait}={t[trait]}; {definition}."
        f" Illustration type: {illustration}; illustration reference: {reference_sentence}"
        " Formatted to three significant digits; no imputation, behavioral inference, individual identity,"
        " Measurement describes the selected image; it is not an estimate of species variation."
        " Brosse, Charpin, Su, Toussaint, Herrera-R, Tedesco and Villeger (2021),"
        " doi:10.14278/rodare.1316; associated methods doi:10.1111/geb.13395, Figure 1 and"
        " sections 2.1.1, 2.1.2, 2.2.2 and 4;"
        " https://aureletoussaint.github.io/homepage/data/2021_Brosse_GEB.pdf."
        f" Common name and association: GBIF Secretariat, GBIF Backbone Taxonomy, stable key {row['key']};"
        f" https://www.gbif.org/species/{row['key']}; snapshot retrieved 2026-10-13."
        " Both datasets CC BY 4.0; formatted and filtered by Inspire."
        " Exact API inputs are pinned in ../sources.json; trait CSV, units README and methods"
        " are pinned in ../traits-sources.json. Full license inherited from ../../../LICENSE.txt.")
    return {"id": f"fishmorph-{trait.lower()}-{t['SpecCode']}", "text": text, "provenance": provenance}


def run():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--replay", action="store_true")
    args = parser.parse_args()
    CACHE.mkdir(parents=True, exist_ok=True)
    manifest_path = HERE / "traits-sources.json"
    previous = json.loads(manifest_path.read_text()) if manifest_path.exists() else {}
    inputs = {entry["url"]: entry for entry in previous.get("inputs", [])}
    blobs = {}
    for name, url in URLS.items():
        path = CACHE / name
        expected = inputs.get(url, {}).get("sha256")
        if args.replay and (not expected or not path.exists()):
            raise ValueError(f"Missing pinned input: {url}")
        if not path.exists():
            # Windows native TLS trusts the system certificate store. Downloads
            # have no certificate bypass and never execute downloaded content.
            environment = {**os.environ, "INSPIRE_SOURCE_URL": url, "INSPIRE_SOURCE_DEST": str(path)}
            subprocess.run(["pwsh", "-NoProfile", "-Command",
                            "Invoke-WebRequest -Uri $env:INSPIRE_SOURCE_URL -OutFile $env:INSPIRE_SOURCE_DEST"],
                           check=True, env=environment)
        content = path.read_bytes()
        if expected and digest(content) != expected:
            raise ValueError(f"Pinned input changed: {url}")
        inputs[url] = {"url": url, "sha256": digest(content),
                       "bytes": len(content), "cache_file": name}
        blobs[name] = content
    metadata = json.loads(blobs["fishmorph.json"])
    if metadata["metadata"]["license"]["id"] != "CC-BY-4.0" or metadata["metadata"]["version"] != "v1.1":
        raise ValueError("Unexpected FISHMORPH version or license")
    for file in metadata["files"]:
        name = "fishmorph.csv" if file["key"] == "FISHMORPH_Database.csv" else "fishmorph-readme.csv"
        if "md5:" + hashlib.md5(blobs[name]).hexdigest() != file["checksum"]:
            raise ValueError(f"Download differs from publisher checksum: {file['key']}")
    readme = blobs["fishmorph-readme.csv"].decode("cp1252")
    if "Maximum body length in centimetres" not in readme or "photograph or a drawing" not in readme:
        raise ValueError("Source units or measurement basis changed")
    traits = list(csv.DictReader(io.StringIO(blobs["fishmorph.csv"].decode("cp1252")), delimiter=";"))
    if len(traits) != 8342:
        raise ValueError("Unexpected FISHMORPH v1.1 row count")
    extinction_study = blobs["paddlefish-abstract.xml"].decode("utf-8")
    if "should be considered extinct" not in extinction_study:
        raise ValueError("Primary extinction-study evidence changed")
    by_name = defaultdict(list)
    for row in traits:
        by_name[row["Genus species"].strip()].append(row)
    spec = importlib.util.spec_from_file_location("animals_import", HERE.parents[1] / "import_data.py")
    helper = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(helper)
    helper.CACHE = CACHE
    tax_path = HERE / "sources.json"
    old_tax = json.loads(tax_path.read_text()) if tax_path.exists() else {}
    sources = {entry["url"]: entry for entry in old_tax.get("inputs", [])
               if "/species/search?" not in entry["url"]
               or urllib.parse.parse_qs(urllib.parse.urlsplit(entry["url"]).query).get("offset") == ["0"]}
    keys = []
    for order in ORDERS:
        params = urllib.parse.urlencode({"name": order, "strict": "true", "rank": "ORDER", "kingdom": "Animalia"})
        match = helper.retrieve(helper.API + "species/match?" + params, sources, args.replay)
        if (match.get("matchType") != "EXACT" or match.get("status") != "ACCEPTED"
                or match.get("rank") != "ORDER" or match.get("synonym")
                or match.get("canonicalName") != order):
            raise ValueError(f"Order did not match accepted source taxonomy: {order}: {match}")
        key = match["usageKey"]
        source = helper.retrieve(helper.API + "species/" + str(key), sources, args.replay)
        if source.get("deleted") or source.get("taxonomicStatus") != "ACCEPTED":
            raise ValueError(f"Deleted or unaccepted source order: {order}")
        keys.append(key)
    rows = {}
    retrievals = []
    for order, key in zip(ORDERS, keys):
        params = {"datasetKey": helper.DATASET, "highertaxonKey": key, "rank": "SPECIES",
                  "status": "ACCEPTED", "limit": 1000, "offset": 0}
        page = helper.retrieve(helper.API + "species/search?" + urllib.parse.urlencode(params), sources, args.replay)
        retrievals.append({"taxon": order, "key": key, "query": "", "available": page["count"], "fetched": len(page["results"])})
        for row in page["results"]:
            if (row.get("order") == order and row.get("canonicalName") != "Psephurus gladius"
                    and not row.get("deleted") and helper.make_record(row)):
                rows[row["key"]] = row
    tax = {"dataset": helper.DATASET, "dataset_url": "https://www.gbif.org/dataset/" + helper.DATASET,
           "license": "CC BY 4.0", "retrieved": "2026-10-13", "taxa": list(ORDERS),
           "queries": [], "max_pages": 1, "per_family_cap": 3, "retained_seed_names": [],
           "retrievals": retrievals, "taxon_rank": "ORDER", "taxon_keys": keys,
           "inputs": sorted(sources.values(), key=lambda entry: entry["url"])}
    groups = defaultdict(list)
    for row in rows.values():
        groups[row["family"]].append(row)
    records = []
    image_records = []
    trait_count = 0
    length_count = 0
    ratio_counts = Counter()
    omitted_images = []
    for family in sorted(groups):
        candidates = sorted(groups[family], key=lambda row: (
            len(by_name[row["canonicalName"]]) != 1,
            digest(f"gbif-{row['key']}".encode())))
        for row in candidates[:3]:
            record = helper.make_record(row)
            record["provenance"] = record["provenance"].replace("the adjacent sources.json", "../sources.json")
            matching = by_name[row["canonicalName"]]
            if len(matching) == 1:
                t = matching[0]
                size = float(t["MBl"]) if t["MBl"] != "NA" else None
                elongation = float(t["BEl"]) if t["BEl"] != "NA" else None
                if (size is not None and not (math.isfinite(size) and 0.5 <= size <= 1000)
                        or elongation is not None and not (math.isfinite(elongation) and 1 <= elongation <= 100)):
                    raise ValueError(f"Implausible source size/elongation: {t}")
                illustration = t["Type of illustration"].strip().lower()
                if illustration not in {"photograph", "drawing"}:
                    raise ValueError(f"Unknown source illustration: {illustration}")
                if size is not None:
                    record["text"] += (
                        f" FISHMORPH compilation: maximum body length {size:g} cm, compiled from FishBase"
                        " (Froese & Pauly 2020); length convention (standard, total or fork) is unspecified.")
                    record["provenance"] += (
                        f" Exact scientific-name join to FISHMORPH v1.1, FishBase species code {t['SpecCode']}."
                        " Maximum body length MBl (cm), compiled from FishBase (Froese & Pauly 2020);"
                        " standard/total/fork-length convention unspecified. Compiled value; no imputation. Brosse, Charpin, Su, Toussaint,"
                        " Herrera-R, Tedesco and Villéger (2021), https://rodare.hzdr.de/record/1316,"
                        " doi:10.14278/rodare.1316; associated paper doi:10.1111/geb.13395; CC BY 4.0."
                        " Trait CSV, metadata and units README are pinned in ../traits-sources.json.")
                    length_count += 1
                cpt = float(t["CPt"]) if t["CPt"] != "NA" else None
                if elongation is not None:
                    image_records.append(make_image_record(row, t, helper.common_names(row)[0]))
                    ratio_counts["BEl"] += 1
                elif cpt is not None and math.isfinite(cpt) and cpt > 0:
                    image_records.append(make_image_record(row, t, helper.common_names(row)[0]))
                    ratio_counts["CPt"] += 1
                else:
                    omitted_images.append({"species": row["canonicalName"], "BEl": t["BEl"], "CPt": t["CPt"],
                                           "reason": "BEl missing; CPt missing or zero-coded rather than a positive physical ratio."})
                trait_count += 1
            records.append(record)
    records.sort(key=lambda record: record["id"])
    assert len(records) == len({r["id"] for r in records})
    assert len(records) == len({rows[int(r["id"][5:])]["canonicalName"] for r in records})
    image_records.sort(key=lambda record: record["id"])
    assert len(image_records) == len({record["id"] for record in image_records})
    for child, payload in (("species", records), ("image-measurements", image_records)):
        output = HERE / child
        output.mkdir(exist_ok=True)
        (output / "records.json").write_text(json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        (output / "node.toml").write_text(f"weight = {len(payload)}\n")
    (HERE / "records.json").unlink(missing_ok=True)
    (HERE / "node.toml").write_text(f"weight = {len(records) + len(image_records)}\n")
    tax.update(record_count=len(records), selection="At most three per GBIF family; exact unique FISHMORPH matches first, then SHA-256 of GBIF ID; finite first API page per order.")
    tax_path.write_text(json.dumps(tax, indent=2) + "\n")
    manifest = {"dataset": "FISHMORPH", "version": "v1.1", "license": "CC BY 4.0",
                "source": SOURCE, "doi": "10.14278/rodare.1316", "source_rows": len(traits),
                "joined_records": trait_count, "record_count": len(records), "families": len(groups),
                "length_profiles": length_count, "image_records": len(image_records),
                "image_ratio_counts": dict(ratio_counts), "omitted_image_records": omitted_images,
                "measurement_method": {"paper": URLS["fishmorph-paper.pdf"],
                    "sections": "2.1.2, 2.2.2 and 4",
                    "scope": "One selected lateral-view depiction per species; no within-species variation or identified individual asserted.",
                    "reuse": "Paper consulted for methodology; PDF and figures not redistributed."},
                "extinction_correction": {"excluded": "Psephurus gladius", "source": URLS["paddlefish-abstract.xml"],
                    "reason": "GBIF flag absent; primary extinction study identifies extinction by 2010.",
                    "reuse": "Abstract consulted solely for exclusion; no abstract content redistributed."},
                "inputs": sorted(inputs.values(), key=lambda entry: entry["url"])}
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n")
    print(json.dumps({"species": len(records), "image_records": len(image_records),
                      "length_profiles": length_count, "ratios": dict(ratio_counts),
                      "descendant_records": len(records) + len(image_records), "families": len(groups), "traits": trait_count,
                      "orders": dict(Counter(rows[int(r["id"][5:])]["order"] for r in records))}))


if __name__ == "__main__":
    run()
