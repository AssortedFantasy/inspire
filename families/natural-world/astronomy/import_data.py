"""Build factual astronomical profiles from pinned NASA archive/JPL tables."""

import argparse
from collections import defaultdict
import hashlib
from html.parser import HTMLParser
from html import unescape
import json
import math
from pathlib import Path
import re
import urllib.request
from urllib.parse import urlencode


ROOT = Path(__file__).resolve().parent
QUERY = ("select pl_name,hostname,pl_refname,pl_orbper,pl_orbperlim,pl_rade,pl_radelim,"
         "pl_masse,pl_masselim,pl_msinie,pl_msinielim,pl_orbsmax,pl_orbsmaxlim,"
         "pl_orbeccen,pl_orbeccenlim,disc_year,discoverymethod "
         "from ps where default_flag=1")
SOURCES = {
    "nasa-exoplanets-with-references.json": {
        "url": "https://exoplanetarchive.ipac.caltech.edu/TAP/sync?"
               + urlencode({"query": QUERY, "format": "json"}),
        "sha256": "b0d2c1dacba02ce86d9d3acee8b8f0aa672dd78e98505b79e5064533f4e83417",
    },
    "jpl-moon-parameters.html": {
        "url": "https://ssd.jpl.nasa.gov/sats/phys_par/",
        "sha256": "0c6323f469e48ac654f7d4dd263ed6c62ad14689218c5451a98da29d625aaf10",
    },
    "jpl-planet-parameters.html": {
        "url": "https://ssd.jpl.nasa.gov/planets/phys_par.html",
        "sha256": "e49a487947295c389ba6e5833dd8fb04b63e826d39f70c123881882096e5e68b",
    },
}
DISCOVERY_LEAVES = {
    "Transit": "photometry/transits",
    "Orbital Brightness Modulation": "photometry/orbital-brightness",
    "Radial Velocity": "dynamical-measurements/radial-velocity",
    "Astrometry": "dynamical-measurements/astrometry",
    "Imaging": "resolved-observations/direct-imaging",
    "Disk Kinematics": "resolved-observations/disk-kinematics",
    "Microlensing": "gravitational-microlensing",
    "Transit Timing Variations": "timing/transit-variations",
    "Eclipse Timing Variations": "timing/eclipse-variations",
    "Pulsar Timing": "timing/pulsars",
    "Pulsation Timing Variations": "timing/stellar-pulsations",
}


def source_bytes(cache, filename):
    target = cache / filename
    source = SOURCES[filename]
    if not target.exists():
        cache.mkdir(parents=True, exist_ok=True)
        target.write_bytes(urllib.request.urlopen(source["url"], timeout=90).read())
    data = target.read_bytes()
    if hashlib.sha256(data).hexdigest() != source["sha256"]:
        raise ValueError(f"Source checksum differs: {filename}")
    return data


def number(value):
    return value is not None and isinstance(value, (int, float)) and math.isfinite(value)


def central(row, key):
    """Require a reported central estimate; omit bounds and unspecified flags."""
    return number(row[key]) and row.get(key + "lim") == 0


def fmt(value):
    if abs(value) >= 10_000:
        return f"{value:.0f}"
    return f"{value:.4g}"


def provenance(filename, identifier, details):
    source = SOURCES[filename]
    return (f"NASA astronomical factual data; source object {identifier}; {source['url']}; "
            f"retrieved 2026-10-13; input SHA256 {source['sha256']}. "
            f"{details} Selected numerical facts formatted by Inspire; no images or article prose used.")


