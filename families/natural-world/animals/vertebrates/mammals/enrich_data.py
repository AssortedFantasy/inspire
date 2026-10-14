"""Add exact-name PanTHERIA 1.0 species estimates to the GBIF mammal corpus.

Run after ../../import_data.py. Raw inputs stay in --cache outside families/;
--replay verifies their recorded SHA-256 hashes without network access.
"""

import argparse
import csv
import hashlib
import io
import json
import math
import re
import tempfile
import urllib.request
from collections import Counter
from pathlib import Path

BASE = "https://esapubs.org/archive/ecol/E090/184/"
DATA = BASE + "PanTHERIA_1-0_WR05_Aug2008.txt"
METADATA = BASE + "metadata.htm"
LICENSE = "https://esapubs.org/archive/copyright.htm"
FIELDS = [
    ("5-1_AdultBodyMass_g", "adult body mass", "g"),
    ("9-1_GestationLen_d", "gestation", "days"),
    ("15-1_LitterSize", "offspring per litter", ""),
    ("17-1_MaxLongevity_m", "maximum recorded longevity", "months"),
]
MARKER = " PanTHERIA 1.0 (2008) species estimates: "


def retrieve(url, cache, sources, replay):
    name = url.rsplit("/", 1)[-1]
    target = cache / name
    expected = sources.get(url, {}).get("sha256")
    if replay and (not expected or not target.exists()):
        raise ValueError(f"Missing pinned input: {url}")
    if not target.exists():
        request = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0"})
        target.write_bytes(urllib.request.urlopen(request, timeout=90).read())
    content = target.read_bytes()
    digest = hashlib.sha256(content).hexdigest()
    if expected and digest != expected:
        raise ValueError(f"Pinned input changed: {url}")
    sources[url] = {"url": url, "sha256": digest, "bytes": len(content), "cache_file": name}
    return content


def number(value):
    # Three significant figures prevent false precision from fitted estimates.
    return format(float(format(value, ".3g")), ",.8f").rstrip("0").rstrip(".")


def run():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cache", type=Path, default=Path(tempfile.gettempdir()) / "inspire-mammal-traits")
    parser.add_argument("--replay", action="store_true")
    args = parser.parse_args()
    leaf = Path(__file__).resolve().parent
    if args.cache.resolve() == leaf.parents[3] or leaf.parents[3] in args.cache.resolve().parents:
        parser.error("Raw cache must be outside the sampling tree")
    args.cache.mkdir(parents=True, exist_ok=True)
    manifest_path = leaf / "traits-sources.json"
    previous = json.loads(manifest_path.read_text(encoding="utf-8")) if manifest_path.exists() else {}
    sources = {entry["url"]: entry for entry in previous.get("inputs", [])}
    raw = retrieve(DATA, args.cache, sources, args.replay)
    retrieve(METADATA, args.cache, sources, args.replay)
    retrieve(LICENSE, args.cache, sources, args.replay)
    rows = list(csv.DictReader(io.StringIO(raw.decode("utf-8-sig")), delimiter="\t"))
    traits = {row["MSW05_Binomial"]: row for row in rows}
    if len(rows) != 5416 or len(traits) != len(rows):
        raise ValueError("Unexpected source count or duplicate binomials")
    records_path = leaf / "records.json"
    records = json.loads(records_path.read_text(encoding="utf-8"))
    counts = Counter()
    unmatched = []
    enriched = 0
    for record in records:
        record["text"] = record["text"].split(MARKER)[0]
        record["provenance"] = record["provenance"].split(" PanTHERIA trait source: ")[0]
        match = re.match(r"[^()]+ \(([^()]+)\)\.", record["text"])
        canonical = match[1] if match else ""
        row = traits.get(canonical)
        if row is None:
            unmatched.append(canonical)
            continue
        values = []
        for field, label, unit in FIELDS:
            value = float(row[field])
            if math.isfinite(value) and value > 0:
                if unit == "g" and value >= 1000:
                    value, unit = value / 1000, "kg"
                values.append(f"{label} approximately {number(value)}" + (f" {unit}" if unit else ""))
                counts[field] += 1
        if values:
            record["text"] += MARKER + "; ".join(values) + "."
            record["provenance"] += (
                f" PanTHERIA trait source: Jones et al. (2009), https://doi.org/10.1890/08-1494.1; {DATA}; "
                f"version 1.0 WR05 August 2008, exact MSW05_Binomial={canonical}. "
                f"Species estimates rounded to three significant figures; missing -999 values omitted. "
                f"Non-EXT fields only; source estimates may include model adjustment or extrapolated literature observations. "
                f"Maximum longevity includes captive and wild observations, not life expectancy. "
                f"Species names may represent older species concepts; no synonym remapping or imputation. "
                f"Source reference indices: {row['References']}. "
                f"CC BY 4.0 per {LICENSE}; dataset metadata additionally states no copyright or proprietary restrictions. "
                "Pinned inputs are in traits-sources.json."
            )
            enriched += 1
    records_path.write_text(json.dumps(records, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    manifest = {"dataset": "PanTHERIA 1.0 WR05 August 2008", "publication": "https://doi.org/10.1890/08-1494.1",
                "license": "CC BY 4.0", "license_url": LICENSE, "retrieved": "2026-10-13",
                "source_records": len(rows), "join": "Exact GBIF canonical binomial = MSW05_Binomial; no remapping",
                "record_count": len(records), "enriched_records": enriched,
                "field_coverage": dict(sorted(counts.items())), "unmatched_binomials": sorted(unmatched),
                "inputs": sorted(sources.values(), key=lambda entry: entry["url"])}
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"records": len(records), "enriched": enriched, "field_coverage": counts, "unmatched": len(unmatched)}))


if __name__ == "__main__":
    run()
