"""Append explicitly selected NOAA glossary definitions to weather corpora."""

import argparse
import hashlib
from html.parser import HTMLParser
import json
from pathlib import Path
import re
import urllib.request


ROOT = Path(__file__).resolve().parent
SELECTION = {
    "clouds": [
        "Banner Cloud", "Cap Cloud", "Cloud Condensation Nuclei", "Cloud Streets",
        "Lenticular Cloud", "Mammatus Clouds", "Nacreous Clouds",
        "Noctilucent Clouds", "Roll Cloud", "Shelf Cloud", "Wall Cloud",
        "Advection Fog", "Radiation Fog", "Steam Fog", "Upslope Fog", "Freezing Fog",
        "Ice Fog", "Tule Fog", "Ground Fog", "Sea Fog", "Evaporation-mixing Fog",
    ],
    "precipitation": [
        "Dew", "Diamond Dust", "Freezing Rain", "Frozen Dew", "Glaze",
        "Hoar Frost", "Ice Crystals", "Ice Pellets", "Lake Effect Snow",
        "Orographic Precipitation", "Rain Shadow", "Rime Ice", "Snow Grains",
        "Snow Pellets", "Snowflake", "Virga",
    ],
    "atmospheric-dynamics": [
        "Adiabatic Process", "Adiabatic Lapse Rate", "Advection", "Air Mass",
        "Anticyclone", "Cold Front", "Convection", "Coriolis Force", "Cyclone",
        "Dry Line", "Environmental Lapse Rate", "Extratropical Cyclone",
        "Frontogenesis", "Gravity Wave", "Gust Front", "Inversion",
        "Jet Stream", "Katabatic Wind", "Kelvin-Helmholtz Waves", "Lake Breeze",
        "Land Breeze", "Lee Wave", "Mesocyclone", "Microburst", "Mountain Wave",
        "Occluded Front", "Rossby Waves", "Sea Breeze", "Stationary Front",
        "Tornado", "Tropical Cyclone", "Urban Heat Island", "Warm Front",
    ],
    "optical-effects": [
        "Anthelion", "Crepuscular Rays", "Fogbow", "Glory", "Halo",
        "Iridescence", "Parhelion", "Rainbow", "Rayleigh Scattering",
        "Refraction", "Sun Pillar", "Undersun",
    ],
}

# Bounded source-grounded edits remove operational guidance and wording that
# confuses deposition with sublimation. They do not add an inferred phenomenon.
PARAPHRASES = {
    "Mesocyclone": (
        "A storm-scale region of rotation, typically around 2–6 miles across. "
        "Its circulation covers an area much larger than the tornado that may develop within it."
    ),
    "Parhelion": (
        "A sun dog: either of two colored luminous spots appearing roughly 22 degrees "
        "to either side of the Sun at the same elevation. Refraction of sunlight "
        "through ice crystals produces these spots."
    ),
    "Hoar Frost": (
        "A deposit of interlocking ice crystals on objects whose surfaces are below "
        "freezing. Water vapor deposits directly as ice on exposed surfaces such as "
        "branches, leaves, wires, and poles."
    ),
    "Ice Pellets": (
        "Sleet: pellets of ice composed of frozen or mostly frozen raindrops, or "
        "refrozen partially melted snowflakes. The pellets usually bounce after "
        "hitting the ground or another hard surface."
    ),
    "Refraction": (
        "A change in the direction of energy propagation as conditions change "
        "within the medium through which it travels. In the atmosphere, refraction "
        "changes the path of light and radar beams."
    ),
}


def clean_definition(value):
    # These artifacts already occur in the downloaded UTF-8 source, rather than
    # arising from the importer's encoding. Only evidenced sequences are fixed.
    for broken, corrected in {"Â½": "½", "Â°": "°", "â€“": "–"}.items():
        value = value.replace(broken, corrected)
    value = re.sub(r" See also [^.]+\.$", "", value)
    return value


class Glossary(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.field = None
        self.parts = []
        self.term = None
        self.entries = {}

    def handle_starttag(self, tag, attrs):
        if tag in {"dt", "dd"}:
            self.field = tag
            self.parts = []
        elif self.field and tag in {"br", "p", "li"}:
            self.parts.append(" ")

    def handle_data(self, data):
        if self.field:
            self.parts.append(data)

    def handle_endtag(self, tag):
        if tag != self.field:
            return
        value = " ".join("".join(self.parts).split())
        if tag == "dt":
            self.term = value
        elif self.term and value:
            values = self.entries.setdefault(self.term, [])
            if value not in values:
                values.append(value)
        self.field = None


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--cache", type=Path, required=True)
    args = parser.parse_args()
    sources = json.loads((ROOT / "noaa_sources.json").read_text(encoding="utf-8"))
    definitions = {}
    source_by_term = {}
    for source in sources:
        target = args.cache / "noaa" / (source["url"][-1] + ".html")
        if not target.exists():
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(urllib.request.urlopen(source["url"], timeout=50).read())
        data = target.read_bytes()
        if hashlib.sha256(data).hexdigest() != source["sha256"]:
            raise ValueError(f"NOAA source checksum differs: {source['url']}")
        glossary = Glossary()
        glossary.feed(data.decode("utf-8"))
        definitions.update(glossary.entries)
        source_by_term.update({term: source for term in glossary.entries})

    selected = {}
    for terms in SELECTION.values():
        for term in terms:
            alternatives = definitions[term]
            if len(alternatives) != 1:
                raise ValueError(f"Selected term has multiple glossary meanings: {term}")
            definition = alternatives[0]
            if definition.lower().startswith(("see ", "same as ")):
                raise ValueError(f"Definition is a cross-reference, not self-contained: {term}")
            selected[term] = PARAPHRASES.get(term, clean_definition(definition))

    additions = 0
    for leaf, terms in SELECTION.items():
        output = ROOT / "weather" / leaf / "records.json"
        records = json.loads(output.read_text(encoding="utf-8")) if output.exists() else []
        records = [record for record in records if not record["id"].startswith("noaa-glossary-")]
        for term in terms:
            definition = selected[term]
            source = source_by_term[term]
            identifier = "noaa-glossary-" + re.sub(r"[^a-z0-9]+", "-", term.lower()).strip("-")
            records.append({
                "id": identifier,
                "text": f"{term} — {definition}",
                "provenance": (
                    "National Weather Service, NOAA glossary; " + source["url"]
                    + "; term " + term + "; downloaded 2026-10-13; input SHA256 "
                    + source["sha256"] + ". Public domain NWS glossary material under "
                    "https://www.weather.gov/disclaimer; "
                    "selected definitions, HTML removed, whitespace normalized and "
                    "evidenced encoding artifacts corrected. "
                    + ("Original factual paraphrase of the downloaded definition. "
                       if term in PARAPHRASES else "Source wording retained except trailing cross-references. ")
                    + "Selection and editorial changes documented in import_weather.py and README.md."
                ),
            })
            additions += 1
        assert len({record["id"] for record in records}) == len(records)
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(json.dumps(records, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"Imported {additions} selected NOAA definitions")


if __name__ == "__main__":
    main()
