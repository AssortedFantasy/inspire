"""Rebuild food and materials corpora from checksum-pinned public datasets."""

import argparse
from collections import Counter, defaultdict
import csv
from decimal import Decimal
import hashlib
import io
import json
from pathlib import Path
import re
import unicodedata
import urllib.request
import zipfile


ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / "scripts/catalogue_sources.json"
REVISION = "2e7adec11a0561696c236ae1fca82e88be173d42"
CORPORA = f"https://raw.githubusercontent.com/dariusk/corpora/{REVISION}/"
FOOD_LISTS = [
    ("herbs_n_spices.json", "herbs", "herbs"),
    ("herbs_n_spices.json", "spices", "spices"),
    ("herbs_n_spices.json", "mixtures", "seasoning-mixtures"),
    ("fruits.json", "fruits", "fruits"),
    ("vegetables.json", "vegetables", "vegetables"),
    ("breads_and_pastries.json", "breads", "breads"),
    ("breads_and_pastries.json", "pastries", "pastries"),
    ("condiments.json", "condiments", "condiments"),
    ("apple_cultivars.json", "cultivars", "apple-cultivars"),
    ("iba_cocktails.json", "cocktails", "cocktails"),
]
MATERIAL_LISTS = [
    ("building-materials.json", "building materials", "building-materials"),
    ("decorative-stones.json", "decorative stones", "decorative-stones"),
    ("fabrics.json", "fabrics", "textiles"),
    ("fibers.json", "fibers", "fibres"),
    ("metals.json", "metals", "metals"),
    ("packaging.json", "packaging", "packaging"),
    ("technical-fabrics.json", "technical fabrics", "technical-textiles"),
]
# Source lists mix material names with processes, places and incomplete adjectives.
# Zorbeez names a finished cleaning-product line; see the manufacturer's filing
# linked in materials/README.md. Retain material trade names in general.
EXCLUDED = {"carding", "desizing", "lenzing, grimsby", "conductive",
            "modified atmosphere", "cushioning", "natural fiber", "hemp", "zorbeez"}
NUTRIENTS = [("1008", "Energy", "kcal"), ("1003", "Protein", "g"),
             ("1004", "Fat", "g"), ("1005", "Carbohydrate", "g"),
             ("1079", "Fibre", "g"), ("1093", "Sodium", "mg")]


def clean(text):
    return " ".join(text.split())


def key(text):
    return clean(unicodedata.normalize("NFKC", text)).casefold()


def slug(text):
    return re.sub(r"[^a-z0-9]+", "-", text.lower()).strip("-")


def load(entry, download):
    path = ROOT / "target/imports" / entry["cache"]
    if not path.exists() and download:
        path.parent.mkdir(parents=True, exist_ok=True)
        with urllib.request.urlopen(entry["url"], timeout=120) as response:
            path.write_bytes(response.read())
    data = path.read_bytes()
    if hashlib.sha256(data).hexdigest() != entry["sha256"]:
        raise ValueError(f"Source checksum mismatch: {path}")
    return data


def lists(family, entries, source_dir, manifest, download):
    files, seen = {}, set()
    for filename, field, leaf in entries:
        source_path = f"data/{source_dir}/{filename}"
        source = json.loads(load(manifest[source_path], download))
        records = []
        for index, original in enumerate(source[field]):
            text = clean(original)
            if family == "materials" and key(text) in EXCLUDED:
                continue
            if leaf == "apple-cultivars":
                text += " apple"
            normalized = key(text)
            if not text or normalized in seen:
                continue
            seen.add(normalized)
            ident = hashlib.sha256(normalized.encode()).hexdigest()[:16]
            records.append({
                "id": f"corpora-{family}-{ident}", "text": text,
                "provenance": f"Darius Kazemi and Corpora contributors, revision {REVISION}; "
                              f"{CORPORA}{source_path}; field {field}, index {index}. "
                              "CC0 1.0. Whitespace normalized and duplicate labels routed to the "
                              "first listed topic; apple cultivar names gain the noun apple. "
                              "Vocabulary labels only; source scope and exclusions in family README.",
            })
        if records:
            files[ROOT / "families" / family / "catalogues" / leaf / "records.json"] = records
    return files


