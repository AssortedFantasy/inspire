"""Replay reviewed complete paragraphs from three checksum-pinned editions."""

import argparse
import hashlib
import json
from pathlib import Path
import re
import urllib.request


ROOT = Path(__file__).resolve().parent
CACHE = ROOT.parents[2] / "target/imports/philosophy"


def paragraphs(raw):
    text = raw.decode("utf-8-sig").replace("\r\n", "\n")
    body = re.split(r"\*\*\* START OF [^\n]+\*\*\*", text, maxsplit=1)[1]
    body = body.split("*** END OF", 1)[0]
    return [re.sub(r"\s+", " ", p).strip()
            for p in re.split(r"\n\s*\n", body) if p.strip()]


def records(source, raw):
    ps = paragraphs(raw)
    result = []
    for selection in source["selections"]:
        indices = selection["paragraphs"]
        assert indices == list(range(indices[0], indices[-1] + 1))
        assert ps[indices[0]].startswith(selection["opening"])
        # Underscores encode italics, and bracketed integers are note calls.
        # Preserve the complete paragraphs' words, spelling and punctuation.
        body = "\n\n".join(re.sub(r"\[\d+\]", "", ps[i]).replace("_", "")
                            for i in indices)
        if source["ebook"] == 9662:
            body = re.sub(r"^\d+\. ", "", body)
        credit = (f"{source['author']}, {source['title']}, "
                  f"{selection['location']} ({source['edition']})")
        locator = ", ".join(str(i + 1) for i in indices)
        result.append({
            "id": selection["id"],
            "text": body + "\n\n— " + credit,
            "provenance": (
                f"{source['catalog_url']}. {selection['location']}. "
                f"Normalized source paragraphs {locator}, counted from the ebook "
                f"start marker before filtering. Pinned UTF-8 SHA-256 "
                f"{source['sha256']}. {source['rights']}"
            ),
        })
    assert len({r["id"] for r in result}) == len(result)
    return result


def license_notice(sources, first_raw):
    notice = [
        "PHILOSOPHY PRIMARY TEXTS — SOURCE NOTICE",
        "Only the underlying public-domain prose is sampled. Inspire claims no "
        "copyright in these passages. Ebook metadata, trademark statements, "
        "editorial notes and licensing boilerplate are excluded from outputs. "
        "Source URLs and checksums identify the transcriptions for replay.",
    ]
    for source in sources:
        notice.append(f"{source['author']}, {source['title']} "
                      f"({source['edition']})\n{source['catalog_url']}\n"
                      f"{source['rights']}")
    notice.extend([
        "Source policy: https://www.gutenberg.org/policy/license.html (1.C)\n"
        "https://www.gutenberg.org/policy/permission.html",
        "The source ebook license is retained below for transparency. It applies "
        "to the trademarked ebook and creates no copyright in the underlying text.",
        first_raw.decode("utf-8-sig").replace("\r\n", "\n").split(
            "START: FULL LICENSE", 1)[1].strip(),
    ])
    return "\n\n".join(notice) + "\n"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--download", action="store_true", help="fetch missing cache files")
    parser.add_argument("--check", action="store_true", help="compare without changing outputs")
    args = parser.parse_args()
    sources = json.loads((ROOT / "sources.json").read_text(encoding="utf-8"))
    outputs = {}
    first_raw = None
    for source in sources:
        cache = CACHE / f"pg{source['ebook']}.txt"
        if args.download and not cache.exists():
            cache.parent.mkdir(parents=True, exist_ok=True)
            raw = urllib.request.urlopen(source["url"], timeout=60).read()
        else:
            raw = cache.read_bytes()
        if hashlib.sha256(raw).hexdigest() != source["sha256"]:
            raise ValueError(f"source checksum changed: {source['url']}")
        if args.download and not cache.exists():
            cache.write_bytes(raw)
        if first_raw is None:
            first_raw = raw
        items = records(source, raw)
        leaf = ROOT / source["path"]
        outputs[leaf / "records.json"] = json.dumps(items, ensure_ascii=False, indent=2) + "\n"
        outputs[leaf / "node.toml"] = f"weight = {len(items)}\n"
        print(f"{source['path']}: {len(items)} complete paragraph units")
    outputs[ROOT / "LICENSE.txt"] = license_notice(sources, first_raw)
    for path, expected in outputs.items():
        if args.check:
            if path.read_text(encoding="utf-8") != expected:
                raise ValueError(f"generated output differs: {path}")
        else:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(expected, encoding="utf-8", newline="\n")


if __name__ == "__main__":
    main()
