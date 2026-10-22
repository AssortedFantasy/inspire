"""Inventory the sampling tree and export reproducible samples for human review.

Run after a build, using --binary target/release/inspire.exe to sample generators.
This report helps find repetition; it does not establish factual correctness.
"""

import argparse
from collections import Counter, defaultdict
import json
from pathlib import Path
import random
import subprocess
import tomllib
import unicodedata
import urllib.request


ROOT = Path(__file__).resolve().parents[1] / "families"


def normalized(text):
    return " ".join(unicodedata.normalize("NFKC", text).casefold().split())


def weighted_samples(binary, count, seed):
    """Exercise the ordinary CLI distribution, keeping every sample reproducible."""
    rng = random.Random(seed)
    samples = []
    for _ in range(count):
        sample_seed = rng.getrandbits(64)
        result = subprocess.run([str(binary), "--seed", str(sample_seed), "--json"],
                                check=True, capture_output=True, text=True, encoding="utf-8")
        samples.append({"seed": sample_seed, **json.loads(result.stdout)})
    return samples


def weight(path):
    config = path / "node.toml"
    return tomllib.loads(config.read_text(encoding="utf-8")).get("weight", 1) if config.exists() else 1


def generator_samples(binary, seeds):
    """Use the embedded dashboard to avoid launching a process for every seed."""
    server = subprocess.Popen([str(binary), "--dashboard", "--port", "0"], stdout=subprocess.PIPE,
                              encoding="utf-8")
    try:
        line = server.stdout.readline()
        if not line.startswith("Dashboard: "):
            raise RuntimeError(f"Dashboard did not start: {line!r}")
        base = line.split()[1].rstrip("/")

        def fetch(path):
            with urllib.request.urlopen(base + path, timeout=30) as response:
                return json.load(response)

        def generators(node):
            if node["kind"] == "generator":
                yield node
            for child in node["children"]:
                yield from generators(child)

        results = {}
        for node in generators(fetch("/api/tree")):
            outputs = []
            offset = 0
            while len(outputs) < seeds:
                batch = fetch(f"/api/outputs?node={node['id']}&seed=0&offset={offset}")
                outputs.extend(item["text"] for item in batch["items"])
                offset = batch["next"]
            results[node["path"]] = outputs[:seeds]
        return results
    finally:
        server.terminate()
        server.wait(timeout=10)
        server.stdout.close()


def review(binary=None, sample_count=3, seeds=128):
    families = defaultdict(lambda: Counter(records=0, corpora=0, generators=0))
    texts = defaultdict(list)
    leaves = []
    generated = generator_samples(binary, max(seeds, sample_count)) if binary else {}

    def visit(path, probability):
        children = sorted(p for p in path.iterdir() if p.is_dir())
        if children:
            total = sum(weight(p) for p in children)
            for child in children:
                visit(child, probability * weight(child) / total)
            return
        relative = path.relative_to(ROOT).as_posix()
        family = relative.split("/")[0]
        entry = {"path": relative, "probability": probability}
        corpus = path / "records.json"
        if corpus.exists():
            records = json.loads(corpus.read_text(encoding="utf-8"))
            families[family].update(records=len(records), corpora=1)
            entry.update(kind="corpus", records=len(records), per_record_probability=probability / len(records),
                         repeat_probability=probability * probability / len(records))
            # Spread samples across the file, including both endpoints.
            indexes = sorted({i * (len(records) - 1) // max(1, sample_count - 1) for i in range(sample_count)})
            entry["samples"] = [records[i] for i in indexes]
            for record in records:
                texts[normalized(record["text"])].append(f"{relative}:{record['id']}")
        else:
            families[family].update(generators=1)
            entry["kind"] = "generator"
            if binary:
                outputs = generated[relative]
                entry.update(sampled_seeds=seeds, distinct_outputs=len(set(outputs[:seeds])),
                             samples=[{"seed": i, "text": text} for i, text in enumerate(outputs[:sample_count])])
        leaves.append(entry)

    visit(ROOT, 1.0)
    corpora = [entry for entry in leaves if entry["kind"] == "corpus"]
    return {"families": dict(sorted(families.items())),
            "totals": dict(sum(families.values(), Counter())),
            "max_leaf_depth": max(entry["path"].count("/") + 1 for entry in leaves),
            "duplicate_texts": [refs for refs in texts.values() if len(refs) > 1],
            # Probability that two independent root draws select the same record.
            # Cross-leaf duplicates and generators are deliberately excluded.
            "corpus_repeat_probability": sum(entry["repeat_probability"] for entry in corpora),
            "most_exposed_records": [entry["path"] for entry in sorted(
                corpora, key=lambda entry: -entry["per_record_probability"])[:20]],
            "leaves": leaves}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, help="Sample generators with this built executable")
    parser.add_argument("--samples", type=int, default=3, help="Samples per leaf (positive integer)")
    parser.add_argument("--seeds", type=int, default=128, help="Generator seeds to inspect (positive integer)")
    parser.add_argument("--output", type=Path, help="Save the full JSON report here")
    parser.add_argument("--weighted-samples", type=int, default=0,
                        help="Include this many ordinary weighted CLI draws (requires --binary)")
    parser.add_argument("--seed", type=int, default=0, help="Seed for choosing weighted sample seeds")
    args = parser.parse_args()
    if args.samples < 1 or args.seeds < 1:
        parser.error("--samples and --seeds must be positive")
    if args.weighted_samples < 0 or (args.weighted_samples and not args.binary):
        parser.error("--weighted-samples must be nonnegative and requires --binary")
    report = review(args.binary.resolve() if args.binary else None, args.samples, args.seeds)
    if args.weighted_samples:
        report["weighted_samples"] = weighted_samples(args.binary.resolve(), args.weighted_samples, args.seed)
    if args.output:
        args.output.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    for family, counts in report["families"].items():
        print(f"{family}: {counts['records']} records, {counts['corpora']} corpora, {counts['generators']} generators")
    print(f"Totals: {report['totals']}; maximum depth: {report['max_leaf_depth']}; "
          f"duplicate text groups: {len(report['duplicate_texts'])}")
    print(f"Same-record probability for two root draws: {report['corpus_repeat_probability']:.6%} "
          "(corpora only; excludes cross-leaf duplicates)")


if __name__ == "__main__":
    main()
