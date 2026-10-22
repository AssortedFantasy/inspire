"""Rebuild public-domain procedure selections; run with Python -B (beautifulsoup4)."""
import argparse
import hashlib
import json
import re
from collections import Counter
from pathlib import Path

from bs4 import BeautifulSoup, Tag

ROOT = Path(__file__).parent
SOURCES = {
    "farmer.html": ("https://www.gutenberg.org/cache/epub/65061/pg65061-images.html", "6d6beef18c37b384169c58276b425794cde2f97a11caf10b950d14f25aff0efe"),
    "higgin.html": ("https://www.gutenberg.org/cache/epub/24964/pg24964-images.html", "13f2589175b45d018d48fa89f85895980cbc8b28b9399b79be5621fe52aefc70"),
    "dillmont.html": ("https://www.gutenberg.org/cache/epub/20776/pg20776-images.html", "482c6206f57b8ad42bc7306433e2c1d1677dca89d21429130773d50ae10ac2d0"),
    "tissandier.html": ("https://www.gutenberg.org/cache/epub/52709/pg52709-images.html", "13b2844007872c5227dec2d1d1098390bc4d4ad80f987772009356233a918d39"),
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
        title = clean(heading.get_text())
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
                    text = clean(node.get_text())
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
            text = f"{title}\n{body}\n\nFannie Merritt Farmer, The Boston Cooking-School Cook Book (1910)."
            row = {"id": "farmer-1910-" + slug(chapter.split()[1]) + "-" + slug(title), "text": text,
                   "provenance": f"Downloaded public-domain primary text, Fannie Merritt Farmer, The Boston Cooking-School Cook Book (1910), {chapter}, heading {title!r}. {SOURCES['farmer.html'][0]}; SHA256 {SOURCES['farmer.html'][1]}. HTML stripped, whitespace normalized, page numbers and image captions omitted; selected foundation methods appended verbatim. Tea's concluding medical claim and lemonade's storage sentence omitted."}
            output.setdefault("cooking/historical-recipes/" + category, []).append(row)
    if missing:
        raise ValueError(f"Missing or ambiguous selected headings: {missing}")
    output.update(embroidery(cache))
    output.update(sewing(cache))
    output.update(kitchen_methods(soup))
    output.update(experiments(cache))
    return output


def embroidery(cache):
    soup = source(cache, "higgin.html")
    paragraphs = [clean(p.get_text()) for p in soup.select("p")]
    # Explicit paragraph selections retain textual operations; image-dependent
    # diagrams, obsolete cleaning chemistry and unsupported natural-history
    # explanations are not imported.
    selectors = {
        "lines": [("stem-stitch", "It may be best described as a long stitch forward", 1), ("split-stitch", "Split Stitch is worked like ordinary", 1), ("japanese-stitch", "Japanese Stitch is a modification", 1)],
        "loops-and-knots": [("french-knot", "Knotted Stitch, or French Knot", 1), ("chain-stitch", "Chain Stitch is but little used", 1), ("tambour", "Tambour Work has fallen into disuse", 1)],
        "surface-filling": [("satin-stitch", "Satin Stitch—French Plumetis", 1)],
        "couching": [("plain-couching", "Plain Couching, or", 1), ("net-couching", "Net-patterned Couching.", 1), ("brick-couching", "Brick Stitch.", 1), ("basket-couching", "Basket Stitch is one", 1)],
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
            if identity == "stem-stitch":
                body = "Stem stitch\n" + body
            text = f"{body}\n\nL. Higgin, Handbook of Embroidery (1880)."
            output.setdefault("craft/needlework/embroidery/" + group, []).append({"id": "higgin-1880-" + identity, "text": text, "provenance": f"Downloaded public-domain primary text, L. Higgin, Handbook of Embroidery, 1880 edition, paragraph beginning {start!r}; {SOURCES['higgin.html'][0]}; SHA256 {SOURCES['higgin.html'][1]}. Textual instructions selected, HTML and page numbers stripped; referenced foundation instructions appended where needed. No illustrations or cleaning treatments reproduced."})
    return output


def paragraphs(soup):
    # Respect spacing in the source instead of inserting spaces at inline tags
    # (which separates punctuation and can split a word across a page marker).
    return [clean(p.get_text()) for p in soup.select("p")]


def selected(paragraphs, start, count=1, stop=None):
    matches = [i for i, p in enumerate(paragraphs) if p.startswith(start)]
    if len(matches) != 1:
        raise ValueError(f"Missing or ambiguous source paragraph: {start!r}")
    index = matches[0]
    body = "\n".join(paragraphs[index:index + count])
    if stop:
        if stop not in body:
            raise ValueError(f"Missing excerpt boundary: {stop!r}")
        body = body.split(stop)[0].rstrip()
    return body


def without_figure_navigation(body):
    body = re.sub(r" \(figs?\. [\d ,and]+\)", "", body)
    for reference in (" O, as in the illustration", ", as in the illustration", ", as per illustration", ", as shewn in fig. 39", ", as in fig. 32", " as in fig. 39"):
        body = body.replace(reference, "")
    return body


def sewing(cache):
    parts = paragraphs(source(cache, "dillmont.html"))
    # These selections are intelligible without their illustrations. Keep the
    # author's words, including historical measurements and terminology.
    choices = {
        "stitches": [
            ("running-stitch", "(1) Running-stitch", 1),
            ("back-stitch", "(2) Back-stitch", 1),
            ("continuous-stitching", "Stitching (fig. 6", 1),
        ],
        "edges": [("hemming", "(3) Hemming-stitch", 2), ("herring-boning", "Herring-boning (fig. 39", 1)],
        "seams": [
            ("flat-seam", "Flat seam (fig. 9", 2),
            ("selvedge-oversewing", "(4) Top or over-sewing stitch", 1),
            ("french-double-seam", "French double seam", 2),
            ("piping", "Sewing on piping", 1),
        ],
        "gathering": [("gathering", "Gathering (fig. 18", 5), ("whipping", "Whipping (fig. 22", 1)],
        "thread-joins": [("fastening-threads", "Fastening threads off, and on", 1), ("knotting-thread", "Knotting the thread into the needle", 1)],
        "fastenings": [("button-holes", "Button-holes in linen", 1), ("sewing-buttons", "Sewing on buttons", 1), ("binding-slits", "Binding slits", 1)],
        "mending": [
            ("linen-darning", "(1) Linen Darning", 2),
            ("satin-darning", "Begin, as in ordinary darning", 2),
            ("patching", "Patching.", 1),
            ("back-stitched-patch", "Back-stitching and felling in a patch", 1),
            ("top-sewn-patch", "Top-sewing in a patch", 1),
        ],
    }
    output = {}
    for group, selections in choices.items():
        for identity, start, count in selections:
            body = selected(parts, start, count)
            # Figure references are editorial navigation, not the operation.
            body = without_figure_navigation(body)
            if identity == "linen-darning":
                body = "Linen Darning\n" + body[body.index("The longitudinal running"):].split(" The threads are only drawn")[0]
            if identity == "satin-darning":
                body = "Satin or twill darning\n" + body
            row = {"id": "dillmont-" + identity,
                   "text": body + "\n\nThérèse de Dillmont, Encyclopedia of Needlework.",
                   "provenance": f"Verbatim public-domain text, Thérèse de Dillmont, Encyclopedia of Needlework, paragraph beginning {start!r}, {count} consecutive paragraphs. {SOURCES['dillmont.html'][0]}; SHA256 {SOURCES['dillmont.html'][1]}. HTML and page numbers stripped, whitespace normalized, parenthetical and inline figure-navigation references omitted. Linen darning omits its initial figure-dependent exception and closing illustration sentence. No illustrations reproduced."}
            output.setdefault("craft/needlework/sewing/" + group, []).append(row)
    return output


def kitchen_methods(soup):
    parts = paragraphs(soup)
    choices = {
        "measuring": [("dry-ingredients", "Measuring Ingredients."), ("spoonfuls", "To measure tea or table spoonfuls"), ("solid-fats", "Measuring Butter, Lard, etc.")],
        "mixing": [("stirring", "To stir,"), ("beating", "To beat,"), ("cutting-and-folding", "To cut and fold,"), ("creaming-butter", "To Cream Butter.")],
    }
    output = {}
    for group, selections in choices.items():
        for identity, start in selections:
            body = selected(parts, start)
            row = {"id": "farmer-1910-method-" + identity,
                   "text": body + "\n\nFannie Merritt Farmer, The Boston Cooking-School Cook Book (1910).",
                   "provenance": f"Verbatim public-domain text, Fannie Merritt Farmer, The Boston Cooking-School Cook Book (1910), paragraph beginning {start!r}. {SOURCES['farmer.html'][0]}; SHA256 {SOURCES['farmer.html'][1]}. HTML stripped and whitespace normalized."}
            output.setdefault("cooking/methods/" + group, []).append(row)
    return output


def experiments(cache):
    parts = paragraphs(source(cache, "tissandier.html"))
    # A few excerpts stop at a sentence boundary before unrelated apparatus,
    # figure-dependent discussion or an obsolete explanation begins.
    choices = {
        "mechanics/inertia": [
            ("coin-collision", "We can communicate motion by elasticity.", None),
            ("counter-stack", "It is possible, for the same reason, to extract", None),
            ("coin-on-paper", "We may give another experiment concerning Inertia.", None),
        ],
        "fluids/pressure": [
            ("egg-in-glass", "The force of air can very soon be shown", None),
            ("leather-sucker", "An amusing toy, well-known to schoolboys", None),
            ("connected-funnels", "If it is desired to investigate the principles relating to connected vessels", None),
            ("upward-pressure", "A disc of cardboard and a lamp-glass", " The outer pressure"),
        ],
        "fluids/buoyancy": [("ludion", "A pretty little toy may be made", " The reason of the movement")],
        "fluids/surface-tension": [
            ("pennies-in-glass", "Fig. 65 represents a pretty experiment", None),
            ("cork-balls", "Amongst the complicated experiments to demonstrate molecular attraction", " But if the balls"),
        ],
        "fluids/capillarity": [("cloth-syphon", "Apropos of the syphon, we may mention", None)],
        "vision/focus": [("two-pins", "This faculty of accommodation in the eye", " It is only since")],
        "vision/binocular": [("penholder-and-ink-bottle", "If we hold up a penholder", None)],
        "vision/afterimages": [("accidental-colours", "Accidental colours, or ocular spectra", None)],
        "optics/refraction": [("hidden-coin", "One may also place a piece of money", " The glass lenses")],
        "optics/soap-bubbles": [("bubble-colours", "Look at a soap-bubble blown", " The study of soap-bubbles")],
    }
    output = {}
    for group, selections in choices.items():
        for identity, start, stop in selections:
            # Some self-contained experiments begin within a longer paragraph.
            matches = [p for p in parts if start in p]
            if len(matches) != 1:
                raise ValueError(f"Missing or ambiguous experiment: {start!r}")
            body = matches[0][matches[0].index(start):]
            if stop:
                if stop not in body:
                    raise ValueError(f"Missing experiment boundary: {stop!r}")
                body = body.split(stop)[0]
            body = without_figure_navigation(body)
            if identity == "pennies-in-glass":
                body = body[body.index("I take a glass"):]
            row = {"id": "tissandier-" + identity,
                   "text": body + "\n\nGaston Tissandier, Popular Scientific Recreations, English edition.",
                   "provenance": f"Verbatim public-domain excerpt, Gaston Tissandier, Popular Scientific Recreations, English translation and enlargement, paragraph containing {start!r}. {SOURCES['tissandier.html'][0]}; SHA256 {SOURCES['tissandier.html'][1]}. HTML and page numbers stripped, whitespace normalized, parenthetical and inline figure-navigation references omitted. Excerpt boundaries retain complete sentences. No illustrations reproduced."}
            output.setdefault("experiments/" + group, []).append(row)
    return output


def corpus_weights(output):
    weights = Counter()
    for path, rows in output.items():
        parts = path.split("/")
        if parts[0] not in {"cooking", "craft", "experiments"}:
            continue
        for length in range(2, len(parts) + 1):
            weights["/".join(parts[:length])] += len(rows)
    return weights


def main():
    parser = argparse.ArgumentParser(__doc__)
    parser.add_argument("--source-dir", type=Path, default=Path("target/imports/procedures"))
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    output = imported(args.source_dir)
    for path, rows in output.items():
        payload = json.dumps(rows, ensure_ascii=False, indent=2) + "\n"
        destination = ROOT / path / "records.json"
        if args.check:
            if destination.read_text(encoding="utf-8") != payload:
                raise ValueError(f"Replay differs: {path}")
        else:
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_text(payload, encoding="utf-8", newline="\n")
    for path, weight in corpus_weights(output).items():
        destination = ROOT / path / "node.toml"
        payload = f"weight = {weight}\n"
        if args.check:
            if destination.read_text(encoding="utf-8") != payload:
                raise ValueError(f"Weight replay differs: {path}")
        else:
            destination.write_text(payload, encoding="utf-8", newline="\n")
    print(f"{'Verified' if args.check else 'Wrote'} {sum(map(len, output.values()))} records in {len(output)} corpus leaves")


if __name__ == "__main__":
    main()
