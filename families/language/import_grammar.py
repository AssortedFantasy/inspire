"""Import attributed usage quotations from Baskervill and Sewell (1895).

Run from the repository root with python -B families/language/import_grammar.py.
The cached HTML is never embedded. The standard-library parser preserves wording
while dropping typographic emphasis and the textbook's exercise numbering.
"""

import argparse
from dataclasses import dataclass, field
import hashlib
from html.parser import HTMLParser
import json
from pathlib import Path
import re
import urllib.request

ROOT = Path(__file__).resolve().parent
URL = "https://www.gutenberg.org/cache/epub/14006/pg14006-images.html"
CACHE = Path("target/imports/language/grammar-14006.html")
SHA256 = "700d383181edbad977023c138d41227279192dd368fcd171d783799e582c3577"
INCOMPLETE = re.compile(r"\.\.\.|…|\*|\[|\]|\betc\.|\b(?:negro\w*|savage\w*|idiot\w*|mowing|Hottentot\w*)\b", re.I)
# Source paragraph numbers, counted across all <p> elements in the pinned HTML.
# These were read as standalone extracts: no dangling clauses, chopped-off lists,
# interpolated names, exercise instructions, or claims requiring a lost context.
ACCEPTED = {
    113, 121, 122, 125, 128, 142, 189, 275, 296, 298, 299, 360,
    380, 382, 386, 395, 396, 398, 490, 491, 492, 493, 495, 507, 508,
    512, 514, 527, 528, 535, 539, 545, 550, 552, 562, 588, 590, 591,
    612, 613, 616, 617, 621, 634, 642, 644, 646, 650, 662, 677,
    687, 689, 694, 730, 731, 743, 750, 753, 754, 781, 785, 789, 792,
    793, 835, 837, 839, 846, 848, 864, 966, 967, 986, 989, 993,
    998, 1000, 1001, 1005, 1011, 1013, 1029, 1030, 1032, 1087, 1088,
    1090, 1097, 1099, 1103, 1161, 1164, 1179, 1182, 1190, 1194,
    1197, 1199, 1205, 1206, 1208, 1211, 1215, 1216, 1224, 1226, 1229,
    1231, 1232, 1238, 1239, 1247, 1285, 1293, 1296, 1297, 1339, 1344,
    1362, 1375, 1376, 1380, 1383, 1389, 1397, 1398, 1399, 1402,
    1404, 1406, 1414, 1419, 1486, 1496, 1498, 1527, 1558, 1573, 1577,
    1582, 1587, 1596, 1604, 1608, 1609, 1613, 1672, 1706, 1707, 1713,
    1805, 1818, 1821, 1822, 1838, 1856, 1861, 1874, 1889, 1897, 1915,
    1977, 1978, 1980, 1981, 1995, 1996, 1997, 1999, 2002, 2005,
    2009, 2038, 2047, 2048, 2049, 2089, 2093, 2096, 2150, 2164, 2167,
    2179, 2183, 2188, 2197, 2200, 2209, 2212, 2215, 2219, 2228,
    2233, 2259, 2264, 2267, 2275, 2278, 2281, 2282, 2303, 2310,
    2322, 2333, 2337, 2342, 2346, 2349, 2352, 2410, 2415, 2424, 2439,
    2441, 2445, 2449, 2452, 2461, 2470, 2471, 2998, 3006, 3009, 3010,
    3012, 3022, 3049, 3051, 3052, 3058, 3078, 3084, 3085, 3086, 3088,
    3092, 3097, 3104, 3114, 3122, 3129, 3132, 3133, 3134, 3145, 3147,
    3152, 3154, 3162, 3170, 3178, 3179, 3181, 3184, 3186, 3187, 3199,
    3201, 3206, 3207, 3221, 3240, 3246, 3251, 3252, 3253, 3264, 3275,
    3278, 3290, 3300, 3303, 3309, 3322, 3325, 3327, 3335, 3337,
    3339, 3342, 3343, 3344, 3345, 3346, 3348, 3358, 3359, 3367, 3370,
    3371, 3376, 3381, 3384, 3392, 3394, 3419, 3420, 3428, 3431, 3449,
    3450, 3458, 3463, 3471, 3480, 3482, 3486, 3488, 3497, 3498,
    3519, 3524, 3533, 3564, 3571, 3578, 3579, 3598, 3601, 3606, 3615,
    3623, 3632, 3643, 3656, 3659, 3675, 3700, 3713, 3715, 3720, 3722,
    3723, 3725, 3736,
}
NOTICE = """GRAMMAR SOURCE NOTICE

W. M. Baskervill and J. W. Sewell, An English Grammar (1895).
Source transcription: https://www.gutenberg.org/ebooks/14006
Produced by Stephen Schulze and the Distributed Proofreaders Team.

The underlying 1895 text is public domain in the United States. This collection
samples attributed literary quotations rather than the textbook's explanations.
The authors' words and the source's author credits are retained; typography and
exercise numbering are removed and whitespace is normalized. Ebook boilerplate
is excluded from emitted records. No copyright is claimed in the extracts.

The complete source ebook redistribution terms follow for transparency.
"""


