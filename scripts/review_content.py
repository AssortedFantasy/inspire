"""Inventory the sampling tree and export reproducible samples for human review.

Run after a build, using --binary target/release/inspire.exe to sample generators.
This report helps find repetition; it does not establish factual correctness.
"""

import argparse
from collections import Counter, defaultdict
import json
from pathlib import Path
import subprocess
import tomllib
import unicodedata


ROOT = Path(__file__).resolve().parents[1] / "families"


def normalized(text):
    return " ".join(unicodedata.normalize("NFKC", text).casefold().split())


def weight(path):
    config = path / "node.toml"
    return tomllib.loads(config.read_text(encoding="utf-8")).get("weight", 1) if config.exists() else 1


def review(binary=None, sample_count=3):
    families = defaultdict(lambda: Counter(records=0, corpora=0, generators=0))
    texts = defaultdict(list)
    leaves = []

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
            entry.update(kind="corpus", records=len(records), per_record_probability=probability / len(records))
            # Spread samples across the file, including both endpoints.
            indexes = sorted({i * (len(records) - 1) // max(1, sample_count - 1) for i in range(sample_count)})
            entry["samples"] = [records[i] for i in indexes]
            for record in records:
                texts[normalized(record["text"])].append(f"{relative}:{record['id']}")
        else:
            families[family].update(generators=1)
            entry["kind"] = "generator"
            if binary:
                outputs = []
                for seed in range(max(128, sample_count)):
                    result = subprocess.run([str(binary), "--family", relative, "--seed", str(seed)],
                                            check=True, capture_output=True, encoding="utf-8")
                    outputs.append(result.stdout.removesuffix("\n"))
                entry.update(distinct_in_128=len(set(outputs[:128])),
                             samples=[{"seed": i, "text": text} for i, text in enumerate(outputs[:sample_count])])
        leaves.append(entry)

    visit(ROOT, 1.0)
    return {"families": dict(sorted(families.items())),
            "totals": dict(sum(families.values(), Counter())),
            "max_leaf_depth": max(entry["path"].count("/") + 1 for entry in leaves),
            "duplicate_texts": [refs for refs in texts.values() if len(refs) > 1],
            "leaves": leaves}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, help="Sample generators with this built executable")
    parser.add_argument("--samples", type=int, default=3, help="Samples per leaf (positive integer)")
    parser.add_argument("--output", type=Path, help="Save the full JSON report here")
    args = parser.parse_args()
    if args.samples < 1:
        parser.error("--samples must be positive")
    report = review(args.binary.resolve() if args.binary else None, args.samples)
    if args.output:
        args.output.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    for family, counts in report["families"].items():
        print(f"{family}: {counts['records']} records, {counts['corpora']} corpora, {counts['generators']} generators")
    print(f"Totals: {report['totals']}; maximum depth: {report['max_leaf_depth']}; "
          f"duplicate text groups: {len(report['duplicate_texts'])}")


if __name__ == "__main__":
    main()
