"""Import bounded, deduplicated object metadata from the pinned CMA CC0 catalog."""

import argparse
import collections
import csv
import hashlib
import html
import json
from pathlib import Path
import re
import shutil
import urllib.request

ROOT = Path(__file__).resolve().parent
REVISION = "d8dd744bc29e7986df428341133c14e18eb56b3f"
URL = f"https://media.githubusercontent.com/media/ClevelandMuseumArt/openaccess/{REVISION}/data.csv"
SHA256 = "137ddfab3264555d52c0211f804544cab1c62170ce3d753d924c9d464d74e449"
LIMIT = 300

# Reviewed exceptions in the pinned catalog: its broad type labels occasionally
# include objects whose titles/descriptions establish another form or poor data.
EXCLUDED_IDS = {
    "94167": "generic food-model aggregate; retain distinct named children",
    "94437": "generic animal aggregate; retain distinct named children",
    "126793": "modern replacement necklace beads",
    "95239": "creation date contains an internal editorial note",
    "96606": "watchcase alone is neither a complete timepiece nor jewelry",
    "96685": "ear-cleaner misclassified as jewelry; outside current object forms",
    "95890": "stone vessel misclassified as jewelry; outside current materials",
    "95892": "stone plate misclassified as jewelry; outside current materials",
    "152319": "bracelet terminal is a component, not a complete arm ornament",
    "156796": "writing-box base catalogued apart from its complete box",
}
AMBIGUOUS_JEWELRY_IDS = {
    "113907", "128181", "113886", "123779", "124743", "126800", "111696",
    "99393", "113909", "117824", "99430", "99431", "143177", "127480",
    "156850", "156852",
}
SOURCE_ROUTES = {
    "96580": "equipment/timepieces", "99264": "equipment/timepieces",
    "149537": "equipment/timepieces", "141163": "equipment/timepieces",
    "96615": "equipment/timepieces", "96579": "equipment/timepieces",
    "152840": "adornment/amulets", "96835": "adornment/amulets",
    "94783": "adornment/amulets", "94784": "adornment/amulets",
    "99654": "vessels/glass/storage", "94361": "adornment/jewelry/body/ears",
    "132313": "adornment/jewelry/beads", "147529": "adornment/jewelry/beads",
    "97497": "adornment/jewelry/neck/necklaces",
}
JEWELRY_FORMS = [
    ("sets", r"^Jewelry of|^Parure:|Necklace and Nose Ornament|Necklace.*Bracelet Set|^Rings and Bracelet$|^Bracelet and Anklet$|^Brooch and Bracelet Set$"),
    ("head/forehead-ornaments", r"Forehead Pendant"),
    ("head/hair-and-headdress", r"Hair Ornament|Hair Pin|Turban|Headdress|^Comb"),
    ("clothing/brooches", r"Brooch"),
    ("body/ears", r"Earring|Ear Ornament|Ear Stud|Ear Plug|Ear Flare|Pendant Earring"),
    ("body/nose", r"Nose Ornament|Nose Ring"),
    ("body/fingers", r"\bRing\b"),
    ("body/arms", r"Bracelet|Armlet"),
    ("body/feet", r"Anklet|Foot Ornament"),
    ("neck/necklaces", r"Necklace|Necklet|Chain and Pendant|Chain with.*Pendant|Chain.*Cross|Chain for Pendant|Pendant.*Chain|Gorget"),
    ("neck/pendants", r"Pendant|Locket|Medallion|Comma-shaped Jade"),
    ("clothing/fibulae", r"Fibula"),
    ("clothing/pins", r"Pin"),
    ("clothing/buckles", r"Buckle"),
    ("clothing/clasps", r"Clasp"),
    ("clothing/buttons", r"Button"),
    ("clothing/belts-and-chains", r"Belt|Girdle|Chain|Chatelaine"),
    ("beads", r"Bead"),
]


def clean(value):
    return re.sub(r"\s+", " ", html.unescape(re.sub(r"<[^>]*>", "", value))).strip()


def matches(title, pattern):
    return re.search(r"\b(?:" + pattern + r")\b", title, re.I) is not None


