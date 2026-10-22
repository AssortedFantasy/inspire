"""Import precise classes of physical features from pinned Natural Earth data."""

from collections import Counter, defaultdict
from pathlib import Path
import runpy
import re


ROOT = Path(__file__).resolve().parent
POLYGON_CLASSES = {
    "Range/mtn": "land/relief/ranges",
    "Highlands": "land/relief/ranges",
    "Plateau": "land/relief/plateaus",
    "Depression": "land/relief/depressions",
    "Valley": "land/relief/valleys",
    "Gorge": "land/relief/valleys",
    "Desert": "land/landscapes/deserts",
    "Plain": "land/landscapes/plains",
    "Peninsula": "land/coasts/peninsulas",
    "Island": "land/islands/islands",
    "Atoll": "land/islands/islands",
    "Island group": "land/islands/archipelagos",
}
MARINE_CLASSES = {
    "ocean": "water/marine/oceans", "sea": "water/marine/seas",
    "gulf": "water/marine/coastal-waters", "bay": "water/marine/coastal-waters",
    "fjord": "water/marine/coastal-waters", "inlet": "water/marine/coastal-waters",
    "sound": "water/marine/coastal-waters", "strait": "water/marine/passages",
    "channel": "water/marine/passages",
}
TYPE_LABELS = {"Range/mtn": "mountain range", "Island group": "archipelago"}
AMBIGUOUS_DESERT_LABELS = {
    "Jabal as Sawdä’", "CAATINGAS", "PUNJAB", "Makgadikgadi Pan",
    "Etosha Pan", "Salar de Uyuni", "White Sands", "Big Bend",
}
# Cartographic classes sometimes describe label placement rather than landforms.
# Avoid making doubtful classifications into factual stimuli; see physical/README.md.
EXCLUDED_POLYGON_IDS = {
    1159103109,  # Vatnajökull is an ice cap, not a mountain range.
    1730072127,  # Jabal Nafüsah is not safely classified as a plain.
    1159104207,  # Sahel is a climatic/ecological belt, not a specific plain.
    1730074691,  # Repeated Queen Maud Mountains identity under a second NE_ID.
    1730071271,  # Second dateline half of the Lau Group, mislabeled Australasia.
    1730070583,  # Darién Gap is a mainland region.
    1159103775,  # Labrador is on the North American mainland.
    1159102727,  # Rowley Shoals is a coral reef complex, not one island.
    1159104353, 1159104355, 1159104359,  # Melanesia, Micronesia, Polynesia: subregions.
    1159104057,  # Alaska North Slope includes mountains and foothills, not just plains.
    1159104315, 1730073981, 1730072123, 1730072913,  # Steppe labels do not establish a landform type.
    1159104521, 1159104517, 1159104523, 1159104389,  # Ice domes, not plateau labels.
    1159104047,  # Guiana Shield is a geological shield, not a plateau.
    1159103243,  # Massif Central combines plateaus and mountain massifs.
    1730072941,  # Campos Sertão cannot be established as a named plateau.
    1159103135,  # Mogollon Rim is an escarpment.
    1159102603,  # Witwatersrand is a ridge.
    1159103089,  # Calabria label, geometry and translated identity conflict.
}
POLYGON_ID_SUFFIXES = {
    (1159102757, "Seven seas (open ocean)"): (
        "azores", "https://surf.visitazores.com/pt-pt/ilhas/flores"),
    (1159102757, "Asia"): (
        "indonesia", "https://www.indonesia.travel/id/en/destination/bali-nusa-tenggara/east-nusa-tenggara/flores"),
}
POLYGON_CORRECTIONS = {
    1159104635: (None, "North Island Volcanic Plateau", None,
                 "https://teara.govt.nz/en/volcanic-plateau-region/page-1"),
    1730071631: (None, "Aravalli Range", None,
                 "https://foundation.rajasthan.gov.in/Topographicaldivisions.aspx"),
    1159103053: (None, "Central Brahui Range", None, "https://irrigation.sindh.gov.pk/files/Books/133.pdf"),
    1159103903: (None, "Qilian Mountains", None,
                 "https://www.stats.gov.cn/zt_18555/ztsj/hjtjzl/2013/202303/t20230302_1921134.html"),
    1159103155: (None, "Serra do Cachimbo", None,
                 "https://rigeo.sgb.gov.br/items/7c33e029-b971-414e-932f-de261e06b3f7"),
    1730071811: (None, "Serra do Piauí", None, None),
    1730071839: (None, "Serra dos Gradaús", None,
                 "https://geoftp.ibge.gov.br/cartas_e_mapas/folhas_topograficas/editoradas/escala_100mil/projeto_conv_digital/serra_dos_gradaus1340.pdf"),
    1730071835: (None, "Serra Dourada", None, None),
    1730071849: (None, "Pakaraima Mountains", None, "https://factpage.glsc.gov.gy/natural-regions/"),
    1159103611: (None, "Sierra Nevada", None,
                 "https://www.miteco.gob.es/gl/parques-nacionales-oapn/red-parques-nacionales/parques-nacionales/sierra-nevada/guia-visitante.html"),
    1159103113: ("Plateau", "Caribou Mountains", None,
                 "https://www.alberta.ca/caribou-mountains-sub-regional-plan-engagement"),
    1730071739: ("Plateau", "Birch Mountains", None,
                 "https://www.albertaparks.ca/media/3195554/small_patch_communities_birch_mtns_wpp.pdf"),
    1730071815: ("Plateau", "Serra da Ibiapaba", None,
                 "https://www.gov.br/incra/pt-br/assuntos/governanca-fundiaria/relatorio-de-analise-de-mercados-de-terras/ramt_sr02_2017.pdf"),
    1730072769: ("Plateau", "Hadhramaut Plateau", None,
                 "https://ntrs.nasa.gov/api/citations/19690019531/downloads/19690019531.pdf"),
    1730072595: ("Highlands", None, None,
                 "https://science.nasa.gov/photojournal/guiana-highlands-shaded-relief-and-colored-height/"),
    1730073365: ("Highlands", None, None, "https://openjicareport.jica.go.jp/pdf/11740677_02.pdf"),
    1159104165: ("Highlands", None, None,
                 "https://science.nasa.gov/earth/earth-observatory/lake-tana-and-the-ethiopian-highlands-148549/"),
    1730071513: ("Highlands", None, None, "https://www.usgs.gov/centers/eros/science/physical-geography"),
    1730072617: ("Highlands", None, None, "https://www.dfa.gov.pg/relief/"),
    1159103751: (None, "Central Valley (California)", None,
                 "https://ca.water.usgs.gov/projects/central-valley/about-central-valley.html"),
    1159103743: (None, "Three Gorges (Yangtze)", None,
                 "https://english.cq.gov.cn/aboutchongqing/travel/5ALevelTouristAttraction/202606/t20260611_15745310.html"),
    1730072787: ("Plateau", "Tassili oua-n-Ahaggar", None,
                 "https://www.me.gov.dz/fr/biodiversite/"),
    1730072795: (None, "Sarīr Kalanshiyū ar-Ramlī al-Kabīr", None,
                 "https://www.gov.pl/attachment/b1ef2288-c886-482d-a988-d28961f37eb9"),
    1159103769: ("Peninsula", "Nova Scotia", None,
                 "https://novascotia.ca/nse/protectedareas/ecoframe.asp"),
    1730071111: ("Peninsula", "Peloponnese", None,
                 "https://www.visitgreece.gr/es/mainland/Peloponnese"),
    1159102797: ("Island group", "Lofoten", None,
                 "https://www.visitnorway.com/places-to-go/northern-norway/the-lofoten-islands/"),
    1159102799: ("Island group", "Vesterålen", None,
                 "https://www.visitnorway.com/places-to-go/northern-norway/vesteralen/"),
    1159102391: ("Island group", "Pribilof Islands", None,
                 "https://spo.nmfs.noaa.gov/Circulars/CIRC17.pdf"),
    1730070601: ("Atoll", "Aldabra", None, "https://whc.unesco.org/en/list/185"),
    1159102491: ("Atoll", "Nukuoro Atoll", None,
                 "https://www.nasa.gov/image-article/nukuoro-atoll/"),
    1159102469: ("Atoll", "Tematangi Atoll", None,
                 "https://www.service-public.pf/wp-content/uploads/2020/01/TEMATANGI-maj-01082018.pdf"),
    1159102775: (None, "Natuna Besar", None,
                 "https://www.indonesia.travel/us/en/destination/sumatra/riau-islands/the-natuna-islands"),
    1159102735: (None, "K'gari", None,
                 "https://www.nrmmrrd.qld.gov.au/land-property/initiatives/kgari/about"),
    1730070885: (None, None, "Oceania",
                 "https://www.fiji.travel/places-to-go/taveuni-island/locations/ultimate-guide-to-taveuni"),
    1159102901: (None, "Mariana Islands", None, "https://www.visitguam.com/"),
    1159103813: (None, "Solomon Islands", None, "https://solomons.gov.sb/about-solomon-islands/"),
    1730072397: (None, None, "Oceania", "https://unstats.un.org/unsd/methodology/M49"),
    1730071889: (None, "Datu Peninsula", None,
                 "https://www.promotemalaysia.com.tw/info_detail.aspx?class_id=9&data_id=438"),
    1159103191: (None, None, None,
                 "https://www.gov.nu.ca/sites/default/files/publications/2024-12/Boothia_Peninsula_Survey_Leclerc.pdf"),
    1730072869: (None, None, None,
                 "https://publications.gc.ca/collections/collection_2025/rncan-nrcan/m183-2/M183-2-7494-eng.pdf"),
    1730072791: ("Plateau", "Tassili n'Ajjer", None,
                 "https://whc.unesco.org/en/list/179/"),
    1159102987: ("Plateau", "Hardangervidda", None,
                 "https://www.visitnorway.com/places-to-go/fjord-norway/scenic-route-hardangervidda/"),
    1730072927: ("Range/mtn", "Les Laurentides", None,
                 "https://toponymie.gouv.qc.ca/ct/ToposWeb/Fiche.aspx?no_seq=34705"),
    1159104343: ("Peninsula", "Iberian Peninsula", None,
                 "https://education.nationalgeographic.org/resource/peninsula/"),
    1159103261: (None, None, "Asia",
                 "https://static-en.parks.org.il/wp-content/uploads/2018/09/Orange_2016.pdf"),
    1159104493: (None, "Queen Maud Mountains", None,
                 "https://pubs.usgs.gov/pp/p1386b/p1386b.pdf"),
}
# The map's Island group class also includes single islands. Preserve the
# individually verified identities, rather than treating all labels as groups.
SINGLE_ISLAND_CORRECTIONS = [
    ("https://www.gohawaii.com/islands", {
        1159103471: "O‘ahu", 1159103473: "Kaua‘i", 1159103833: "Maui", 1159104283: "Hawai‘i"}),
    ("https://www.fiji.travel/things-to-know/destination-guide/discover-fiji", {
        1159103477: "Vanua Levu", 1159103837: "Viti Levu"}),
    ("https://galapagos.gob.ec/parque-nacional-galapagos/", {
        1159103459: "Santa Cruz", 1159103829: "Isabela"}),
    ("https://www.samoa.travel/discover/the-islands-of-samoa/savaii-island/", {1159103461: "Savai‘i"}),
    ("https://www.visitguam.com/", {1159103479: "Guam"}),
    ("https://visitkiribati.travel/wp-content/uploads/2019/05/Kiribati-Travel-Information-2019.pdf", {1159103469: "Kiritimati"}),
    ("https://www.japan.travel/en/world-heritage/amami-oshima-island-tokunoshima-island-northern-part-of-okinawa-island-and-iriomote-island/", {
        1159103485: "Amami Ōshima", 1730072495: "Okinawa Island"}),
    ("https://pubs.usgs.gov/of/2005/1308/of2005-1308.pdf", {
        1159102911: "Urup", 1159102913: "Iturup", 1159103481: "Paramushir"}),
    ("https://papuanewguinea.travel/new-ireland-province/", {1159103449: "New Ireland"}),
    ("https://papuanewguinea.travel/west-new-britain-province/", {1159104281: "New Britain"}),
    ("https://papuanewguinea.travel/autonomous-region-of-bougainville/", {1159103825: "Bougainville"}),
    ("https://www.vanuatu.travel/en/?catid=2&id=259%3Amalekula-island&option=com_content&view=article", {1159103455: "Malakula"}),
    ("https://www.vanuatu.travel/en/about-vanuatu/provinces/sanma/santo", {1159103811: "Espiritu Santo"}),
    ("https://tahititourisme.pf/en-pf/islands-and-archipelagos/the-society-islands/raiatea/", {1159103463: "Raiatea"}),
    ("https://tahititourisme.pf/en-pf/islands-and-archipelagos/marquesas-islands/hiva-oa/", {1159103465: "Hiva Oa"}),
    ("https://www.tahititourisme.nz/islands/nuku-hiva/", {1159103467: "Nuku Hiva"}),
    ("https://www.tahititourisme.nz/islands/tahiti/", {1159103831: "Tahiti"}),
    ("https://solomons.gov.sb/about-solomon-islands/", {
        1159103453: "Santa Isabel", 1159103451: "Makira (San Cristobal)", 1159103827: "Guadalcanal"}),
]
for reference, names in SINGLE_ISLAND_CORRECTIONS:
    for identifier, name in names.items():
        POLYGON_CORRECTIONS[identifier] = ("Island", name, None, reference)
