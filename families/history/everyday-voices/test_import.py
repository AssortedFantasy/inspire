"""Verify source integrity, excerpt integrity, and exact replay."""

import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import import_data


class SourceTests(unittest.TestCase):
    def test_inline_locators_preserve_surrounding_words(self):
        parser = import_data.Paragraphs()
        parser.feed('<p>A<span class="pagenum">[Pg 2]</span> letter '
                    '<sup><a>1</a></sup>arrived.</p>')
        self.assertEqual(parser.entries, [("p", {}, "A letter arrived.")])

    def test_corrupt_source_is_rejected(self):
        with tempfile.TemporaryDirectory(dir="target") as directory:
            cache = Path(directory)
            (cache / "sancho.html").write_bytes(b"corrupt")
            with self.assertRaisesRegex(ValueError, "Changed source"):
                import_data.render(cache)

    def test_altered_selection_is_rejected(self):
        root = import_data.ROOT
        with tempfile.TemporaryDirectory(dir="target") as directory:
            temporary = Path(directory)
            (temporary / "sources.json").write_bytes((root / "sources.json").read_bytes())
            selections = json.loads((root / "selections.json").read_text(encoding="utf-8"))
            selections[0]["end"] -= 1
            (temporary / "selections.json").write_text(json.dumps(selections), encoding="utf-8")
            with patch.object(import_data, "ROOT", temporary):
                with self.assertRaisesRegex(ValueError, "Changed excerpt"):
                    import_data.render(Path("target/imports/everyday-voices"))

    def test_records_replay_and_cover_only_owned_leaves(self):
        root = import_data.ROOT
        outputs = import_data.render(Path("target/imports/everyday-voices"))
        self.assertEqual(
            {root / leaf / "records.json" for leaf in outputs},
            set(root.rglob("records.json")),
        )
        for leaf, records in outputs.items():
            self.assertEqual(
                json.loads((root / leaf / "records.json").read_text(encoding="utf-8")),
                records,
            )


if __name__ == "__main__":
    unittest.main()
