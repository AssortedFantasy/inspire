"""Import structural comparisons and empirical formulas from pinned PubChem data."""
import argparse
import hashlib
import itertools
import json
import math
import re
import urllib.parse
from collections import defaultdict
from import_data import BASE, CACHE, ROOT, composition, empirical, fetch, write

STEREO = [
    ("cis-2-butene", "trans-2-butene", "geometric diastereomers (Z / E)"),
    ("maleic acid", "fumaric acid", "geometric diastereomers (Z / E)"),
    ("L-alanine", "D-alanine", "enantiomers (S / R)"),
]
ALDOSES = ROOT / "compounds/organic/biomolecules/carbohydrates/monosaccharides/aldoses"


def structural_record(kind, entries, relation, source):
    formula = entries[0]["MolecularFormula"]
    assert all(entry["MolecularFormula"] == formula for entry in entries)
    assert len({entry["CID"] for entry in entries}) == len(entries)
    assert len({entry["SMILES"] for entry in entries}) == len(entries)
    cids = "-".join(str(entry["CID"]) for entry in entries)
    structures = "\n".join(f'{entry["name"]}: {entry["SMILES"]}' for entry in entries)
    return {
        "id": f"{kind}-{cids}",
        "text": f"{structures}\n\n{formula} — {relation}",
        "provenance": f"PubChem CIDs {cids}; factual identities and source SMILES. {source}",
    }


def constitutional_records(compounds):
    groups = defaultdict(list)
    for entry in compounds:
        if not entry["category"].startswith("organic/") or "biomolecules" in entry["category"]:
            continue
        # These selected nonbiomolecular inputs have no specified stereocenters.
        # Distinct canonical source SMILES identify different connectivity here.
        assert not any(token in entry["SMILES"] for token in "@/\\"), entry["name"]
        groups[entry["MolecularFormula"]].append(entry)
    return [
        structural_record(
            "constitutional", sorted(group, key=lambda entry: entry["CID"]),
            "constitutional isomers", "Pinned inputs in chemistry/source_manifest.json.",
        )
        for group in groups.values() if len(group) > 1
    ]


