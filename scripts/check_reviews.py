"""Check dedicated leaf-review coverage against the current sampling tree."""

import argparse
from collections import Counter
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1] / "families"


def check(directory):
    leaves = {
        path.parent.relative_to(ROOT).as_posix()
        for name in ("records.json", "generator.rs")
        for path in directory.rglob(name)
    }
    manifest = directory / "review_manifest.json"
    data = json.loads(manifest.read_text(encoding="utf-8"))
    entries = data if isinstance(data, list) else data["leaves"]
    seen = set()
    reviewers = set()
    for entry in entries:
        raw = entry["path"].removeprefix("families/")
        candidates = {raw}
        for parent in (directory, *directory.parents):
            if not parent.is_relative_to(ROOT):
                break
            candidates.add((parent / raw).relative_to(ROOT).as_posix())
        matches = candidates & leaves
        assert len(matches) == 1, f"{manifest}: unknown or ambiguous leaf {raw!r}"
        path = matches.pop()
        assert path not in seen, f"{manifest}: duplicate review for {path}"
        seen.add(path)
        reviewer = entry["reviewer"]
        assert reviewer.startswith("/root/"), f"{path}: missing canonical reviewer"
        assert reviewer not in reviewers, f"{reviewer}: multiple final leaves"
        reviewers.add(reviewer)
        assert entry.get("status", "completed") in {"completed", "complete", "reviewed", "passed"}, path
        assert entry["checks"], f"{path}: missing substantive review notes"
    assert seen == leaves, f"{manifest}: missing reviews: {sorted(leaves - seen)}"
    return len(leaves)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--family", help="Check one family or reviewed branch")
    args = parser.parse_args()
    selected = (ROOT / args.family).resolve() if args.family else ROOT
    if not selected.is_relative_to(ROOT) or not selected.is_dir():
        parser.error("family must name a directory under families/")
    directories = [selected] if args.family else sorted(path for path in ROOT.iterdir() if path.is_dir())
    counts = Counter()
    for directory in directories:
        counts[directory.relative_to(ROOT).as_posix()] = check(directory)
    for family, count in counts.items():
        print(f"{family}: {count} distinct leaf reviewers")
    print(f"Total: {sum(counts.values())} reviewed leaves")


if __name__ == "__main__":
    main()
