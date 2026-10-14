"""Check compact profile formatting, exact replay and pinned-source safeguards."""

import contextlib
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("people_import", ROOT / "import_data.py")
IMPORTER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(IMPORTER)


def payloads(root):
    return {file.relative_to(root).as_posix(): hashlib.sha256(file.read_bytes()).hexdigest()
            for filename in ("records.json", "node.toml") for file in root.rglob(filename)}


def main():
    before = payloads(ROOT)
    with contextlib.redirect_stdout(io.StringIO()):
        IMPORTER.build()
    assert payloads(ROOT) == before, "Rebuild changed current payloads"
    records = [record for file in ROOT.rglob("records.json") for record in json.loads(file.read_text("utf-8"))]
    assert records and all(" - " in record["text"] for record in records), "Profile separator changed"
    assert all("\n" not in record["text"] for record in records), "Profile text contains a footer"
    assert all(not any(marker in record["text"] for marker in ("AgeDataset", "CC BY", "http", "Source:"))
               for record in records), "Source boilerplate leaked into sampled text"
    cache = IMPORTER.CACHE
    cache.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="verify-", dir=cache) as temporary:
        fixture = Path(temporary).resolve()
        assert fixture.is_relative_to(cache.resolve())
        corrupt_cache = fixture / "corrupt-cache"
        corrupt_cache.mkdir()
        manifest = json.loads((ROOT / "sources.json").read_text("utf-8"))
        first_source = sorted(manifest)[0]
        (corrupt_cache / first_source).write_bytes(b"tampered source")
        IMPORTER.CACHE = corrupt_cache
        try:
            next(IMPORTER.sources())
            raise AssertionError("Corrupt source was accepted")
        except ValueError as error:
            assert "Checksum mismatch" in str(error)
        finally:
            IMPORTER.CACHE = cache
        assert payloads(ROOT) == before, "Checksum failure changed payloads"

        isolated = fixture / "isolated-family"
        isolated.mkdir()
        for filename in ("sources.json", "selection.json", "curated.json", "corrections.json", "node.toml"):
            (isolated / filename).write_bytes((ROOT / filename).read_bytes())
        selection = json.loads((ROOT / "selection.json").read_text("utf-8"))
        removed = next(iter(selection.values()))[0]
        corrections = json.loads((isolated / "corrections.json").read_text("utf-8"))
        corrections[removed] = {"exclude": True, "reason": "Fixture-only exclusion", "source": "test fixture"}
        (isolated / "corrections.json").write_text(json.dumps(corrections), "utf-8")
        IMPORTER.ROOT = isolated
        with contextlib.redirect_stdout(io.StringIO()):
            IMPORTER.build()
        actual_ids = {record["id"] for file in isolated.rglob("records.json")
                      for record in json.loads(file.read_text("utf-8"))}
        expected_ids = {record["id"] for file in ROOT.rglob("records.json")
                        for record in json.loads(file.read_text("utf-8"))}
        assert actual_ids == expected_ids - {"wikidata-" + removed.lower()}, "Exclusion introduced replacements"
        def count(node):
            corpus = node / "records.json"
            total = len(json.loads(corpus.read_text("utf-8"))) if corpus.exists() else sum(count(child) for child in node.iterdir() if child.is_dir())
            if node != isolated:
                assert tomllib.loads((node / "node.toml").read_text("utf-8"))["weight"] == total
            return total
        assert count(isolated) == len(expected_ids) - 1
        IMPORTER.ROOT = ROOT
    print("Compact profiles replay exactly; source checksums refuse corrupted inputs; exclusions add no replacements; descendant-count weights verified.")


if __name__ == "__main__":
    main()
