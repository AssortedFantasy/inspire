"""Rebuild people from the pinned, downloaded AgeDataset V1 (2022)."""

import argparse
from collections import defaultdict
import csv
import hashlib
import json
from pathlib import Path
import re
import unicodedata
import urllib.request

ROOT = Path(__file__).resolve().parent
CACHE = ROOT.parents[1] / "target" / "people-sources"
COMMIT = "776b4811e992ed593a59c01ff327dcc056e3ab4d"
LIMIT = 120
ATTRIBUTION = "Source: Issa Annamoradnejad & Rahimberdi Annamoradnejad, AgeDataset V1 (2022), adapted metadata, https://github.com/Moradnejad/AgeDataset; CC BY 4.0, https://creativecommons.org/licenses/by/4.0/."

# Only explicit role words in downloaded descriptions; first-match placement
# is a browsing convention, never a claim that a person worked in one field.
ROLES = {
    "arts/literature/poets": r"\bpoet(?:ess)?\b",
    "arts/literature/dramatists": r"\b(?:playwright|dramatist)\b",
    "arts/literature/fiction": r"\b(?:novelist|(?<!non-)(?<!non )fiction writer|short[ -]story writer)\b",
    "arts/visual/painters": r"\bpainter\b",
    "arts/visual/sculptors": r"\bsculptor\b",
    "arts/visual/photographers": r"\bphotographer\b",
    "arts/visual/architects": r"\barchitect\b",
    "arts/music/composers": r"\bcomposer\b",
    "arts/music/performers": r"\b(?:singer|pianist|violinist|guitarist|trumpeter|saxophonist|cellist|drummer|musician)\b",
    "arts/music/conductors": r"\bconductor\b",
    "arts/performance/filmmakers": r"\b(?:film director|filmmaker)\b",
    "arts/performance/actors": r"\b(?:actor|actress)\b",
    "arts/performance/dancers": r"\b(?:dancer|choreographer)\b",
    "thought/philosophers": r"\bphilosopher\b",
    "thought/scholarship/historians": r"\bhistorian\b",
    "thought/scholarship/linguists": r"\b(?:linguist|philologist)\b",
    "thought/scholarship/anthropologists": r"\b(?:anthropologist|ethnologist)\b",
    "society/reformers": r"\b(?:reformer|abolitionist|civil rights activist|women.s rights activist|suffragist|suffragette|human rights activist)\b",
    "society/knowledge/economists": r"\beconomist\b",
    "society/knowledge/educators": r"\b(?:educator|pedagogue|educational reformer)\b",
    "society/explorers": r"\bexplorer\b",
    "science/formal/statisticians": r"\bstatistician\b",
    "science/formal/mathematicians": r"\bmathematician\b",
    "science/physical/physicists": r"\bphysicist\b",
    "science/physical/chemists": r"\bchemist\b",
    "science/physical/astronomers": r"\bastronomer\b",
    "science/physical/geoscientists": r"\b(?:geologist|geophysicist|meteorologist|oceanographer|seismologist)\b",
    "science/life/naturalists": r"\bnaturalist\b",
    "science/life/botanists": r"\bbotanist\b",
    "science/life/zoologists": r"\b(?:zoologist|entomologist|ornithologist)\b",
    "science/life/biomedical": r"\b(?:physiologist|immunologist|microbiologist|biochemist|geneticist|neuroscientist|physician|surgeon)\b",
    "technology/computer-scientists": r"\b(?:computer scientist|computer programmer|programming pioneer)\b",
    "technology/engineering/inventors": r"\binventor\b",
    "technology/engineering/engineers": r"\bengineer\b",
}
PATTERNS = [(path, re.compile(pattern, re.I)) for path, pattern in ROLES.items()]
EXCLUDED = re.compile(r"\b(?:fictional|mythological|legendary|biblical|god|goddess|deity|current|serving|since|alleged|convicted|pornographic|adult film)\b", re.I)


def normalize(name):
    ascii_name = unicodedata.normalize("NFKD", name).encode("ascii", "ignore").decode()
    return re.sub(r"[^a-z0-9]", "", ascii_name.lower())


def sources():
    CACHE.mkdir(parents=True, exist_ok=True)
    manifest = json.loads((ROOT / "sources.json").read_text("utf-8"))
    for filename, metadata in sorted(manifest.items()):
        cached = CACHE / filename
        if not cached.exists():
            request = urllib.request.Request(metadata["url"], headers={"User-Agent": "InspireCorpus/1.0"})
            with urllib.request.urlopen(request, timeout=90) as response:
                cached.write_bytes(response.read())
        if hashlib.sha256(cached.read_bytes()).hexdigest() != metadata["sha256"]:
            raise ValueError(f"Checksum mismatch: {filename}")
        with cached.open(encoding="utf-8-sig", newline="") as handle:
            for row in csv.DictReader(handle):
                yield filename, metadata, row


