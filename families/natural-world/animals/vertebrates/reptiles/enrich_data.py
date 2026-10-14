"""Rebuild taxonomy and enrich exact species joins from pinned ReptTraits v4."""

import argparse
import hashlib
import json
import math
import os
import re
import subprocess
import sys
import urllib.request
from collections import Counter
from pathlib import Path

sys.dont_write_bytecode = True
import openpyxl

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[4]
CACHE = ROOT / "target/import-cache/reptiles"
META_URL = "https://api.figshare.com/v2/articles/24572683/versions/4"
FILE_URL = "https://ndownloader.figshare.com/files/45408133"
DOI = "https://doi.org/10.6084/m9.figshare.24572683.v4"


def write_json(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def load_input(name, url, inputs, replay):
    path = CACHE / name
    expected = next((row for row in inputs if row["url"] == url), None)
    if replay and (expected is None or not path.exists()):
        raise ValueError(f"Missing pinned source: {url}")
    if not path.exists():
        path.write_bytes(urllib.request.urlopen(url, timeout=120).read())
    raw = path.read_bytes()
    digest = hashlib.sha256(raw).hexdigest()
    if expected and expected["sha256"] != digest:
        raise ValueError(f"Pinned source changed: {url}")
    entry = {"url": url, "cache_file": name, "bytes": len(raw), "sha256": digest}
    if expected is None:
        inputs.append(entry)
    return raw


def number(value, minimum, maximum):
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        return None
    if not math.isfinite(value) or not minimum <= value <= maximum:
        return None
    return f"{value:.4g}"


def traits(row, citations):
    # Only literal categorical values and source-defined measurements. Broad
    # validation bounds reject malformed values, not statistically rare species.
    fields = []
    def add(index, label, value):
        if value and citations[index] not in (None, "NA", "No", ""):
            fields.append((index, label, value))
    if row[18] in {"Carnivorous", "Herbivorous", "Omnivorous"}:
        add(18, "diet", row[18].lower())
    if row[20] in {"Diurnal", "Nocturnal", "Cathemeral", "Crepuscular"}:
        add(20, "activity", row[20].lower())
    if row[34] in {"oviparous", "viviparous", "ovoviviparous"}:
        add(34, "reproductive mode", row[34])
    add(26, "maximum reported longevity (years)", number(row[26], 0.1, 300))
    add(28, "maximum total length (mm)", number(row[28], 5, 15000))
    # SVL entries contain obvious total-length contamination (e.g. tuatara
    # 800 mm). Retain only the explicitly distinct turtle carapace metric.
    if row[1] == "Testudines":
        add(29, "maximum straight carapace length (mm)", number(row[29], 10, 3000))
    return fields


def run():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--replay", action="store_true")
    args = parser.parse_args()
    CACHE.mkdir(parents=True, exist_ok=True)
    manifest_path = HERE / "trait_sources.json"
    old = json.loads(manifest_path.read_text(encoding="utf-8")) if manifest_path.exists() else {}
    inputs = old.get("inputs", [])
    meta = json.loads(load_input("metadata.json", META_URL, inputs, args.replay))
    if meta["version"] != 4 or meta["license"]["name"] != "CC BY 4.0":
        raise ValueError("Unexpected source version or license")
    raw = load_input("ReptTraits.xlsx", FILE_URL, inputs, args.replay)
    source_file = next(f for f in meta["files"] if f["download_url"] == FILE_URL)
    if hashlib.md5(raw).hexdigest() != source_file["computed_md5"]:
        raise ValueError("Workbook differs from published source MD5")
    command = [sys.executable, "-B", str(HERE.parents[1] / "import_data.py")]
    for taxon in ("Squamata", "Testudines", "Crocodylia", "Sphenodontia"):
        command.extend(["--taxon", taxon])
    command.extend(["--rank", "CLASS", "--output", str(HERE), "--pages", "5", "--per-family", "3"])
    for query in ("", "rubber boa", "prairie rattlesnake", "bullsnake"):
        command.extend(["--query", query])
    if args.replay:
        command.append("--replay")
    # Keep taxonomy cache beside workbook even across Windows sandbox profiles.
    environment = dict(os.environ, TEMP=str(CACHE), TMP=str(CACHE))
    subprocess.run(command, check=True, env=environment)
    workbook = openpyxl.load_workbook(CACHE / "ReptTraits.xlsx", read_only=True, data_only=True)
    iterator = workbook["Data"].iter_rows(values_only=True)
    headers = next(iterator)
    data_rows = list(iterator)
    rows = {row[0]: row for row in data_rows}
    if len(rows) != len(data_rows):
        raise ValueError("Duplicate source species names")
    iterator = workbook["Sources of individual trait val"].iter_rows(values_only=True)
    if tuple(next(iterator)) != tuple(headers[:48]):
        raise ValueError("Trait citation columns changed")
    citations = {row[0]: row for row in iterator}
    definitions = {row[1]: row[2] for row in workbook["Trait definitions"].iter_rows(values_only=True) if row[1]}
    for i in (18, 20, 26, 28, 29, 34):
        if headers[i].strip() not in definitions:
            raise ValueError(f"Missing trait definition: {headers[i]}")
    records = json.loads((HERE / "records.json").read_text(encoding="utf-8"))
    counts = Counter()
    joins = []
    for record in records:
        name = re.search(r"\(([^()]*)\)", record["text"]).group(1)
        if name not in rows or name not in citations:
            continue
        fields = traits(rows[name], citations[name])
        if not fields:
            continue
        record["text"] += " ReptTraits: " + "; ".join(f"{label}: {value}" for _, label, value in fields) + "."
        record["provenance"] += " Exact-binomial join to ReptTraits, Oskyrko et al., Figshare article version 4, workbook v1-2; " + DOI + "; CC BY 4.0. Source-reported values; longevity includes captive records where present. Per-trait citations and exclusions are in trait_sources.json."
        joins.append({"id": record["id"], "species": name, "traits": [{"field": headers[i], "value": rows[name][i], "citation": citations[name][i]} for i, _, _ in fields]})
        counts.update(label for _, label, _ in fields)
    workbook.close()
    write_json(HERE / "records.json", records)
    write_json(manifest_path, {"doi": DOI, "version": 4, "file": source_file["name"], "license": "CC BY 4.0", "inputs": inputs, "join": "exact scientific binomial, no synonym or fuzzy matches", "record_count": len(records), "enriched_records": len(joins), "coverage": dict(sorted(counts.items())), "used_definitions": {headers[i]: definitions[headers[i].strip()] for i in (18, 20, 26, 28, 29, 34)}, "excluded_fields": {"Maximum body mass (g)": "Mixed measured and allometrically derived source entries; omitted rather than emitted as measured maxima.", "Microhabitat": "Source inconsistency: Crocodylus porosus marked Arboreal/Saxicolous/Terrestrial. Entire column omitted.", "SVL and sex-specific lengths": "Source inconsistency: Sphenodon punctatus maximum SVL 800 mm appears to be total length. All non-turtle SVL omitted; no inferred conversions."}, "joins": joins})
    print(json.dumps({"records": len(records), "enriched": len(joins), "coverage": dict(counts)}))


if __name__ == "__main__":
    run()
