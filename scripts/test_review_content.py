"""Check exposure calculations against a deliberately uneven sampling tree."""

import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import review_content


class ExposureTests(unittest.TestCase):
    def test_local_weights_and_record_counts_determine_repeat_risk(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for name, count, weight in (("large", 9, 3), ("small", 1, 1), ("disabled", 1, 0)):
                leaf = root / name
                leaf.mkdir()
                (leaf / "node.toml").write_text(f"weight = {weight}\n", encoding="utf-8")
                records = [{"id": str(i), "text": f"{name} {i}", "provenance": "fixture"}
                           for i in range(count)]
                (leaf / "records.json").write_text(json.dumps(records), encoding="utf-8")
            with patch.object(review_content, "ROOT", root):
                report = review_content.review()
        leaves = {entry["path"]: entry for entry in report["leaves"]}
        self.assertAlmostEqual(leaves["large"]["per_record_probability"], 1 / 12)
        self.assertAlmostEqual(leaves["small"]["per_record_probability"], 1 / 4)
        self.assertEqual(leaves["disabled"]["repeat_probability"], 0)
        # 9*(1/12)^2 + (1/4)^2 = 1/8, rather than 1/11.
        self.assertAlmostEqual(report["corpus_repeat_probability"], 1 / 8)
        self.assertEqual(report["most_exposed_records"][0], "small")


if __name__ == "__main__":
    unittest.main()
