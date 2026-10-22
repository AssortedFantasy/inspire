"""Offline checks of pinned inputs and emitted records; no third-party modules."""
import hashlib
import json
import re
from pathlib import Path
from import_data import CACHE, ROOT, CONVENTIONAL_FORMULAS, composition, empirical


def check_record_weights(branch):
    records = branch / "records.json"
    if records.exists():
        return len(json.loads(records.read_text(encoding="utf-8")))
    total = 0
    for child in (path for path in branch.iterdir() if path.is_dir()):
        count = check_record_weights(child)
        assert (child / "node.toml").read_text().strip() == f"weight = {count}", child
        total += count
    return total


def check_reaction_weights(branch):
    generator = branch / "generator.rs"
    if generator.exists():
        count = generator.read_text(encoding="utf-8").count("Reaction {")
        assert count > 0, generator
        return count
    total = 0
    for child in (path for path in branch.iterdir() if path.is_dir()):
        count = check_reaction_weights(child)
        assert (child / "node.toml").read_text().strip() == f"weight = {count}", child
        total += count
    return total


def main():
    manifest = {}
    for path in ROOT.rglob("source_manifest.json"):
        for key, entry in json.loads(path.read_text()).items():
            assert key not in manifest or manifest[key] == entry, (path, key)
            manifest[key] = entry
    for key, entry in manifest.items():
        assert hashlib.sha256((CACHE / (key + ".json")).read_bytes()).hexdigest() == entry["sha256"], key
    table = json.loads((CACHE / "periodic-table.json").read_text())["Table"]
    z_values = set()
    cores = {"He": 2, "Ne": 10, "Ar": 18, "Kr": 36, "Xe": 54, "Rn": 86}
    for row in table["Row"]:
        data = dict(zip(table["Columns"]["Column"], row["Cell"], strict=True))
        z = int(data["AtomicNumber"])
        assert z not in z_values
        z_values.add(z)
        config = re.sub(r" \((calculated|predicted)\)$", "", data["ElectronConfiguration"]).replace(" ", "")
        prefix = re.match(r"\[([A-Za-z]+)\]", config)
        count = cores[prefix[1]] if prefix else 0
        suffix = config[prefix.end():] if prefix else config
        orbitals = re.findall(r"(\d)([spdf])(\d+?)(?=\d[spdf]|$)", suffix)
        assert "".join("".join(o) for o in orbitals) == suffix, config
        for shell, orbital, n in orbitals:
            assert 0 < int(n) <= {"s": 2, "p": 6, "d": 10, "f": 14}[orbital], config
            assert int(shell) >= {"s": 1, "p": 2, "d": 3, "f": 4}[orbital], config
            count += int(n)
        assert count == z, (data["Name"], config, count, z)
    assert z_values == set(range(1, 119))
    data = json.loads((CACHE / "selected-compounds.json").read_text())
    formulas = {d["name"]: d["MolecularFormula"] for d in data}
    assert formulas["water"] == "H2O"
    assert formulas["benzene"] == "C6H6"
    assert formulas["glycine"] == "C2H5NO2"
    assert formulas["sodium chloride"] == "ClNa"  # PubChem Hill order, not NaCl.
    for name, formula in CONVENTIONAL_FORMULAS.items():
        assert composition(formula) == composition(formulas[name]), name
    assert empirical(composition("C6H12O6")) == "CH2O"
    assert empirical(composition("C2H6O")) == "C2H6O"
    assert check_record_weights(ROOT / "elements") == 118
    compound_count = check_record_weights(ROOT / "compounds")
    assert check_reaction_weights(ROOT / "reactions") == 87
    texts = set()
    count = 0
    for path in ROOT.rglob("records.json"):
        records = json.loads(path.read_text(encoding="utf-8"))
        assert records, path
        ids = set()
        for record in records:
            assert set(record) == {"id", "text", "provenance"}, path
            assert all(isinstance(value, str) and value.strip() and "\0" not in value for value in record.values()), path
            assert record["id"] not in ids, path
            assert record["text"] not in texts, path
            ids.add(record["id"])
            texts.add(record["text"])
            count += 1
    print(f"Verified {len(manifest)} source hashes, 118 electron configurations, {compound_count} compound records, 87 reaction weights and {count} unique corpus records.")


if __name__ == "__main__":
    main()
