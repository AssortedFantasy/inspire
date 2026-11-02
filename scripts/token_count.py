# /// script
# requires-python = ">=3.11"
# dependencies = ["tiktoken==0.12.0"]
# ///
"""Count UTF-8 text tokens or audit inspiration lengths with a local tokenizer.

Run with uv; its script metadata installs the pinned dependency automatically.
Only initial dependency/encoding downloads need network access.
"""

import argparse
from contextlib import nullcontext
import json
import os
from pathlib import Path
import sys

from balance_sampling import dashboard, walk


ROOT = Path(__file__).resolve().parents[1] / "families"


def jsonl_rows(stream):
    for line_number, line in enumerate(stream, 1):
        if not line.strip():
            continue
        try:
            row = json.loads(line)
        except ValueError as error:
            raise ValueError(f"line {line_number}: invalid JSON: {error}") from error
        if not isinstance(row, dict) or not isinstance(row.get("text"), str):
            raise ValueError(f"line {line_number}: expected an object with a string 'text'")
        yield row


def count(stream, encoding, jsonl=False, max_tokens=None):
    rows = jsonl_rows(stream) if jsonl else [{"text": stream.read()}]
    over_limit = False
    for row in rows:
        tokens = len(encoding.encode_ordinary(row["text"]))
        over_limit |= max_tokens is not None and tokens > max_tokens
        print(json.dumps({**row, "tokens": tokens}, ensure_ascii=False) if jsonl else tokens)
    return int(over_limit)


def leaf_samples(node, fetch, seeds, seed, root):
    if node["kind"] == "dataset":
        records = json.loads((root / node["path"] / "records.json").read_text(encoding="utf-8"))
        if len(records) != node["count"]:
            raise ValueError(f"source/binary record count differs: {node['path']}; rebuild the binary")
        for index, record in enumerate(records, 1):
            yield {"record_id": record["id"], "record": index, "text": record["text"]}
    else:
        offset = 0
        while offset < seeds:
            batch = fetch(f"/api/outputs?node={node['id']}&seed={seed}&offset={offset}")
            if not batch["items"] or batch["next"] <= offset:
                raise ValueError(f"generator sampling did not advance: {node['path']}")
            yield from batch["items"][:seeds - offset]
            offset = batch["next"]


def audit(fetch, encoding, max_tokens, seeds, seed, family="", root=ROOT):
    tree = fetch("/api/tree")
    nodes = list(walk(tree))
    nodes = [node for node in nodes if not family or node["path"] == family
             or node["path"].startswith(family + "/")]
    if not nodes:
        raise ValueError(f"unknown family: {family}")
    source_paths = {path.parent.relative_to(root).as_posix()
                    for path in (root / family).rglob("*")
                    if path.name in {"records.json", "generator.rs"}}
    if source_paths != {node["path"] for node in nodes}:
        raise ValueError("source/binary leaf paths differ; rebuild the binary")

    leaves, over_limit = [], []
    total_tokens = 0
    for node in nodes:
        samples, longest, token_sum, long_count = 0, 0, 0, 0
        kind = "corpus" if node["kind"] == "dataset" else "generator"
        for sample in leaf_samples(node, fetch, seeds, seed, root):
            tokens = len(encoding.encode_ordinary(sample["text"]))
            samples += 1
            token_sum += tokens
            longest = max(longest, tokens)
            if tokens > max_tokens:
                long_count += 1
                over_limit.append({"path": node["path"], "kind": kind, **sample, "tokens": tokens})
        total_tokens += token_sum
        leaves.append({"path": node["path"], "kind": kind, "samples": samples,
                       "mean_tokens": token_sum / samples if samples else 0,
                       "max_tokens": longest, "over_limit": long_count})
    leaves.sort(key=lambda row: (-row["max_tokens"], row["path"]))
    over_limit.sort(key=lambda row: -row["tokens"])
    return {"encoding": encoding.name, "max_tokens": max_tokens,
            "generator_seeds_per_leaf": seeds, "generator_start_seed": str(seed), "family": family,
            "totals": {"leaves": len(leaves), "samples": sum(row["samples"] for row in leaves),
                       "tokens": total_tokens, "over_limit": len(over_limit),
                       "corpus_leaves": sum(row["kind"] == "corpus" for row in leaves),
                       "generator_leaves": sum(row["kind"] == "generator" for row in leaves)},
            "leaves": leaves, "over_limit": over_limit}


