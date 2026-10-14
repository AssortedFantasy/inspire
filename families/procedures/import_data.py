"""Rebuild public-domain procedure selections; run with Python -B (beautifulsoup4)."""
import argparse
import hashlib
import json
import re
from pathlib import Path

from bs4 import BeautifulSoup, Tag

ROOT = Path(__file__).parent
SOURCES = {
    "farmer.html": ("https://www.gutenberg.org/cache/epub/65061/pg65061-images.html", "6d6beef18c37b384169c58276b425794cde2f97a11caf10b950d14f25aff0efe"),
    "higgin.html": ("https://www.gutenberg.org/cache/epub/24964/pg24964-images.html", "13f2589175b45d018d48fa89f85895980cbc8b28b9399b79be5621fe52aefc70"),
    "dillmont.html": ("https://www.gutenberg.org/cache/epub/20776/pg20776-images.html", "482c6206f57b8ad42bc7306433e2c1d1677dca89d21429130773d50ae10ac2d0"),
}


def clean(text):
    return re.sub(r"\s+", " ", text).strip()


def slug(text):
    return re.sub(r"[^a-z0-9]+", "-", text.lower()).strip("-")


def source(cache, name):
    raw = (cache / name).read_bytes()
    expected = SOURCES[name][1]
    actual = hashlib.sha256(raw).hexdigest()
    if actual != expected:
        raise ValueError(f"{name}: SHA256 {actual}, expected {expected}")
    soup = BeautifulSoup(raw.decode("utf-8"), "html.parser")
    for item in soup.select(".pageno,.pagenum,.caption,figure,.figcenter"):
        item.decompose()
    return soup


def farmer_sections(soup):
    chapter = ""
    for heading in soup.find_all(["h2", "h3", "h4"]):
        title = clean(heading.get_text(" ", strip=True))
        if heading.name == "h2":
            chapter = title
            continue
        blocks = []
        for node in heading.next_elements:
            if isinstance(node, Tag) and node.name in ("h2", "h3", "h4"):
                break
            if not isinstance(node, Tag):
                continue
            # Ingredient quantities use div.line, not tables. Preserve each line.
            if node.name == "p" or (node.name == "div" and "line" in node.get("class", [])):
                if not node.find_parent("p"):
                    text = clean(node.get_text(" ", strip=True))
                    if text:
                        blocks.append(text)
        yield chapter, title, blocks


# Explicit selection avoids historical preservation, medical, raw-egg, foraging,
# and incomplete recipes. Full ingredient lines and methods are retained.
RECIPES = {
    "beverages/tea": ["How to Make Tea", "Iced Tea"],
    "beverages/cocoa": ["Cocoa Shells", "Cracked Cocoa", "Breakfast Cocoa", "Chocolate I"],
    "beverages/fruit": ["Lemonade", "Pineapple Lemonade", "Fruit Punch I", "Fruit Punch II", "Fruit Punch III", "Fruit Punch IV"],
    "cereals": ["Boiled Rice", "Steamed Rice", "Boiled Macaroni", "Macaroni with White Sauce"],
    "vegetables/boiling": ["Boiled Artichokes", "Boiled Asparagus", "String Beans", "Shell Beans", "Boiled Beets", "Boiled Cabbage", "Boiled Green Corn", "Boiled Cucumbers", "Boiled Onions", "Boiled Peas", "Boiled Spinach", "Boiled Summer Squash", "Mashed Turnip"],
    "vegetables/steaming": ["Steamed Winter Squash"],
    "vegetables/baking": ["Baked Chestnuts", "Glazed Onions", "Baked Winter Squash I", "Baked Winter Squash II", "Baked Tomatoes I"],
    "vegetables/stewing": ["German Cabbage", "Spinach (French Style)", "Stewed Tomatoes"],
    "potatoes/boiling-and-mashing": ["Riced Potatoes", "Mashed Potatoes", "Persillade Potatoes", "Boiled Sweet Potatoes", "Mashed Sweet Potatoes"],
    "potatoes/baking": ["Chambery Potatoes", "Anna Potatoes", "Glazed Sweet Potatoes", "Sweet Potatoes, Georgian Style", "Sweet Potatoes en Brochette", "Sweet Potatoes au Gratin"],
    "potatoes/pan-cooking": ["Potato Omelet", "Potato Cakes", "Sautéd Potatoes", "Lyonnaise Potatoes I", "Lyonnaise Potatoes II"],
    "fruit/baking": ["Baked Apples", "Baked Sweet Apples", "Baked Bananas I", "Baked Bananas II"],
    "fruit/stewing": ["Apple Sauce", "Spiced Apple Sauce", "Cranberry Sauce", "Stewed Prunes", "Rhubarb Sauce"],
    "sauces/thickened": ["Thin White Sauce", "White Sauce I", "White Sauce II", "Thick White Sauce (for Cutlets and Croquets)", "Drawn Butter Sauce", "Tomato Sauce I (without Stock)", "Lemon Sauce II"],
    "sauces/syrups": ["Lemon Sauce I", "Molasses Sauce", "Mint Sauce"],
    "sauces/butters": ["Maître d’Hôtel Butter", "Lemon Butter", "Hard Sauce", "Sterling Sauce"],
    "sauces/cold": ["Cucumber Sauce I", "Cucumber Sauce II", "Vinaigrette Sauce", "Apricot Sauce"],
}


