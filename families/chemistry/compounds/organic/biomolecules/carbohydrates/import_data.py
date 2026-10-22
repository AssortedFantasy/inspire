"""Regenerate carbohydrate records from pinned development-only PubChem caches."""
import argparse
import hashlib
import json
import re
import subprocess
import sys
import time
import urllib.parse
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent
REPO = next(parent for parent in ROOT.parents if (parent / "Cargo.toml").exists())
CACHE = REPO / "target" / "chemistry-sources"
BASE = "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/"
PROPERTIES = "/property/MolecularFormula,MolecularWeight,IUPACName,SMILES,Charge/JSON"
SELECTION = {
    "monosaccharides/ketoses": {"D-fructose": 2723872},
    "oligosaccharides": {"sucrose": 5988, "lactose": 6134, "maltose": 439186},
}


def write(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def merge(*manifests):
    merged = {}
    for manifest in manifests:
        for key, pin in manifest.items():
            if key in merged and pin != merged[key]:
                raise ValueError(f"Conflicting source pin: {key}")
            merged[key] = pin
    return merged


def import_records(category, download):
    previous = json.loads((ROOT / "source_manifest.json").read_text(encoding="utf-8"))
    records, manifest = [], {}
    for name, cid in SELECTION[category].items():
        key = "compound-" + re.sub(r"[^a-z0-9]+", "-", name.lower()).strip("-")
        url = BASE + urllib.parse.quote(name, safe="") + PROPERTIES
        cache = CACHE / (key + ".json")
        if download and not cache.exists():
            time.sleep(0.5)
            with urllib.request.urlopen(url, timeout=60) as response:
                payload = response.read()
            cache.parent.mkdir(parents=True, exist_ok=True)
            cache.write_bytes(payload)
        payload = cache.read_bytes()
        pin = {"url": url, "sha256": hashlib.sha256(payload).hexdigest()}
        if previous.get(key) != pin:
            raise ValueError(f"Changed or unpinned source: {key}")
        manifest[key] = pin
        properties = json.loads(payload)["PropertyTable"]["Properties"]
        assert len(properties) == 1, name
        data = properties[0]
        assert data["CID"] == cid and data["Charge"] == 0, (name, data)
        formula = "C6H12O6" if name == "D-fructose" else "C12H22O11"
        mass = 180.156 if name == "D-fructose" else 342.297
        assert data["MolecularFormula"] == formula, name
        assert abs(float(data["MolecularWeight"]) - mass) < 0.01, name
        text = f"{name} ({formula})\n"
        if name == "D-fructose":
            assert data["IUPACName"].startswith("(3S,4R,5R)"), name
            text += "Pyranose, anomer unspecified\n"
        text += f"Molar mass: {float(data['MolecularWeight']):g} g/mol\nSMILES: {data['SMILES']}"
        records.append({
            "id": f"pubchem-cid-{cid}", "text": text,
            "provenance": f"Selected factual identity and PubChem-calculated composition/molar mass/SMILES; {url}; CID {cid}; NCBI policy https://www.ncbi.nlm.nih.gov/home/about/policies/; original formatting and class mapping. No third-party descriptive annotations. See chemistry/LICENSE.md and carbohydrates/source_manifest.json.",
        })
    write(ROOT / category / "records.json", records)
    write(ROOT / category / "source_manifest.json", manifest)
    return manifest


def import_monosaccharides(download):
    root = ROOT / "monosaccharides"
    command = [sys.executable, "-B", str(root / "aldoses" / "import_data.py")]
    if download:
        command.append("--download")
    subprocess.run(command, check=True)
    aldoses = json.loads((root / "aldoses" / "source_manifest.json").read_text(encoding="utf-8"))
    ketoses = import_records("monosaccharides/ketoses", download)
    manifest = merge(aldoses, ketoses)
    write(root / "source_manifest.json", manifest)
    return manifest


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--download", action="store_true")
    parser.add_argument("--monosaccharides-only", action="store_true")
    args = parser.parse_args()
    manifest = import_monosaccharides(args.download)
    if not args.monosaccharides_only:
        manifest = merge(manifest, import_records("oligosaccharides", args.download))
        write(ROOT / "source_manifest.json", manifest)
    print(f"Imported carbohydrate sources: {len(manifest)} pinned responses.")


if __name__ == "__main__":
    main()
