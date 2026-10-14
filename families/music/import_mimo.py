"""Import contextualized instrument kinds from the pinned MIMO vocabulary.

python -B families/music/import_mimo.py [--cache target/imports/mimo]
Only English SKOS preferred/alternate labels and broader edges are selected.
Development dependency: rdflib==7.2.1 (never needed by the executable).
"""
import argparse
from collections import defaultdict
import hashlib
import json
from pathlib import Path
import re
import sys
from urllib.request import urlopen

ROOT = Path(__file__).resolve().parent
REVISION = "9cfb5c9cc3147084c9ac01161102f8ac368b8f1b"
BASE = f"https://raw.githubusercontent.com/DOREMUS-ANR/knowledge-base/{REVISION}/"
FILES = {
    "vocabularies/mop-mimo.ttl": "adebe2bc255e6947971e32bf261018acfe7a96123387ff228247fb25786ca1dd",
    "vocabularies/README.md": "4cab9868b68045163a3f1efcc78f3c8badcb6c568a45ad42c09d2418b85176c4",
}
PREFIX = "http://www.mimo-db.eu/InstrumentsKeywords/"
SKOS = "http://www.w3.org/2004/02/skos/core#"
FAMILIES = {"2370": "percussion", "2230": "keyboard", "3101": "stringed", "3730": "wind", "2208": "electronic", "4475": "mechanical", "2347": "mirlitons"}
OMIT = {"Others", "Other instruments", "Electronic", "Mechanical", "Gamelan"}
OMIT_IDS = {"2348", "2216"}  # duplicate umbrella mirlitons; generic MIDI instrument
EXCLUDE_IDS = {"5291", "3777", "4766", "6795", "3239", "3705"}
CORRECTIONS = {"3613": "Appalachian dulcimer", "4296": "Bass ophicleide"}
SUBDIVIDE = {"keyboard", "percussion", "stringed", "wind"}


def source(cache, relative):
    path = cache / relative
    if not path.exists():
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(urlopen(BASE + relative, timeout=90).read())
    raw = path.read_bytes()
    assert hashlib.sha256(raw).hexdigest() == FILES[relative], f"Source checksum mismatch: {relative}"
    return raw.decode("utf8")


def parse(text):
    from rdflib import Graph, Literal, URIRef

    graph = Graph().parse(data=text, format="turtle")
    labels, alternate, parents = {}, defaultdict(set), defaultdict(set)
    for subject, predicate, value in sorted(graph):
        uri = str(subject)
        if predicate == URIRef(SKOS + "prefLabel") and isinstance(value, Literal) and value.language == "en":
            assert uri not in labels or labels[uri] == str(value), f"Conflicting preferred labels: {uri}"
            labels[uri] = str(value)
        elif predicate == URIRef(SKOS + "altLabel") and isinstance(value, Literal) and value.language == "en":
            alternate[uri].add(str(value))
        elif predicate == URIRef(SKOS + "broader") and isinstance(value, URIRef):
            parents[uri].add(str(value))
    print(f"Parsed {len(graph)} RDF triples, {len(labels)} English labels, {len(parents)} broader concepts")
    return labels, alternate, parents


def routes(uri, parents, active=()):
    if uri in active:
        raise ValueError(f"Cyclic broader relationship: {uri}")
    if uri.startswith(PREFIX) and uri[len(PREFIX):] in FAMILIES:
        return [(uri,)]
    return [(uri,) + path for parent in sorted(parents.get(uri, ())) for path in routes(parent, parents, active + (uri,))]