SUBREGION_CORRECTIONS = {
    1159103191: None, 1730072869: None, 1730070885: "Fiji", 1730072397: "Melanesia",
}
NGA_BRAZIL_COAST = (
    "https://msi.nga.mil/api/publications/download?key=16694491%2FSFH00000%2FPub124bk.pdf"
)
POINT_CORRECTIONS = {
    1159105001: ("Point Hope", None,
                 "https://repository.library.noaa.gov/view/noaa/12975/noaa_12975_DS1.pdf"),
    1159105117: ("Ponta de São Sebastião", None,
                 "https://www.visitmozambique.gov.mz/wp-content/uploads/2025/04/MAPA-FRENTE-Portugues.pdf"),
    1159104689: ("Land's End", None, "https://www.visitcornwall.com/destinations/land-s-end"),
    1159104993: ("Punta Eugenia", None,
                 "https://diredimoat.semar.gob.mx/derrotero/derrotero%20pacifico%20completo.pdf"),
    1159104937: ("Mys Yuzhnyy", None,
                 "https://msi.nga.mil/api/publications/download?key=16694491%2FSFH00000%2FPub155bk.pdf"),
    1159105133: (None, (-23.0166667, -42.0), NGA_BRAZIL_COAST),
    1159105125: (None, (-5.4833333, -35.2666667), NGA_BRAZIL_COAST),
    1159105049: (None, (-55.9833333, -67.2666667), NGA_BRAZIL_COAST),
}
EXCLUDED_ELEVATION_IDS = {
    1159107445, 1159108499,  # Dingli Cliffs and The Bluff: cliffs.
    1159106673,  # Uluru: a monolith, not a mountain peak.
    1159105965,  # Suur Munamägi: a hill, not a mountain.
    1159106003,  # Ambiguous Lenina identity at a 150 m Moscow location.
}
ELEVATION_CORRECTIONS = {
    1159108833: ("land/relief/plateaus", "plateau", None,
                 "https://www.gov.br/icmbio/pt-br/assuntos/biodiversidade/unidade-de-conservacao/unidades-de-biomas/cerrado/lista-de-ucs/parna-da-chapada-dos-veadeiros/linha-de-atuacao/arquivos/pm_chapada_dos_veadeiros_1.pdf"),
    1159106291: ("land/relief/valleys", "valley", None,
                 "https://www.nps.gov/deva/learn/nature/faults.htm"),
    1159105935: (None, None, "Galdhøpiggen",
                 "https://www.visitnorway.com/things-to-do/outdoor-activities/hiking/hike-norways-highest-mountain/"),
    1159107023: (None, None, "Hvannadalshnjúkur", "https://www.visiticeland.com/the-regions/the-south/"),
    1159106467: (None, None, "Volcán Villarrica", "https://www.sernageomin.cl/wp-content/uploads/2017/11/1_Villarrica.pdf"),
    1159106257: (None, None, "Kuwohi (formerly Clingmans Dome)",
                 "https://www.nps.gov/grsm/learn/news/kuwohi-name-restored-to-the-highest-peak-in-the-smokies.htm"),
}