def imported(cache):
    output = {}
    soup = source(cache, "farmer.html")
    available = {}
    for chapter, title, blocks in farmer_sections(soup):
        if chapter.startswith("CHAPTER") and blocks:
            available.setdefault(title, []).append((chapter, blocks))
    missing = []
    for category, titles in RECIPES.items():
        for title in titles:
            choices = available.get(title, [])
            if title == "Maître d’Hôtel Butter":
                choices = [choice for choice in choices if choice[0].startswith("CHAPTER XVIII ")]
            if len(choices) != 1:
                missing.append((title, len(choices)))
                continue
            chapter, blocks = choices[0]
            blocks = [block for block in blocks if block != "French Chef"]
            if title == "Stewed Tomatoes":
                blocks = blocks[:1]  # Next recipe uses an inline rather than h3 heading.
            body = "\n".join(blocks)
            if title == "How to Make Tea":
                body = body.split(" Avoid second steeping")[0]
            if title == "Lemonade":
                body = body.split(" Lemon syrup may be bottled")[0]
            # Referenced foundation is appended rather than leaving an opaque step.
            if "Make same as Thin White Sauce" in body:
                body += "\nFoundation method (Thin White Sauce): " + " ".join(available["Thin White Sauce"][0][1][5:])
            if title == "Macaroni with White Sauce":
                body += "\nFoundation method (Boiled Macaroni): " + " ".join(available["Boiled Macaroni"][0][1][4:])
            if title in ("Potato Omelet", "Mashed Potatoes"):
                body += "\nFoundation method (Riced Potatoes): " + " ".join(available["Riced Potatoes"][0][1])
            if title == "Potato Omelet":
                body += "\nFoundation method (Mashed Potatoes): " + " ".join(available["Mashed Potatoes"][0][1])
            if title == "Iced Tea":
                foundation = " ".join(available["How to Make Tea"][0][1][2:]).split(" Avoid second steeping")[0]
                body += "\nFoundation method (tea infusion, using quantities above): " + foundation
            text = f"Historical recipe: {title}\nFannie Merritt Farmer, revised 1910 edition; {chapter}.\n{body}"
            row = {"id": "farmer-1910-" + slug(chapter.split()[1]) + "-" + slug(title), "text": text,
                   "provenance": f"Downloaded public-domain primary text, Fannie Merritt Farmer, The Boston Cooking-School Cook Book (1910), {chapter}, heading {title!r}. {SOURCES['farmer.html'][0]}; SHA256 {SOURCES['farmer.html'][1]}. HTML stripped, whitespace normalized, page numbers and image captions omitted; selected foundation methods appended verbatim. Tea's concluding medical claim and lemonade's storage sentence omitted."}
            output.setdefault("cooking/historical-recipes/" + category, []).append(row)
    if missing:
        raise ValueError(f"Missing or ambiguous selected headings: {missing}")
    output.update(embroidery(cache))
    return output


