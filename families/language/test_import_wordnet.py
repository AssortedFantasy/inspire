"""Source and replay checks; uses the importer's pinned development cache."""

from contextlib import redirect_stdout
import hashlib
import io
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import import_wordnet as wordnet


REPO = Path(__file__).resolve().parents[2]
ARCHIVE = REPO / "target/imports/wordnet/WordNet-3.0.tar.gz"


class WordNetSourceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        if not ARCHIVE.exists():
            raise unittest.SkipTest("Run import_wordnet.py to populate the pinned archive cache")
        if hashlib.sha256(ARCHIVE.read_bytes()).hexdigest() != wordnet.SHA256:
            raise ValueError("WordNet archive checksum mismatch")
        cls.synsets, _ = wordnet.load(ARCHIVE)

    def test_selected_senses_preserve_source_words_and_glosses(self):
        for path in (wordnet.ROOT / "lexicon/senses").rglob("records.json"):
            for record in json.loads(path.read_text(encoding="utf-8")):
                _, pos, offset = record["id"].split("-")
                synset = self.synsets[(pos, offset)]
                heading, definition = record["text"].split("\n", 1)
                words = heading.rsplit(" (", 1)[0].split(" / ")
                with self.subTest(id=record["id"]):
                    self.assertEqual(definition, synset["definition"])
                    self.assertTrue(set(words) <= set(synset["words"]))
                    self.assertTrue(any(synset["counts"][word.lower()] for word in words))

    def test_antonyms_preserve_exact_lexical_pointers(self):
        for path in (wordnet.ROOT / "lexicon/relations/antonyms").rglob("records.json"):
            for record in json.loads(path.read_text(encoding="utf-8")):
                _, _, pos, first, second, _ = record["id"].split("-")
                endpoints = [self.synsets[(pos, offset)] for offset in (first, second)]
                lines = [line.split(" — ", 1) for line in record["text"].splitlines()]
                linked = False
                for source, target in (endpoints, endpoints[::-1]):
                    for symbol, offset, target_pos, indices in source["pointers"]:
                        if symbol != "!" or indices == "0000" or offset != target["offset"]:
                            continue
                        words = (source["words"][int(indices[:2], 16) - 1],
                                 target["words"][int(indices[2:], 16) - 1])
                        expected = [[words[0], source["definition"]], [words[1], target["definition"]]]
                        linked |= lines == expected or lines == expected[::-1]
                with self.subTest(id=record["id"]):
                    self.assertTrue(linked)

    def test_reviewed_scope_preserves_meaningful_senses(self):
        records = {record["id"]: record for path in (wordnet.ROOT / "lexicon").rglob("records.json")
                   for record in json.loads(path.read_text(encoding="utf-8"))}
        for excluded in ("wn30-n-13846900", "wn30-n-13846199", "wn30-r-00102986",
                         "wn30-r-00344771", "wn30-r-00476680", "wn30-r-00344500"):
            self.assertNotIn(excluded, records)
        self.assertIn("wn30-r-00506928", records)  # Ordinary temporal first.
        self.assertIn("wn30-ant-a-00956131-00957176-321c4690", records)
        self.assertEqual(records["wn30-a-00922439"]["text"].splitlines()[0],
                         "tickling / tingling (adj.)")

    def test_generated_files_reproduce_from_pinned_source(self):
        with redirect_stdout(io.StringIO()):
            wordnet.import_records(ARCHIVE, check=True)


class WordNetCheckTests(unittest.TestCase):
    def test_check_does_not_write_a_mismatch(self):
        scratch = REPO / "target/wordnet-tests"
        scratch.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=scratch) as temporary:
            path = Path(temporary) / "records.json"
            path.write_text("reviewed\n", encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "Generated file differs"):
                wordnet.write_text(path, "changed\n", check=True)
            self.assertEqual(path.read_text(encoding="utf-8"), "reviewed\n")

    def test_sense_exclusion_removes_a_pair_without_refilling(self):
        scratch = REPO / "target/wordnet-tests"
        scratch.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=scratch) as temporary:
            root = Path(temporary)
            excluded = root / "lexicon/senses/nouns/relations/exclusions.json"
            excluded.parent.mkdir(parents=True)
            excluded.write_text(json.dumps([{"id": "wn30-n-13846900", "reason": "reviewed"}]),
                                encoding="utf-8")
            pair = {"id": "wn30-ant-n-13846900-14429985-01234567", "text": "excluded"}
            retained = {"id": "wn30-ant-n-11111111-22222222-01234567", "text": "retained"}
            with patch.object(wordnet, "ROOT", root), redirect_stdout(io.StringIO()):
                wordnet.write("relations/antonyms/nouns", [pair, retained])
            actual = json.loads((root / "lexicon/relations/antonyms/nouns/records.json").read_text())
            self.assertEqual(actual, [retained])


if __name__ == "__main__":
    unittest.main()