class Tables(HTMLParser):
    """Read cells, including JPL's implicit closing td tags, without browser code."""

    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.tables = []
        self.table = None
        self.row = None
        self.cell = None

    def flush_cell(self):
        if self.cell is not None and self.row is not None:
            self.row.append(" ".join("".join(self.cell).split()))
        self.cell = None

    def handle_starttag(self, tag, attrs):
        if tag == "table":
            self.table = []
        elif self.table is not None and tag == "tr":
            self.row = []
        elif self.row is not None and tag in {"td", "th"}:
            self.flush_cell()
            self.cell = []
        elif self.cell is not None and tag == "br":
            self.cell.append(" ")

    def handle_data(self, data):
        if self.cell is not None:
            self.cell.append(data)

    def handle_endtag(self, tag):
        if tag in {"td", "th"}:
            self.flush_cell()
        elif tag == "tr" and self.row is not None:
            self.flush_cell()
            if self.row:
                self.table.append(self.row)
            self.row = None
        elif tag == "table" and self.table is not None:
            self.tables.append(self.table)
            self.table = None


def first_number(cell):
    match = re.match(r"\s*([-+]?(?:[0-9]*\.)?[0-9]+(?:[eE][-+]?[0-9]+)?)", cell)
    return float(match.group(1)) if match else None


def exoplanets(cache, groups):
    filename = "nasa-exoplanets-with-references.json"
    rows = json.loads(source_bytes(cache, filename))
    assert len({row["pl_name"] for row in rows}) == len(rows)
    for row in sorted(rows, key=lambda r: r["pl_name"]):
        details = []
        for key, label, unit in [
            ("pl_orbper", "orbital period", "days"),
            ("pl_rade", "planet radius", "Earth radii"),
            ("pl_masse", "planet mass", "Earth masses"),
            ("pl_orbsmax", "orbital semimajor axis", "AU"),
        ]:
            value = row[key]
            if central(row, key) and value > 0:
                details.append(f"reported {label} ≈ {fmt(value)} {unit}")
        minimum_mass = row["pl_msinie"]
        if central(row, "pl_msinie") and minimum_mass > 0:
            details.append(f"reported minimum mass m·sin(i) ≈ {fmt(minimum_mass)} Earth masses")
        eccentricity = row["pl_orbeccen"]
        if central(row, "pl_orbeccen") and 0 <= eccentricity < 1:
            details.append(f"reported orbital eccentricity {fmt(eccentricity)}")
        if not details:
            continue
        method = row["discoverymethod"]
        leaf = DISCOVERY_LEAVES[method]
        year = row["disc_year"]
        raw_reference = row["pl_refname"] or ""
        reference = " ".join(unescape(re.sub(r"<[^>]+>", "", raw_reference)).split())
        urls = re.findall(r"href\s*=\s*[\"']?([^\"'\s>]+)", raw_reference)
        text = (f"{row['pl_name']}, orbiting {row['hostname']}. " + "; ".join(details)
                + f". Discovery method: {method}"
                + (f"; discovery year: {year}" if year else "")
                + ".")
        groups["extrasolar-planets/" + leaf].append({
            "id": "nasa-ps-" + row["pl_name"],
            "text": text,
            "provenance": provenance(
                filename, row["pl_name"],
                "PS default_flag=1 selects one internally sourced literature solution per planet. "
                "No PSCompPars mass-radius infill, no equilibrium-temperature/surface-temperature "
                "equivalence, and no habitability inference. Mass and m*sin(i) are separate source columns. "
                "Only source limit flag=0 estimates emitted; upper/lower limits and unspecified flags omitted. "
                "Four significant figures; uncertainties are not inferred or invented. "
                "Column definitions: https://exoplanetarchive.ipac.caltech.edu/docs/API_PS_columns.html.",
            ) + f" Default literature reference: {reference}; " + "; ".join(urls) + ".",
        })