def nonnegative(value):
    number = int(value)
    if number < 0:
        raise argparse.ArgumentTypeError("must be nonnegative")
    return number


def positive(value):
    number = nonnegative(value)
    if number == 0:
        raise argparse.ArgumentTypeError("must be positive")
    return number


def main():
    common = argparse.ArgumentParser(add_help=False)
    common.add_argument("--encoding", default="o200k_base", help="tiktoken encoding (default: o200k_base)")
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    counter = commands.add_parser("count", parents=[common], help="Count a text file or stdin")
    counter.add_argument("file", nargs="?", type=Path, help="UTF-8 input (default: stdin)")
    counter.add_argument("--jsonl", action="store_true", help="Count each JSONL object's text, preserving fields")
    counter.add_argument("--max-tokens", type=nonnegative, help="Exit 1 if any text exceeds this count")
    reviewer = commands.add_parser("audit", parents=[common], help="Check all records and sample every generator")
    reviewer.add_argument("--binary", type=Path, required=True, help="Freshly built inspire executable")
    reviewer.add_argument("--family", default="", help="Limit to a family, branch, or leaf path")
    reviewer.add_argument("--max-tokens", type=nonnegative, default=256, help="Review threshold (default: 256)")
    reviewer.add_argument("--seeds", type=positive, default=128, help="Samples per generator (default: 128)")
    reviewer.add_argument("--seed", type=nonnegative, default=0, help="First generator seed (default: 0)")
    reviewer.add_argument("--output", type=Path, help="Write full JSON report, including over-limit texts")
    args = parser.parse_args()
    if args.command == "audit" and args.seed + args.seeds > 2**64:
        parser.error("generator seed range must fit in unsigned 64-bit integers")
    try:
        import tiktoken
    except ImportError:
        parser.error("tiktoken is required; run with 'uv run scripts/token_count.py ...'")
    try:
        os.environ.setdefault("TIKTOKEN_CACHE_DIR", str(ROOT.parent / "target" / "tiktoken-cache"))
        encoding = tiktoken.get_encoding(args.encoding)
        if args.command == "count":
            context = args.file.open(encoding="utf-8", newline="") if args.file else nullcontext(sys.stdin)
            with context as stream:
                return count(stream, encoding, args.jsonl, args.max_tokens)
        with dashboard(args.binary.resolve()) as fetch:
            report = audit(fetch, encoding, args.max_tokens, args.seeds, args.seed, args.family)
        report.update(tiktoken_version=tiktoken.__version__, binary=str(args.binary.resolve()),
                      corpus_source=str(ROOT))
        if args.output:
            args.output.parent.mkdir(parents=True, exist_ok=True)
            args.output.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        totals = report["totals"]
        print(f"{totals['samples']} samples across {totals['leaves']} leaves; "
              f"{totals['over_limit']} exceed {args.max_tokens} tokens ({encoding.name}).")
        print("Max\tMean\tOver\tSamples\tPath")
        for leaf in report["leaves"][:20]:
            print(f"{leaf['max_tokens']}\t{leaf['mean_tokens']:.1f}\t{leaf['over_limit']}\t"
                  f"{leaf['samples']}\t{leaf['path']}")
        if args.output:
            print(f"Report: {args.output}")
        return int(bool(report["over_limit"]))
    except (OSError, ValueError, RuntimeError) as error:
        parser.exit(2, f"{parser.prog}: {error}\n")


if __name__ == "__main__":
    sys.stdin.reconfigure(encoding="utf-8", newline="")
    sys.stdout.reconfigure(encoding="utf-8")
    sys.exit(main())
