"""Reproduce the embedded CC0 opening book from pinned Lichess TSV files."""
import csv
import hashlib
import io
import json
from pathlib import Path
import urllib.request

REVISION = "a6189a30dc273ccb21fc2536a9a2fefd5592a67a"
BASE = f"https://raw.githubusercontent.com/lichess-org/chess-openings/{REVISION}/"
ROOT = Path(__file__).resolve().parent
CACHE = ROOT.parents[1] / "target" / "games-import"


def category(eco, name):
    # Named flank systems retain their identities through transposing move orders.
    flank_systems = {
        "Hungarian Opening": "fianchetto-attacks/hungarian",
        "Lasker Simul Special": "fianchetto-attacks/hungarian",
        "Nimzo-Larsen Attack": "fianchetto-attacks/nimzo-larsen",
        "Van Geet Opening": "knight-openings/van-geet",
        "Amar Opening": "knight-openings/amar",
        "Sodium Attack": "knight-openings/sodium",
    }
    for prefix, path in flank_systems.items():
        if name.startswith(prefix):
            return "flank/other-systems/" + path
    if name.startswith("Colle System"):
        return "queen-pawn/queen-gambit/other-systems"
    if name.startswith("Modern Defense"):
        return "king-pawn/semi-open/other-defenses"
    if name.startswith("Sicilian"):
        return "king-pawn/semi-open/sicilian"
    if name.startswith("French"):
        return "king-pawn/semi-open/french"
    if name.startswith("Caro-Kann"):
        return "king-pawn/semi-open/caro-kann"
    if eco[0] == "B":
        return "king-pawn/semi-open/other-defenses"
    if eco[0] == "C":
        return "king-pawn/open-games"
    if "Slav" in name:
        return "queen-pawn/queen-gambit/slav"
    if eco[0] == "D" and int(eco[1:]) < 70:
        return "queen-pawn/queen-gambit/other-systems"
    if name.startswith("King's Indian"):
        return "queen-pawn/indian/kings-indian"
    if name.startswith("Nimzo-Indian"):
        return "queen-pawn/indian/nimzo-indian"
    if name.startswith("Queen's Indian"):
        return "queen-pawn/indian/queens-indian"
    if "Grünfeld" in name:
        return "queen-pawn/indian/grunfeld"
    if eco[0] == "E" or (eco[0] == "D" and int(eco[1:]) >= 70):
        return "queen-pawn/indian/other-systems"
    if eco[0] == "A" and int(eco[1:]) >= 40:
        return "queen-pawn/asymmetric-defenses"
    if name.startswith("English"):
        return "flank/english"
    if name.startswith(("Réti", "Zukertort")):
        return "flank/reti-and-zukertort"
    return "flank/other-systems"


def main():
    CACHE.mkdir(parents=True, exist_ok=True)
    previous = ROOT / "openings-source.json"
    expected = json.loads(previous.read_text(encoding="utf-8"))["sha256"] if previous.exists() else {}
    groups = {}
    hashes = {}
    for volume in "abcde":
        filename = volume + ".tsv"
        cached = CACHE / filename
        if not cached.exists():
            cached.write_bytes(urllib.request.urlopen(BASE + filename, timeout=60).read())
        data = cached.read_bytes()
        hashes[filename] = hashlib.sha256(data).hexdigest()
        if filename in expected and hashes[filename] != expected[filename]:
            raise ValueError(f"Pinned input checksum mismatch: {filename}")
        for row in csv.DictReader(io.StringIO(data.decode("utf-8")), delimiter="\t"):
            eco, name, pgn = row["eco"], row["name"], row["pgn"]
            stable = hashlib.sha256((eco + "\t" + name + "\t" + pgn).encode()).hexdigest()[:16]
            groups.setdefault(category(eco, name), []).append({
                "id": f"lichess-{eco.lower()}-{stable}",
                "text": f"{name} ({eco})\n{pgn}",
                "provenance": f"Lichess chess-openings, revision {REVISION}; {BASE}{filename}; CC0 1.0. Exact source name, ECO, and PGN; no move truncation. Legal SAN replay is checked by tests/chess_openings.rs."
            })
    for group, records in sorted(groups.items()):
        dest = ROOT / "board" / "chess" / "openings" / group
        dest.mkdir(parents=True, exist_ok=True)
        dest.joinpath("records.json").write_text(json.dumps(records, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    cached_license = CACHE / "COPYING.txt"
    if not cached_license.exists():
        cached_license.write_bytes(urllib.request.urlopen(BASE + "COPYING.txt", timeout=60).read())
    license_data = cached_license.read_bytes()
    hashes["COPYING.txt"] = hashlib.sha256(license_data).hexdigest()
    if "COPYING.txt" in expected and hashes["COPYING.txt"] != expected["COPYING.txt"]:
        raise ValueError("Pinned license checksum mismatch")
    license_path = ROOT / "board" / "chess" / "openings" / "LICENSE"
    license_path.write_bytes(license_data)
    ROOT.joinpath("openings-source.json").write_text(json.dumps({"revision": REVISION, "sha256": hashes, "counts": {group: len(records) for group, records in sorted(groups.items())}}, indent=2) + "\n", encoding="utf-8")
    print(sum(map(len, groups.values())), "opening records in", len(groups), "leaves")


if __name__ == "__main__":
    main()
