"""Rebuild geography corpora from pinned public-domain data and factual lists."""

import argparse
from collections import Counter, defaultdict
import hashlib
import json
from pathlib import Path
import urllib.request


ROOT = Path(__file__).resolve().parent
COMMIT = "f1890d9f152c896d250a77557a5751a93d494776"
BASE = f"https://raw.githubusercontent.com/nvkelso/natural-earth-vector/{COMMIT}/geojson/"
SOURCES = {
    "ne_50m_populated_places.geojson": "da4662b7bbfeb897d02f228c5839131dce27acff5717630f91ccff4f67828ee7",
    "ne_50m_admin_0_countries.geojson": "3e458fc036ad0a66411f2c1e6cac49c5d7bfb81cb1123bc513b22511a2b7fdeb",
}
CITY_PROVENANCE = (
    "Natural Earth 1:50m populated places v5.1.2, NE_ID {id}; "
    f"{BASE}ne_50m_populated_places.geojson; "
    "public domain: https://www.naturalearthdata.com/about/terms-of-use/. "
    "Names only; curated formatting and continental grouping; see geography/README.md."
)

# Use explicit alternatives only: some NAME_EN values name the wrong city.
ENGLISH_LABELS = {
    "Poitier", "Basse-terre", "Grand Turk", "Agana", "Tarawa", "Andorra",
    "Sholapur", "Nasik", "Jullundur", "Moradabad", "St.  Paul", "Ft.  Worth",
    "Port-of-Spain", "Mt.  Hagen", "Rostov", "Yuzhno Sakhalinsk",
    "Dalandzadgad", "Dund-Us", "Huaiyin", "Coihaique", "Los Angeles",
    "St.-Denis", "Tucumán", "Las Palmas", "Archangel", "Bandjarmasin",
    "Vishakhapatnam", "Banghazi", "Santa Cruz", "St.  Petersburg",
    "Xian", "Shenyeng", "København", "Bogota",
}
COUNTRY_LABELS = {
    "Aland": "Åland", "Curacao": "Curaçao", "Cape Verde": "Cabo Verde",
    "Congo (Brazzaville)": "Republic of the Congo",
    "Congo (Kinshasa)": "Democratic Republic of the Congo",
    "East Timor": "Timor-Leste", "eSwatini": "Eswatini",
    "Guinea Bissau": "Guinea-Bissau", "Hong Kong S.A.R.": "Hong Kong",
    "Macau S.A.R": "Macao", "Svalbard and Jan Mayen Islands": "Svalbard",
    "United States of America": "United States", "Vatican": "Vatican City",
}

US_STATES = """AL Alabama
AK Alaska
AZ Arizona
AR Arkansas
CA California
CO Colorado
CT Connecticut
DE Delaware
FL Florida
GA Georgia
HI Hawaii
ID Idaho
IL Illinois
IN Indiana
IA Iowa
KS Kansas
KY Kentucky
LA Louisiana
ME Maine
MD Maryland
MA Massachusetts
MI Michigan
MN Minnesota
MS Mississippi
MO Missouri
MT Montana
NE Nebraska
NV Nevada
NH New Hampshire
NJ New Jersey
NM New Mexico
NY New York
NC North Carolina
ND North Dakota
OH Ohio
OK Oklahoma
OR Oregon
PA Pennsylvania
RI Rhode Island
SC South Carolina
SD South Dakota
TN Tennessee
TX Texas
UT Utah
VT Vermont
VA Virginia
WA Washington
WV West Virginia
WI Wisconsin
WY Wyoming"""
CANADA = """AB Alberta
BC British Columbia
MB Manitoba
NB New Brunswick
NL Newfoundland and Labrador
NT Northwest Territories
NS Nova Scotia
NU Nunavut
ON Ontario
PE Prince Edward Island
QC Quebec
SK Saskatchewan
YT Yukon"""
AUSTRALIA = """ACT Australian Capital Territory
NSW New South Wales
NT Northern Territory
QLD Queensland
SA South Australia
TAS Tasmania
VIC Victoria
WA Western Australia"""


def source(name, source_dir):
    data = ((source_dir / name).read_bytes() if source_dir else
            urllib.request.urlopen(BASE + name, timeout=60).read())
    if hashlib.sha256(data).hexdigest() != SOURCES[name]:
        raise ValueError(f"Source checksum mismatch: {name}")
    return [feature["properties"] for feature in json.loads(data)["features"]]


