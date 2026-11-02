"""Reproduce the bounded WordNet 3.0 lexical collections from the official tarball.

Run from the repository root: python -B families/language/import_wordnet.py
Downloads are development-only; generated records are embedded by the Rust build.
"""

import argparse
from collections import Counter, defaultdict
import hashlib
import json
from pathlib import Path
import re
import tarfile
import urllib.request

ROOT = Path(__file__).resolve().parent
URL = "https://wordnetcode.princeton.edu/3.0/WordNet-3.0.tar.gz"
SHA256 = "640db279c949a88f61f851dd54ebbb22d003f8b90b85267042ef85a3781d3a52"
PROVENANCE = (
    f"Princeton WordNet 3.0 (2006), {URL}; archive SHA256 {SHA256}. "
    "Redistributed under its permissive WordNet license; full notice in lexicon/LICENSE. "
    "Definitions extracted verbatim, usage quotations omitted. IDs preserve POS and byte offset. "
    "Development-time filtering and selection: families/language/import_wordnet.py."
)
# Each leaf uses one native lexical domain. No topical classification is inferred.
DOMAINS = {
    "noun.attribute": "nouns/attributes", "noun.cognition": "nouns/cognition",
    "noun.communication": "nouns/communication", "noun.feeling": "nouns/feelings",
    "noun.phenomenon": "nouns/phenomena", "noun.process": "nouns/processes",
    "noun.relation": "nouns/relations", "noun.shape": "nouns/shapes",
    "noun.state": "nouns/states", "noun.time": "nouns/time",
    "verb.change": "verbs/change", "verb.cognition": "verbs/cognition",
    "verb.communication": "verbs/communication", "verb.creation": "verbs/creation",
    "verb.emotion": "verbs/emotion", "verb.motion": "verbs/motion",
    "verb.perception": "verbs/perception", "verb.social": "verbs/social",
}
BAD = re.compile(
    r"\b(obsolete|archaic|offensive|derogatory|slur|vulgar|obscen\w*|racial|racially|"
    r"negro\w*|nigger\w*|retard\w*|idiot\w*|moron\w*|imbecil\w*|savage\w*|"
    r"primitive\w*|lunatic\w*|prostitut\w*|sodom\w*|onan\w*|copulat\w*|"
    r"masturbat\w*|genital\w*|penis|vagina|intercourse|rape|raping|rapist|"
    r"killing|murder\w*|suicide|suicidal|drug addict|mental retardation|cathode|anode)\b", re.I
)
# A reviewed source synonym makes this uncommon tagged usage intelligible without
# replacing its historical gloss. Every displayed word still belongs to the synset.
SENSE_LEMMAS = {("a", "00922439"): ("tickling", "tingling")}


def clean_word(raw):
    return re.sub(r"\((?:a|p|ip)\)$", "", raw).replace("_", " ")


def load(archive):
    """Parse semantic pointers separately from lexical word-number pointers."""
    synsets = {}
    counts = defaultdict(Counter)
    with tarfile.open(archive, "r:gz") as source:
        def read(name):
            member = source.extractfile("WordNet-3.0/" + name)
            if member is None:
                raise ValueError("Missing archive member: " + name)
            return member.read().decode("utf-8")

        license_text = read("LICENSE")
        domain_names = {int(fields[0]): fields[1] for line in read("dict/lexnames").splitlines()
                        if (fields := line.split())}
        for line in read("dict/index.sense").splitlines():
            key, offset, _sense, count = line.split()
            lemma, key_fields = key.split("%")
            pos = {"1": "n", "2": "v", "3": "a", "4": "r", "5": "a"}[key_fields[0]]
            counts[(pos, offset)][clean_word(lemma).lower()] += int(count)
        for filename, pos in (("noun", "n"), ("verb", "v"), ("adj", "a"), ("adv", "r")):
            for line in read("dict/data." + filename).splitlines():
                if not line or line.startswith(" "):
                    continue
                data, gloss = line.split("|", 1)
                fields = data.split()
                offset, domain, subtype, word_count = fields[:4]
                word_count = int(word_count, 16)
                words = [clean_word(fields[4 + 2 * i]) for i in range(word_count)]
                pointer_start = 4 + 2 * word_count
                pointer_count = int(fields[pointer_start])
                pointers = [fields[pointer_start + 1 + 4*i:pointer_start + 5 + 4*i]
                            for i in range(pointer_count)]
                # Usage quotations carry independent wording and are not needed for senses.
                definition = re.split(r';\s*"', gloss.strip(), maxsplit=1)[0].strip()
                synsets[(pos, offset)] = {
                    "pos": pos, "offset": offset, "domain": domain_names[int(domain)], "words": words,
                    "definition": definition, "pointers": pointers, "subtype": subtype,
                    "counts": counts[(pos, offset)],
                }
    return synsets, license_text


def usable(synset, min_count=2):
    definition = synset["definition"]
    words = synset["words"]
    return (sum(synset["counts"].values()) >= min_count
            and 18 <= len(definition) <= 240
            and '"' not in definition and not BAD.search(definition + " " + " ".join(words))
            and all(re.fullmatch(r"[A-Za-z][A-Za-z '\-]*", word) for word in words)
            and all(len(word) <= 32 for word in words)
            and not re.search(r"\b(?:someone|something) (?:who|that)\b", definition)
            and not re.search(r"\([^)]*(?:medicin|psycholog|psychiatr|slang)[^)]*\)", definition, re.I))


