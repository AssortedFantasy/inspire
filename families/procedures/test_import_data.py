"""Run from the repository root: python -B -m unittest discover -s families/procedures."""
import json
import unittest
from pathlib import Path

from bs4 import BeautifulSoup

import import_data


class ImportTests(unittest.TestCase):
    def test_nested_heading_boundaries_and_ingredient_lines(self):
        html = '<h2>CHAPTER I</h2><h3>First</h3><div><div class="line">½ cup flour</div></div><p>Mix.</p><div><h4>Second</h4></div><p>Bake.</p>'
        sections = list(import_data.farmer_sections(BeautifulSoup(html, "html.parser")))
        self.assertEqual(sections, [("CHAPTER I", "First", ["½ cup flour", "Mix."]), ("CHAPTER I", "Second", ["Bake."])])

    def test_hashes_exact_replay_schema_and_original_preservation(self):
        cache = Path("target/imports/procedures")
        for name in import_data.SOURCES:
            import_data.source(cache, name)
        expected = import_data.baseline()
        expected.update(import_data.imported(cache))
        corpus_paths = {p.parent.relative_to(import_data.ROOT).as_posix() for p in import_data.ROOT.rglob("records.json")}
        self.assertEqual(corpus_paths, set(expected))
        records = []
        for path, rows in expected.items():
            payload = (import_data.ROOT / path / "records.json").read_text(encoding="utf-8")
            self.assertEqual(payload, json.dumps(rows, ensure_ascii=False, indent=2) + "\n")
            records.extend(rows)
            for row in rows:
                self.assertEqual(set(row), {"id", "text", "provenance"})
                self.assertTrue(all(isinstance(value, str) and value.strip() and "\0" not in value for value in row.values()))
        self.assertEqual(len(records), len({row["id"] for row in records}))
        self.assertEqual(len(records), len({row["text"] for row in records}))
        originals = [row["record"] for row in json.loads((import_data.ROOT / "baseline_records.json").read_text(encoding="utf-8"))]
        actual = {row["id"]: row for row in records}
        self.assertEqual(len(originals), 60)
        for row in originals:
            self.assertEqual(actual[row["id"]], row)

    def test_selected_recipes_keep_quantities_and_resolve_foundations(self):
        rows = [r for records in import_data.imported(Path("target/imports/procedures")).values() for r in records]
        by_id = {r["id"]: r for r in rows}
        self.assertIn("1½ tablespoons prepared cocoa", by_id["farmer-1910-iii-breakfast-cocoa"]["text"])
        self.assertIn("Foundation method (tea infusion", by_id["farmer-1910-iii-iced-tea"]["text"])
        self.assertIn("Foundation method (Thin White Sauce)", by_id["farmer-1910-xviii-white-sauce-ii"]["text"])
        self.assertNotIn("various ills", by_id["farmer-1910-iii-how-to-make-tea"]["text"])
        self.assertNotIn("bottled and kept", by_id["farmer-1910-iii-lemonade"]["text"])
        self.assertNotIn("Scalloped Tomatoes", by_id["farmer-1910-xix-stewed-tomatoes"]["text"])
        self.assertIn("Foundation (stem stitch)", by_id["higgin-1880-split-stitch"]["text"])

    def test_tree_has_only_nonempty_unmixed_nodes(self):
        for path in [import_data.ROOT, *[p for p in import_data.ROOT.rglob("*") if p.is_dir()]]:
            children = [p for p in path.iterdir() if p.is_dir()]
            payloads = [p for p in path.iterdir() if p.name in ("records.json", "generator.rs")]
            self.assertTrue(bool(children) != bool(payloads), str(path))
            self.assertLessEqual(len(payloads), 1, str(path))


if __name__ == "__main__":
    unittest.main()