def embroidery(cache):
    soup = source(cache, "higgin.html")
    paragraphs = [clean(p.get_text(" ", strip=True)) for p in soup.select("p")]
    # Explicit paragraph selections retain textual operations; image-dependent
    # diagrams, obsolete cleaning chemistry and unsupported natural-history
    # explanations are not imported.
    selectors = {
        "lines": [("stem-stitch", "It may be best described as a long stitch forward", 1), ("split-stitch", "Split Stitch is worked like ordinary", 1), ("japanese-stitch", "Japanese Stitch is a modification", 1)],
        "loops-and-knots": [("french-knot", "Knotted Stitch , or French Knot", 1), ("chain-stitch", "Chain Stitch is but little used", 1), ("tambour", "Tambour Work has fallen into disuse", 1)],
        "surface-filling": [("satin-stitch", "Satin Stitch — French Plumetis", 1)],
        "couching": [("plain-couching", "Plain Couching , or", 1), ("net-couching", "Net-patterned Couching.", 1), ("brick-couching", "Brick Stitch.", 1), ("basket-couching", "Basket Stitch is one", 1)],
        "counted-canvas": [("tent-stitch", "Tent Stitch may be placed first", 1), ("cross-stitch", "Simple Cross Stitch.", 1), ("persian-cross", "Persian Cross Stitch.", 1), ("cushion-stitches", "Cushion Stitches are taken", 1)],
    }
    output = {}
    stem = next(p for p in paragraphs if p.startswith("It may be best described as a long stitch forward"))
    tent = next(p for p in paragraphs if p.startswith("Tent Stitch may be placed first"))
    for group, choices in selectors.items():
        for identity, start, count in choices:
            matches = [i for i, p in enumerate(paragraphs) if p.startswith(start)]
            if len(matches) != 1:
                raise ValueError(f"Ambiguous embroidery paragraph: {start}")
            index = matches[0]
            body = "\n".join(paragraphs[index:index + count])
            if identity in ("split-stitch", "japanese-stitch"):
                body += "\nFoundation (stem stitch): " + stem
            if identity == "cross-stitch":
                body += "\nFoundation (tent stitch): " + tent
            text = f"Historical embroidery procedure: {identity.replace('-', ' ').capitalize()}\nL. Higgin, Handbook of Embroidery (1880 edition), hand and frame stitches. Period terminology.\n{body}"
            output.setdefault("craft/needlework/embroidery/" + group, []).append({"id": "higgin-1880-" + identity, "text": text, "provenance": f"Downloaded public-domain primary text, L. Higgin, Handbook of Embroidery, 1880 edition, paragraph beginning {start!r}; {SOURCES['higgin.html'][0]}; SHA256 {SOURCES['higgin.html'][1]}. Textual instructions selected, HTML and page numbers stripped; referenced foundation instructions appended where needed. No illustrations or cleaning treatments reproduced."})
    return output