def clean_name(name, kind):
    name = " ".join(name.split())
    if name.isupper():
        name = name.title()
    # Expand only conventional, type-specific cartographic abbreviations.
    replacements = {"Range/mtn": {"Mts .": "Mountains", "Mts.": "Mountains", "Ra.": "Range", "Cord.": "Cordillera"},
                    "Plateau": {"Plat.": "Plateau"},
                    "Island": {"I.": "Island"},
                    "Island group": {"Is.": "Islands", "Arch.": "Archipelago"},
                    "Peninsula": {"Pen.": "Peninsula"}}
    for abbreviated, expanded in replacements.get(kind, {}).items():
        name = re.sub(r"(?<!\w)" + re.escape(abbreviated) + r"(?!\w)", expanded, name,
                      flags=re.IGNORECASE)
    return name


def coordinate_label(latitude, longitude):
    return (f"{round(abs(latitude), 1):g}°{'N' if latitude >= 0 else 'S'}, "
            f"{round(abs(longitude), 1):g}°{'E' if longitude >= 0 else 'W'}")


def point_location(latitude, longitude, region):
    location = coordinate_label(latitude, longitude)
    if region and region != "Seven seas (open ocean)":
        location += f" ({region})"
    return location


def vertices(coordinates):
    if not coordinates:
        return
    if isinstance(coordinates[0], (int, float)):
        yield coordinates
    else:
        for part in coordinates:
            yield from vertices(part)


