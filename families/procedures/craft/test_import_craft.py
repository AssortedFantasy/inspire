"""Focused source and corpus checks; cache sources with import_craft.py --download."""

from itertools import combinations
import json
import re
import unittest

import import_craft


class CraftImportTests(unittest.TestCase):
    def test_lookup_rejects_missing_ambiguous_or_incomplete_selections(self):
        selection = {"id": "probe", "start": "One.", "paragraphs": 1}
        with self.assertRaises(ValueError):
            import_craft.extract(["Other."], selection)
        with self.assertRaises(ValueError):
            import_craft.extract(["One.", "One. Again."], selection)
        with self.assertRaises(ValueError):
            import_craft.extract(["One."], {**selection, "paragraphs": 2})
        with self.assertRaises(ValueError):
            import_craft.extract(["One."], {**selection, "stop": "Absent."})
        self.assertEqual(import_craft.extract(["One. Next."], {**selection, "stop": " Next."}), "One.")

    def test_pinned_replay_schema_weights_and_no_unresolved_figures(self):
        output = import_craft.imported()
        paths = {p.parent.relative_to(import_craft.ROOT).as_posix()
                 for root in ("bookbinding", "pottery")
                 for p in (import_craft.ROOT / root).rglob("records.json")}
        self.assertEqual(paths, set(output))
        records = []
        for path, rows in output.items():
            self.assertEqual((import_craft.ROOT / path / "records.json").read_text(encoding="utf-8"),
                             json.dumps(rows, ensure_ascii=False, indent=2) + "\n")
            records.extend(rows)
            for row in rows:
                self.assertEqual(set(row), {"id", "text", "provenance"})
                self.assertTrue(all(isinstance(v, str) and v.strip() and "\0" not in v for v in row.values()))
                self.assertIsNone(re.search(r"\bfig\.|\bchapter [IVX]+\b", row["text"], re.IGNORECASE))
        self.assertEqual(len(records), 43)
        self.assertEqual(len(records), len({r["id"] for r in records}))
        self.assertEqual(len(records), len({r["text"] for r in records}))
        for path, weight in import_craft.corpus_weights(output).items():
            self.assertEqual((import_craft.ROOT / path / "node.toml").read_text().strip(), f"weight = {weight}")
        existing = [r["text"] for p in (import_craft.ROOT / "needlework").rglob("records.json")
                    for r in json.loads(p.read_text(encoding="utf-8"))]
        self.assertFalse({r["text"] for r in records}.intersection(existing))

    def test_source_selections_do_not_overlap(self):
        spans = {}
        for key, source in import_craft.MANIFEST["sources"].items():
            parts = import_craft.source_paragraphs((import_craft.DEFAULT_CACHE / source["file"]).read_bytes())
            offsets = [0]
            for part in parts:
                offsets.append(offsets[-1] + len(part) + 2)
            for selection in import_craft.MANIFEST["selections"]:
                if selection["source"] != key:
                    continue
                first, = [i for i, p in enumerate(parts) if p.startswith(selection["start"])]
                text = "\n\n".join(parts[first:first + selection["paragraphs"]])
                start, stop = offsets[first], offsets[first] + len(text)
                if "begin" in selection:
                    start += text.index(selection["begin"])
                if "stop" in selection:
                    stop = offsets[first] + text.index(selection["stop"])
                spans[selection["id"]] = (key, start, stop)
        for (left, a), (right, b) in combinations(spans.items(), 2):
            if a[0] == b[0]:
                self.assertLessEqual(min(a[2], b[2]), max(a[1], b[1]), (left, right))


if __name__ == "__main__":
    unittest.main()
