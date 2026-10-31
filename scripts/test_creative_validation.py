"""Synthetic budget, resume, blinding and paired-score checks; never calls the API."""

from concurrent.futures import ThreadPoolExecutor
import copy
import json
from pathlib import Path
import struct
import tempfile
import unittest
from unittest.mock import Mock, patch

import requests

import creative_validation as trial


def completed(text="Finished work"):
    return {"status": "completed", "output": [{"content": [{"type": "output_text", "text": text}]}],
            "usage": {"input_tokens": 100, "output_tokens": 50,
                      "input_tokens_details": {"cached_tokens": 20}}}


def http(status=200, payload=None):
    response = Mock()
    response.status_code = status
    response.json.return_value = payload if payload is not None else completed()
    return response


def fixture(base):
    jobs = []
    for task, task_prompt in trial.TASKS.items():
        for replicate in range(1, 4):
            for condition in ["control", "inspire"]:
                for position in range(1, 11):
                    job_id = f"{task}-r{replicate}-{condition}-{position:02d}"
                    inspiration = {"seed": str(len(jobs)), "payload": {"text": f"Entropy {len(jobs)}"}} if condition == "inspire" else None
                    prompt = trial.PREFIX.format(inspiration["payload"]["text"], task_prompt) if inspiration else task_prompt
                    body = trial.request(trial.GENERATOR, prompt)
                    job = {"id": job_id, "task": task, "replicate": replicate,
                           "condition": condition, "position": position, "prompt": prompt,
                           "inspiration": inspiration, "request": body,
                           "request_sha256": trial.sha(trial.encoded(body))}
                    jobs.append(job)
                    trial.dump(base / "records" / f"{job_id}.json",
                               {"request_sha256": job["request_sha256"], "status": "completed",
                                "output_text": f"Work {len(jobs)}\n untrimmed "})
                    if task == "svg":
                        path = base / "rendered" / f"{job_id}.png"
                        path.parent.mkdir(exist_ok=True)
                        path.write_bytes(b"\x89PNG\r\n\x1a\n" + b"\0" * 8 + struct.pack(">II", 400, 400))
    manifest = {"status": "frozen", "fixed": {"seed": 4201}, "jobs": jobs}
    trial.dump(base / "manifest.json", manifest)
    return manifest


def verdict(case_id, diversity, quality):
    return {"case_id": case_id, "diversity_score": diversity, "quality_score": quality,
            "task_failures": [], "a_redundant_groups": [{"items": ["A1", "A10"], "reason": "Similar structure"}],
            "b_redundant_groups": [], "reason": "A1 and B2 illustrate the difference."}