def extent_center(geometry):
    """A bounding-box label point, explicitly not a centroid or summit."""
    points = list(vertices(geometry["coordinates"]))
    xs = [p[0] for p in points]
    ys = [p[1] for p in points]
    if max(xs) - min(xs) > 180:
        xs = [x % 360 for x in xs]
    x = (min(xs) + max(xs)) / 2
    if x > 180:
        x -= 360
    return (min(ys) + max(ys)) / 2, x


def import_physical(source_dir=None):
    # run_path avoids creating __pycache__ inside the sampling tree.
    shared = runpy.run_path(str(ROOT / "import_data.py"))
    source = shared["source"]
    provenance = shared["pinned_provenance"]
    grouped = defaultdict(list)
    seen = set()
    seen_text = set()

    def add(path, filename, identifier, name, kind, location, correction="", source_identifier=None):
        key = path, identifier
        if key in seen:
            return
        seen.add(key)
        name = clean_name(name, kind)
        text = f"{name} - {kind}" + (f"\n{location}" if location else "")
        if (path, text) in seen_text:
            return
        seen_text.add((path, text))
        grouped[path].append({
            "id": f"geography-physical-{filename[3:-8]}-{identifier}",
            "text": text,
            "provenance": provenance(filename, source_identifier if source_identifier is not None else identifier) + correction,
        })

    filename = "ne_10m_geography_regions_elevation_points.geojson"
    for p in source(filename, source_dir):
        if not p["name"] or p["ne_id"] in EXCLUDED_ELEVATION_IDS:
            continue
        if p["featurecla"] == "mountain":
            path, kind = "land/relief/peaks", "mountain"
        elif p["featurecla"] == "depression" and p["ne_id"] in {1159105287, 1159105721, 1159106291}:
            path, kind = "land/relief/depressions", "depression"
        else:
            continue
        name = p["name"]
        correction = ""
        if p["ne_id"] in ELEVATION_CORRECTIONS:
            fixed_path, fixed_kind, fixed_name, reference = ELEVATION_CORRECTIONS[p["ne_id"]]
            path, kind, name = fixed_path or path, fixed_kind or kind, fixed_name or name
            correction = f" Name or landform classification corrected against {reference}; geographic facts, no source prose copied."
        add(path, filename, p["ne_id"], name, kind,
            point_location(p["lat_y"], p["long_x"], p["region"]), correction)

    filename = "ne_10m_geography_regions_polys.geojson"
    features = shared["source_features"](filename, source_dir)
    labels = Counter((f["properties"]["NAME"], f["properties"]["FEATURECLA"],
                      f["properties"]["REGION"], f["properties"]["SUBREGION"])
                     for f in features)
    for feature in features:
        p = feature["properties"]
        kind = p["FEATURECLA"]
        name = p["NAME"]
        region = p["REGION"]
        subregion = SUBREGION_CORRECTIONS.get(p["NE_ID"], p["SUBREGION"])
        correction = ""
        fixed_name = None
        if p["NE_ID"] in EXCLUDED_POLYGON_IDS:
            continue
        if p["NE_ID"] in POLYGON_CORRECTIONS:
            fixed_kind, fixed_name, fixed_region, reference = POLYGON_CORRECTIONS[p["NE_ID"]]
            if reference is None and fixed_name != p["NAME_PT"]:
                raise ValueError(f"Name expansion is absent from source NAME_PT: {p['NE_ID']}")
            kind, name, region = fixed_kind or kind, fixed_name or name, fixed_region or region
            correction = (f" Name, landform type or region corrected against {reference}; geographic facts, no source prose copied."
                          if reference else " Name expanded from the source NAME_PT label, retaining the source identity.")
        if kind == "Desert" and name in AMBIGUOUS_DESERT_LABELS:
            continue
        if kind == "Pen/cape":
            # This class mixes peninsulas, headlands and broad geographic
            # regions. Accept only an explicit peninsula label, not the class
            # wholesale (which also labels India and Scandinavia).
            labels_text = " ".join(x for x in (name, p["LABEL"], p["NAME_EN"]) if x).casefold()
            if not any(marker in labels_text for marker in (
                    "peninsula", "península", "péninsule", "pen.", "pén.")):
                continue
            kind = "Peninsula"
            if not fixed_name and p["NAME_EN"] and p["NAME_EN"].casefold().endswith("peninsula"):
                name = p["NAME_EN"]
        if kind not in POLYGON_CLASSES:
            continue
        location = ", ".join(x for x in (subregion, region)
                             if x and x != "Seven seas (open ocean)"
                             and x.casefold() != name.casefold())
        if not location:
            location = f"Map extent center {coordinate_label(*extent_center(feature['geometry']))}"
        repeated_range_id = p["NE_ID"] in {1159103969, 1159103971}
        if kind == "Range/mtn" and p["NE_ID"] != 1159104505 and (
                repeated_range_id or labels[(p["NAME"], kind, p["REGION"], p["SUBREGION"])] > 1):
            location += f"\nMap extent center {coordinate_label(*extent_center(feature['geometry']))}"
        identifier = p["NE_ID"]
        if repeated_range_id:
            latitude, _ = extent_center(feature["geometry"])
            suffix = "peru" if latitude < 0 else "northern-andes"
            identifier = f"{identifier}-{suffix}"
            reference = ("https://app4.ign.gob.pe/capitulos/tres/relieve.php" if latitude < 0 else
                         "https://portal.gestiondelriesgo.gov.co/Paginas/Noticias/2020/Colombia-un-pais-con-multiples-amenazas-geologicas-pero-por-que-sucede-esto-en-la-UNGRD-se-lo-explicamos.aspx")
            correction += f" Source NE_ID reused for distinct Peruvian and northern Andean ranges; geographic suffix and map extent preserve both identities; verified against {reference}."
        distinct_identity = POLYGON_ID_SUFFIXES.get((identifier, p["REGION"]))
        if distinct_identity:
            suffix, reference = distinct_identity
            identifier = f"{identifier}-{suffix}"
            correction += f" The source reuses this NE_ID for Flores in the Azores and Indonesia; a geographic suffix distinguishes the record; identity verified against {reference}."
        kind_label = "gorges" if p["NE_ID"] == 1159103743 else TYPE_LABELS.get(kind, kind.lower())
        add(POLYGON_CLASSES[kind], filename, identifier, clean_name(name, kind),
            kind_label, location, correction, p["NE_ID"])

    filename = "ne_10m_geography_regions_points.geojson"
    for feature in shared["source_features"](filename, source_dir):
        p = feature["properties"]
        kind = p["featurecla"]
        if kind not in {"cape", "waterfall"} or p["ne_id"] == 1159104955:
            continue
        path = "land/coasts/capes" if kind == "cape" else "water/inland/waterfalls"
        longitude, latitude = feature["geometry"]["coordinates"][:2]
        name = p["name"]
        correction = ""
        if p["ne_id"] in POINT_CORRECTIONS:
            fixed_name, fixed_position, reference = POINT_CORRECTIONS[p["ne_id"]]
            name = fixed_name or name
            if fixed_position:
                latitude, longitude = fixed_position
            correction = f" Name or position corrected against {reference}; geographic facts, no source prose copied."
        # Several Antarctic capes and Victoria Falls have stale property
        # coordinates; use the actual mapped point, not lat_y/long_x fields.
        add(path, filename, p["ne_id"], name, kind,
            point_location(latitude, longitude, p["region"]), correction)

    filename = "ne_10m_geography_marine_polys.geojson"
    for p in source(filename, source_dir):
        if not p["name"] or p["featurecla"] not in MARINE_CLASSES:
            continue
        add(MARINE_CLASSES[p["featurecla"]], filename, p["ne_id"],
            p["name"], p["featurecla"], "")

    # Named inland waters need location because unrelated places share names.
    filename = "ne_10m_lakes.geojson"
    for feature in shared["source_features"](filename, source_dir):
        p = feature["properties"]
        if not p["name"]:
            continue
        path = {"Reservoir": "water/inland/reservoirs", "Lake": "water/inland/lakes",
                "Alkaline Lake": "water/inland/alkaline-lakes"}[p["featurecla"]]
        location = coordinate_label(*extent_center(feature["geometry"]))
        add(path, filename, p["ne_id"], p["name"], p["featurecla"].lower(),
            f"Map extent center {location}")

    filename = "ne_10m_rivers_lake_centerlines.geojson"
    rivers = defaultdict(list)
    for feature in shared["source_features"](filename, source_dir):
        p = feature["properties"]
        if p["featurecla"] != "River" or not p.get("name") or not p.get("dissolve"):
            continue
        if not feature["geometry"] or not list(vertices(feature["geometry"]["coordinates"])):
            continue
        rivers[p["dissolve"]].append(feature)
    for identifier, parts in sorted(rivers.items()):
        names = {feature["properties"]["name"] for feature in parts}
        if len(names) != 1:
            raise ValueError(f"Conflicting river names for dissolve ID {identifier}: {names}")
        # The stable dissolve ID identifies all mapped pieces of a named river.
        geometry = {"coordinates": [feature["geometry"]["coordinates"] for feature in parts]}
        location = coordinate_label(*extent_center(geometry))
        add("water/inland/rivers", filename, identifier, names.pop(), "river",
            f"Map extent center {location}")

    for path, records in sorted(grouped.items()):
        shared["write"](f"physical/{path}", records)
    # Explicit legacy leaves only; never sweep unrelated descendants.
    for relative in ("water/marine/inlets", "water/marine/open-waters"):
        legacy = ROOT / "physical" / relative
        (legacy / "records.json").unlink(missing_ok=True)
        if legacy.exists() and not any(legacy.iterdir()):
            legacy.rmdir()
    shared["count_weights"]("physical")


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-dir", type=Path)
    import_physical(parser.parse_args().source_dir)
