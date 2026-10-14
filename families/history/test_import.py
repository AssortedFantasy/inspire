"""Source replay, corruption rejection and preservation checks for history."""

import json
from pathlib import Path
import tempfile
import unittest

from import_data import ROOT, Paragraphs, render


class SourceTests(unittest.TestCase):
    def test_inline_locators_do_not_pollute_the_quote(self):
        parser = Paragraphs()
        parser.feed('<p>A well<span class="tei tei-pb">[pg 2]</span> has '
                    '<a id="noteref_3"><span class="tei tei-noteref">'
                    '<span>3</span></span></a>a bucket.</p>')
        self.assertEqual(parser.entries, [("p", {}, "A well has a bucket.")])

    def test_source_corruption_fails_before_importing(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            first = json.loads((ROOT / "sources.json").read_text(encoding="utf-8"))[0]
            (path / f'{first["book"]}.html').write_bytes(b"changed source")
            with self.assertRaisesRegex(ValueError, "Checksum mismatch"):
                render(path)

    def test_all_selections_replay_and_original_ids_survive(self):
        outputs = render(Path("target/imports/history"))
        for leaf, expected in outputs.items():
            actual = json.loads((ROOT / leaf / "records.json").read_text(encoding="utf-8"))
            self.assertEqual(actual, expected, leaf)
        originals = json.loads((ROOT / "original-records.json").read_text(encoding="utf-8"))
        self.assertEqual(sum(map(len, originals.values())), 72)
        for leaf, ids in originals.items():
            actual = json.loads((ROOT / leaf / "records.json").read_text(encoding="utf-8"))
            self.assertEqual(set(ids), {record["id"] for record in actual})
        ids, texts = set(), set()
        for file in ROOT.rglob("records.json"):
            for record in json.loads(file.read_text(encoding="utf-8")):
                self.assertEqual(set(record), {"id", "text", "provenance"})
                self.assertTrue(all(isinstance(x, str) and x.strip() for x in record.values()))
                self.assertNotIn("\0", record["text"])
                self.assertNotIn(record["id"], ids)
                self.assertNotIn(record["text"], texts)
                ids.add(record["id"])
                texts.add(record["text"])


if __name__ == "__main__":
    unittest.main()
