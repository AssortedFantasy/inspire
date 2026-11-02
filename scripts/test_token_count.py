"""Exercise token counting and complete, reproducible leaf coverage."""

from contextlib import redirect_stdout
import io
import json
import os
from pathlib import Path
import tempfile
import unittest

import tiktoken

import token_count


class TokenCountTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        os.environ.setdefault("TIKTOKEN_CACHE_DIR", str(token_count.ROOT.parent / "target" / "tiktoken-cache"))
        cls.encoding = tiktoken.get_encoding("o200k_base")

    def count(self, text, **kwargs):
        output = io.StringIO()
        with redirect_stdout(output):
            status = token_count.count(io.StringIO(text), self.encoding, **kwargs)
        return status, output.getvalue()

    def test_text_and_exact_threshold(self):
        self.assertEqual(self.count("hello world", max_tokens=2), (0, "2\n"))
        self.assertEqual(self.count("hello world!", max_tokens=2), (1, "3\n"))
        self.assertEqual(self.count("", max_tokens=0), (0, "0\n"))

    def test_jsonl_counts_decoded_text_and_preserves_metadata(self):
        rows = [{"text": "hello world", "path": "fixture", "seed": "18446744073709551615"},
                {"text": "日本語\n<|endoftext|>", "record_id": "unicode"}]
        status, output = self.count("\n" + "\n\n".join(map(json.dumps, rows)), jsonl=True, max_tokens=2)
        counted = [json.loads(line) for line in output.splitlines()]
        self.assertEqual(status, 1)
        self.assertEqual(counted[0], {**rows[0], "tokens": 2})
        self.assertEqual(counted[1], {**rows[1], "tokens": len(self.encoding.encode_ordinary(rows[1]["text"]))})
        # JSON quotes, field names, and escaped newlines must not contribute.
        self.assertLess(counted[1]["tokens"], len(self.encoding.encode_ordinary(json.dumps(rows[1]))))

    def test_bad_jsonl_identifies_line(self):
        for text in ('\n{', '\n[]', '\n{"text": 7}'):
            with self.subTest(text=text), self.assertRaisesRegex(ValueError, "line 2:"):
                list(token_count.jsonl_rows(io.StringIO(text)))

    def test_audit_covers_records_and_partial_generator_batch(self):
        with tempfile.TemporaryDirectory(dir=token_count.ROOT.parent / "target") as directory:
            root = Path(directory)
            corpus = root / "demo" / "corpus"
            generator = root / "demo" / "generator"
            corpus.mkdir(parents=True)
            generator.mkdir()
            (corpus / "records.json").write_text(json.dumps([
                {"id": "short", "text": "hello world"},
                {"id": "long", "text": "hello world!"}]), encoding="utf-8")
            (generator / "generator.rs").touch()
            nodes = [{"id": 1, "path": "demo/corpus", "kind": "dataset", "count": 2, "children": []},
                     {"id": 2, "path": "demo/generator", "kind": "generator", "children": []}]
            calls = []

            def fetch(url):
                if url == "/api/tree":
                    return {"children": nodes}
                calls.append(url)
                offset = 0 if url.endswith("offset=0") else 2
                return {"items": [{"seed": str(10 + i), "text": "hello world!"}
                                  for i in range(offset, offset + 2)], "next": offset + 2}

            report = token_count.audit(fetch, self.encoding, 2, 3, 10, "demo", root)
            self.assertEqual(report["totals"], {"leaves": 2, "samples": 5, "tokens": 14,
                                               "over_limit": 4, "corpus_leaves": 1, "generator_leaves": 1})
            self.assertEqual(report["token_histogram"], {2: 1, 3: 4})
            long_record = report["over_limit"][0]
            self.assertEqual((long_record["record_id"], long_record["record"]), ("long", 2))
            generated = [row for row in report["over_limit"] if row["kind"] == "generator"]
            self.assertEqual([row["seed"] for row in generated], ["10", "11", "12"])
            self.assertEqual(calls, ["/api/outputs?node=2&seed=10&offset=0",
                                     "/api/outputs?node=2&seed=10&offset=2"])
            filtered = token_count.audit(fetch, self.encoding, 3, 3, 10, "demo/corpus", root)
            self.assertEqual(filtered["totals"]["samples"], 2)
            self.assertEqual(filtered["over_limit"], [])
            with self.assertRaisesRegex(ValueError, "unknown family"):
                token_count.audit(fetch, self.encoding, 2, 3, 10, "dem", root)
            nodes[0]["count"] = 3
            with self.assertRaisesRegex(ValueError, "record count differs"):
                token_count.audit(fetch, self.encoding, 2, 3, 10, "", root)
            nodes.pop(0)
            with self.assertRaisesRegex(ValueError, "leaf paths differ"):
                token_count.audit(fetch, self.encoding, 2, 3, 10, "", root)


if __name__ == "__main__":
    unittest.main()