def import_records(cache):
    declaration = source(cache, "vocabularies/README.md")
    assert "Creative Commons Attribution 4.0" in declaration and "copyright of MIMO" in declaration
    labels, alternate, parents = parse(source(cache, "vocabularies/mop-mimo.ttl"))
    grouped = defaultdict(list)
    skipped = defaultdict(int)
    for uri, name in sorted(labels.items()):
        source_id = uri.removeprefix(PREFIX)
        if source_id in EXCLUDE_IDS:
            skipped["opaque/conflicting/duplicate concept"] += 1
            continue
        if not uri.startswith(PREFIX) or uri[len(PREFIX):] in FAMILIES or uri[len(PREFIX):] in OMIT_IDS or name in OMIT or name == "MIDI instrument":
            skipped["scheme/category/ensemble"] += 1
            continue
        paths = routes(uri, parents)
        paths = [path for path in paths if all(node in labels for node in path)]
        if not paths:
            skipped["outside selected instrument families"] += 1
            continue
        # The pinned snapshot gives one selected family per retained instrument.
        categories = {FAMILIES[path[-1][len(PREFIX):]] for path in paths}
        if len(categories) != 1:
            skipped["multiple catalogue families"] += 1
            continue
        category = categories.pop()
        subfamily = None
        if category in SUBDIVIDE:
            # Root-adjacent source umbrellas become branches, not payload records.
            umbrella_ids = sorted({path[-2] for path in paths if len(path) > 1}, key=lambda p: int(p.removeprefix(PREFIX)))
            if uri in umbrella_ids:
                skipped["subfamily umbrella"] += 1
                continue
            subfamily = umbrella_ids[0]
        context = sorted({labels[path[1]] for path in paths if len(path) > 1 and labels[path[1]] not in OMIT})
        if not context and category in {"electronic", "mechanical"}:
            context = [labels[paths[0][-1]]]
        if not context:
            skipped["opaque broader context"] += 1
            continue
        if len(name) > 120 or not name.strip() or any(ord(c) < 32 for c in name):
            skipped["malformed label"] += 1
            continue
        name = CORRECTIONS.get(source_id, name)
        aliases = sorted(a for a in alternate.get(uri, ()) if a.casefold() != name.casefold() and len(a) <= 80)
        text = name
        useful_context = [kind for kind in context if kind.casefold() != name.casefold()]
        if useful_context:
            text += "\nType: " + "; ".join(useful_context) + "."
        if aliases:
            text += "\nAlso called: " + "; ".join(aliases[:4]) + "."
        slug = re.sub(r"[^a-z0-9]+", "-", labels[subfamily].lower()).strip("-") if subfamily else None
        relative = f"{category}/{slug}" if slug else category
        grouped[relative].append({"id": "mimo-" + source_id, "text": text,
            "provenance": f"Musical Instrument Museums Online (MIMO), DOREMUS-ANR vocabulary export, {BASE}vocabularies/mop-mimo.ttl; SHA256 {FILES['vocabularies/mop-mimo.ttl']}. MIMO copyright retained. CC BY 4.0 per {BASE}vocabularies/README.md and https://api.bnf.fr/fr/dumps-DOREMUS . English SKOS preferred/alternate labels and explicit broader relationships selected; no geographic origin or acoustic mechanism inferred. Historical museum vocabulary, not a universal classification. Accessed 2026-10-13."})
    counts = defaultdict(int)
    for relative, records in sorted(grouped.items()):
        assert len({r["id"] for r in records}) == len(records)
        assert len({r["text"] for r in records}) == len(records)
        path = ROOT / "instruments/catalogue" / relative / "records.json"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(records, ensure_ascii=False, indent=2) + "\n", encoding="utf8")
        (path.parent / "node.toml").write_text(f"weight = {len(records)}\n", encoding="utf8")
        category = relative.split("/")[0]
        counts[category] += len(records)
        print(f"{relative}: {len(records)}")
    for category in SUBDIVIDE:
        parent = ROOT / "instruments/catalogue" / category
        old_payload = parent / "records.json"
        if old_payload.exists():
            old_payload.unlink()
        (parent / "node.toml").write_text(f"weight = {counts[category]}\n", encoding="utf8")
    print("Excluded:", dict(skipped))
    print("Total:", sum(map(len, grouped.values())))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cache", type=Path, default=ROOT.parent.parent / "target/imports/mimo")
    cache = parser.parse_args().cache
    if (cache / "python-deps").is_dir():
        sys.path.insert(0, str(cache / "python-deps"))
    import_records(cache)