def classify(row):
    kind, title = row["type"], row["title"]
    if row["id"] in EXCLUDED_IDS:
        return None
    if row["id"] in SOURCE_ROUTES:
        return "artifacts/" + SOURCE_ROUTES[row["id"]]
    if kind == "Coins":
        if matches(title, r"electrotype|replica"):
            return None
        return "artifacts/currency/coins"
    if kind == "Basketry" and matches(title, r"basket|baskets|bowl|bowls|tray|trays|olla|jar"):
        return "artifacts/vessels/fiber/baskets"
    if kind in {"Ceramic", "Glass", "Jade", "Jewelry", "Lacquer", "Metalwork", "Miscellaneous", "Sculpture", "Stone", "Wood"} and matches(title, r"inkwell|inkstand|inkstone|pen and ink box|brush holder|brush rest|writing box"):
        return "artifacts/equipment/writing"
    if kind == "Amulets" and matches(title, r"inlay|inlays"):
        return None
    if kind == "Jewelry":
        if row["id"] in AMBIGUOUS_JEWELRY_IDS:
            return None
        for form, pattern in JEWELRY_FORMS:
            if re.search(pattern, title, re.I):
                return "artifacts/adornment/jewelry/" + form
        return None
    material = {"Ceramic": "ceramic", "Glass": "glass", "Metalwork": "metal", "Silver": "metal"}.get(kind)
    if material:
        forms = [
            ("drinking", r"cup|cups|goblet|goblets|beaker|beakers|tumbler|tumblers|chalice|teabowl"),
            ("pouring", r"ewer|ewers|jug|jugs|pitcher|pitchers|teapot|teapots|coffee\s*pot"),
            ("vases", r"vase|vases"),
            ("storage", r"jar|jars|bottle|bottles|flask|flasks|amphora|amphoriskos"),
            ("bowls", r"bowl|bowls"),
            ("plates", r"plate|plates|dish|dishes|platter|platters|saucer|saucers"),
        ]
        # Only populated, distinct form/material combinations are retained.
        for form, pattern in forms:
            if matches(title, pattern):
                if material == "glass" and form in {"plates", "bowls"}:
                    return None
                if material == "metal" and form in {"vases", "storage"}:
                    return None
                if material == "metal" and form in {"bowls", "plates"}:
                    form = "serving"
                return f"artifacts/vessels/{material}/{form}"
    if kind == "Furniture and woodwork":
        for form, pattern in [
            ("seating", r"chair|chairs|armchair|armchairs|stool|stools|bench|settee|headrest"),
            ("storage", r"chest|cassone|cabinet|armoire|cupboard|coffer|box|boxes"),
            ("tables", r"table|tables|desk|desks|lectern"),
        ]:
            if matches(title, pattern):
                return f"artifacts/furnishings/{form}"
    if kind == "Lamp" or (kind in {"Metalwork", "Silver", "Glass", "Furniture and woodwork"} and matches(title, r"lamp|lamps|candlestick|candlesticks|candelabrum|candelabra")):
        return "artifacts/furnishings/lighting"
    direct = {
        "Amulets": "adornment/amulets",
        "Netsuke": "adornment/netsuke", "Mask": "adornment/masks",
        "Garment": "textiles/garments", "Lace": "textiles/lace",
        "Embroidery": "textiles/embroidery", "Tapestry": "textiles/tapestries",
        "Carpet": "textiles/carpets", "Musical Instrument": "equipment/musical-instruments",
        "Timepiece": "equipment/timepieces", "Spindle Whorl": "equipment/spindle-whorls",
        "Funerary Equipment": "ritual/funerary", "Scarabs": "ritual/scarabs",
    }
    if kind in direct:
        if kind == "Scarabs" and not matches(title, r"scarab|scarabs"):
            return None
        return "artifacts/" + direct[kind]
    if kind == "Tools and Equipment" and matches(title, r"knife|knives|blade|blades|palette|float|weight|weights|adze|axe|axes|hammer|spatula|mortar|pestle|needle|needles|comb|combs|spoon|spoons|trowel|shuttle|spindle|drill"):
        return "artifacts/equipment/tools"
    if kind == "Arms and Armor":
        if matches(title, r"guard|tsuba|scabbard|sheath|hilt|mount|pommel|fitting|fittings"):
            return None  # Weapon components are not complete weapons.
        if matches(title, r"sword|swords|dagger|daggers|knife|knives|axe|axes|spear|spearhead|arrow|arrowhead|halberd|crossbow|bow|pistol|gun|musket|rapier|mace"):
            return "artifacts/arms/weapons"
        if matches(title, r"armor|armour|helmet|helm|breastplate|gauntlet|shield|cuirass|sallet|gorget|greave|pauldron|vambrace|sabaton|basinet|morion|cuisse|burgonet|mail|cap-a-pie"):
            return "artifacts/arms/armor"
    return None


