"""Import explicitly genus-level, strongly supported Cefas benthic traits.

The matrix assigns confidence scores, not frequencies. A score of zero is not
evidence of absence. Only score 3 (strong evidence) contributes emitted traits.
Repeated rows within a genus must agree exactly before they are deduplicated.
"""

import argparse
import csv
import hashlib
import io
import json
import tempfile
import urllib.request
from collections import defaultdict
from pathlib import Path

CACHE = Path(tempfile.gettempdir()) / "inspire-animal-traits"
SOURCES = {
    "cefas-traits.csv": ("https://data-api.cefas.co.uk/api/export/11935",
                         "f03dcd43f7dcab4970a5c31dbf9816c859414891c686c0e4ae7e44af5a2102cc"),
    "cefas-metadata.json": ("https://data-api.cefas.co.uk/api/holdings/21362",
                           "1005ba5a35c0fe94a57e831d0ed30a337cdd6c2cda4f9f22aa0396fb7012aa80"),
}
GROUPS = {"annelids": "Annelida", "crustaceans": "Arthropoda", "molluscs": "Mollusca",
          "cnidarians": "Cnidaria", "echinoderms": "Echinodermata"}
TRAITS = {
    "m_Soft": "soft-bodied morphology", "m_Tunic": "a tunic",
    "m_Exoskeleton": "hard skeletal structures",
    "m_Crustose": "crustose growth", "m_Cushion": "cushion-shaped growth", "m_Stalked": "stalked growth",
    "ed_Asexual": "asexual reproduction", "ed_Sexual_pelagic": "pelagic egg development",
    "ed_Sexual_benthic": "benthic egg development", "ed_Sexual_brooded": "brooded egg development",
    "ld_Pelagic_planktotrophic": "feeding pelagic larvae", "ld_Pelagic_lecithotrophic": "nonfeeding pelagic larvae",
    "ld_Benthic_direct": "direct benthic development", "lh_Tube_dwelling": "tube dwelling",
    "lh_Burrow_dwelling": "burrow dwelling", "lh_Free_living": "free living",
    "lh_Crevice_hole_under_stones": "living in crevices, holes or beneath stones",
    "lh_Epi_endo_biotic": "living on or within another organism", "lh_Attached_to_substratum": "attachment to the substrate",
    "sp_Surface": "living at the sediment surface", "sp_Shallow_infauna_0_to_5cm": "living within sediment at 0–5 cm",
    "sp_Mid_depth_infauna_5_to_10cm": "living within sediment at 5–10 cm",
    "sp_Deep_infauna_more_than_10cm": "living more than 10 cm into sediment",
    "f_Suspension": "suspension feeding", "f_Surface_deposit": "surface deposit feeding",
    "f_Subsurface_deposit": "subsurface deposit feeding", "f_Scavenger": "scavenging",
    "f_Predator": "predation", "f_Parasite": "parasitism", "mob_Sessile": "sessile adult mobility",
    "mob_Swim": "swimming", "mob_Crawl_creep_climb": "crawling, creeping or climbing",
    "mob_Burrower": "burrowing", "b_Diffusive_mixing": "diffusive sediment mixing",
    "b_Surface_deposition": "surface sediment deposition", "b_Upward_conveyor": "upward conveyor sediment transport",
    "b_Downward_conveyer": "downward conveyor sediment transport", "b_None": "no bioturbation",
}
TRAIT_GROUPS = {
    "m": "Morphology", "ed": "Egg development", "ld": "Larval development",
    "lh": "Living habit", "sp": "Sediment position", "f": "Feeding",
    "mob": "Mobility", "b": "Sediment mixing",
}


def inputs(replay=False):
    CACHE.mkdir(parents=True, exist_ok=True)
    result = {}
    for filename, (url, digest) in SOURCES.items():
        path = CACHE / filename
        if not path.exists():
            if replay:
                raise ValueError(f"Missing pinned input {path}")
            request = urllib.request.Request(url, headers={"Accept": "application/json" if filename.endswith("json") else "text/csv"})
            path.write_bytes(urllib.request.urlopen(request, timeout=90).read())
        data = path.read_bytes()
        if hashlib.sha256(data).hexdigest() != digest:
            raise ValueError(f"Cefas source changed; review an explicit dataset upgrade: {url}")
        result[filename] = data
    return result


