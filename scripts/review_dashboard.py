"""Standalone, local human review UI. Uses only Python's standard library."""

import argparse
from collections import OrderedDict
from datetime import datetime, timezone
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
import os
from pathlib import Path
import random
import re
import subprocess
import threading
import uuid
import webbrowser

ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / "review-dashboard"
FEEDBACK_HEADER = "# Inspire human feedback\n\nSamples use the CLI's weighted tree and embedded payloads.\n"


def fenced(text):
    """Keep arbitrary samples and reviewer notes literal in Markdown."""
    longest = max((len(run) for run in re.findall(r"`+", text)), default=0)
    fence = "`" * max(3, longest + 1)
    return f"{fence}\n{text}\n{fence}\n"


class Review:
    def __init__(self, root, binary, feedback):
        self.root = root
        self.binary = binary
        self.feedback = feedback
        listing = self.invoke(["--list"])
        self.families = sorted({line.split()[0].split("/")[0] for line in listing.splitlines()})
        self.pending = OrderedDict()
        self.saved = {}
        self.lock = threading.Lock()

    def invoke(self, args):
        result = subprocess.run(
            [str(self.binary), *args], capture_output=True, encoding="utf-8", timeout=30,
        )
        if result.returncode:
            raise ValueError(result.stderr.strip() or "Inspire failed to sample")
        return result.stdout

    def sample(self, family=""):
        if family and family not in self.families:
            raise ValueError("Unknown family")
        seed = random.getrandbits(64)
        args = ["--seed", str(seed)] + (["--family", family] if family else [])
        sample = json.loads(self.invoke([*args, "--json"]))
        source = f"families/{sample['path']}/" + (
            "records.json" if sample["kind"] == "record" else "generator.rs")
        sample.update(id=uuid.uuid4().hex, source=source, seed=str(seed),
                      command="inspire " + " ".join(args))
        # Source metadata must match the embedded record; never substitute source text.
        if sample["kind"] == "record":
            try:
                record = json.loads((self.root / source).read_text(encoding="utf-8"))[sample["record_index"]]
                if record["text"] == sample["text"]:
                    sample["record_id"] = record["id"]
            except (OSError, ValueError, IndexError, KeyError, TypeError):
                pass
        if not sample["text"].strip():
            raise ValueError(f"Empty sample from {sample['path']}")
        with self.lock:
            self.pending[sample["id"]] = sample
            if len(self.pending) > 256:
                self.pending.popitem(last=False)
        return sample

    def save(self, sample_id, rating, notes):
        if rating not in ("LGTM", "JANK"):
            raise ValueError("Rating must be LGTM or JANK")
        if not isinstance(notes, str) or len(notes) > 16000:
            raise ValueError("Notes must be text, up to 16,000 characters")
        if not isinstance(sample_id, str):
            raise ValueError("Missing sample ID")
        with self.lock:
            if sample_id in self.saved:
                if self.saved[sample_id] != (rating, notes):
                    raise ValueError("This sample already has feedback")
                return len(self.saved)
            sample = self.pending.get(sample_id)
            if sample is None:
                raise ValueError("Sample expired; reload to get a new sample")
            stamp = datetime.now(timezone.utc).isoformat(timespec="seconds")
            entry = f"\n## {rating} — {stamp}\n\n"
            entry += f"- Source: {sample['source']}\n"
            if "record_id" in sample:
                entry += "- Record ID: " + json.dumps(sample["record_id"], ensure_ascii=False) + "\n"
            elif sample.get("record_index") is not None:
                entry += f"- Record index (zero-based): {sample['record_index']}\n"
            entry += f"- Reproduce: `{sample['command']}`\n"
            entry += "\n### Sample\n\n" + fenced(sample["text"])
            if notes.strip():
                entry += "\n### Feedback\n\n" + fenced(notes)
            self.feedback.parent.mkdir(parents=True, exist_ok=True)
            # Mark saved only after a durable write. Retries within this session are idempotent.
            with self.feedback.open("a+", encoding="utf-8", newline="\n") as output:
                output.seek(0, os.SEEK_END)
                start = output.tell()
                try:
                    if start == 0:
                        output.write(FEEDBACK_HEADER)
                    output.write(entry)
                    output.flush()
                    os.fsync(output.fileno())
                except OSError:
                    output.seek(start)
                    output.truncate()
                    raise
            self.saved[sample_id] = (rating, notes)
            del self.pending[sample_id]
            return len(self.saved)


