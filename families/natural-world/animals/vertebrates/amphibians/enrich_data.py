"""Rebuild amphibian profiles from pinned GBIF taxonomy and AmphiBIO v1.

Run with python -B; --replay rejects missing or changed cached inputs.
"""

import argparse
import csv
import hashlib
import importlib.util
import io
import json
import re
import sys
import urllib.request
import zipfile
from collections import Counter
from decimal import Decimal
from pathlib import Path

sys.dont_write_bytecode = True
LEAF = Path(__file__).resolve().parent
ROOT = LEAF.parents[4]
CACHE = ROOT / "target/import-cache/amphibians"
METADATA_URL = "https://api.figshare.com/v2/articles/4644424/versions/5"
ARCHIVE_URL = "https://ndownloader.figshare.com/files/8828578"
DOI = "https://doi.org/10.6084/m9.figshare.4644424.v5"
NUMERIC = {
    "Longevity_max_y": ("Maximum lifespan", "years"),
    "Reproductive_output_y": ("Maximum reproductive events per year", ""),
}
GROUPS = [
    ("Recorded foraging strata", {"Fos": "fossorial", "Ter": "terrestrial", "Aqu": "aquatic", "Arb": "arboreal"}),
    ("Recorded adult diet includes", {"Leaves": "leaves", "Flowers": "flowers", "Seeds": "seeds", "Fruits": "fruits", "Arthro": "arthropods", "Vert": "vertebrates"}),
    ("Recorded activity periods", {"Diu": "day", "Noc": "night", "Crepu": "twilight"}),
    ("Recorded breeding strategies", {"Dir": "direct development", "Lar": "larval development", "Viv": "viviparity"}),
]


def digest(data):
    return hashlib.sha256(data).hexdigest()


def fetch(url, name, expected, replay):
    path = CACHE / name
    if replay and not expected:
        raise ValueError(f"Unpinned source: {url}")
    if not path.exists():
        if replay:
            raise ValueError(f"Missing cached source: {path}")
        request = urllib.request.Request(url, headers={"User-Agent": "Inspire-corpus-import/1.0"})
        path.write_bytes(urllib.request.urlopen(request, timeout=90).read())
    data = path.read_bytes()
    if expected and expected != digest(data):
        raise ValueError(f"Changed pinned input: {url}")
    return data, {"url": url, "cache_file": name, "sha256": digest(data), "bytes": len(data)}


