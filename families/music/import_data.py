"""Rebuild music corpora from checksummed development-time downloads.

python -B families/music/import_data.py [--cache target/imports]
Dynamic Open Opus downloads are accepted only if they match this snapshot.
"""
import argparse
from collections import defaultdict
import hashlib
import json
from pathlib import Path
import re
import unicodedata
from urllib.request import urlopen

ROOT = Path(__file__).resolve().parent
OPUS_URL = "https://api.openopus.org/work/dump.json"
OPUS_SHA = "6e610314c613eba428907a790ffb632446322439c0c2b526305e06d113ca164f"
REV = "2e7adec11a0561696c236ae1fca82e88be173d42"
INSTRUMENT_URL = f"https://raw.githubusercontent.com/dariusk/corpora/{REV}/data/music/instruments.json"
INSTRUMENT_SHA = "a2be9c68eb6a536bd37961c78598cbf7c4a303b77ae495f24da27aeff01d2527"


def source(path, url, digest):
    path.parent.mkdir(parents=True, exist_ok=True)
    if not path.exists():
        path.write_bytes(urlopen(url, timeout=90).read())
    raw = path.read_bytes()
    actual = hashlib.sha256(raw).hexdigest()
    if actual != digest:
        raise ValueError(f"Snapshot mismatch for {url}: {actual}; retain the pinned input, do not silently refresh")
    return json.loads(raw)


def clean(value):
    return " ".join(unicodedata.normalize("NFC", value or "").split())


def write(relative, records):
    assert records and len({r["id"] for r in records}) == len(records)
    assert len({r["text"] for r in records}) == len(records)
    path = ROOT / relative / "records.json"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(sorted(records, key=lambda r: r["id"]), ensure_ascii=False, indent=2) + "\n", encoding="utf8")
    print(f"{relative}: {len(records)}")


def works(cache):
    data = source(cache / "music/openopus.json", OPUS_URL, OPUS_SHA)
    assert data["status"]["success"] == "true"
    grouped = defaultdict(list)
    seen = set()
    rejected = 0
    for composer in data["composers"]:
        name = clean(composer["complete_name"])
        for work in composer["works"]:
            title, subtitle, genre = (clean(work[key]) for key in ("title", "subtitle", "genre"))
            key = (name.casefold(), title.casefold())
            # Exclude bare catalogue fragments, corrupt/control text, and exact work duplicates.
            if key in seen or not title or re.fullmatch(r"(?:no\.?\s*)?\d+[a-z]?", title, re.I) or any(ord(c) < 32 or c == "\ufffd" for c in title):
                rejected += 1
                continue
            seen.add(key)
            assert genre in {"Chamber", "Keyboard", "Orchestral", "Stage", "Vocal"}
            identity = "\0".join((name, title)).encode("utf8")
            text = f"{name} — {title}"
            if subtitle and subtitle.casefold() != title.casefold():
                text += f"\n{subtitle}"
            text += f"\nPeriod: {clean(composer['epoch'])}."
            grouped[genre.lower()].append({"id": "openopus-" + hashlib.sha256(identity).hexdigest()[:20], "text": text,
                "provenance": f"Open Opus public-domain/CC0 metadata; {OPUS_URL}; API 1.20.6 snapshot 2026-10-13, SHA256 {OPUS_SHA}. Normalized whitespace/NFC; source classification retained. Licensing: https://www.openopus.org/ . No score, recording or lyrics."})
    for genre, records in grouped.items():
        write(f"repertoire/classical/{genre}", records)
    print(f"Open Opus rejected duplicates/fragments: {rejected}")