@dataclass
class Element:
    tag: str
    attrs: dict = field(default_factory=dict)
    children: list = field(default_factory=list)

    def text(self):
        return "".join(child.text() if isinstance(child, Element) else child
                       for child in self.children)

    def walk(self):
        yield self
        for child in self.children:
            if isinstance(child, Element):
                yield from child.walk()


class Document(HTMLParser):
    def __init__(self, html):
        super().__init__(convert_charrefs=True)
        self.root = Element("document")
        self.stack = [self.root]
        self.feed(html)

    def handle_starttag(self, tag, attrs):
        element = Element(tag, dict(attrs))
        self.stack[-1].children.append(element)
        if tag not in {"area", "base", "br", "col", "embed", "hr", "img", "input",
                       "link", "meta", "param", "source", "track", "wbr"}:
            self.stack.append(element)

    def handle_endtag(self, tag):
        for i in range(len(self.stack) - 1, 0, -1):
            if self.stack[i].tag == tag:
                del self.stack[i:]
                break

    def handle_data(self, data):
        self.stack[-1].children.append(data)


def clean(text):
    return re.sub(r"\s+", " ", text).strip()


def candidates(html):
    part = ""
    section = ""
    paragraph = 0
    for element in Document(html).root.walk():
        if element.tag == "h2":
            ids = {e.attrs.get("id", "") for e in element.walk()}
            if "PART_I" in ids:
                part = "parts-of-speech"
            elif "PART_II" in ids:
                part = "sentence-analysis"
            elif "PART_III" in ids:
                part = "syntax"
            elif "INDEX" in ids:
                part = ""
            section = clean(element.text())
        if element.tag != "p":
            continue
        paragraph += 1
        if not part:
            continue
        authors = [e for e in element.walk() if e.attrs.get("class") == "smcap"]
        if len(authors) != 1:
            continue
        credit = clean(authors[0].text())
        text = clean(element.text())
        if not credit.startswith("—") or not text.endswith(credit):
            continue
        text = text[:-len(credit)].rstrip()
        text = re.sub(r"^(?:\d+\.\s*|\(\d+\)\s*)", "", text)
        yield part, paragraph, section, text, credit


def selected(html):
    rows = []
    for part, paragraph, section, text, credit in candidates(html):
        if paragraph not in ACCEPTED:
            continue
        if (not 9 <= len(text.split()) <= 65 or len(text) > 420
                or INCOMPLETE.search(text)
                or not re.search(r'[.!?][\"\'”’]?$', text)):
            raise ValueError(f"Invalid reviewed quotation at paragraph {paragraph}")
        author = credit[1:].rstrip(".")
        rows.append({
            "id": f"baskervill-sewell-p{paragraph:04d}",
            "text": f"{text}\n—{author}, quoted in An English Grammar (1895)",
            "provenance": (
                f"{author}, as quoted in W. M. Baskervill and J. W. Sewell, "
                f"An English Grammar (1895), {part}, {section} HTML paragraph {paragraph}. "
                f"{URL} Source SHA256 {SHA256}. Public-domain text. "
                "Verbatim wording; HTML emphasis and exercise numbering removed, "
                "whitespace normalized. Historical usage, without the textbook's "
                "grammatical judgments. Importer: families/language/import_grammar.py."
            ),
        })
    if len(rows) != len(ACCEPTED):
        raise ValueError("Missing reviewed source paragraphs")
    if len({row["text"] for row in rows}) != len(rows):
        raise ValueError("Duplicate quotations")
    return rows


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, default=CACHE)
    parser.add_argument("--preview", action="store_true")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    if not args.source.exists():
        args.source.parent.mkdir(parents=True, exist_ok=True)
        args.source.write_bytes(urllib.request.urlopen(URL).read())
    raw = args.source.read_bytes()
    if hashlib.sha256(raw).hexdigest() != SHA256:
        raise ValueError("Source differs from the reviewed snapshot")
    html = raw.decode("utf-8").replace("\r\n", "\n")
    rows = selected(html)
    footer = next(e for e in Document(html).root.walk()
                  if e.attrs.get("id") == "pg-footer")
    footer_text = re.sub(r"\n(?:[ \t]*\n){2,}", "\n\n", footer.text().strip())
    outputs = {
        ROOT / "grammar/records.json": json.dumps(rows, ensure_ascii=False, indent=2) + "\n",
        ROOT / "grammar/LICENSE.md": NOTICE + "\n" + footer_text + "\n",
    }
    if args.preview:
        for row in rows:
            print(row["id"], row["text"], sep="\n")
    else:
        for output, rendered in outputs.items():
            if args.check:
                if output.read_text(encoding="utf-8") != rendered:
                    raise ValueError(f"{output} differs from the importer")
            else:
                output.write_text(rendered, encoding="utf-8")
    print(f"{len(rows)} attributed grammar quotations")


if __name__ == "__main__":
    main()