def solar_system(cache, groups):
    filename = "jpl-planet-parameters.html"
    tables = Tables()
    tables.feed(source_bytes(cache, filename).decode("utf-8"))
    planets = {"Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune"}
    dwarfs = {"Ceres", "Pluto", "Eris", "Makemake", "Haumea"}
    for table in tables.tables:
        for row in table:
            if row[0] not in planets | dwarfs:
                continue
            assert len(row) == 11, row
            name = row[0]
            radius, rotation, orbit = (first_number(row[index]) for index in (2, 5, 6))
            assert radius > 0 and rotation != 0 and orbit > 0
            kind = "planet" if name in planets else "dwarf planet"
            sense = "retrograde" if rotation < 0 else "prograde"
            period_unit = "year" if orbit == 1 else "years"
            text = (f"{name}, a Solar System {kind}. Reported volume-equivalent mean radius "
                    f"≈ {fmt(radius)} km; sidereal rotation ≈ {fmt(abs(rotation))} days ({sense}); "
                    f"sidereal orbital period ≈ {fmt(orbit)} {period_unit}. Sidereal periods are measured "
                    "relative to fixed stars.")
            groups["solar-system/" + ("planets" if name in planets else "dwarf-planets")].append({
                "id": "jpl-planet-" + name.lower(),
                "text": text,
                "provenance": provenance(
                    filename, name,
                    "JPL selected physical parameters table. Mean radius is the radius of an "
                    "equal-volume sphere, not equatorial radius; rotation is sidereal, not a solar day. "
                    "Signed rotation is rendered as magnitude and sense. Original table reference "
                    "letters and numerical uncertainties remain in the pinned HTML input.",
                ),
            })

    filename = "jpl-moon-parameters.html"
    tables = Tables()
    tables.feed(source_bytes(cache, filename).decode("utf-8"))
    for table in tables.tables:
        for row in table:
            if len(row) != 12 or not row[2].isdigit():
                continue
            planet, moon, code = row[:3]
            radius, sigma = (first_number(row[index]) for index in (6, 7))
            if radius is None or radius <= 0:
                continue
            # Preserve enough central-value precision for the quoted sigma:
            # rounding Moon 1737.4±0.1 to 1737±0.1 would change the interval.
            uncertainty = f" ± {sigma:.12g}" if sigma is not None and sigma >= 0 else ""
            text = (f"{moon}, a natural satellite of {planet}. "
                    f"Reported volume-equivalent mean radius {radius:.12g}{uncertainty} km: "
                    "the radius of a sphere with the same volume as the moon.")
            groups["solar-system/natural-satellites"].append({
                "id": "jpl-satellite-" + code,
                "text": text,
                "provenance": provenance(
                    filename, code + " " + moon,
                    "JPL planetary satellite physical parameters; mean radius and quoted sigma, "
                    "not diameter. Missing/upper-limit GM and poorly constrained derived densities "
                    "are not emitted. Original physical-parameter references remain in pinned HTML.",
                ),
            })


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--cache", type=Path, required=True)
    args = parser.parse_args()
    groups = defaultdict(list)
    exoplanets(args.cache, groups)
    solar_system(args.cache, groups)
    for leaf, records in sorted(groups.items()):
        assert records and len({record["id"] for record in records}) == len(records)
        target = ROOT / leaf / "records.json"
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(json.dumps(records, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    # The discovery-method hierarchy must not make two rare stellar-clock
    # planets drown out thousands of different transit discoveries. Uniform
    # named-planet selection applies within this branch; its incoming share
    # remains topic-balanced against Solar System objects.
    extrasolar = ROOT / "extrasolar-planets"

    def count(node):
        children = sorted(path for path in node.iterdir() if path.is_dir())
        records = node / "records.json"
        total = sum(count(child) for child in children) if children else len(
            json.loads(records.read_text(encoding="utf-8")))
        if node != extrasolar:
            (node / "node.toml").write_text(f"weight = {total}\n", encoding="utf-8")
        return total

    count(extrasolar)
    (ROOT / "sources.json").write_text(json.dumps(SOURCES, indent=2) + "\n", encoding="utf-8")
    print({leaf: len(records) for leaf, records in sorted(groups.items())})


if __name__ == "__main__":
    main()
