"""Import factual structural pairs using chemistry's pinned PubChem cache."""
import argparse
import itertools
import json
import re
import urllib.parse
from collections import defaultdict
from import_data import BASE, CACHE, ROOT, fetch, write

STEREO = [
    ("cis-2-butene", "trans-2-butene", "geometric diastereomers", "The Z (cis) and E (trans) configurations have different arrangements across the carbon-carbon double bond."),
    ("maleic acid", "fumaric acid", "geometric diastereomers", "Maleic acid is Z-butenedioic acid; fumaric acid is E-butenedioic acid."),
    ("L-alanine", "D-alanine", "enantiomers", "L-alanine has S configuration and D-alanine has R configuration at the alpha carbon. D/L labels specify configuration; optical-rotation sign is a separate property."),
    ("D-glucose", "D-galactose", "diastereomers", "In their open-chain forms these aldohexoses differ at carbon 4 and are C4 epimers; glucose and fructose instead differ in connectivity."),
]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--download", action="store_true")
    args = parser.parse_args()
    compounds = json.loads((CACHE / "selected-compounds.json").read_text())
    groups = defaultdict(list)
    for data in compounds:
        if not data["category"].startswith("organic/") or "biomolecules" in data["category"]:
            continue
        groups[data["MolecularFormula"]].append(data)
    records = []
    for formula, group in groups.items():
        for a, b in itertools.combinations(sorted(group, key=lambda d: d["CID"]), 2):
            records.append({"id": f'constitutional-{a["CID"]}-{b["CID"]}',
                "text": f'{a["name"]} and {b["name"]}\nConstitutional isomers with molecular formula {formula}; their atoms have different connectivity.\nSMILES ({a["name"]}): {a["SMILES"]}\nSMILES ({b["name"]}): {b["SMILES"]}',
                "provenance": f'Pair of selected neutral organic compounds with identical composition and chemically distinct connectivity, excluding carbohydrates to avoid confusing epimers with constitutional isomers. PubChem CIDs {a["CID"]}, {b["CID"]}; input URLs/hashes in source_manifest.json. Original explanatory prose.'})
    write(ROOT / "formulas/isomers/constitutional/records.json", records)
    manifest_path = ROOT / "source_manifest.json"
    previous = json.loads(manifest_path.read_text())
    manifest = dict(previous)
    records = []
    for name_a, name_b, relation, explanation in STEREO:
        pair = []
        for name in [name_a, name_b]:
            key = "compound-" + re.sub(r"[^a-z0-9]+", "-", name.lower()).strip("-")
            url = BASE + "compound/name/" + urllib.parse.quote(name, safe="") + "/property/MolecularFormula,MolecularWeight,IUPACName,SMILES,Charge/JSON"
            values = fetch(key, url, args.download, previous, manifest)["PropertyTable"]["Properties"]
            assert len(values) == 1, name
            pair.append(values[0])
        a, b = pair
        assert a["MolecularFormula"] == b["MolecularFormula"]
        assert a["CID"] != b["CID"]
        formula = a["MolecularFormula"]
        records.append({"id": f'stereo-{a["CID"]}-{b["CID"]}',
            "text": f'{name_a} and {name_b}\n{relation.capitalize()} with molecular formula {formula}. {explanation}\nSMILES ({name_a}): {a["SMILES"]}\nSMILES ({name_b}): {b["SMILES"]}',
            "provenance": f'PubChem structure identities {a["CID"]} and {b["CID"]}; explicit stereochemical relation reviewed independently. Original prose and formatting; factual identity/calculated structures only. URLs and checksums in chemistry/source_manifest.json.'})
    write(ROOT / "formulas/isomers/stereochemical/records.json", records)
    write(manifest_path, manifest)
    print("Imported structural pairs; stereochemical pairs:", len(records))


if __name__ == "__main__":
    main()