class CreativeTrialTests(unittest.TestCase):
    def setUp(self):
        root = trial.ROOT / "target"
        root.mkdir(exist_ok=True)
        self.temporary = tempfile.TemporaryDirectory(prefix="creative-test-", dir=root)
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)

    def test_actual_usage_includes_cached_reads_and_cache_writes(self):
        usage = {"input_tokens": 1000, "output_tokens": 100,
                 "input_tokens_details": {"cached_tokens": 200, "cache_write_tokens": 100}}
        self.assertAlmostEqual(trial.usage_cost(trial.GENERATOR, usage), .001335)
        self.assertAlmostEqual(trial.usage_cost(trial.JUDGE, usage), .00267)
        usage["input_tokens_details"]["cached_tokens"] = 1000
        with self.assertRaises(ValueError):
            trial.usage_cost(trial.GENERATOR, usage)

    def test_atomic_write_retries_transient_windows_read_lock(self):
        path = self.base / "atomic.json"
        trial.dump(path, {"old": True})
        original = Path.replace
        attempts = []

        def replace(source, destination):
            attempts.append(source.read_bytes())
            if len(attempts) < 3:
                raise PermissionError("Synthetic Windows reader")
            return original(source, destination)

        with patch.object(Path, "replace", replace), patch.object(trial.time, "sleep") as sleep:
            trial.dump(path, {"new": True})
        self.assertEqual(len(attempts), 3)
        self.assertTrue(all(data == attempts[0] for data in attempts))
        self.assertEqual(sleep.call_count, 2)
        self.assertEqual(trial.load(path), {"new": True})

    def test_atomic_write_retry_is_bounded_and_preserves_old_file(self):
        path = self.base / "atomic.json"
        trial.dump(path, {"old": True})
        with patch.object(Path, "replace", side_effect=PermissionError("Persistent lock")) as replace, patch.object(trial.time, "sleep") as sleep:
            with self.assertRaises(PermissionError):
                trial.dump(path, {"new": True})
        self.assertEqual(replace.call_count, 20)
        self.assertEqual(sleep.call_count, 19)
        self.assertEqual(trial.load(path), {"old": True})

    def test_failed_reservation_write_never_calls_http_or_leaves_ghost_entry(self):
        body = trial.request(trial.GENERATOR, "Task")
        budget = trial.Budget(self.base)
        post = Mock()
        with patch.object(budget, "save", side_effect=PermissionError("Persistent lock")):
            with self.assertRaises(PermissionError):
                trial.paid_call(self.base, "unstarted", body, "secret", budget, post=post)
        self.assertEqual(budget.entries, {})
        self.assertFalse((self.base / "records/unstarted.json").exists())
        post.assert_not_called()
        budget.reserve("next", body)
        self.assertEqual(set(trial.load(self.base / "ledger.json")["entries"]), {"next"})

    def test_runner_amendment_is_bound_to_frozen_manifest_and_hash_chain(self):
        manifest = {"fixed": {"runner_sha256": "0" * 64}}
        trial.dump(self.base / "manifest.json", manifest)
        with self.assertRaisesRegex(ValueError, "without a recorded"):
            trial.validate_runner_provenance(self.base, manifest)
        provenance = {"manifest_sha256": trial.sha((self.base / "manifest.json").read_bytes()),
                      "amendments": [{"previous_runner_sha256": "0" * 64,
                                      "runner_sha256": trial.sha(Path(trial.__file__).read_bytes())}]}
        trial.dump(self.base / "runner-amendments.json", provenance)
        trial.validate_runner_provenance(self.base, manifest)
        provenance["amendments"][0]["previous_runner_sha256"] = "1" * 64
        trial.dump(self.base / "runner-amendments.json", provenance)
        with self.assertRaisesRegex(ValueError, "Broken runner"):
            trial.validate_runner_provenance(self.base, manifest)

    def test_reservations_are_atomic_across_workers(self):
        body = trial.request(trial.GENERATOR, "Task")
        amount = trial.reservation(body)
        budget = trial.Budget(self.base, cap=amount * 2.01)

        def reserve(index):
            try:
                return budget.reserve(str(index), body)
            except RuntimeError:
                return False

        with ThreadPoolExecutor(max_workers=8) as executor:
            results = list(executor.map(reserve, range(16)))
        self.assertEqual(sum(results), 2)
        self.assertLessEqual(trial.load(self.base / "ledger.json")["accounted_usd"], budget.cap)

    def test_generation_reserves_judge_budget(self):
        body = trial.request(trial.GENERATOR, "Task")
        budget = trial.Budget(self.base, cap=8, reserve_floor=8)
        with self.assertRaisesRegex(RuntimeError, "budget cap"):
            budget.reserve("one", body)
        self.assertEqual(budget.entries, {})

    def test_completed_call_is_reused_without_a_second_charge(self):
        body = trial.request(trial.GENERATOR, "Task")
        post = Mock(return_value=http())
        budget = trial.Budget(self.base)
        first = trial.paid_call(self.base, "one", body, "secret", budget, post=post)
        second = trial.paid_call(self.base, "one", body, "secret", budget, post=post)
        self.assertEqual(first, second)
        self.assertEqual(post.call_count, 1)
        self.assertNotIn("secret", (self.base / "records/one.json").read_text())
        changed = copy.deepcopy(body)
        changed["max_output_tokens"] -= 1
        with self.assertRaisesRegex(ValueError, "Changed saved request"):
            trial.paid_call(self.base, "one", changed, "secret", budget, post=post)
        self.assertEqual(post.call_count, 1)

    def test_ambiguous_network_result_keeps_full_reservation_and_never_retries(self):
        body = trial.request(trial.GENERATOR, "Task")
        post = Mock(side_effect=requests.Timeout("secret should not be persisted"))
        budget = trial.Budget(self.base)
        with self.assertRaisesRegex(RuntimeError, "ambiguous"):
            trial.paid_call(self.base, "one", body, "secret", budget, post=post)
        saved = trial.load(self.base / "records/one.json")
        self.assertEqual(saved["status"], "ambiguous")
        self.assertEqual(budget.entries["one"]["accounted_usd"], trial.reservation(body))
        with self.assertRaisesRegex(RuntimeError, "no automatic retry"):
            trial.paid_call(self.base, "one", body, "secret", budget, post=post)
        self.assertEqual(post.call_count, 1)
        self.assertNotIn("secret", json.dumps(saved))

    def test_rate_limit_retry_is_bounded(self):
        body = trial.request(trial.GENERATOR, "Task")
        post = Mock(return_value=http(429, {"error": "rate limit"}))
        sleep = Mock()
        budget = trial.Budget(self.base)
        result = trial.paid_call(self.base, "one", body, "secret", budget, post=post, sleep=sleep)
        self.assertEqual(post.call_count, 5)
        self.assertEqual(sleep.call_count, 4)
        self.assertEqual(result["status"], "failed")
        self.assertEqual(budget.entries["one"]["accounted_usd"], 0)

    def test_incomplete_response_is_retained_and_not_regenerated(self):
        payload = completed("Partial poem")
        payload["status"] = "incomplete"
        payload["incomplete_details"] = {"reason": "max_output_tokens"}
        body = trial.request(trial.GENERATOR, "Task")
        post = Mock(return_value=http(200, payload))
        budget = trial.Budget(self.base)
        result = trial.paid_call(self.base, "one", body, "secret", budget, post=post)
        trial.paid_call(self.base, "one", body, "secret", budget, post=post)
        self.assertEqual(result["output_text"], "Partial poem")
        self.assertEqual(result["status"], "incomplete")
        self.assertEqual(post.call_count, 1)

    def test_cases_are_blind_mirrored_and_frozen(self):
        manifest = fixture(self.base)
        key = trial.create_cases(self.base)
        self.assertEqual(len(key["pairs"]), 12)
        self.assertEqual(len(key["cases"]), 26)
        self.assertEqual(len(key["controls"]), 2)
        by_id = {entry["id"]: entry for entry in key["cases"]}
        for pair in key["pairs"]:
            first, mirror = [trial.load(self.base / by_id[c]["file"]) for c in pair["cases"]]
            self.assertEqual(set(first), {"case_id", "task", "instructions", "streams"})
            for side, other in [("A", "B"), ("B", "A")]:
                a = [{k: v for k, v in row.items() if k != "label"} for row in first["streams"][side]]
                b = [{k: v for k, v in row.items() if k != "label"} for row in mirror["streams"][other]]
                self.assertEqual(a, b)
                self.assertEqual(len(a), 10)
            for side in ["A", "B"]:
                for row in first["streams"][side]:
                    self.assertIn(set(row), [{"label", "text"}, {"label", "image"}])
        for case_id in key["controls"]:
            case = trial.load(self.base / by_id[case_id]["file"])
            for a, b in zip(case["streams"]["A"], case["streams"]["B"]):
                self.assertEqual({k: v for k, v in a.items() if k != "label"}, {k: v for k, v in b.items() if k != "label"})
        self.assertEqual(trial.create_cases(self.base), key)
        job = manifest["jobs"][0]
        record_path = self.base / "records" / f"{job['id']}.json"
        changed = trial.load(record_path)
        changed["output_text"] += " changed"
        trial.dump(record_path, changed)
        with self.assertRaisesRegex(ValueError, "Frozen file changed"):
            trial.create_cases(self.base)

    def test_svg_requests_use_only_images_and_keep_conservative_image_reserve(self):
        fixture(self.base)
        key = trial.create_cases(self.base)
        entry = next(c for c in key["cases"] if c["task"] == "svg")
        content = entry["request"]["input"][1]["content"]
        self.assertEqual(sum(p["type"] == "input_image" for p in content), 20)
        self.assertNotIn("<svg", json.dumps(content))
        self.assertGreater(trial.reservation(entry["request"]), .5)

    def test_aggregate_maps_direction_and_averages_mirrors_per_pair(self):
        fixture(self.base)
        key = trial.create_cases(self.base)
        for entry in key["cases"]:
            sign = 1 if entry["sides"]["A"] == "inspire" else -1
            value = verdict(entry["id"], sign * (1 if entry["mirrored"] else 3), sign * -2)
            if entry["id"] in key["controls"]:
                value = verdict(entry["id"], 0, 0)
            trial.dump(self.base / "judgments" / f"{entry['id']}.json",
                       {"status": "completed", "request_sha256": entry["request_sha256"],
                        "output_text": json.dumps(value)})
        result = trial.aggregate(self.base)
        self.assertFalse(result["invalid_or_missing_judgments"])
        self.assertEqual(len(result["pairs"]), 12)
        for pair in result["pairs"]:
            self.assertEqual(pair["diversity_score"], 2)
            self.assertEqual(pair["quality_score"], -2)
            self.assertTrue(pair["diversity_score_mirror_sign_agreement"])
        for task in trial.TASKS:
            self.assertEqual(result["tasks"][task]["diversity_score"]["inspire_wins"], 3)

    def test_verdict_ranges_and_indices_are_validated(self):
        value = verdict("case001", 0, 1)
        trial.validate_verdict(value, "case001")
        for edit in [lambda v: v.update(diversity_score=True),
                     lambda v: v.update(quality_score=4),
                     lambda v: v["a_redundant_groups"][0].update(items=["A1", "B2"]),
                     lambda v: v.update(task_failures=[{"item": "A11", "reason": "Invalid"}])]:
            changed = copy.deepcopy(value)
            edit(changed)
            with self.assertRaises(ValueError):
                trial.validate_verdict(changed, "case001")


if __name__ == "__main__":
    unittest.main()