def select(rows):
    """Prefer distinctive catalog facts and balance coarse catalog cultures."""
    groups = collections.defaultdict(lambda: collections.defaultdict(list))
    seen = set()
    candidates = []
    for row in sorted(rows, key=lambda item: int(item["id"])):
        row = {key: clean(row[key]) for key in ("id", "accession_number", "title", "type", "culture", "creation_date", "technique", "url")}
        if any(row[key] in {"[]", "None", "null", "?"} for key in ("culture", "creation_date", "technique")):
            continue
        if not all(row[key] for key in ("title", "culture", "creation_date", "technique")):
            continue
        if matches(row["culture"] + " " + row["title"], r"forgery|fake"):
            continue
        if matches(row["title"], r"fragment|fragments|stopper|lid|table top|one of|part [0-9]+|cover for|door to"):
            continue
        if re.search(r"\((?:cover|stand|base|table top)\)", row["title"], re.I):
            continue
        leaf = classify(row)
        if not leaf:
            continue
        candidates.append((leaf, row))
    accessions = {row["accession_number"] for _, row in candidates}
    for leaf, row in candidates:
        accession = row["accession_number"]
        # Both 1975.168.1 and 1916.105.a can denote individually catalogued
        # components. Never discard a child merely because an excluded aggregate
        # exists in the raw catalog: only eligible parents participate.
        bases = {
            re.sub(r"\.[a-z0-9]+(?:[-–].*)?$", "", accession),
            re.sub(r"[a-z](?:[-–].*)?$", "", accession),
        }
        if any(base != accession and base in accessions for base in bases):
            continue
        signature = (leaf, row["title"].casefold(), row["culture"].casefold(), row["creation_date"], row["technique"].casefold())
        if signature in seen:
            continue
        seen.add(signature)
        # Preserve uncertainty and dynasty/place detail verbatim, but balance using
        # the first catalog culture component instead of its many subcategories.
        culture = row["culture"].split(",")[0].casefold()
        groups[leaf][culture].append(row)
    selected = {}
    for leaf, cultures in sorted(groups.items()):
        queues = {key: collections.deque(value) for key, value in sorted(cultures.items())}
        result = []
        while queues and len(result) < LIMIT:
            for key in list(queues):
                result.append(queues[key].popleft())
                if not queues[key]:
                    del queues[key]
                if len(result) == LIMIT:
                    break
        selected[leaf] = result
    return selected


def record(row):
    text = (
        f"{row['title']} - {row['creation_date'].rstrip('.')}\n"
        f"Culture - {row['culture'].rstrip('.')}\n"
        f"Materials - {row['technique'].rstrip('.')}"
    )
    return {
        "id": "cma-" + row["id"],
        "text": text,
        "provenance": f"Derived structured metadata from the Cleveland Museum of Art CC0 Open Access catalog, revision {REVISION}; {row['url']}; input {URL}, SHA-256 {SHA256}. Catalog title, date, culture, and technique retained; no image or interpretive description reproduced. Function grouping is Inspire's editorial classification, not a new museum attribution.",
    }


def count_weights(node, check=False):
    """Keep museum objects equally likely despite their browsing depth."""
    corpus = node / "records.json"
    count = (len(json.loads(corpus.read_text(encoding="utf-8"))) if corpus.exists()
             else sum(count_weights(child, check) for child in node.iterdir() if child.is_dir()))
    if node != ROOT / "artifacts":
        path = node / "node.toml"
        expected = f"weight = {count}\n"
        if check:
            if not path.exists() or path.read_text(encoding="utf-8") != expected:
                raise SystemExit(f"Stale object weight: {path}")
        else:
            path.write_text(expected, encoding="utf-8")
    return count


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cache", type=Path, default=ROOT.parents[1] / "target/imports/cleveland")
    parser.add_argument("--check", action="store_true", help="Verify committed records without writing.")
    args = parser.parse_args()
    path = args.cache / "data.csv"
    if not path.exists():
        args.cache.mkdir(parents=True, exist_ok=True)
        urllib.request.urlretrieve(URL, path)
    if hashlib.sha256(path.read_bytes()).hexdigest() != SHA256:
        raise SystemExit("CMA input checksum mismatch")
    with path.open(encoding="utf-8-sig", newline="") as source:
        rows = list(csv.DictReader(source))
    selected = select(rows)
    expected_paths = {ROOT / leaf / "records.json" for leaf in selected}
    if args.check:
        unexpected = set((ROOT / "artifacts").rglob("records.json")) - expected_paths
        if unexpected:
            raise SystemExit(f"Unexpected object datasets: {sorted(unexpected)}")
        if not (ROOT / "artifacts/LICENSE").is_file():
            raise SystemExit("Missing CMA CC0 license")
    for leaf, items in selected.items():
        destination = ROOT / leaf / "records.json"
        rendered = json.dumps([record(item) for item in items], ensure_ascii=False, indent=2) + "\n"
        if args.check:
            if destination.read_text(encoding="utf-8") != rendered:
                raise SystemExit(f"Stale object records: {destination}")
        else:
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_text(rendered, encoding="utf-8")
        print(f"{leaf}: {len(items)}")
    count_weights(ROOT / "artifacts", args.check)
    if not args.check:
        license_path = args.cache / "LICENSE"
        if not license_path.exists():
            urllib.request.urlretrieve(f"https://raw.githubusercontent.com/ClevelandMuseumArt/openaccess/{REVISION}/LICENSE", license_path)
        shutil.copyfile(license_path, ROOT / "artifacts/LICENSE")
    print(f"Total: {sum(map(len, selected.values()))} records in {len(selected)} leaves")


if __name__ == "__main__":
    main()