def score(synset):
    # Stable diversity sample of corpus-attested senses, rather than only top frequent words.
    return hashlib.sha256((synset["pos"] + synset["offset"]).encode()).hexdigest()


def record(synset):
    part = {"n": "n.", "v": "v.", "a": "adj.", "r": "adv."}[synset["pos"]]
    words = sorted({word for word in synset["words"] if synset["counts"][word.lower()]},
                   key=lambda word: (-synset["counts"][word.lower()], word))[:6]
    reviewed = SENSE_LEMMAS.get((synset["pos"], synset["offset"]))
    if reviewed:
        if not set(reviewed) <= set(synset["words"]):
            raise ValueError("Reviewed display lemma is absent from its source synset")
        words = reviewed
    wording = " / ".join(words)
    return {
        "id": f"wn30-{synset['pos']}-{synset['offset']}",
        "text": f"{wording} ({part})\n{synset['definition']}",
        "provenance": PROVENANCE + (
            " Reviewed display includes the source synonym tickling to identify the sense of tingling."
            if reviewed else ""
        ),
    }


def write(path, records, check=False):
    destination = ROOT / "lexicon" / path
    exclusions = {}
    for exclusion_file in (ROOT / "lexicon").rglob("exclusions.json"):
        for item in json.loads(exclusion_file.read_text(encoding="utf-8")):
            assert set(item) == {"id", "reason"} and item["reason"].strip(), exclusion_file
            exclusions[item["id"]] = item["reason"]

    def excluded(item):
        if item["id"] in exclusions:
            return True
        # Reject a bad historical sense wherever it occurs in a relation too.
        pair = re.fullmatch(r"wn30-ant-([nav])-([0-9]{8})-([0-9]{8})-[0-9a-f]{8}", item["id"])
        return bool(pair and any(f"wn30-{pair[1]}-{offset}" in exclusions
                                 for offset in (pair[2], pair[3])))

    # Apply reviewed exclusions AFTER bounded selection: replay never adds unseen
    # replacement records to a leaf whose dedicated review is already complete.
    records = [item for item in records if not excluded(item)]
    assert records, path
    assert len({record["id"] for record in records}) == len(records), path
    text = json.dumps(records, ensure_ascii=False, indent=2) + "\n"
    write_text(destination / "records.json", text, check)
    print(f"{path}: {len(records)}")


def write_text(path, text, check):
    if check:
        if not path.exists() or path.read_text(encoding="utf-8") != text:
            raise ValueError("Generated file differs: " + str(path))
    else:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")


def import_records(archive, check=False):
    synsets, license_text = load(archive)
    accepted = {key: value for key, value in synsets.items() if usable(value)}
    for domain, path in DOMAINS.items():
        candidates = sorted((value for value in accepted.values() if value["domain"] == domain), key=score)
        write("senses/" + path, [record(value) for value in candidates[:160]], check)
    # Adjectives and adverbs have no native topical lexical-file subdivisions.
    for pos, path in (("a", "adjectives"), ("r", "adverbs")):
        candidates = sorted((value for value in accepted.values() if value["pos"] == pos), key=score)
        write("senses/" + path, [record(value) for value in candidates[:240]], check)
    for pos, path in (("n", "nouns"), ("v", "verbs"), ("a", "adjectives")):
        pairs = {}
        for key, value in accepted.items():
            if key[0] != pos:
                continue
            for symbol, offset, target_pos, word_indices in value["pointers"]:
                if symbol != "!" or word_indices == "0000":
                    continue
                target_key = ("a" if target_pos == "s" else target_pos, offset)
                target = accepted.get(target_key)
                if target is None:
                    continue
                source_index, target_index = int(word_indices[:2], 16) - 1, int(word_indices[2:], 16) - 1
                source_word = value["words"][source_index]
                target_word = target["words"][target_index]
                if (source_word.lower() == target_word.lower()
                        or not value["counts"][source_word.lower()]
                        or not target["counts"][target_word.lower()]):
                    continue
                identity = tuple(sorted(((key, source_word), (target_key, target_word))))
                pairs[identity] = {
                    "id": f"wn30-ant-{pos}-{identity[0][0][1]}-{identity[1][0][1]}-"
                          + hashlib.sha256(repr(identity).encode()).hexdigest()[:8],
                    "text": f"{source_word} — {value['definition']}\n"
                            f"{target_word} — {target['definition']}",
                    "provenance": PROVENANCE + " Direct lexical ! pointer with exact source/target word numbers; unordered pairs deduplicated.",
                }
        chosen = sorted(pairs.values(), key=lambda value: hashlib.sha256(value["id"].encode()).hexdigest())[:160]
        write("relations/antonyms/" + path, chosen, check)
    write_text(ROOT / "lexicon" / "LICENSE",
        "\n".join(line.rstrip() for line in license_text.splitlines()) + "\n",
        check,
    )


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--archive", type=Path, default=ROOT.parents[1] / "target/imports/wordnet/WordNet-3.0.tar.gz")
    parser.add_argument("--check", action="store_true", help="Verify generated files without rewriting them")
    args = parser.parse_args()
    if not args.archive.exists():
        args.archive.parent.mkdir(parents=True, exist_ok=True)
        urllib.request.urlretrieve(URL, args.archive)
    if hashlib.sha256(args.archive.read_bytes()).hexdigest() != SHA256:
        raise ValueError("WordNet archive checksum mismatch")
    import_records(args.archive, args.check)


if __name__ == "__main__":
    main()