def baseline():
    output = {}
    graph_groups = {
        "depth-first-reachability": "traversal/depth-first", "breadth-first-distance": "traversal/breadth-first", "connected-components": "connectivity/undirected-components", "directed-cycle": "ordering/cycle-detection", "topological-order": "ordering/topological", "strong-components": "connectivity/strong-components", "prim-tree": "spanning-trees/prim", "kruskal-tree": "spanning-trees/kruskal", "spanning-forest": "spanning-trees/forests", "dijkstra-paths": "shortest-paths/dijkstra", "dag-shortest-paths": "shortest-paths/dag", "bellman-ford-paths": "shortest-paths/bellman-ford",
    }
    sewing_groups = {"running-stitch": "stitches", "continuous-backstitch": "stitches", "double-turned-hem": "edges", "flat-felled-seam": "seams", "rolled-seam": "seams", "selvedge-oversewing": "seams", "french-seam": "seams", "gathering-and-band": "gathering", "second-gathering-row": "gathering", "whipped-gathering": "gathering", "thread-splice-without-knot": "thread-joins", "tacked-stitching-guide": "marking"}
    bench_groups = {"paper-bridge-shapes": "mechanics/bridges", "paper-bridge-span": "mechanics/bridges", "marker-chromatography": "separation/chromatography", "radial-ink-separation": "separation/chromatography", "color-afterimage": "vision/afterimages", "paper-rotor": "mechanics/rotors", "rotor-direction": "mechanics/rotors", "paper-drag-comparison": "mechanics/drag", "condiment-diver": "fluids/buoyancy", "moon-phase-model": "optics/illumination", "floating-clip-soap": "fluids/surface-tension", "blind-spot-card": "vision/blind-spot"}
    kitchen_groups = {"level-flour-measure": "methods/measuring", "folding-airy-mixture": "methods/mixing", "creaming-butter": "methods/mixing", "white-sauce": "adapted-recipes/sauces", "parsley-lemon-butter": "adapted-recipes/sauces", "drained-rice": "adapted-recipes/cereals", "baked-spiced-apples": "adapted-recipes/baking", "buttered-croutons": "adapted-recipes/baking", "cheese-crackers": "adapted-recipes/baking", "cauliflower-crumb-topping": "adapted-recipes/vegetables", "lemon-parsley-potatoes": "adapted-recipes/vegetables", "apple-bread-pudding": "adapted-recipes/baking"}
    for item in json.loads((ROOT / "baseline_records.json").read_text(encoding="utf-8")):
        row, old = item["record"], item["path"]
        if old == "algorithms/sorting":
            identity = row["id"]
            if identity.startswith("max-heap"):
                path = "algorithms/sequences/priority-queues/max-heap/methods"
            elif identity == "quickselect":
                path = "algorithms/sequences/selection/quickselect/method"
            elif identity in ("top-down-mergesort", "bottom-up-mergesort"):
                path = "algorithms/sequences/sorting/merge-sort/methods"
            elif identity in ("randomized-quicksort", "three-way-partition"):
                path = "algorithms/sequences/sorting/quicksort/methods"
            else:
                path = "algorithms/sequences/sorting/" + identity + "/method"
        elif old == "algorithms/graphs":
            path = "algorithms/graphs/" + graph_groups[row["id"]] + "/method"
        elif old == "craft/sewing":
            path = "craft/needlework/sewing/" + sewing_groups[row["id"]]
        elif old == "experiments/bench":
            path = "experiments/" + bench_groups[row["id"]]
        else:
            path = "cooking/" + kitchen_groups[row["id"]]
        output.setdefault(path, []).append(row)
    return output


def main():
    parser = argparse.ArgumentParser(__doc__)
    parser.add_argument("--source-dir", type=Path, default=Path("target/imports/procedures"))
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    output = baseline()
    for path, rows in imported(args.source_dir).items():
        output.setdefault(path, []).extend(rows)
    for path, rows in output.items():
        payload = json.dumps(rows, ensure_ascii=False, indent=2) + "\n"
        destination = ROOT / path / "records.json"
        if args.check:
            if destination.read_text(encoding="utf-8") != payload:
                raise ValueError(f"Replay differs: {path}")
        else:
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_text(payload, encoding="utf-8")
    print(f"{'Verified' if args.check else 'Wrote'} {sum(map(len, output.values()))} records in {len(output)} corpus leaves")


if __name__ == "__main__":
    main()