def instruments(cache):
    data = source(cache / "corpora/data/music/instruments.json", INSTRUMENT_URL, INSTRUMENT_SHA)
    categories = {
        "air-driven/flutes": "fife|flute|pan flute|piccolo",
        "air-driven/reeds": "bagpipe|bassoon|clarinet|harmonica|melodica|oboe|saxophone|accordion|concertina",
        "air-driven/lip-excited": "bugle|didgeridoo|flugelhorn|French horn|sousaphone|trombone|tuba|vuvuzela",
        "air-driven/pressure-fed": "air horn|calliope|pipe organ",
        "strings/plucked": "banjo|bass guitar|dobro|guitar|harp|harpsichord|lute|lyre|mandolin|sitar|ukulele|washtub bass",
        "strings/friction": "cello|fiddle|hurdy-gurdy|viola|violin",
        "strings/keyboard-struck": "piano|clavichord|grand piano|baby grand piano",
        "strings/varied-excitation": "dulcimer|zither",
        "membranes/struck": "kick drum|tambourine",
        "membranes/voice-excited": "kazoo",
        "solid-bodies": "glockenspiel|marimba|xylophone",
        "electronic": "Mellotron|theremin",
    }
    mapping = {item: category for category, items in categories.items() for item in items.split("|")}
    corrections = {"saxaphone": "saxophone", "mellotran": "Mellotron"}
    specific = {
        "air horn": "Compressed air drives a vibrating diaphragm; a horn projects the resulting sound.",
        "calliope": "Pressurized steam or compressed air sounds a bank of whistles, usually operated from a keyboard.",
        "pipe organ": "A wind supply feeds pipes controlled from keyboards and pedals. Flue pipes sound through an edge-directed air stream; reed pipes use vibrating reeds.",
        "accordion": "Bellows supply air to free reeds; keys or buttons select the sounding pitches.",
        "concertina": "Bellows supply air to free reeds; buttons select the sounding pitches.",
        "harmonica": "The player's breath sounds free reeds mounted inside the instrument.",
        "melodica": "A mouth-blown keyboard instrument with free reeds.",
        "bagpipe": "Air stored in a bag supplies one or more reed pipes; typical configurations include a melody pipe and drones.",
        "bassoon": "A double reed excites the air column of a folded conical bore.",
        "oboe": "A double reed excites the air column of a conical bore.",
        "clarinet": "A single reed excites the air column of a predominantly cylindrical bore.",
        "saxophone": "A single reed excites the air column of a conical bore.",
        "harpsichord": "Keys operate jacks whose plectra pluck the instrument's strings.",
        "hurdy-gurdy": "A hand-cranked wheel continuously rubs the strings; keys shorten the melody string's vibrating length.",
        "piano": "Keys operate hammers that strike strings; dampers control the decay of most notes.",
        "clavichord": "Keys raise metal tangents that strike the strings and remain in contact to define their vibrating length.",
        "dulcimer": "The name covers several string instruments: hammered dulcimers are struck with beaters; Appalachian dulcimers are commonly strummed or plucked.",
        "zither": "A structural family with strings extending across a body without a distinct neck. Different kinds may be plucked, struck or bowed.",
        "kick drum": "A drumhead is struck, usually by a beater operated with a foot pedal.",
        "tambourine": "A headed tambourine combines a struck membrane with sounding jingles. Headless variants sound through their jingles alone.",
        "kazoo": "Humming into the instrument excites a membrane that modifies the player's vocal sound; it is a mirliton.",
        "Mellotron": "A keyboard controls playback of sampled instrument sounds. Original Mellotrons use magnetic-tape strips; later digital models replay recorded samples electronically.",
        "theremin": "Electronic oscillators produce the sound; hand positions near antennas control pitch and volume without touching the instrument.",
        "marimba": "Mallets strike tuned bars, commonly coupled to resonators.",
        "xylophone": "Mallets strike tuned bars arranged by pitch.",
        "glockenspiel": "Mallets strike tuned metal bars arranged by pitch.",
    }
    grouped = defaultdict(list)
    for original in data["instruments"]:
        name = corrections.get(original, original)
        # Keep distinct traditional names, but remove redundant piano size descriptors.
        if name in {"baby grand piano", "grand piano"}:
            continue
        category = mapping[name]
        description = specific.get(name)
        text = name[0].upper() + name[1:]
        if description:
            text += f"\n{description}"
        grouped[category].append({"id": "corpora-instrument-" + hashlib.sha256(original.encode()).hexdigest()[:16], "text": text,
            "provenance": f"Name from Darius Kazemi and Corpora contributors, CC0: {INSTRUMENT_URL}. Original sound-mechanism grouping, not a complete Hornbostel–Sachs code. Corrected saxaphone→saxophone and mellotran→Mellotron; omitted piano size duplicates. Mechanism descriptions are original factual framing."})
    for category, records in grouped.items():
        write(f"instruments/sound-mechanism/{category}", records)
    for former in ("air-driven", "strings", "membranes"):
        old_payload = ROOT / "instruments/sound-mechanism" / former / "records.json"
        if old_payload.exists():
            old_payload.unlink()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cache", type=Path, default=ROOT.parent.parent / "target/imports")
    args = parser.parse_args()
    works(args.cache)
    instruments(args.cache)
