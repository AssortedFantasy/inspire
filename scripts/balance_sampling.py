"""Build an auditable, reproducible bank for offline sampling-balance experiments.

Run ``cargo build --release --locked`` first, then ``python -B
scripts/balance_sampling.py --binary target/release/inspire.exe``. This never
changes the sampling tree. Exact corpus probabilities use every source record;
semantic banks are stratified quadrature samples, not independent root draws.
"""

import argparse
from collections import Counter, defaultdict
from contextlib import contextmanager
from fractions import Fraction
import hashlib
import json
from pathlib import Path
import random
import subprocess
import time
import tomllib
import urllib.request


ROOT = Path(__file__).resolve().parents[1]


def digest(data):
    return hashlib.sha256(data).hexdigest()


def text_hash(text):
    return digest(text.encode("utf-8"))


def keyed_seed(master, *parts):
    return int.from_bytes(hashlib.sha256(json.dumps([master, *parts]).encode()).digest()[:8], "big")


def write_json(path, value):
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def write_row(stream, row):
    stream.write(json.dumps(row, ensure_ascii=False, separators=(",", ":")) + "\n")


def merge_corpus_record(exact, path, index, record, record_mass):
    """Aggregate exact output identity across leaves, retaining every source ID."""
    key = text_hash(record["text"])
    if key not in exact:
        exact[key] = {"text_sha256": key, "text": record["text"], "probability": 0.0, "members": []}
    entry = exact[key]
    assert entry["text"] == record["text"], "SHA256 collision"
    entry["probability"] += float(record_mass)
    entry["members"].append({"path": path, "record_index": index, "record_id": record["id"],
                             "probability": float(record_mass), "probability_fraction": str(record_mass)})
    return key