def food_profiles(manifest, download):
    with zipfile.ZipFile(io.BytesIO(load(manifest["usda-sr-legacy"], download))) as archive:
        def rows(filename):
            matches = [name for name in archive.namelist() if name.endswith("/" + filename)]
            if len(matches) != 1:
                raise ValueError(f"Missing or ambiguous source table: {filename}")
            return csv.DictReader(io.StringIO(archive.read(matches[0]).decode("utf-8-sig")))

        categories = {row["id"]: row["description"] for row in rows("food_category.csv")}
        units = {row["id"]: row["unit_name"].lower() for row in rows("nutrient.csv")}
        for nutrient, _, unit in NUTRIENTS:
            if units[nutrient] != unit:
                raise ValueError(f"Unexpected unit for nutrient {nutrient}")
        wanted = {entry[0] for entry in NUTRIENTS}
        amounts = defaultdict(dict)
        for row in rows("food_nutrient.csv"):
            nutrient, food = row["nutrient_id"], row["fdc_id"]
            if nutrient in wanted and row["amount"]:
                if nutrient in amounts[food]:
                    raise ValueError(f"Repeated nutrient {nutrient} for {food}")
                amount = Decimal(row["amount"])
                if amount < 0 or not amount.is_finite():
                    raise ValueError(f"Invalid nutrient amount: {row}")
                formatted = format(amount, "f")
                if "." in formatted:
                    formatted = formatted.rstrip("0").rstrip(".")
                amounts[food][nutrient] = formatted
        grouped = defaultdict(list)
        seen = set()
        for row in rows("food.csv"):
            food = row["fdc_id"]
            if food in seen:
                raise ValueError(f"Repeated food ID: {food}")
            seen.add(food)
            if not {"1008", "1003", "1004", "1005"}.issubset(amounts[food]):
                continue
            name = clean(row["description"])
            category = categories[row["food_category_id"]]
            lines = [name, "Per 100 g edible portion"]
            lines += [f"{label} {amounts[food][nutrient]} {unit}"
                      for nutrient, label, unit in NUTRIENTS if nutrient in amounts[food]]
            grouped[slug(category)].append({
                "id": f"usda-fdc-{food}", "text": "\n".join(lines),
                "provenance": f"USDA FoodData Central SR Legacy (April 2018), FDC ID {food}; "
                              f"https://fdc.nal.usda.gov/food-details/{food}/nutrients; "
                              f"{manifest['usda-sr-legacy']['url']}; SHA-256 "
                              f"{manifest['usda-sr-legacy']['sha256']}. "
                              f"Source food category: {category}. Public-domain USDA data. "
                              "Food descriptions and nutrient units retained; numeric trailing zeros removed. "
                              "Nutrient IDs 1008/1003/1004/1005/1079/1093, amounts per 100 g edible portion; "
                              "missing optional fibre or sodium omitted, never treated as zero.",
            })
        return {ROOT / "families/food/nutrition" / category / "records.json": records
                for category, records in sorted(grouped.items())}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--family", choices=["food", "materials"])
    parser.add_argument("--download", action="store_true")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    load(manifest["README.md"], args.download)
    files = {}
    if args.family in (None, "food"):
        files.update(lists("food", FOOD_LISTS, "foods", manifest, args.download))
        files.update(food_profiles(manifest, args.download))
    if args.family in (None, "materials"):
        files.update(lists("materials", MATERIAL_LISTS, "materials", manifest, args.download))
    outputs, counts = {}, Counter()
    for path, records in files.items():
        if len({r["id"] for r in records}) != len(records):
            raise ValueError(f"Repeated record IDs: {path}")
        outputs[path] = json.dumps(records, ensure_ascii=False, indent=2) + "\n"
        weight = min(len(records), 100) if path.parent.name == "apple-cultivars" else len(records)
        outputs[path.with_name("node.toml")] = f"weight = {weight}\n"
        counts[path.relative_to(ROOT / "families").parts[0]] += len(records)
    for path, text in outputs.items():
        if args.check:
            if not path.exists() or path.read_text(encoding="utf-8") != text:
                raise ValueError(f"Import differs: {path}")
        else:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text, encoding="utf-8", newline="\n")
    print(json.dumps(dict(counts), sort_keys=True))


if __name__ == "__main__":
    main()
