"""Import study/sample observations, never universal species constants.

Prefer observations with several independent recorded traits. Brain-size values
retain their original units so measured volume is not relabelled as brain mass.
Metabolic watts are the source's standardized conversion, identified as such.
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
from collections import defaultdict
from pathlib import Path

SOURCE = Path(__file__).with_name("animaltraits_source.json")
CACHE = Path(tempfile.gettempdir()) / "inspire-animal-traits"
CLASSES = {"insects": "Insecta", "arachnids": "Arachnida"}
# Spelling repairs have direct study or specimen evidence, not inferred synonyms.
FAMILY_CORRECTIONS = {"Tettigonildae": "Tettigoniidae", "Aeschnidae": "Aeshnidae"}
SPECIES_CORRECTIONS = {
    "Reticlitermes flavipes": "Reticulitermes flavipes",
    "Pholcus phalangioideus": "Pholcus phalangioides",
    "Alaskozeres antarcticus": "Alaskozetes antarcticus",
    "Misumenoidea formosipes": "Misumenoides formosipes",
}
ORDER_CORRECTIONS = {"Orthoperta": "Orthoptera"}
CORRECTION_EVIDENCE = {
    "Tettigonildae": "Requena primary study title: https://doi.org/10.1242/jeb.178.1.21",
    "Aeschnidae": "Smithsonian Anax junius specimen taxonomy: https://www.si.edu/object/anax-junius%3Anmnhentomology_9301938",
    "Reticlitermes flavipes": "Shelton & Appel primary study title: https://doi.org/10.1016/S0022-1910(00)00111-6",
    "Orthoperta": "Hack primary study abstract: https://doi.org/10.1111/j.1365-3032.1997.tb01176.x",
    "Pholcus phalangioideus": "Schmitz primary study title: https://pubmed.ncbi.nlm.nih.gov/26022251/",
    "Alaskozeres antarcticus": "Young & Block primary study abstract in institutional repository: https://nora.nerc.ac.uk/id/eprint/525070/",
    "Misumenoidea formosipes": "ITIS verified species TSN 883799: https://www.itis.gov/servlet/SingleRpt/SingleRpt?search_topic=TSN&search_value=883799; primary study: https://www.americanarachnology.org/journal-joa/joa-all-articles/article/download/arac-39-01-41.pdf/?no_cache=1",
}
EXCLUDED_SPECIES = {
    "Lygus kalmii": "Unresolved source name/family combination (Lygaeidae); primary identity unverified",
    "Multareoides bifuractus": "Unresolved source epithet; primary study identity unverified",
}
QUESADA_CNS_EVIDENCE = "https://stri-sites.si.edu/docs/publications/pdfs/STRI-W_Wcislo_2011_Quesada_et_al.pdf"


def number(raw):
    value = float(raw)
    if not math.isfinite(value) or value <= 0:
        raise ValueError(f"Invalid positive measurement: {raw}")
    return re.sub(r"e([+-]?)0*(\d+)$", lambda match: "e" + ("-" if match[1] == "-" else "") + match[2], f"{value:.3g}")


def read_inputs(replay):
    manifest = json.loads(SOURCE.read_text(encoding="utf-8"))
    CACHE.mkdir(parents=True, exist_ok=True)
    data = {}
    for source in manifest["inputs"]:
        path = CACHE / source["cache_file"]
        if not path.exists():
            if replay:
                raise ValueError(f"Missing pinned input {path}")
            path.write_bytes(urllib.request.urlopen(source["url"], timeout=90).read())
        raw = path.read_bytes()
        if hashlib.sha256(raw).hexdigest() != source["sha256"]:
            raise ValueError(f"Changed AnimalTraits input: {source['url']}")
        data[source["cache_file"]] = raw.decode(source["encoding"])
    return manifest, list(csv.DictReader(io.StringIO(data["observations.csv"])))


def observation(row, index):
    traits = []
    if row["body mass"]:
        if row["body mass - units"] != "kg":
            raise ValueError("Unexpected standardized body-mass units")
        if row["body mass - minimum"] and row["body mass - maximum"]:
            lower, upper = (float(row["body mass - " + bound]) * 1000 for bound in ("minimum", "maximum"))
            if lower > upper:
                raise ValueError("Reversed body-mass range")
            traits.append("Body mass: " + number(str(lower)) + "–" + number(str(upper)) + " g")
        else:
            traits.append("Body mass: " + number(str(float(row["body mass"]) * 1000)) + " g")
    if row["metabolic rate"]:
        if row["metabolic rate - units"] != "W":
            raise ValueError("Unexpected standardized metabolic units")
        traits.append("Metabolic rate (normalized to 25 °C): " + number(row["metabolic rate"]) + " W")
    if row["original brain size"]:
        units = row["original brain size - units"]
        if not units:
            raise ValueError("Brain observation lacks original units")
        brain = row["original brain size"]
        factor = re.fullmatch(r"x([0-9.eE+-]+) (.+)", units)
        if factor:
            brain = str(float(brain) * float(factor[1]))
            units = factor[2]
        units = {"um3": "µm³", "mm3": "mm³", "ug": "µg"}.get(units, units)
        if row["inTextReference"] == "Quesada et al., 2011":
            label = "Central nervous system neuropil volume"
        else:
            label = "Neural-tissue size" if row["brain size - comments"] == "ganglia and ventral nerve cord" else "Brain size"
        traits.append(label + ": " + number(brain) + " " + units)
    if len(traits) < 2:
        return None
    name = row["species"]
    if name in EXCLUDED_SPECIES:
        return None
    if not re.fullmatch(r"[A-Z][a-z]+ [a-z]+", name):
        return None
    name = SPECIES_CORRECTIONS.get(name, name)
    family = FAMILY_CORRECTIONS.get(row["family"], row["family"])
    order = ORDER_CORRECTIONS.get(row["order"], row["order"])
    if not family or not row["inTextReference"] or not row["fullReference"]:
        return None
    lines = [name, f"Order: {order}", f"Family: {family}", *traits]
    if row["sex"]:
        lines.append("Sex: " + row["sex"])
    caste = row["brain size - comments"]
    if family == "Formicidae" and caste in {"major", "minor", "worker", "soldier", "gyne", "queen"}:
        lines.append("Caste: " + caste)
    if row["sampleSizeValue"]:
        lines.append("Sample size: " + row["sampleSizeValue"])
    if row["metabolic rate"] and row["original temperature"]:
        # The source column is temperature in degrees Celsius; keep exact source
        # value rather than introducing a precision or rounding transformation.
        lines.append("Measurement temperature: " + row["original temperature"] + " °C")
    if row["species"] == "Damon annulatipes" and row["original temperature"] == "21":
        lines[-1] += " (metadata comment gives 25 °C)"
    if row["species"] == "Mysmenopsis tengellacompa" and row["inTextReference"] == "Quesada et al., 2011" and row["sampleSizeValue"] == "1":
        lines[lines.index("Sample size: 1")] = "Sample size: 1 in dataset, 3 females in study table"
    if row["metabolic rate"] and row["metabolic rate - method"] not in {"", "not specified"}:
        lines.append("Metabolic measure: " + row["metabolic rate - method"])
    if row["brain size - comments"] == "ganglia and ventral nerve cord":
        lines.append("Neural tissue: ganglia and ventral nerve cord")
    if row["original brain size"] and row["inTextReference"] == "Quesada et al., 2011":
        lines.append("Neural tissue: supra- and sub-oesophageal ganglia, neuropil only (cortex excluded)")
    if row["original brain size"] and row["brain size - method"] not in {"", "not specified"}:
        lines.append("Brain measurement: " + row["brain size - method"].strip().rstrip("."))
    lines.append("Study: " + row["inTextReference"])
    corrections = []
    for field, mapping in (("species", SPECIES_CORRECTIONS), ("order", ORDER_CORRECTIONS), ("family", FAMILY_CORRECTIONS)):
        original = row[field]
        if original in mapping:
            corrections.append(f" {field.capitalize()} spelling {original} corrected to {mapping[original]}; evidence: {CORRECTION_EVIDENCE[original]}.")
    if row["original brain size"] and row["inTextReference"] == "Quesada et al., 2011":
        corrections.append(" Source brain-size column identified as CNS neuropil-only volume from the study's supplementary Table 2 (N column); evidence: " + QUESADA_CNS_EVIDENCE + ".")
    record = {"id": f"animaltraits-observation-{index}", "text": "\n".join(lines), "provenance":
              "AnimalTraits v1.0.7, McLean (2022), https://doi.org/10.5281/zenodo.6468938; CC0 1.0. "
              "Dataset descriptor https://doi.org/10.1038/s41597-022-01364-9. "
              f"Pinned observation CSV data row {index} (one-based, header excluded), "
              "repository revision 06c872793e83d448538a12f6f98dbad389a86703. "
              "Study observations, not maxima or taxonomically current species constants. "
              "Order/family labels follow the source dataset and may be historical; only documented spelling corrections are applied. "
              "Body mass converted from standardized kg to g (explicit ranges retained); metabolic rate is source-normalized to 25 °C using Q10=2 and expressed in W; original brain mass/volume dimension retained and any explicit unit multiplier applied. "
              "Measurements formatted to three significant figures. Original study: " + row["fullReference"] + "".join(corrections)}
    return family, len(traits), record


def run():
    global CACHE
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--group", choices=CLASSES, required=True)
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
    source, raw = read_inputs(args.replay)
    if len(raw) != source["observation_count"]:
        raise ValueError("Pinned observation count changed")
    species = {}
    for index, row in enumerate(raw, 1):
        if row["class"] != CLASSES[args.group]:
            continue
        result = observation(row, index)
        if not result:
            continue
        # One richest single study record per species, not an average across
        # incompatible sex, method, temperature or measurement regimes.
        name = SPECIES_CORRECTIONS.get(row["species"], row["species"])
        if name not in species or result[1] > species[name][1]:
            species[name] = result
    families = defaultdict(list)
    for family, _, record in species.values():
        families[family].append(record)
    records = []
    for family in sorted(families):
        candidates = sorted(families[family], key=lambda record: hashlib.sha256(record["id"].encode()).hexdigest())
        records.extend(candidates[:args.per_family])
    records.sort(key=lambda record: record["id"])
    if not records or len({record["id"] for record in records}) != len(records):
        raise ValueError("Empty corpus or duplicate observation IDs")
    args.output.mkdir(parents=True, exist_ok=True)
    (args.output / "records.json").write_text(json.dumps(records, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    (args.output / "node.toml").write_text(f"weight = {len(records)}\n", encoding="utf-8")
    manifest = {**source, "group": args.group, "class": CLASSES[args.group], "per_family_cap": args.per_family,
                "eligible_species": len(species), "families": len(families), "record_count": len(records),
                "selection": "One richest study observation per species (first source row breaks ties); stable ID-hash family caps",
                "taxonomy_policy": "Source-recorded order/family labels, potentially historical; documented spelling corrections only",
                "family_corrections": FAMILY_CORRECTIONS, "species_corrections": SPECIES_CORRECTIONS,
                "order_corrections": ORDER_CORRECTIONS, "correction_evidence": CORRECTION_EVIDENCE,
                "excluded_species": EXCLUDED_SPECIES,
                "neural_structure_clarifications": {"Quesada et al., 2011": {
                    "measurement": "CNS neuropil-only volume, supra- and sub-oesophageal ganglia; cortex excluded",
                    "evidence": QUESADA_CNS_EVIDENCE,
                    "sample_size_discrepancy": "Mysmenopsis tengellacompa source sample size 1; supplementary Table 2 reports 3 females; source field retained with explicit note"}},
                "temperature_discrepancies": {"Damon annulatipes": "Original temperature column 21 °C; metadata comment mentions 25 °C; source values retained with explicit note"}}
    (args.output / "sources.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({"group": args.group, "records": len(records), "eligible_species": len(species), "families": len(families)}))


if __name__ == "__main__":
    run()
