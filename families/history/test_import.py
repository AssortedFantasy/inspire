"""Source replay and corruption rejection for history."""

import json
import importlib.util
from pathlib import Path
import tempfile
import unittest

from import_data import ROOT, Paragraphs, render


def branch_importer(branch):
    spec = importlib.util.spec_from_file_location(branch.replace("-", "_"), ROOT / branch / "import_data.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


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

    def test_all_records_replay(self):
        outputs = render(Path("target/imports/history"))
        voices = branch_importer("everyday-voices").render(Path("target/imports/everyday-voices"))
        outputs.update({"everyday-voices/" + leaf: rows for leaf, rows in voices.items()})
        work = branch_importer("work-observations").render(Path("target/imports/work-observations"))
        outputs["work-observations/apollo-12"] = json.loads(work)
        for leaf, expected in outputs.items():
            actual = json.loads((ROOT / leaf / "records.json").read_text(encoding="utf-8"))
            self.assertEqual(actual, expected, leaf)
        self.assertEqual(
            {ROOT / leaf / "records.json" for leaf in outputs},
            set(ROOT.rglob("records.json")),
        )
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
