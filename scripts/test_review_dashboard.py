"""Persistence and HTTP regression checks for the external review tool."""

from concurrent.futures import ThreadPoolExecutor
from http.server import ThreadingHTTPServer
import json
from pathlib import Path
import tempfile
import threading
import unittest
import subprocess
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
        self.feedback = self.root / "docs" / "reviews" / "feedback.md"
        self.sample_data = {"path": "demo/leaf", "kind": "record", "record_index": 0,
                            "text": "Unicode α\n```\n# sample"}
        self.invoke_patch = patch.object(Review, "invoke", side_effect=self.invoke_fixture)
        self.invoke = self.invoke_patch.start()
        self.addCleanup(self.invoke_patch.stop)
        self.review = Review(self.root, binary, self.feedback)

    def invoke_fixture(self, args):
        if args == ["--list"]:
            return "demo/leaf [corpus]\tweight=2\teffective=90%\ngenerated [generator]\tweight=1\teffective=10%\n"
        return json.dumps(self.sample_data)

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
        self.assertNotIn("A source", text)
        for omitted in ("Review ID:", "Binary SHA-256:", "- Seed:", "Record index", "### Provenance"):
            self.assertNotIn(omitted, text)
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

    def test_generator_preserves_reproduction_command(self):
        source = self.root / "families" / "generated" / "generator.rs"
        source.parent.mkdir()
        source.write_text("", encoding="utf-8")
        review = Review(self.root, self.review.binary, self.feedback)
        self.sample_data = {"path": "generated", "kind": "generator", "record_index": None,
                            "text": "Generated α"}
        sample = review.sample("generated")
        self.assertEqual(sample["text"], "Generated α")
        self.assertEqual(self.invoke.call_args.args[0],
                         ["--seed", sample["seed"], "--family", "generated", "--json"])
        review.save(sample["id"], "JANK", "")
        text = self.feedback.read_text(encoding="utf-8")
        self.assertIn(f"--seed {sample['seed']}", text)
        self.assertIn("--family generated", text)
        self.assertNotIn("### Feedback", text)
        self.assertNotIn("(No notes)", text)

    def test_cli_invocation_passes_scope_and_seed(self):
        self.invoke_patch.stop()

        def run(args, **kwargs):
            stdout = self.invoke_fixture(args[1:])
            return subprocess.CompletedProcess(args, 0, stdout, "")

        with patch("review_dashboard.subprocess.run", side_effect=run) as cli:
            review = Review(self.root, self.review.binary, self.feedback)
            with patch("review_dashboard.random.getrandbits", return_value=42):
                review.sample()
                self.assertEqual(cli.call_args.args[0], [str(review.binary), "--seed", "42", "--json"])
                sample = review.sample("demo")
                self.assertEqual(cli.call_args.args[0],
                                 [str(review.binary), "--seed", "42", "--family", "demo", "--json"])
        self.assertEqual(sample["command"], "inspire --seed 42 --family demo")

    def test_stale_source_cannot_replace_embedded_sample_or_metadata(self):
        self.sample_data["text"] = "Text embedded in an older binary"
        sample = self.review.sample()
        self.assertEqual(sample["text"], "Text embedded in an older binary")
        self.assertNotIn("record_id", sample)
        self.assertNotIn("provenance", sample)
        self.review.save(sample["id"], "LGTM", "")
        text = self.feedback.read_text(encoding="utf-8")
        self.assertIn(sample["command"], text)
        self.assertIn("Record index (zero-based): 0", text)

    def test_whitespace_only_notes_are_omitted(self):
        sample = self.review.sample()
        self.review.save(sample["id"], "LGTM", " \n\t")
        text = self.feedback.read_text(encoding="utf-8")
        self.assertNotIn("### Feedback", text)
        self.assertNotIn("(No notes)", text)

    def test_http_save_validation_and_origin(self):
        server = ThreadingHTTPServer(("127.0.0.1", 0), handler_for(self.review))
        worker = threading.Thread(target=server.serve_forever, daemon=True)
        worker.start()
        self.addCleanup(server.server_close)
        self.addCleanup(server.shutdown)
        base = f"http://127.0.0.1:{server.server_port}"
        for path, content_type, marker in (
            ("/", "text/html", b"<html"),
            ("/review.js", "text/javascript", b"/api/"),
            ("/review.css", "text/css", b"{"),
        ):
            with urlopen(base + path) as response:
                self.assertEqual(response.headers.get_content_type(), content_type)
                self.assertIn(marker, response.read())
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
