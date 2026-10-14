"""Import a pinned PubChem snapshot; downloads are development-only.

python -B families/chemistry/import_data.py --download
python -B families/chemistry/import_data.py
The first import creates source_manifest.json; later downloads verify its hashes.
"""
import argparse
import hashlib
import json
import math
import re
import time
import urllib.parse
import urllib.request
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent
CACHE = ROOT.parents[1] / "target" / "chemistry-sources"
BASE = "https://pubchem.ncbi.nlm.nih.gov/rest/pug/"
SELECTION = {
    "organic/hydrocarbons/acyclic": "methane|ethane|propane|butane|isobutane|pentane|isopentane|neopentane|hexane|heptane|octane|ethylene|propylene|1-butene|acetylene|propyne|1-butyne",
    "organic/hydrocarbons/cyclic": "cyclopropane|cyclobutane|cyclopentane|cyclohexane|benzene|toluene|ethylbenzene|naphthalene|anthracene|phenanthrene",
    "organic/oxygen/alcohols-and-phenols": "methanol|ethanol|1-propanol|2-propanol|1-butanol|ethylene glycol|glycerol|phenol|benzyl alcohol",
    "organic/oxygen/carbonyls": "formaldehyde|acetaldehyde|propanal|butanal|benzaldehyde|acetone|2-butanone|cyclohexanone",
    "organic/oxygen/carboxylic-acids": "formic acid|acetic acid|propionic acid|butyric acid|benzoic acid|oxalic acid|citric acid|succinic acid",
    "organic/oxygen/esters-and-ethers": "methyl formate|ethyl acetate|methyl acetate|dimethyl ether|diethyl ether|tetrahydrofuran|anisole|dioxane",
    "organic/nitrogen/amines-and-amides": "methylamine|ethylamine|dimethylamine|trimethylamine|aniline|urea|acetamide|formamide",
    "organic/nitrogen/heterocycles": "pyridine|pyrrole|imidazole|indole|pyrimidine|purine|caffeine",
    "organic/biomolecules/amino-acids": "glycine|L-alanine|L-valine|L-leucine|L-serine|L-cysteine|L-lysine|L-tryptophan",
    "organic/biomolecules/carbohydrates": "D-glucose|D-fructose|sucrose|lactose|maltose|D-ribose",
    "inorganic/molecules": "water|ammonia|carbon monoxide|carbon dioxide|sulfur dioxide|hydrogen sulfide|carbon disulfide|nitric oxide|hydrogen peroxide",
    "inorganic/ionic-solids": "sodium chloride|potassium chloride|calcium carbonate|sodium bicarbonate|sodium nitrate|ammonium chloride|magnesium oxide|calcium oxide",
}


def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def fetch(key, url, download, previous, manifest):
    path = CACHE / (key + ".json")
    if download and not path.exists():
        time.sleep(0.4)  # Under three requests/second; one sequential download.
        with urllib.request.urlopen(url, timeout=60) as response:
            payload = response.read()
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(payload)
    if not path.exists():
        raise RuntimeError(f"Missing {path}; run with --download")
    payload = path.read_bytes()
    digest = hashlib.sha256(payload).hexdigest()
    if key in previous and previous[key]["sha256"] != digest:
        raise RuntimeError(f"Source changed: {key}; explicitly review a new snapshot")
    manifest[key] = {"url": url, "sha256": digest}
    return json.loads(payload)


def composition(formula):
    tokens = re.findall(r"([A-Z][a-z]?)(\d*)", formula)
    assert "".join(element + count for element, count in tokens) == formula, formula
    result = defaultdict(int)
    for element, count in tokens:
        result[element] += int(count or 1)
    return dict(result)