def handler_for(review):
    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *_args):
            pass

        def respond(self, status, body, content_type="application/json; charset=utf-8"):
            data = json.dumps(body, ensure_ascii=False).encode() if isinstance(body, dict) else body
            self.send_response(status)
            self.send_header("Content-Type", content_type)
            self.send_header("Content-Length", str(len(data)))
            self.send_header("Cache-Control", "no-store")
            self.send_header("X-Content-Type-Options", "nosniff")
            self.end_headers()
            self.wfile.write(data)

        def allowed(self):
            expected = f"127.0.0.1:{self.server.server_port}"
            origin = self.headers.get("Origin")
            if self.headers.get("Host") != expected or (origin and origin != f"http://{expected}"):
                self.respond(403, {"error": "Use the printed local URL"})
                return False
            return True

        def do_GET(self):
            from urllib.parse import parse_qs, urlsplit

            if not self.allowed():
                return
            url = urlsplit(self.path)
            try:
                if url.path == "/api/config":
                    self.respond(200, {"families": review.families, "feedback": str(review.feedback),
                                       "reviewed": len(review.saved)})
                elif url.path == "/api/sample":
                    family = parse_qs(url.query).get("family", [""])[0]
                    self.respond(200, review.sample(family))
                elif url.path in ("/", "/review.js", "/review.css"):
                    name, mime = {"/": ("index.html", "text/html"),
                                  "/review.js": ("review.js", "text/javascript"),
                                  "/review.css": ("review.css", "text/css")}[url.path]
                    self.respond(200, (ASSETS / name).read_bytes(), mime + "; charset=utf-8")
                else:
                    self.respond(404, {"error": "Not found"})
            except ValueError as error:
                self.respond(400, {"error": str(error)})
            except (OSError, subprocess.SubprocessError) as error:
                self.respond(500, {"error": f"Could not load sample: {error}"})

        def do_POST(self):
            if not self.allowed():
                return
            if self.path != "/api/feedback":
                self.respond(404, {"error": "Not found"})
                return
            try:
                length = int(self.headers.get("Content-Length", "0"))
                if not 0 < length <= 100000 or self.headers.get_content_type() != "application/json":
                    raise ValueError("Expected a JSON feedback request, up to 100 KB")
                body = json.loads(self.rfile.read(length))
                if not isinstance(body, dict):
                    raise ValueError("Expected a JSON object")
                count = review.save(body.get("id"), body.get("rating"), body.get("notes", ""))
                self.respond(200, {"reviewed": count})
            except (ValueError, UnicodeError) as error:
                self.respond(400, {"error": str(error)})
            except OSError as error:
                self.respond(500, {"error": f"Feedback was not saved: {error}"})

    return Handler


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, help="Use an existing binary; skip rebuilding")
    parser.add_argument("--feedback", type=Path, default=ROOT / "reviews" / "human-feedback.md")
    parser.add_argument("--port", type=int, default=8766, help="Local port; 0 picks an available port")
    parser.add_argument("--no-open", action="store_true", help="Do not open the browser automatically")
    args = parser.parse_args()
    if not 0 <= args.port <= 65535:
        parser.error("--port must be between 0 and 65535")
    if args.binary:
        binary = args.binary.resolve()
    else:
        print("Building current dataset…", flush=True)
        subprocess.run(["cargo", "build", "--release", "--locked"], cwd=ROOT, check=True)
        binary = ROOT / "target" / "release" / ("inspire.exe" if os.name == "nt" else "inspire")
    review = Review(ROOT, binary, args.feedback.resolve())
    with ThreadingHTTPServer(("127.0.0.1", args.port), handler_for(review)) as server:
        server.daemon_threads = True
        url = f"http://127.0.0.1:{server.server_port}/"
        print(f"Review: {url}\nFeedback: {review.feedback}\nStop with Ctrl+C.", flush=True)
        if not args.no_open:
            webbrowser.open(url)
        try:
            server.serve_forever()
        except KeyboardInterrupt:
            pass


if __name__ == "__main__":
    main()