def empirical_records(compounds):
    records = []
    for entry in compounds:
        if not (entry["category"].startswith("organic/") or entry["category"] == "inorganic/molecules"):
            continue
        formula = entry["MolecularFormula"]
        atoms = composition(formula)
        divisor = math.gcd(*atoms.values())
        if divisor == 1:
            continue
        reduced = empirical(atoms)
        assert composition(reduced) == {element: count // divisor for element, count in atoms.items()}
        assert math.gcd(*composition(reduced).values()) == 1
        records.append({
            "id": f'empirical-{entry["CID"]}',
            "text": f'{entry["name"]} — {formula}\nEmpirical formula: {reduced}',
            "provenance": f'PubChem CID {entry["CID"]}, molecular formula from chemistry/source_manifest.json. Empirical formula reduced by the greatest common divisor of the atom counts.',
        })
    return records


def open_chain_aldoses():
    """Read only completely specified structures, checking each source pin."""
    manifest = json.loads((ALDOSES / "source_manifest.json").read_text())
    selected = json.loads((CACHE / "monosaccharides-aldoses-selected.json").read_text())
    groups = defaultdict(list)
    for entry in selected:
        key = entry["key"]
        payload = (CACHE / (key + ".json")).read_bytes()
        assert hashlib.sha256(payload).hexdigest() == manifest[key]["sha256"], key
        properties = json.loads(payload)["PropertyTable"]["Properties"]
        assert len(properties) == 1 and properties[0] == entry["data"], key
        data = properties[0]
        # Cyclic sources leave the anomer unspecified; don't present those as
        # single fully assigned stereoisomers.
        if "oxane" in data["IUPACName"]:
            continue
        match = re.fullmatch(r"\((\d+[RS](?:,\d+[RS])*)\)-(.+)", data["IUPACName"])
        assert match, data["IUPACName"]
        centers = dict(re.findall(r"(\d+)([RS])", match[1]))
        assert len(centers) == entry["carbon"] - 2
        assert data["SMILES"].count("@H") == len(centers)
        assert data["Charge"] == 0
        assert data["MolecularFormula"] == f'C{entry["carbon"]}H{2 * entry["carbon"]}O{entry["carbon"]}'
        groups[match[2]].append(({**data, "name": entry["name"]}, centers))
    return groups


def aldose_stereo_records():
    records = []
    for group in open_chain_aldoses().values():
        for (a, centers_a), (b, centers_b) in itertools.combinations(group, 2):
            assert centers_a.keys() == centers_b.keys()
            different = [position for position in centers_a if centers_a[position] != centers_b[position]]
            assert different, (a["name"], b["name"])
            if len(different) == len(centers_a):
                relation = "enantiomers"
            elif len(different) == 1:
                relation = f"C{different[0]} epimers"
            else:
                relation = "diastereomers"
            records.append(structural_record(
                "stereo", [a, b], relation + " · open-chain forms",
                "Pinned inputs in chemistry/compounds/organic/biomolecules/carbohydrates/monosaccharides/aldoses/source_manifest.json. "
                "Identical open-chain IUPAC backbones and stereocenter positions; relation determined from the complete R/S assignments.",
            ))
    return records


def emit(path, records, check):
    assert records and len({entry["id"] for entry in records}) == len(records)
    assert len({entry["text"] for entry in records}) == len(records)
    if check:
        assert json.loads(path.read_text(encoding="utf-8")) == records, path
    else:
        write(path, records)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--download", action="store_true")
    parser.add_argument("--check", action="store_true", help="Verify the emitted records without rewriting them")
    args = parser.parse_args()
    manifest_path = ROOT / "source_manifest.json"
    previous = json.loads(manifest_path.read_text())
    manifest = dict(previous)
    compounds = json.loads((CACHE / "selected-compounds.json").read_text())
    # Verify the derived selection cache against the pinned original responses.
    for entry in compounds:
        key = "compound-" + re.sub(r"[^a-z0-9]+", "-", entry["name"].lower()).strip("-")
        values = fetch(key, previous[key]["url"], args.download, previous, manifest)["PropertyTable"]["Properties"]
        assert len(values) == 1 and all(entry[field] == value for field, value in values[0].items()), key
    constitutional = constitutional_records(compounds)
    empirical_formulas = empirical_records(compounds)
    stereochemical = []
    for name_a, name_b, relation in STEREO:
        pair = []
        for name in [name_a, name_b]:
            key = "compound-" + re.sub(r"[^a-z0-9]+", "-", name.lower()).strip("-")
            url = BASE + "compound/name/" + urllib.parse.quote(name, safe="") + "/property/MolecularFormula,MolecularWeight,IUPACName,SMILES,Charge/JSON"
            values = fetch(key, url, args.download, previous, manifest)["PropertyTable"]["Properties"]
            assert len(values) == 1 and values[0]["Charge"] == 0, name
            pair.append({**values[0], "name": name})
        stereochemical.append(structural_record(
            "stereo", pair, relation,
            "Explicit geometrical or alpha-carbon assignments reviewed independently; pinned inputs in chemistry/source_manifest.json.",
        ))
    stereochemical.extend(aldose_stereo_records())
    emit(ROOT / "formulas/composition/empirical-formulas/records.json", empirical_formulas, args.check)
    emit(ROOT / "formulas/isomers/constitutional/records.json", constitutional, args.check)
    emit(ROOT / "formulas/isomers/stereochemical/records.json", stereochemical, args.check)
    if manifest != previous:
        assert not args.check, "New source pins require an import"
        write(manifest_path, manifest)
    print(f"{'Verified' if args.check else 'Imported'} {len(empirical_formulas)} empirical formulas, {len(constitutional)} constitutional comparisons and {len(stereochemical)} stereochemical pairs.")


if __name__ == "__main__":
    main()