def trait_sentences(row):
    sentences = []
    size = row["Body_size_mm"]
    if size not in ("", "NA"):
        assert Decimal(size).is_finite() and Decimal(size) > 0
        measure = "snout-to-vent length" if row["Order"] == "Anura" else "total length"
        sentences.append(f"Maximum adult {measure}: {size} mm")
    for field, (label, unit) in NUMERIC.items():
        value = row[field]
        if value not in ("", "NA"):
            assert Decimal(value).is_finite() and Decimal(value) >= 0
            suffix = f" {unit}" if unit else ""
            sentences.append(f"{label}: {value}{suffix}")
    for lower, upper, label, unit in (
        ("Age_at_maturity_min_y", "Age_at_maturity_max_y", "Age at sexual maturity", "years"),
        ("Litter_size_min_n", "Litter_size_max_n", "Clutch size", "offspring or eggs"),
    ):
        low, high = row[lower], row[upper]
        available = [value for value in (low, high) if value not in ("", "NA")]
        assert all(Decimal(value).is_finite() and Decimal(value) >= 0 for value in available)
        if len(available) == 2:
            assert Decimal(low) <= Decimal(high)
            value = low if Decimal(low) == Decimal(high) else low + "–" + high
            sentences.append(f"{label}: {value} {unit}")
        elif available:
            bound = "Minimum" if low not in ("", "NA") else "Maximum"
            sentences.append(f"{bound} {label.lower()}: {available[0]} {unit}")
    for label, fields in GROUPS:
        assert all(row[key] in ("0", "1", "NA", "") for key in fields)
        present = [name for key, name in fields.items() if row[key] == "1"]
        if present:
            label = label.removeprefix("Recorded ").replace("adult diet includes", "adult diet")
            sentences.append(f"{label.capitalize()}: {', '.join(present)}")
    return sentences


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--replay", action="store_true")
    args = parser.parse_args()
    CACHE.mkdir(parents=True, exist_ok=True)
    manifest_path = LEAF / "trait-sources.json"
    old = json.loads(manifest_path.read_text(encoding="utf-8")) if manifest_path.exists() else {}
    expected = {entry["url"]: entry["sha256"] for entry in old.get("inputs", [])}
    # Verify all trait inputs before changing any records.
    metadata, metadata_input = fetch(METADATA_URL, "figshare-v5.json", expected.get(METADATA_URL), args.replay)
    archive, archive_input = fetch(ARCHIVE_URL, "AmphiBIO_v1.zip", expected.get(ARCHIVE_URL), args.replay)
    article = json.loads(metadata)
    assert article["version"] == 5 and article["license"]["name"] == "CC BY 4.0"
    assert any(file["id"] == 8828578 and file["download_url"] == ARCHIVE_URL for file in article["files"])
    with zipfile.ZipFile(io.BytesIO(archive)) as zipped:
        members = {name: zipped.read(name) for name in sorted(zipped.namelist())}
    extracted = {name: digest(data) for name, data in members.items()}
    if old and old["archive_member_sha256"] != extracted:
        raise ValueError("Changed archive member or trait metadata")
    traits = list(csv.DictReader(io.StringIO(members["AmphiBIO_v1.csv"].decode("cp1252"))))
    names = Counter(row["Species"] for row in traits)
    assert all(count == 1 for count in names.values())
    by_name = {row["Species"]: row for row in traits}
    importer_path = LEAF.parents[1] / "import_data.py"
    specification = importlib.util.spec_from_file_location("animal_taxonomy_import", importer_path)
    importer = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(importer)
    # Use a workspace cache so sandbox and ordinary shells share the same bytes.
    importer.CACHE = CACHE / "taxonomy"
    command = [str(importer_path), "--taxon", "Amphibia", "--rank", "CLASS", "--output", str(LEAF),
               "--pages", "5", "--per-family", "3"]
    if args.replay:
        command.append("--replay")
    previous_argv = sys.argv
    try:
        sys.argv = command
        importer.run()
    finally:
        sys.argv = previous_argv
    records = json.loads((LEAF / "records.json").read_text(encoding="utf-8"))
    exact = enriched = sized = 0
    joins = []
    for record in records:
        scientific = re.search(r"\(([^()]+)\)$", record["text"].splitlines()[0]).group(1)
        row = by_name.get(scientific)
        if row is None:
            continue
        assert row["Order"] in ("Anura", "Caudata", "Gymnophiona")
        # Updated family names do not prevent an exact binomial join; order
        # disagreement would invalidate the length definition and is rejected.
        assert f"Order: {row['Order']}" in record["text"].splitlines()
        exact += 1
        sentences = trait_sentences(row)
        if sentences:
            record["text"] += "\n" + "\n".join(sentences)
            record["provenance"] += (
                f" Exact scientific-binomial join to AmphiBIO species {row['id']}: {scientific}; "
                f"Oliveira, São-Pedro, Santos-Barrera, Penone and Costa (2017), {DOI}; CC BY 4.0. "
                "Formatting adapted by Inspire. Quantitative maxima and binary categories copied from "
                "the source; NA and zero-valued binary categories omitted, without inference. "
                "Definitions and units verified against Metadata.pdf inside the pinned archive."
            )
            enriched += 1
            sized += row["Body_size_mm"] not in ("", "NA")
            joins.append({"gbif_id": record["id"], "amphibio_id": row["id"], "scientific_name": scientific})
    assert len({record["id"] for record in records}) == len(records)
    assert all(set(record) == {"id", "text", "provenance"} and all(record.values()) for record in records)
    manifest = {"dataset": "AmphiBIO v1", "version": 5, "doi": DOI, "license": "CC BY 4.0",
                "retrieved": "2026-10-13", "source_rows": len(traits), "record_count": len(records),
                "exact_binomial_matches": exact, "trait_profiles": enriched, "adult_size_profiles": sized,
                "selection": "Shared GBIF hash ordering, cap 3 per family, plus original-name retention; exact binomial joins only.",
                "excluded_fields": {"Body_mass_g": "Omitted after sample QA found implausible maximum adult masses; values were not corrected."},
                "inputs": [metadata_input, archive_input], "archive_member_sha256": extracted, "joins": joins}
    (LEAF / "records.json").write_text(json.dumps(records, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    manifest_path.write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({key: manifest[key] for key in ("record_count", "exact_binomial_matches", "trait_profiles", "adult_size_profiles")}))


if __name__ == "__main__":
    main()
