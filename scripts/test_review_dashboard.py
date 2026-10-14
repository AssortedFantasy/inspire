"""Persistence and HTTP regression checks for the external review tool."""

from concurrent.futures import ThreadPoolExecutor
from http.server import ThreadingHTTPServer
import json
from pathlib import Path
import tempfile
import threading
import unittest
from unittest.mock import patch
from urllib.error import HTTPError
from urllib.request import Request, urlopen

from review_dashboard import Review, fenced, handler_for


class ReviewTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        source = self.root / "families" / "demo" / "leaf" / "records.json"
        source.parent.mkdir(parents=True)
        source.write_text(json.dumps([{"id": "record-1", "text": "Unicode α\n```\n# sample",
                                       "provenance": "A source"}]), encoding="utf-8")
        binary = self.root / "inspire"
        binary.write_bytes(b"test binary")
        self.feedback = self.root / "reviews" / "feedback.md"
        self.review = Review(self.root, binary, self.feedback)

    def test_both_ratings_append_exact_source_and_notes(self):
        for rating in ("LGTM", "JANK"):
            sample = self.review.sample()
            self.review.save(sample["id"], rating, "My note\n```\nα")
        text = self.feedback.read_text(encoding="utf-8")
        self.assertEqual(text.count("# Inspire human feedback"), 1)
        self.assertIn("## LGTM", text)
        self.assertIn("## JANK", text)
        self.assertIn("families/demo/leaf/records.json", text)
        self.assertIn('Record ID: "record-1"', text)
        self.assertIn("A source", text)
        self.assertIn(fenced("Unicode α\n```\n# sample"), text)
        self.assertIn(fenced("My note\n```\nα"), text)
        # A new session appends instead of overwriting earlier reviews.
        restarted = Review(self.root, self.review.binary, self.feedback)
        sample = restarted.sample()
        restarted.save(sample["id"], "LGTM", "")
        self.assertTrue(self.feedback.read_text(encoding="utf-8").startswith(text))

    def test_concurrent_retries_append_once(self):
        sample = self.review.sample()
        with ThreadPoolExecutor(max_workers=8) as pool:
            counts = list(pool.map(lambda _: self.review.save(sample["id"], "JANK", "Fix it"), range(12)))
        self.assertEqual(counts, [1] * 12)
        self.assertEqual(self.feedback.read_text(encoding="utf-8").count("## JANK"), 1)
        with self.assertRaisesRegex(ValueError, "already has feedback"):
            self.review.save(sample["id"], "LGTM", "Changed")

    def test_failed_write_keeps_sample_and_rolls_back(self):
        first = self.review.sample()
        self.review.save(first["id"], "LGTM", "First")
        before = self.feedback.read_bytes()
        second = self.review.sample()
        with patch("review_dashboard.os.fsync", side_effect=OSError("Disk failure")):
            with self.assertRaises(OSError):
                self.review.save(second["id"], "JANK", "Second")
        self.assertEqual(self.feedback.read_bytes(), before)
        self.assertIn(second["id"], self.review.pending)
        self.assertEqual(self.review.save(second["id"], "JANK", "Second"), 2)

    def test_generator_preserves_seed_and_binary_identity(self):
        source = self.root / "families" / "generated" / "generator.rs"
        source.parent.mkdir()
        source.write_text("", encoding="utf-8")
        review = Review(self.root, self.review.binary, self.feedback)
        with patch("review_dashboard.subprocess.run") as run:
            run.return_value.stdout = "Generated α\r\n"
            sample = review.sample("generated")
        self.assertEqual(sample["text"], "Generated α")
        self.assertEqual(run.call_args.args[0][-1], sample["seed"])
        review.save(sample["id"], "JANK", "")
        text = self.feedback.read_text(encoding="utf-8")
        self.assertIn(f"--seed {sample['seed']}", text)
        self.assertIn(review.binary_hash, text)

    def test_http_save_validation_and_origin(self):
        server = ThreadingHTTPServer(("127.0.0.1", 0), handler_for(self.review))
        worker = threading.Thread(target=server.serve_forever, daemon=True)
        worker.start()
        self.addCleanup(server.server_close)
        self.addCleanup(server.shutdown)
        base = f"http://127.0.0.1:{server.server_port}"
        with urlopen(base + "/api/sample") as response:
            sample = json.load(response)
        body = json.dumps({"id": sample["id"], "rating": "JANK", "notes": "HTTP note"}).encode()
        request = Request(base + "/api/feedback", body, {"Content-Type": "application/json"})
        with urlopen(request) as response:
            self.assertEqual(json.load(response)["reviewed"], 1)
        for data in (b'[]', b'{"id":"fake","rating":"JANK"}', b'not json'):
            with self.assertRaises(HTTPError) as caught:
                urlopen(Request(base + "/api/feedback", data, {"Content-Type": "application/json"}))
            self.assertEqual(caught.exception.code, 400)
            caught.exception.close()
        with self.assertRaises(HTTPError) as caught:
            urlopen(Request(base + "/api/feedback", body,
                            {"Content-Type": "application/json", "Origin": "https://other.example"}))
        self.assertEqual(caught.exception.code, 403)
        caught.exception.close()


if __name__ == "__main__":
    unittest.main()
