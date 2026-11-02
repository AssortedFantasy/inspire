"""Run from the repository root: python -B -m unittest discover -s families/procedures."""
import json
import unittest
from fractions import Fraction
from pathlib import Path

from bs4 import BeautifulSoup

import import_data
from craft import import_craft


def all_corpora():
    original = import_data.imported(Path("target/imports/procedures"))
    additional = import_craft.imported(Path("target/imports/craft"))
    return {**original, **{"craft/" + path: rows for path, rows in additional.items()}}


class ImportTests(unittest.TestCase):
    def test_nested_heading_boundaries_and_ingredient_lines(self):
        html = '<h2>CHAPTER I</h2><h3>First</h3><div><div class="line">½ cup flour</div></div><p>Mix.</p><div><h4>Second</h4></div><p>Bake.</p>'
        sections = list(import_data.farmer_sections(BeautifulSoup(html, "html.parser")))
        self.assertEqual(sections, [("CHAPTER I", "First", ["½ cup flour", "Mix."]), ("CHAPTER I", "Second", ["Bake."])])

    def test_hashes_exact_replay_schema_and_unique_records(self):
        cache = Path("target/imports/procedures")
        for name in import_data.SOURCES:
            import_data.source(cache, name)
        expected = all_corpora()
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

    def test_excerpt_lookup_rejects_ambiguous_or_missing_boundaries(self):
        with self.assertRaises(ValueError):
            import_data.selected(["Same text.", "Same text again."], "Same")
        with self.assertRaises(ValueError):
            import_data.selected(["Complete sentence."], "Missing")
        with self.assertRaises(ValueError):
            import_data.selected(["Complete sentence."], "Complete", stop="Absent")
        self.assertEqual(import_data.selected(["First. Next.", "Second."], "First", stop=" Next."), "First.")

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

    def test_replacements_retain_primary_source_words(self):
        cache = Path("target/imports/procedures")
        groups = {**import_data.sewing(cache), **import_data.experiments(cache)}
        by_id = {r["id"]: r for records in groups.values() for r in records}
        self.assertIn("Pass the needle in and out of the material, at regular intervals", by_id["dillmont-running-stitch"]["text"])
        self.assertIn("the two raw edges are enclosed between", by_id["dillmont-french-double-seam"]["text"])
        self.assertIn("leave a small hole about as large as a pin’s head", by_id["tissandier-ludion"]["text"])
        self.assertNotIn("candle", by_id["tissandier-hidden-coin"]["text"])
        self.assertNotIn("Cramer", by_id["tissandier-two-pins"]["text"])
        for row in by_id.values():
            self.assertIn("Verbatim public-domain", row["provenance"])

    def test_record_weights_are_uniform_within_each_corpus_topic(self):
        output = all_corpora()
        weights = import_data.corpus_weights(output)
        self.assertTrue(all(not path.startswith("algorithms/") for path in weights))
        for path, weight in weights.items():
            self.assertEqual((import_data.ROOT / path / "node.toml").read_text().strip(), f"weight = {weight}")
        for topic in ("cooking", "craft", "experiments"):
            total = sum(len(rows) for path, rows in output.items() if path.startswith(topic + "/"))
            for path, rows in output.items():
                if not path.startswith(topic + "/"):
                    continue
                probability = Fraction(1, len(rows))
                parts = path.split("/")
                for length in range(2, len(parts) + 1):
                    node = "/".join(parts[:length])
                    parent = node.rsplit("/", 1)[0]
                    sibling_total = sum(weight for sibling, weight in weights.items() if sibling.rsplit("/", 1)[0] == parent)
                    probability *= Fraction(weights[node], sibling_total)
                self.assertEqual(probability, Fraction(1, total), path)

    def test_tree_has_only_nonempty_unmixed_nodes(self):
        for path in [import_data.ROOT, *[p for p in import_data.ROOT.rglob("*") if p.is_dir()]]:
            children = [p for p in path.iterdir() if p.is_dir()]
            payloads = [p for p in path.iterdir() if p.name in ("records.json", "generator.rs")]
            self.assertTrue(bool(children) != bool(payloads), str(path))
            self.assertLessEqual(len(payloads), 1, str(path))


if __name__ == "__main__":
    unittest.main()
