"""Import 20 standard protein amino-acid identities from pinned PubChem inputs.

Run with Python -B; --download permits fetching missing development cache files.
"""
import argparse
import hashlib
import json
import math
import re
import time
import urllib.parse
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent
CHEMISTRY = ROOT.parents[3]
CACHE = CHEMISTRY.parents[1] / "target" / "chemistry-sources"
BASE = "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/"
SELECTION = {
    "aliphatic": "glycine|L-alanine|L-valine|L-leucine|L-isoleucine|L-proline",
    "aromatic": "L-phenylalanine|L-tyrosine|L-tryptophan",
    "hydroxyl": "L-serine|L-threonine",
    "sulfur": "L-cysteine|L-methionine",
    "amides": "L-asparagine|L-glutamine",
    "acidic": "L-aspartic acid|L-glutamic acid",
    "basic": "L-lysine|L-arginine|L-histidine",
}
EXPECTED_CIDS = {
    "glycine": 750, "L-alanine": 5950, "L-valine": 6287,
    "L-leucine": 6106, "L-isoleucine": 6306, "L-proline": 145742,
    "L-phenylalanine": 6140, "L-tyrosine": 6057, "L-tryptophan": 6305,
    "L-serine": 5951, "L-threonine": 6288, "L-cysteine": 5862,
    "L-methionine": 6137, "L-asparagine": 6267, "L-glutamine": 5961,
    "L-aspartic acid": 5960, "L-glutamic acid": 33032,
    "L-lysine": 5962, "L-arginine": 6322, "L-histidine": 6274,
}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--download", action="store_true")
    args = parser.parse_args()
    manifest_path = ROOT / "source_manifest.json"
    previous = json.loads(manifest_path.read_text()) if manifest_path.exists() else {}
    shared = json.loads((CHEMISTRY / "source_manifest.json").read_text())
    manifest = {}
    seen = set()
    for category, names in SELECTION.items():
        records = []
        for name in names.split("|"):
            key = "compound-" + re.sub(r"[^a-z0-9]+", "-", name.lower()).strip("-")
            url = BASE + urllib.parse.quote(name, safe="") + "/property/MolecularFormula,MolecularWeight,IUPACName,SMILES,Charge/JSON"
            cache = CACHE / (key + ".json")
            if args.download and not cache.exists():
                time.sleep(0.5)
                with urllib.request.urlopen(url, timeout=60) as response:
                    payload = response.read()
                cache.parent.mkdir(parents=True, exist_ok=True)
                cache.write_bytes(payload)
            payload = cache.read_bytes()
            digest = hashlib.sha256(payload).hexdigest()
            pin = previous.get(key, shared.get(key))
            if pin is not None and (pin["sha256"] != digest or pin["url"] != url):
                raise ValueError(f"Changed source: {key}")
            manifest[key] = {"url": url, "sha256": digest}
            properties = json.loads(payload)["PropertyTable"]["Properties"]
            assert len(properties) == 1, name
            data = properties[0]
            cid = data["CID"]
            assert cid == EXPECTED_CIDS[name] and cid not in seen, (name, cid)
            seen.add(cid)
            assert data["Charge"] == 0 and "[" not in data["SMILES"].replace("[C@@H]", "").replace("[C@H]", ""), name
            stereo = {"L-cysteine": "(2R)", "L-isoleucine": "(2S,3S)", "L-threonine": "(2S,3R)"}.get(name, "(2S)")
            if name == "glycine":
                assert "@" not in data["SMILES"]
                stereo_text = "Glycine is achiral."
            else:
                assert data["IUPACName"].startswith(stereo), (name, data["IUPACName"])
                assert data["SMILES"].count("@") > 0, name
                stereo_text = f"L stereoisomer; IUPAC name: {data['IUPACName']}."
            formula = data["MolecularFormula"]
            atoms = {element: int(count or 1) for element, count in re.findall(r"([A-Z][a-z]?)(\d*)", formula)}
            assert "".join(e + (str(n) if n != 1 else "") for e, n in atoms.items()) == formula
            divisor = math.gcd(*atoms.values())
            empirical = "".join(e + (str(n // divisor) if n // divisor != 1 else "") for e, n in atoms.items())
            inventory = ", ".join(f"{e}: {n}" for e, n in atoms.items())
            formula_line = f"Molecular formula: {formula}"
            if empirical != formula:
                formula_line += f"; empirical formula: {empirical}"
            text = f"{name}\n{formula_line}.\nMolar mass (calculated): {data['MolecularWeight']} g/mol.\nAtoms per molecule: {inventory}.\n{stereo_text}\nSMILES: {data['SMILES']}\nNeutral structural representation; protonation in solution depends on pH."
            records.append({"id": f"pubchem-cid-{cid}", "text": text,
                "provenance": f"Selected factual identity and PubChem-calculated composition/molar mass/SMILES; {url}; CID {cid}; NCBI policy https://www.ncbi.nlm.nih.gov/home/about/policies/; original formatting and side-chain mapping. No third-party descriptive annotations. See chemistry/LICENSE.md and amino-acids/source_manifest.json."})
        destination = ROOT / category / "records.json"
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_text(json.dumps(records, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    assert len(seen) == 20
    manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    print("Imported 20 amino acids into 7 side-chain leaves.")


if __name__ == "__main__":
    main()