def continent(place, countries):
    code = place["ADM0_A3"]
    explicit = {"GIB": "Europe", "SJM": "Europe", "SSD": "Africa",
                "MUS": "Africa", "SYC": "Africa", "MDV": "Asia"}
    result = explicit.get(code) or countries[code]["CONTINENT"]
    # Split overseas places and transcontinental countries by place, not sovereignty.
    if code == "RUS":
        result = "Asia" if place["LONGITUDE"] >= 60 else "Europe"
        if place["NAME"] == "Vorkuta":
            result = "Europe"
        elif place["NAME"] == "Nizhny Tagil":
            result = "Asia"
    elif code == "TUR" and place["NAME"] == "Istanbul":
        result = "Europe"
    elif code == "USA" and place["ADM1NAME"] == "Hawaii":
        result = "Oceania"
    elif code == "IDN" and place["NAME"] == "Jayapura":
        result = "Oceania"
    elif code == "FRA":
        if place["ADM1NAME"] in {"Guinaa", "Guyane"}:
            result = "South America"
        elif place["ADM1NAME"] in {"Guadeloupe", "Martinique"}:
            result = "North America"
        elif place["ADM1NAME"] == "La Réunion":
            result = "Africa"
    elif code == "ESP" and place["NAME"] == "Las Palmas":
        result = "Africa"
    return result.lower().replace(" ", "-")


def write(relative, records):
    path = ROOT / relative / "records.json"
    path.parent.mkdir(parents=True, exist_ok=True)
    records.sort(key=lambda record: (record["text"].casefold(), record["id"]))
    path.write_text(json.dumps(records, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"{relative}: {len(records)}")


def cities(source_dir):
    places = source("ne_50m_populated_places.geojson", source_dir)
    countries = {p["ADM0_A3"]: p for p in source("ne_50m_admin_0_countries.geojson", source_dir)}
    # This extra Niamey record is located in Maradi; omit rather than infer identity.
    places = [p for p in places if p["FEATURECLA"] not in {"Scientific station", "Historic place"}
              and p["NE_ID"] != 1159149391]
    duplicates = Counter((p["NAME"], p["ADM0NAME"]) for p in places)
    grouped = defaultdict(list)
    for p in places:
        name = p["NAME_EN"] if p["NAME"] in ENGLISH_LABELS else p["NAME"]
        name = " ".join(name.split())
        country = COUNTRY_LABELS.get(p["ADM0NAME"], p["ADM0NAME"])
        region = None
        correction = ""
        if p["ADM0_A3"] in {"USA", "CAN"} or duplicates[(p["NAME"], p["ADM0NAME"])] > 1:
            region = p["ADM1NAME"]
        if p["NAME"] == "Fort Smith" and p["ADM0_A3"] == "CAN":
            region = "Northwest Territories"
            correction = " Province corrected against https://www.fortsmith.ca/contact-us (2026-10-13); geographic fact, no prose copied."
        if p["ADM0_A3"] == "FRA" and p["ADM1NAME"] in {"Guinaa", "Guadeloupe", "Martinique", "La Réunion"}:
            region = {"Guinaa": "French Guiana", "La Réunion": "Réunion"}.get(p["ADM1NAME"], p["ADM1NAME"])
        if p["NAME"] == "Las Palmas":
            region = "Canary Islands"
        if p["NAME"] == "Sevastopol":
            country = "Crimea"
        if p["NAME"] == "Washington,  D.C.":
            region = None
        text = f"{name} — {region + ', ' if region else ''}{country}"
        grouped[continent(p, countries)].append({
            "id": f"geography-city-ne-{p['NE_ID']}", "text": text,
            "provenance": CITY_PROVENANCE.format(id=p["NE_ID"]) + correction,
        })
    records = [record for values in grouped.values() for record in values]
    if len(records) != 1209 or len({r["id"] for r in records}) != 1209:
        raise ValueError("Unexpected source count or duplicate Natural Earth ID")
    if len({r["text"] for r in records}) != 1209:
        raise ValueError("Duplicate city stimulus")
    for group, values in sorted(grouped.items()):
        write(f"cities/{group}", values)


def subdivisions(relative, data, country, url, expected):
    rows = [line.split(" ", 1) for line in data.splitlines()]
    if len(rows) != expected or len({code for code, _ in rows}) != expected:
        raise ValueError(f"Unexpected subdivision count: {relative}")
    write(f"subdivisions/{relative}", [{
        "id": f"geography-subdivision-{relative}-{code.lower()}",
        "text": f"{name} — {country}",
        "provenance": f"Independently assembled geographic name fact; verified against {url} (accessed 2026-10-13). No source prose or expressive descriptions copied.",
    } for code, name in rows])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-dir", type=Path, help="Read the two pinned GeoJSON files locally instead of downloading")
    args = parser.parse_args()
    cities(args.source_dir)
    subdivisions("us-states", US_STATES, "United States",
                 "https://www.census.gov/library/reference/code-lists/ansi/ansi-codes-for-states.html", 50)
    subdivisions("canada-provinces-and-territories", CANADA, "Canada",
                 "https://our-languages.canada.ca/en/writing-tips-plus/abbreviations-canadian-provinces-and-territories", 13)
    subdivisions("australia-states-and-mainland-territories", AUSTRALIA, "Australia",
                 "https://www.abs.gov.au/statistics/standards/australian-statistical-geography-standard-asgs/edition-3-july-2021-june-2026/main-structure-and-greater-capital-city-statistical-areas/australia-and-stateterritory", 8)


if __name__ == "__main__":
    main()