def records(group, per_family, data):
    raw = list(csv.DictReader(io.StringIO(data.decode("utf-8-sig"))))
    if len(raw) != 1025:
        raise ValueError("Pinned matrix row count changed")
    genera = {}
    source_ids = defaultdict(list)
    for row in raw:
        if row["Phylum"] != GROUPS[group] or not row["Genus"]:
            continue
        if group == "crustaceans" and row["Class"] == "Pycnogonida":
            continue  # Sea spiders are chelicerates, not crustaceans.
        key = tuple(row[field] for field in ("Class", "_Order", "Family", "Genus"))
        codes = tuple(row[field] for field in TRAITS)
        if any(value not in {"0", "1", "2", "3"} for value in codes):
            raise ValueError(f"Unexpected confidence code: {key}")
        if key in genera and codes != tuple(genera[key][field] for field in TRAITS):
            raise ValueError(f"Conflicting repeated genus evidence: {key}")
        genera[key] = row
        source_ids[key].append(row["AphiaID"])
    by_family = defaultdict(list)
    for key, row in genera.items():
        strong = [text for field, text in TRAITS.items() if row[field] == "3"]
        if len(strong) < 2 or not row["Family"]:
            continue
        name = row["Genus"]
        placement = (f"unresolved ({row['Family']})"
                     if "incertae sedis" in row["Family"] else row["Family"])
        lines = [f"{name} (genus)", f"Class: {row['Class']}", f"Family: {placement}",
                 "Regional trait synthesis: Northwest European marine benthos"]
        for prefix, label in TRAIT_GROUPS.items():
            values = [text for field, text in TRAITS.items()
                      if field.split("_", 1)[0] == prefix and row[field] == "3"]
            if values:
                lines.append(f"{label}: " + ", ".join(values))
        record = {"id": "cefas-genus-" + name.lower(), "text": "\n".join(lines),
                  "provenance": "Clare et al. (2022), Ten key biological traits of marine benthic invertebrates surveyed in Northwest Europe, Cefas, UK, V2. "
                  "DOI https://doi.org/10.14466/CefasDataHub.123; https://data.cefas.co.uk/view/21362. "
                  "Open Government Licence v3.0, https://www.nationalarchives.gov.uk/doc/open-government-licence/version/3/. "
                  "Includes Crown copyright, Cefas [2021]. Snapshot downloaded 2026-10-13; exact input hashes in sources.json. "
                  "Only confidence score 3 categories retained; repeated genus rows deduplicated after equality checks. "
                  "The broad matrix exoskeleton morphology category is rendered as hard skeletal structures, avoiding an external-skeleton assertion for echinoderms. "
                  "Original Aphia IDs: " + ", ".join(sorted(source_ids[key])) + ". "
                  "Trait synthesis includes literature and professional judgement; categories are not measurements or universal species-level assertions."}
        by_family[row["Family"]].append(record)
    selected = []
    for family in sorted(by_family):
        candidates = sorted(by_family[family], key=lambda record: hashlib.sha256(record["id"].encode()).hexdigest())
        selected.extend(candidates[:per_family])
    selected.sort(key=lambda record: record["id"])
    if not selected or len({record["id"] for record in selected}) != len(selected):
        raise ValueError("Empty corpus or duplicate genus IDs")
    return selected, len(genera), len(by_family)


def run():
    global CACHE
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--group", choices=GROUPS, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--per-family", type=int, default=3)
    parser.add_argument("--replay", action="store_true")
    parser.add_argument("--cache-dir", type=Path)
    args = parser.parse_args()
    if args.cache_dir:
        CACHE = args.cache_dir
    tree = next(parent for parent in Path(__file__).resolve().parents if parent.name == "families")
    if CACHE.resolve().is_relative_to(tree):
        parser.error("Keep raw input caches outside the families sampling tree")
    if args.per_family < 1:
        parser.error("--per-family must be positive")
    data = inputs(args.replay)
    selected, count, families = records(args.group, args.per_family, data["cefas-traits.csv"])
    manifest = {"source": "Cefas trait matrix V2", "doi": "10.14466/CefasDataHub.123", "license": "OGL v3.0",
                "scope": "Marine benthos of Northwest Europe; genus-level trait synthesis", "group": args.group,
                "per_family_cap": args.per_family, "source_genera": count, "families": families,
                "record_count": len(selected), "confidence_filter": 3,
                "inputs": [{"url": url, "sha256": digest, "cache_file": name, "bytes": len(data[name])}
                           for name, (url, digest) in SOURCES.items()]}
    args.output.mkdir(parents=True, exist_ok=True)
    (args.output / "records.json").write_text(json.dumps(selected, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    (args.output / "node.toml").write_text(f"weight = {len(selected)}\n", encoding="utf-8")
    (args.output / "sources.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"group": args.group, "records": len(selected), "source_genera": count, "families": families}))


if __name__ == "__main__":
    run()