def collision_statistics(counts):
    """U-statistic for two independent generator draws, excluding self-pairs."""
    n = sum(counts.values())
    if n < 2:
        raise ValueError("At least two independent draws are required")
    pairs = sum(c * (c - 1) // 2 for c in counts.values())
    return {"draws": n, "distinct": len(counts), "equal_pairs": pairs,
            "self_collision_estimate": 2 * pairs / (n * (n - 1)),
            "empirical_self_collision": sum(c * c for c in counts.values()) / n ** 2}


def mixture_collision(corpus_masses, generator_counts, leaf_probabilities):
    """Combine exact corpora with unbiased within-generator collision estimates.

    Cross-leaf products use separate seed banks; only each generator's diagonal
    needs the U-statistic correction. Corpus masses have already merged identical
    text globally, so all corpus and generator cross-terms remain represented.
    """
    mixture = defaultdict(float, corpus_masses)
    correction = 0.0
    for path, counts in generator_counts.items():
        probability = leaf_probabilities[path]
        stats = collision_statistics(counts)
        correction += probability ** 2 * (stats["self_collision_estimate"] - stats["empirical_self_collision"])
        for key, count in counts.items():
            mixture[key] += probability * count / stats["draws"]
    return sum(p * p for p in mixture.values()) + correction


def inventory():
    leaves = []
    files = []

    def weight(path):
        config = path / "node.toml"
        return tomllib.loads(config.read_text(encoding="utf-8")).get("weight", 1) if config.exists() else 1

    def visit(path, probability):
        children = sorted(p for p in path.iterdir() if p.is_dir())
        if children:
            total = sum(weight(p) for p in children)
            for child in children:
                visit(child, probability * Fraction(weight(child), total))
            return
        corpus = path / "records.json"
        source = corpus if corpus.exists() else path / "generator.rs"
        leaves.append({"path": path.relative_to(ROOT / "families").as_posix(),
                       "kind": "corpus" if corpus.exists() else "generator",
                       "probability": float(probability), "probability_fraction": str(probability),
                       "weight": weight(path), "source_sha256": digest(source.read_bytes()),
                       "record_count": len(json.loads(corpus.read_text(encoding="utf-8"))) if corpus.exists() else None})

    visit(ROOT / "families", Fraction(1))
    for base in [ROOT / "families", ROOT / "src", ROOT / "dashboard"]:
        for path in sorted(base.rglob("*")):
            if path.is_file():
                files.append((path.relative_to(ROOT).as_posix(), digest(path.read_bytes())))
    for name in ["Cargo.toml", "Cargo.lock", "build.rs", "build_support.rs"]:
        files.append((name, digest((ROOT / name).read_bytes())))
    return leaves, {"source_fingerprint": digest(json.dumps(files, separators=(",", ":")).encode()),
                    "source_files": len(files), "source_hashes": dict(files)}


@contextmanager
def dashboard(binary):
    server = subprocess.Popen([str(binary), "--dashboard", "--port", "0"],
                              stdout=subprocess.PIPE, encoding="utf-8")
    try:
        line = server.stdout.readline()
        if not line.startswith("Dashboard: "):
            raise RuntimeError(f"Dashboard did not start: {line!r}")
        base = line.split()[1].rstrip("/")

        def fetch(path):
            with urllib.request.urlopen(base + path, timeout=60) as response:
                return json.load(response)

        yield fetch
    finally:
        server.terminate()
        server.wait(timeout=10)
        server.stdout.close()


def walk(node):
    if not node["children"]:
        yield node
    for child in node["children"]:
        yield from walk(child)


def semantic_row(leaf, split, position, text, size, method, **extra):
    return {"sample_id": f"{split}:{leaf['path']}:{position}", "split": split,
            "path": leaf["path"], "kind": leaf["kind"], "text": text,
            "text_sha256": text_hash(text), "conditional_weight": 1 / size,
            "probability": leaf["probability"] / size, "sampling_method": method, **extra}


def build_bank(binary, output, master_seed=20261030, generator_draws=512, semantic_draws=64):
    started = time.time()
    output.mkdir(parents=True, exist_ok=True)
    leaves, fingerprints = inventory()
    metadata = {"git_revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
                "git_status": subprocess.check_output(["git", "status", "--short"], cwd=ROOT, text=True).splitlines(),
                "binary": str(binary), "binary_sha256": digest(binary.read_bytes()),
                "binary_size": binary.stat().st_size, "master_seed": master_seed,
                "generator_draws_per_split": generator_draws, "semantic_draws_per_split": semantic_draws,
                "seed_design": "SHA256-keyed independent base seeds per leaf, split, block; each API block uses 12 adjacent u64 seeds with independent StdRng initialization; last block truncated",
                "corpus_design": "All corpus records determine exact probabilities. Semantic banks enumerate leaves <=semantic_draws in both splits; larger leaves use disjoint uniform subsets of min(semantic_draws, floor(n/2)) from one seeded permutation.",
                "semantic_weights": "conditional_weight sums to 1 within leaf/split; probability is leaf_probability*conditional_weight, not the chance of drawing an individual source record",
                **fingerprints}
    exact = {}
    verification = {"tree_leaves_checked": 0, "corpus_embedded_texts_checked": 0,
                    "generator_cli_reproductions_checked": 0}
    generator_counts = {split: {} for split in ["train", "validation"]}
    exact_leaf_collision = {}
    with dashboard(binary) as fetch, (output / "generators.jsonl").open("w", encoding="utf-8") as generators, (output / "train.jsonl").open("w", encoding="utf-8") as train, (output / "validation.jsonl").open("w", encoding="utf-8") as validation:
        api_leaves = {node["path"]: node for node in walk(fetch("/api/tree"))}
        assert set(api_leaves) == {leaf["path"] for leaf in leaves}, "Source and binary leaf sets differ"
        for leaf in leaves:
            node = api_leaves[leaf["path"]]
            assert node["kind"] == ("dataset" if leaf["kind"] == "corpus" else "generator")
            assert node["weight"] == leaf["weight"] and abs(node["probability"] - leaf["probability"]) < 1e-14
            assert node["count"] == leaf["record_count"]
            leaf["node_id"] = node["id"]
            verification["tree_leaves_checked"] += 1
            if leaf["kind"] == "corpus":
                records = json.loads((ROOT / "families" / leaf["path"] / "records.json").read_text(encoding="utf-8"))
                record_mass = Fraction(leaf["probability_fraction"]) / len(records)
                local = Counter()
                for index, record in enumerate(records):
                    key = merge_corpus_record(exact, leaf["path"], index, record, record_mass)
                    local[key] += 1
                exact_leaf_collision[leaf["path"]] = sum(c * c for c in local.values()) / len(records) ** 2
                sample_seed = keyed_seed(master_seed, leaf["path"], "embedded-verification")
                batch = fetch(f"/api/outputs?node={node['id']}&seed={sample_seed}&offset=0")
                for item in batch["items"]:
                    assert item["text"] == records[item["record"] - 1]["text"], f"Stale embedded corpus: {leaf['path']}"
                    verification["corpus_embedded_texts_checked"] += 1
                order = list(range(len(records)))
                bank_seed = keyed_seed(master_seed, leaf["path"], "corpus-holdout")
                random.Random(bank_seed).shuffle(order)
                size = len(records) if len(records) <= semantic_draws else min(semantic_draws, len(records) // 2)
                leaf["semantic_rows_per_split"] = size
                leaf["corpus_bank_seed"] = str(bank_seed)
                for split, stream, start in [("train", train, 0), ("validation", validation, size)]:
                    indexes = range(len(records)) if len(records) <= semantic_draws else order[start:start + size]
                    method = "corpus-enumeration" if len(records) <= semantic_draws else "corpus-disjoint-uniform-subset"
                    for position, index in enumerate(indexes):
                        record = records[index]
                        write_row(stream, semantic_row(leaf, split, position, record["text"], size, method,
                                                       record_index=index, record_id=record["id"],
                                                       bank_seed=str(bank_seed), nominal_record_probability=float(record_mass)))
            else:
                leaf["semantic_rows_per_split"] = semantic_draws
                for split, stream in [("train", train), ("validation", validation)]:
                    rng = random.Random(keyed_seed(master_seed, leaf["path"], split, "generator-blocks"))
                    outputs = []
                    block = 0
                    while len(outputs) < generator_draws:
                        base_seed = rng.getrandbits(64)
                        batch = fetch(f"/api/outputs?node={node['id']}&seed={base_seed}&offset=0")
                        for item in batch["items"][:generator_draws - len(outputs)]:
                            outputs.append({"text": item["text"], "seed": item["seed"],
                                            "block": block, "block_seed": str(base_seed)})
                        block += 1
                    generator_counts[split][leaf["path"]] = Counter(text_hash(item["text"]) for item in outputs)
                    # Every generator is checked against direct seeded leaf sampling.
                    if split == "train":
                        result = subprocess.run([str(binary), "--family", leaf["path"], "--seed", outputs[0]["seed"], "--json"],
                                                check=True, capture_output=True, text=True, encoding="utf-8")
                        assert json.loads(result.stdout)["text"] == outputs[0]["text"]
                        verification["generator_cli_reproductions_checked"] += 1
                    for position, item in enumerate(outputs):
                        row = semantic_row(leaf, split, position, item.pop("text"), generator_draws,
                                           "generator-random-base-adjacent-seed-blocks", **item)
                        write_row(generators, row)
                        if position < semantic_draws:
                            row = {**row, "conditional_weight": 1 / semantic_draws,
                                   "probability": leaf["probability"] / semantic_draws}
                            write_row(stream, row)
            if verification["tree_leaves_checked"] % 100 == 0:
                print(f"Collected {verification['tree_leaves_checked']}/{len(leaves)} leaves", flush=True)
    with (output / "corpus.jsonl").open("w", encoding="utf-8") as stream:
        for key in sorted(exact):
            write_row(stream, exact[key])
    corpus_collision = sum(row["probability"] ** 2 for row in exact.values())
    leaf_by_path = {leaf["path"]: leaf for leaf in leaves}
    split_metrics = {}
    all_generators = []
    for split, counts_by_leaf in generator_counts.items():
        for path, counts in counts_by_leaf.items():
            probability = leaf_by_path[path]["probability"]
            stats = collision_statistics(counts)
            all_generators.append({"split": split, "path": path, **stats,
                                   "root_collision_contribution": probability ** 2 * stats["self_collision_estimate"],
                                   "most_common": [{"text_sha256": key, "count": count,
                                                    "estimated_root_probability": probability * count / stats["draws"]}
                                                   for key, count in counts.most_common(3)]})
        split_metrics[split] = {"exact_corpus_collision": corpus_collision,
                                "total_exact_text_collision_estimate": mixture_collision(
                                    {key: entry["probability"] for key, entry in exact.items()}, counts_by_leaf,
                                    {path: leaf["probability"] for path, leaf in leaf_by_path.items()}),
                                "zero_observed_collision_generator_leaves": sum(row["equal_pairs"] == 0 for row in all_generators if row["split"] == split)}
    metadata["elapsed_seconds"] = time.time() - started
    metadata["binary_verification"] = verification
    metadata["artifacts"] = {name: {"sha256": digest((output / name).read_bytes()), "bytes": (output / name).stat().st_size}
                             for name in ["corpus.jsonl", "generators.jsonl", "train.jsonl", "validation.jsonl"]}
    write_json(output / "inventory.json", {"schema_version": 1, "metadata": metadata, "leaves": leaves})
    summary = {"schema_version": 1, "leaf_count": len(leaves),
               "corpus_leaves": sum(leaf["kind"] == "corpus" for leaf in leaves),
               "generator_leaves": sum(leaf["kind"] == "generator" for leaf in leaves),
               "corpus_records": sum(len(row["members"]) for row in exact.values()),
               "unique_corpus_texts": len(exact), "corpus_probability": sum(row["probability"] for row in exact.values()),
               "generator_probability": sum(leaf["probability"] for leaf in leaves if leaf["kind"] == "generator"),
               "split_metrics": split_metrics,
               "hottest_corpus_texts": sorted(exact.values(), key=lambda row: -row["probability"])[:20],
               "generator_repetition": sorted(all_generators, key=lambda row: -row["root_collision_contribution"]),
               "exact_corpus_leaf_self_collision": exact_leaf_collision,
               "uncertainty": "Corpus probabilities are exact, including raw-text duplicates across leaves. Generator estimates are finite-sample U-statistics; zero observed equal pairs does not establish zero collision. Adjacent seeds within independently keyed blocks are deterministic pseudorandom streams, not independent operating-system entropy. Train/validation agreement measures stability, not a distribution-free confidence interval. Corpus semantic enumeration reuses small leaves in both splits; larger leaves have disjoint record identities, but duplicate text may still cross splits.",
               "elapsed_seconds": metadata["elapsed_seconds"], "binary_verification": verification}
    write_json(output / "sampling-summary.json", summary)
    print(json.dumps({key: summary[key] for key in ["leaf_count", "corpus_records", "unique_corpus_texts", "split_metrics", "elapsed_seconds", "binary_verification"]}, indent=2))
    return summary


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, default=ROOT / "target/balance-study")
    parser.add_argument("--seed", type=int, default=20261030)
    parser.add_argument("--generator-draws", type=int, default=512)
    parser.add_argument("--semantic-draws", type=int, default=64)
    args = parser.parse_args()
    if not 1 <= args.semantic_draws <= args.generator_draws or args.generator_draws < 2:
        parser.error("require 1 <= semantic-draws <= generator-draws and generator-draws >= 2")
    build_bank(args.binary.resolve(), args.output, args.seed, args.generator_draws, args.semantic_draws)


if __name__ == "__main__":
    main()