def empirical(atoms):
    divisor = math.gcd(*atoms.values())
    return "".join(e + (str(n // divisor) if n // divisor != 1 else "") for e, n in atoms.items())


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--download", action="store_true")
    args = parser.parse_args()
    manifest_path = ROOT / "source_manifest.json"
    previous = json.loads(manifest_path.read_text()) if manifest_path.exists() else {}
    manifest = dict(previous)
    table = fetch("periodic-table", BASE + "periodictable/JSON", args.download, previous, manifest)["Table"]
    categories = defaultdict(list)
    for row in table["Row"]:
        data = dict(zip(table["Columns"]["Column"], row["Cell"], strict=True))
        group = data["GroupBlock"]
        z = int(data["AtomicNumber"])
        category = {
            "Alkali metal": "main-group/metals", "Alkaline earth metal": "main-group/metals",
            "Post-transition metal": "main-group/metals", "Metalloid": "main-group/metalloids-and-nonmetals",
            "Nonmetal": "main-group/metalloids-and-nonmetals", "Halogen": "main-group/halogens",
            "Noble gas": "main-group/noble-gases", "Transition metal": "transition-metals",
            "Lanthanide": "inner-transition/lanthanides", "Actinide": "inner-transition/actinides",
        }[group]
        text = f'{data["Name"]} ({data["Symbol"]})\nAtomic number: {z}; {z} protons per nucleus and {z} electrons in a neutral atom.\nClassification: {group}.'
        if data["ElectronConfiguration"]:
            text += f'\nNeutral-atom electron configuration: {data["ElectronConfiguration"]}.'
        if z >= 104:
            text += " Configurations for these short-lived superheavy elements are theoretical assignments."
        categories[category].append({"id": f"pubchem-element-{z}", "text": text,
            "provenance": "PubChem periodic table numerical/identity facts, " + manifest["periodic-table"]["url"] + "; NCBI/NLM acknowledgement; no third-party prose imported. See chemistry/LICENSE.md and source_manifest.json."})
    assert sum(map(len, categories.values())) == 118
    for category, rows in categories.items():
        write(ROOT / "elements" / category / "records.json", rows)
    cids = set()
    compounds = []
    for category, names in SELECTION.items():
        records_by_category = defaultdict(list)
        for name in names.split("|"):
            key = "compound-" + re.sub(r"[^a-z0-9]+", "-", name.lower()).strip("-")
            url = BASE + "compound/name/" + urllib.parse.quote(name, safe="") + "/property/MolecularFormula,MolecularWeight,IUPACName,SMILES,Charge/JSON"
            response = fetch(key, url, args.download, previous, manifest)
            properties = response["PropertyTable"]["Properties"]
            assert len(properties) == 1, (name, "Ambiguous name")
            data = properties[0]
            cid = data["CID"]
            assert cid not in cids, (name, cid, "Duplicate compound")
            cids.add(cid)
            assert data["Charge"] == 0, name
            formula = data["MolecularFormula"]
            atoms = composition(formula)
            unit = "formula unit" if category == "inorganic/ionic-solids" else "molecule"
            label = "Composition formula" if unit == "formula unit" else "Molecular formula"
            inventory = ", ".join(f"{element}: {count}" for element, count in atoms.items())
            ratio = empirical(atoms)
            formula_line = f'{label}: {formula}'
            if ratio != formula:
                formula_line += f'; empirical formula: {ratio}'
            text = f'{name}\n{formula_line}.\nMolar mass (calculated): {data["MolecularWeight"]} g/mol.\nAtoms per {unit}: {inventory}.\nSMILES: {data["SMILES"]}'
            if unit == "formula unit":
                text += " Ionic solids form extended lattices; the formula gives the empirical formula unit."
            leaf_category = category
            if category == "organic/hydrocarbons/cyclic":
                leaf_category += "/saturated" if name.startswith("cyclo") else "/aromatic"
            elif category == "organic/oxygen/esters-and-ethers":
                leaf_category = "organic/oxygen/" + ("esters" if name in ["methyl formate", "ethyl acetate", "methyl acetate"] else "ethers")
            elif category == "organic/nitrogen/amines-and-amides":
                leaf_category = "organic/nitrogen/" + ("amides" if name in ["urea", "acetamide", "formamide"] else "amines")
            records_by_category[leaf_category].append({"id": f"pubchem-cid-{cid}", "text": text, "provenance": f"Selected factual identity and PubChem-calculated composition/molar mass/SMILES; {url}; CID {cid}; NCBI policy https://www.ncbi.nlm.nih.gov/home/about/policies/; original formatting and class mapping. No third-party descriptive annotations. See chemistry/LICENSE.md and source_manifest.json."})
            compounds.append({"name": name, "category": leaf_category, **data})
        for leaf_category, records in records_by_category.items():
            write(ROOT / "compounds" / leaf_category / "records.json", records)
    write(manifest_path, manifest)
    write(CACHE / "selected-compounds.json", compounds)
    print(f"Imported 118 elements and {len(compounds)} compounds; 22 corpus leaves.")


if __name__ == "__main__":
    main()