def build(resolve=False):
    curated_file = ROOT / "curated.json"
    curated = json.loads(curated_file.read_text("utf-8"))
    corrections = json.loads((ROOT / "corrections.json").read_text("utf-8"))
    original_names = defaultdict(list)
    for item in curated:
        original_names[normalize(item["record"]["text"].split(" — ")[0])].append(item)
    identities = defaultdict(list)
    candidates = defaultdict(lambda: defaultdict(list))
    scanned = 0
    for filename, metadata, row in sources():
        scanned += 1
        qid, name = row["Id"].strip(), row["Name"].strip()
        description = " ".join(row["Short description"].split())
        correction = corrections.get(qid, {})
        if correction.get("exclude"):
            continue
        name = correction.get("name", name)
        description = correction.get("description", description)
        if normalize(name) in original_names:
            identities[normalize(name)].append((qid, name, description))
        if resolve:
            continue
        if not re.fullmatch(r"Q[1-9][0-9]*", qid) or len(description) < 20 or EXCLUDED.search(description):
            continue
        for path, pattern in PATTERNS:
            if pattern.search(description):
                if path == "arts/music/conductors" and re.search(r"\b(?:railway|railroad|train|bus)\b", description, re.I):
                    continue
                country = row["Country"].strip() or "unspecified"
                bucket = candidates[path][country]
                # Source order follows numeric QID. Bounded candidate prefixes
                # contain many familiar entities but are not notability scores.
                if len(bucket) < LIMIT:
                    bucket.append((qid, name, description, filename, metadata))
                break
    if resolve:
        for normalized, items in original_names.items():
            matches = {row[0]: row for row in identities[normalized]}
            if len(matches) == 1:
                for item in items:
                    item["qid"] = next(iter(matches))
                    item["identity_name"] = next(iter(matches.values()))[1]
            else:
                print("Identity requires review:", [i["record"]["id"] for i in items], list(matches.values()))
        curated_file.write_text(json.dumps(curated, ensure_ascii=False, indent=2) + "\n", "utf-8")
        print("Scanned", scanned, "rows; resolved", sum(bool(i.get("qid")) for i in curated), "original identities")
        return
    if any(not item.get("qid") for item in curated):
        raise ValueError("Resolve every original profile to an explicit source QID before rebuilding")
    leaves = defaultdict(list)
    seen = set()
    for item in curated:
        if item["qid"] in seen:
            raise ValueError("Duplicate curated identity: " + item["qid"])
        leaves[item["path"]].append(item["record"])
        seen.add(item["qid"])
    for path, buckets in candidates.items():
        added = 0
        while buckets and added < LIMIT:
            for country in sorted(list(buckets)):
                qid, name, description, filename, metadata = buckets[country].pop(0)
                if not buckets[country]:
                    del buckets[country]
                if qid in seen:
                    continue
                leaves[path].append({
                    "id": "wikidata-" + qid.lower(),
                    "text": f"{name} — {description.rstrip('.')}.\n{ATTRIBUTION}",
                    "provenance": f"AgeDataset V1 (2022), {filename}, Wikidata {qid}, https://www.wikidata.org/wiki/{qid}; {metadata['url']}; SHA-256 {metadata['sha256']}; CC BY 4.0. English description retained with whitespace/final punctuation normalization; inferred date/gender/death fields omitted. Role and citizenship-bucket selection: people/README.md." + (f" Reviewed correction: {corrections[qid]['reason']} Source: {corrections[qid]['source']}; checked 2026-10-13." if qid in corrections else ""),
                })
                seen.add(qid)
                added += 1
                if added >= LIMIT:
                    break
    for path, records in sorted(leaves.items()):
        leaf = ROOT / path
        leaf.mkdir(parents=True, exist_ok=True)
        (leaf / "records.json").write_text(json.dumps(records, ensure_ascii=False, indent=2) + "\n", "utf-8")
        print(path, len(records))
    for legacy in ["arts/writers", "science/biomedical", "science/chemists", "science/mathematicians", "science/naturalists", "science/physicists", "technology/inventors", "thought/philosophy/philosophers", "society/public-life/reformers", "society/fieldwork/explorers", "technology/computing/computer-scientists"]:
        old_leaf = ROOT / legacy
        if old_leaf.exists():
            for filename in ["records.json", "node.toml"]:
                (old_leaf / filename).unlink(missing_ok=True)
            old_leaf.rmdir()
    for legacy in ["thought/philosophy", "society/public-life", "society/fieldwork", "technology/computing"]:
        old_branch = ROOT / legacy
        if old_branch.exists():
            (old_branch / "node.toml").unlink(missing_ok=True)
            old_branch.rmdir()
    def weights(node):
        record_file = node / "records.json"
        count = len(json.loads(record_file.read_text("utf-8"))) if record_file.exists() else sum(weights(p) for p in node.iterdir() if p.is_dir())
        if node != ROOT:
            (node / "node.toml").write_text(f"weight = {count}\n", "utf-8")
        return count
    print("Scanned", scanned, "source rows; final profiles", weights(ROOT))


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--resolve-identities", action="store_true", help="resolve exact normalized source names for manual review")
    build(parser.parse_args().resolve_identities)
