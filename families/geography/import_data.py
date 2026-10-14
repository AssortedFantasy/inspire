"""Rebuild geography corpora from pinned public-domain Natural Earth data."""

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
    "ne_10m_populated_places.geojson": "9b8e3de09048ef00dfc70357dbb9fa324493f214b5e0ae4daf1aa79a8d10116b",
    "ne_10m_admin_1_states_provinces.geojson": "22d0e3ad85eb3e27f17cabf8ba2d50e554fbc27a87796ff891d958185da62fb5",
    "ne_10m_geography_regions_elevation_points.geojson": "f98a16867867146ec4146d6d4b18c823eeedb2825947de666116cf9a4e3f43cb",
    "ne_10m_geography_regions_points.geojson": "25821b530966e62756faebf53699f12bf73550f8228aa4bbdf3e4132dd7cf806",
    "ne_10m_geography_regions_polys.geojson": "b7b26e50ea917d3696aec87f932def2bf5f890f5770e441d59c162c6f4c92a77",
    "ne_10m_geography_marine_polys.geojson": "53f865e8ffa966cdd402145c82c5cd14ee7ce974cd0eb9a3f59f03a4cfd2d66c",
    "ne_10m_lakes.geojson": "2d036f53dedec578001c5c30c2959ee7d4eebc1306900fa4367c49929ec8f2d9",
    "ne_10m_rivers_lake_centerlines.geojson": "bb854a900ecbd3b408df46d5e16e3e0f974ba55993f9d8b5c26e855273c0905a",
    "ne_50m_admin_0_countries.geojson": "3e458fc036ad0a66411f2c1e6cac49c5d7bfb81cb1123bc513b22511a2b7fdeb",
}
CITY_PROVENANCE = (
    "Natural Earth 1:10m populated places v5.1.2, NE_ID {id}; "
    f"{BASE}ne_10m_populated_places.geojson; "
    "public domain: https://www.naturalearthdata.com/about/terms-of-use/. "
    "Geographic names; curated formatting, continental grouping and dated source "
    "centre classifications; see geography/README.md."
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
    "Sao Tome and Principe": "São Tomé and Príncipe",
    "South Georgia and the Islands": "South Georgia and South Sandwich Islands",
    "Macedonia": "North Macedonia", "Czech Republic": "Czechia",
    "Republic of Serbia": "Serbia",
    "Macao S.A.R": "Macao",
}

# Correct source location/name errors against primary references; never infer a
# place identity from its coordinates alone. Values are name, country, and source.
CITY_CORRECTIONS = {
    1159116505: (None, "Lesotho", "https://lesothohousing.org.ls/city/qachas-nek/"),
    1159119035: (None, "Equatorial Guinea", "https://www.guineaecuatorialpress.com/noticias/kie-ntem"),
    1159149439: (None, "Mauritania", "https://visitmauritania.com/gorgol/"),
    1159150681: ("Gqeberha", None, "https://www.gov.za/news/media-statements/sport-arts-and-culture-transformation-our-heritage-landscape-25-feb-2021"),
    1159149741: (None, "Western Sahara", "https://www.un.org/sg/en/content/sg/notes-correspondents/2023-09-04/note-correspondents-western-sahara"),
    1159148261: (None, "Western Sahara", "https://www.un.org/sg/en/content/sg/notes-correspondents/2023-09-04/note-correspondents-western-sahara"),
    1159137019: ("Makhanda", None, "https://www.gov.za/news/media-statements/minister-nathi-mthethwa-announces-renaming-grahamstown-makhanda-02-jul-2018"),
    1159148713: ("Mafeteng", None, "https://www.gov.ls/government/comms-minister-tours-mohales-hoek-data-centre/"),
    1159117509: ("Mohale's Hoek", None, "https://www.gov.ls/government/comms-minister-tours-mohales-hoek-data-centre/"),
    1159136575: ("Pofadder", None, "https://www.justice.gov.za/eqcact/docs/2011eqc-a5-booklet.pdf"),
    1159148015: ("Vanrhynsdorp", None, "https://www.justice.gov.za/eqcact/docs/2011eqc-a5-booklet.pdf"),
    1159148069: ("Mthatha", None, "https://www.statssa.gov.za/?id=5198&page_id=4286"),
    1159138771: ("Otukpo", None, "https://bengis.benuestate.gov.ng/"),
    1159144921: ("Tlemcen", None, "https://www.mta.gov.dz/annuaire/?lang=fr"),
    1159112843: ("Sidi Bouzid", None, "https://www.tunisie.gov.tn/annuaireAdministration/134/89-%D8%A7%D9%84%D8%A5%D8%AF%D8%A7%D8%B1%D8%A9-%D8%A7%D9%84%D8%AC%D9%87%D9%88%D9%8A%D8%A9-%D9%84%D8%A3%D9%85%D9%84%D8%A7%D9%83-%D8%A7%D9%84%D8%AF%D9%88%D9%84%D8%A9-%D9%88-%D8%A7%D9%84%D8%B4%D8%A4%D9%88%D9%86-%D8%A7%D9%84%D8%B9%D9%82%D8%A7%D8%B1%D9%8A%D8%A9-%D8%A8%D8%B3%D9%8A%D8%AF%D9%8A-%D8%A8%D9%88%D8%B2%D9%8A%D8%AF.htm"),
    1159148257: (None, "Western Sahara", "https://www.un.org/geospatial/content/western-sahara-1"),
    1159139309: (None, "Western Sahara", "https://documents.un.org/doc/undoc/gen/n23/287/38/pdf/n2328738.pdf"),
    1159148209: (None, "Senegal", "https://whc.unesco.org/en/list/956"),
    1159145011: ("Tiaret", None, "https://www.interieur.gov.dz/index.php/fr/actualit%C3%A9s.html?start=100"),
    1159119553: ("Tougué", None, "https://assembleenationale.gov.gn/tougue-femmes/"),
    1159139283: (None, "Western Sahara", "https://www.geonames.org/2462188/lemsid.html"),
    1159139279: (None, "Western Sahara", "https://www.geonames.org/9645564/daoura.html"),
    1159137085: (None, "Kenya", "https://manderamunicipality.go.ke/"),
    1159139267: (None, "Angola", "https://ana.gov.ao/web/provincias-detalhes/5"),
    1159136625: (None, "Zambia", "https://www.luangwacouncil.gov.zm/wp-content/uploads/2023/12/LUANGWA-IDP-PSIR-FINAL-DRAFT-UPDATED-2.pdf"),
    1159142173: (None, "Cameroon", "https://www.moloundou.cm/"),
    1159137011: ("Kariega", None, "https://www.gov.za/news/media-statements/sport-arts-and-culture-transformation-our-heritage-landscape-25-feb-2021"),
    1159136819: ("Bekily", None, "https://www.assemblee-nationale.mg/wp-content/uploads/2019/11/Loi-n%C2%B02018-011-CTD-compress%C3%A9.pdf"),
    1159136779: ("Ubombo", None, "https://www.statssa.gov.za/?id=9379&page_id=4286"),
    1159141917: ("Okondja", None, "https://interieur.gouv.ga/decoupage-administratif/"),
    1159137033: ("Komani", None, "https://www.ecpgnc.gov.za/chris-hani-district/"),
    1159134855: (None, "South Africa", "https://www.sars.gov.za/customs-and-excise/customs-offices-and-contacts/border-posts/"),
    1159141895: ("Bifoun", None, "https://interieur.gouv.ga/decoupage-administratif/"),
    1159141681: ("Mitzic", None, "https://interieur.gouv.ga/decoupage-administratif/"),
    1159136699: ("Winnie Mandela", None, "https://www.dsac.gov.za/node/568"),
    1159122651: ("Aktau", None, "https://mangystau.invest.gov.kz/about/info/"),
    1159150965: ("Astana", None, "https://old.adilet.zan.kz/rus/docs/U2200001017"),
    1159146993: ("Tianmen", None, "https://www.tianmen.gov.cn/zjtm/zrdl/dlgk/t_1930189.shtml"),
    1159149169: ("Prayagraj", None, "https://prayagrajdivision.nic.in/about-department/introduction/"),
    1159134353: ("Istaravshan", None, "https://www.mvd.tj/617-istaravshan.html"),
    1159115163: ("Suphan Buri", None, "https://www.tourismthailand.org/Destinations/Provinces/SuphanBuri/239"),
    1159149161: ("Chhatrapati Sambhajinagar", None, "https://aurangabad.gov.in/en/public-utility/chhatrapati-sambhajinagar-mahanagarpalika/"),
    1159147203: ("Ahilyanagar", None, "https://gsda.maharashtra.gov.in/en-ahmednagar-district/"),
    1159144595: (None, "Armenia", "https://www.syunik.aero/"),
    1159139209: ("Hyesan", None, "https://www.fao.org/4/J0741e/J0741e00.htm"),
    1159147285: ("Kostanay", None, "https://www.gov.kz/memleket/entities/kostanay?lang=en"),
    1159127463: ("Mersin", None, "https://www.mersin.gov.tr/kurum-mevzuati"),
    1159147029: ("Zhaotong", None, "https://www.norad.no/contentassets/99da0412ef4840558333f12fad4fc6cb/zhaotong-capacity-building-project-final-evaluation-report.pdf"),
    1159116619: ("Puttalam", None, "https://puttalam.mc.gov.lk/"),
    1159134565: ("Lào Cai", None, "https://laocai.gov.vn/district-city-en/lao-cai-city-250856"),
    1159140881: ("Xingtai", None, "https://english.www.gov.cn/news/photos/202301/28/content_WS63d50f25c6d0a757729e618e.html"),
    1159140947: ("Binzhou", None, "https://english.www.gov.cn/news/202410/11/content_WS670880f5c6d0868f4e8ebafd.html"),
    1159130757: ("Az Zubayr", None, "https://iraq.un.org/sites/default/files/remote-resources/60e6334040986c5da1eb1c3877e0021a.pdf"),
    1159130429: ("Gurugram", None, "https://gurugram.gov.in/history/"),
    1159137977: ("Ormoc", None, "https://ormoc.gov.ph/pages/history.php"),
    1159139785: ("Pekalongan", None, "https://pekalongankota.go.id/"),
    1159139823: ("Prabumulih", None, "https://www.kotaprabumulih.go.id/"),
    1159147069: ("Yingkou", None, "https://swt.ln.gov.cn/commerce/investment_liaoning/2024122617164269217/index.shtml"),
    1159147155: ("Mudanjiang", None, "https://www.mdj.gov.cn/mdjsrmzf/c105809/202504/1002822/files/0a5193a53dda4358b10522b0f0327110.pdf"),
    1159139707: ("Ningde", None, "https://www.fujian.gov.cn/english/thisisfujian/regions/202605/t20260513_7147517.htm"),
    1159146571: ("Nizhnevartovsk", None, "https://www.n-vartovsk.ru/"),
    1159128937: ("Kupino", None, "https://econom.nso.ru/sites/econom.nso.ru/wodby_files/files/migrate/deyatelnost/territ_razv/kupinsky/02.pdf"),
    1159146661: ("Novy Uoyan", None, "https://sr2021.rzd.ru/download/full-reports/csr_en_annual-report_spreads_rzd_2021.pdf"),
    1159134155: (None, "Crimea", "https://www.un.org/sg/en/content/former-deputy-secretary-general/press-events/2014-03-05/deputy-secretary-generals-telephone-press-conference-kyiv-ukraine"),
    1159140617: ("Plzeň", None, "https://plzen.eu/"),
    1159114767: ("Alba Iulia", None, "https://www.apulum.ro/index.php/primaria/informatii"),
    1159114889: ("Miercurea-Ciuc", None, "https://miercureaciuc.ro/ro/ugyfelfogadas"),
    1159134205: ("Kropyvnytskyi", None, "https://www.president.gov.ua/en/news/u-kropivnickomu-volodimir-zelenskij-proviv-naradu-shodo-soci-88789"),
    1159122463: ("Ústí nad Labem", None, "https://www.usti.cz/cz/"),
    1159149325: ("Odesa", None, "https://odesa365.omr.gov.ua/en"),
    1159114867: ("Sfântu Gheorghe", None, "https://sfantugheorgheinfo.ro/contact?lang=ro"),
    1159114857: ("Râmnicu Vâlcea", None, "https://portal.primariavl.ro/"),
    1159114907: ("Piatra-Neamț", None, "https://consultari.primariapn.ro/"),
    1159142205: ("Ioannina", None, "https://ioannina.gr/"),
    1159119151: ("Komotini", None, "https://komotini.gr/en"),
    1159128249: ("Panevėžys", None, "https://www.panevezys.lt/lt/gyventojams/apie-panevezi/miesto-vizija-iki-q15b.html"),
    1159134151: (None, "Crimea", "https://ukraineun.org/en/press-center/87-statement-by-the-delegation-of-ukraine-at-the-26th-splos/"),
    1159147519: (None, "Crimea", "https://ukraineun.org/en/press-center/87-statement-by-the-delegation-of-ukraine-at-the-26th-splos/"),
    1159134145: (None, "Crimea", "https://ukraineun.org/en/press-center/87-statement-by-the-delegation-of-ukraine-at-the-26th-splos/"),
    1159134141: (None, "Crimea", "https://ppu.gov.ua/en/press-center/update-on-the-situation-in-the-temporarily-occupied-territory-of-the-autonomous-republic-of-crimea-and-the-city-of-sevastopol-as-of-january-26-2026/"),
    1159146449: ("Liepāja", None, "https://www.liepaja.lv/en"),
    1159126825: ("Chornomorsk", None, "https://ukrstat.gov.ua/druk/publicat/kat_u/2018/zb/08/Ukr_in_figeres_2017_e.pdf"),
    1159137647: ("Sol-Iletsk", None, "https://travel.orb.ru/sol-iletsk/?lang=en"),
    1159137203: ("Vyksa", None, "https://en.vyksaair.com/"),
    1159137361: ("Stary Oskol", None, "https://products.metalloinvest.com/upload/iblock/66b/66b05a2d6dd04ac86b4c91746925d1e8.pdf"),
    1159125419: ("Bartlett", None, "https://www.cityofbartlett.org/"),
    1159148851: ("Grande Prairie", None, "https://cityofgp.com/"),
    1159131443: ("Délı̨nę", None, "https://www.ece.gov.nt.ca/en/services/geographical-names/official-nwt-community-names"),
    1159147257: ("Qeqertarsuaq", None, "https://kommuneplania.qeqertalik.gl/en/plans-for-towns-and-settlements/qeqertarsuaq/"),
    1159138363: ("Navojoa", None, "https://www.navojoa.gob.mx/"),
    1159138371: ("Mazatán", None, "https://boletinoficial.sonora.gob.mx/boletin/images/boletinesPdf/2019/06/2019CCIII44III.pdf"),
    1159117637: ("Santa Tecla", None, "https://www.asamblea.gob.sv/sites/default/files/documents/decretos/171117_073502281_archivo_documento_legislativo.pdf"),
    1159122697: ("San Ignacio", None, "https://www.sisetc.gov.bz/history.php"),
    1159134025: ("Kotlik", None, "https://www.commerce.alaska.gov/web/dcra/ResiliencePlanningLandManagement/EVCs"),
    1159147649: ("Ulukhaktok", None, "https://www.gov.nt.ca/careers/en/ulukhaktok"),
    1159148867: ("Kinngait", None, "https://www.gov.nu.ca/en/communities/kinngait"),
    1159131427: ("Sanirajak", None, "https://www.gov.nu.ca/en/communities/sanirajak"),
    1159151731: ("Wānaka", "New Zealand", "https://www.newzealand.com/sg/lake-wanaka/"),
    1159147761: ("Leonora", None, "https://www.leonora.wa.gov.au/locals/our-community/leonora-township.aspx"),
    1159141881: ("Nadi", None, "https://www.fiji.travel/places-to-go/nadi"),
    1159151737: (None, "New Zealand", "https://www.cic.govt.nz/your-council/visit-the-chatham-islands"),
    1159131943: ("Doctor Pedro P. Peña", "Paraguay", "https://www.ande.gov.py/documentos/proyectos/101602/codigo_de_localidades.pdf"),
    1159131921: ("Presidencia Roque Sáenz Peña", None, "https://saenzpena.gob.ar/historia"),
    1159131117: ("Guaíra", None, "https://www.ibge.gov.br/cidades-e-estados/pr/guaira.html"),
    1159136049: ("Mayor Pablo Lagerenza", None, "https://mdn.gov.py/energia-para-impulsar-el-desarrollo-del-chaco/"),
    1159122009: ("San Juan de los Morros", None, "https://loeu.opsu.gob.ve/detalle-localidad/1078/"),
    1159143455: ("Campo Mourão", None, "https://www.ibge.gov.br/biblioteca/visualizacao/dtb/parana/campomourao.pdf"),
    1159147607: ("Chapecó", None, "https://www.ibge.gov.br/cidades-e-estados/sc/chapeco.html"),
    1159131639: ("Comandante Luis Piedrabuena", None, "https://www.argentina.gob.ar/normativa/nacional/norma-241644"),
    1159135991: (None, "Ecuador", "https://www.arcotel.gob.ec/claro-reporto-afectacion-del-servicio-brindado-en-la-poblacion-de-nuevo-rocafuerte-canton-aguarico-provincia-de-orellana/"),
    1159131899: ("Yacuiba", "Bolivia", "https://www.aduana.gob.bo/sites/default/files/etds/ETD%202023%20-%20Yacuiba.pdf"),
    1159144265: ("Sierra Colorada", None, "https://rionegro.gov.ar/articulo/61247/ya-tiene-fecha-la-inauguracion-de-la-nueva-red-de-gas-en-sierra-colorada"),
    1159147589: ("Guajará-Mirim", None, "https://www.ibge.gov.br/cidades-e-estados/ro/guajara-mirim.html"),
    1159130593: ("Villarrica", None, "https://www.munivillarrica.cl/historia/"),
    1159143383: ("Juína", None, "https://juina.mt.gov.br/servicos"),
    1159132115: ("Jaboatão", None, "https://jaboatao.pe.gov.br/jaboatao-dos-guararapes"),
    1159144269: ("Villa Mercedes", None, "https://villamercedes.gov.ar/ficha-tecnica/villa-mercedes-historica"),
}

CITY_REGION_CORRECTIONS = {
    1159125063: ("Missouri", "https://www.cityofcapegirardeau.org/"),
    1159143923: ("Ontario", "https://www.pembroke.ca/"),
    1159150231: ("Northwest Territories", "https://www.fortsmith.ca/contact-us"),
    1159151737: ("Chatham Islands", "https://www.cic.govt.nz/your-council/visit-the-chatham-islands"),
    1159148899: ("Amapá", "https://oiapoque.portal.ap.gov.br/pagina/municipio/historia"),
    1159131775: ("Buenos Aires", "https://www.argentina.gob.ar/obras-publicas/ex-fideicomiso-procrear-predio-mercedes"),
    1159131943: ("Boquerón", "https://www.ande.gov.py/documentos/proyectos/101602/codigo_de_localidades.pdf"),
    1159135991: ("Orellana", "https://www.arcotel.gob.ec/claro-reporto-afectacion-del-servicio-brindado-en-la-poblacion-de-nuevo-rocafuerte-canton-aguarico-provincia-de-orellana/"),
    1159131899: ("Tarija", "https://www.aduana.gob.bo/sites/default/files/etds/ETD%202023%20-%20Yacuiba.pdf"),
}

# These are stations despite the source's generic populated-place classification.
CITY_STATIONS = {
    1159147255: "https://dce.au.dk/en/current/news/news/artikel/aarhus-university-builds-research-station-in-north-greenland",
    1159147247: "https://www.dmi.dk/fileadmin/Rapporter/TR/tr12-15.pdf",
}

def source_features(name, source_dir):
    data = ((source_dir / name).read_bytes() if source_dir else
            urllib.request.urlopen(BASE + name, timeout=60).read())
    if hashlib.sha256(data).hexdigest() != SOURCES[name]:
        raise ValueError(f"Source checksum mismatch: {name}")
    return json.loads(data)["features"]


def source(name, source_dir):
    return [feature["properties"] for feature in source_features(name, source_dir)]


def continent(place, countries):
    code = place["ADM0_A3"]
    explicit = {"GIB": "Europe", "SJM": "Europe", "SSD": "Africa",
                "MUS": "Africa", "SYC": "Africa", "MDV": "Asia", "TKL": "Oceania",
                "SGS": "South America"}
    result = explicit.get(code) or countries[code]["CONTINENT"]
    # Split overseas places and transcontinental countries by place, not sovereignty.
    if code == "RUS":
        result = "Asia" if place["LONGITUDE"] >= 60 or place["LONGITUDE"] < 0 else "Europe"
        if place["NAME"] == "Vorkuta":
            result = "Europe"
        elif place["NAME"] == "Nizhny Tagil":
            result = "Asia"
    elif code == "TUR" and (place["NAME"] == "Istanbul"
                            or place["ADM1NAME"] in {"Edirne", "Kirklareli", "Tekirdag"}):
        result = "Europe"
    elif code == "USA" and place["ADM1NAME"] == "Hawaii":
        result = "Oceania"
    elif code == "IDN" and place["ADM1NAME"] in {"Papua", "Irian Jaya Barat"}:
        result = "Oceania"
    elif code == "FRA":
        if place["ADM1NAME"] in {"Guinaa", "Guyane"} or place["NAME"] == "Saint-Georges":
            result = "South America"
        elif place["ADM1NAME"] in {"Guadeloupe", "Martinique"}:
            result = "North America"
        elif place["ADM1NAME"] in {"La Réunion", "Moyotte"}:
            result = "Africa"
    elif code == "ESP" and (place["NAME"] in {"Las Palmas", "Arrecife", "Santa Cruz de Tenerife"}
                            or place["ADM1NAME"] in {"Ceuta", "Melilla"}):
        result = "Africa"
    return result.lower().replace(" ", "-")


def write(relative, records):
    if not records or any(set(r) != {"id", "text", "provenance"}
                          or any(not isinstance(v, str) or not v.strip() or "\0" in v
                                 for v in r.values()) for r in records):
        raise ValueError(f"Invalid records: {relative}")
    if len({r["id"] for r in records}) != len(records):
        raise ValueError(f"Duplicate record ID: {relative}")
    if len({r["text"] for r in records}) != len(records):
        raise ValueError(f"Duplicate stimulus: {relative}")
    path = ROOT / relative / "records.json"
    path.parent.mkdir(parents=True, exist_ok=True)
    records.sort(key=lambda record: (record["text"].casefold(), record["id"]))
    path.write_text(json.dumps(records, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"{relative}: {len(records)}")


def cities(source_dir):
    places = source("ne_10m_populated_places.geojson", source_dir)
    countries = {p["ADM0_A3"]: p for p in source("ne_50m_admin_0_countries.geojson", source_dir)}
    # Omit the mislocated Niamey row and repeated Bandar Lampung / Kasserine / Aktau
    # source identities; coordinates and spelling differ, but these aren't new towns.
    places = [p for p in places if p["FEATURECLA"] not in {"Scientific station", "Historic place", "Meteorological Station"}
              and p["NE_ID"] not in {1159149391, 1159139819, 1159133873, 1159148679}
              and p["NE_ID"] not in CITY_STATIONS]
    duplicates = Counter((p["NAME"], p["ADM0NAME"]) for p in places)
    grouped = defaultdict(list)
    for p in places:
        name = p["NAME_EN"] if p["NAME"] in ENGLISH_LABELS else p["NAME"]
        name = " ".join(name.replace("\u200e", "").replace("\u200f", "").split())
        country = COUNTRY_LABELS.get(p["ADM0NAME"], p["ADM0NAME"])
        region = None
        correction = ""
        if p["NE_ID"] in CITY_CORRECTIONS:
            fixed_name, fixed_country, reference = CITY_CORRECTIONS[p["NE_ID"]]
            name = fixed_name or name
            country = fixed_country or country
            correction += f" Geographic name/location corrected against {reference} (2026-10-13); no source prose copied."
        if p["ADM0_A3"] in {"USA", "CAN"} or duplicates[(p["NAME"], p["ADM0NAME"])] > 1:
            region = p["ADM1NAME"]
        if p["NE_ID"] in CITY_REGION_CORRECTIONS:
            region, reference = CITY_REGION_CORRECTIONS[p["NE_ID"]]
            correction += f" Region corrected against {reference} (2026-10-13); no source prose copied."
        if p["ADM0_A3"] == "FRA" and p["ADM1NAME"] in {"Guinaa", "Guadeloupe", "Martinique", "La Réunion", "Moyotte"}:
            region = {"Guinaa": "French Guiana", "La Réunion": "Réunion", "Moyotte": "Mayotte"}.get(p["ADM1NAME"], p["ADM1NAME"])
        if p["ADM0_A3"] == "FRA" and p["NAME"] == "Saint-Georges":
            region = "French Guiana"
            correction += " Region corrected against https://www.banatic.interieur.gouv.fr/commune/97308-Saint-Georges (2026-10-13); no source prose copied."
        if p["ADM0_A3"] == "ESP" and p["NAME"] in {"Las Palmas", "Arrecife", "Santa Cruz de Tenerife"}:
            region = "Canary Islands"
        if p["NAME"] == "Sevastopol":
            country = "Crimea"
        if p["NAME"] == "Washington,  D.C.":
            region = None
        text = f"{name} - {region + ', ' if region else ''}{country}"
        classification = p["FEATURECLA"]
        if classification in {"Admin-0 capital", "Admin-0 capital alt"}:
            kind = "national-centres"
        elif classification in {"Admin-0 region capital", "Admin-1 capital", "Admin-1 region capital"}:
            kind = "administrative-centres"
        elif classification in {"Populated place", "Populated Place"}:
            kind = "settlements"
        else:
            raise ValueError(f"Unexpected settlement classification: {classification}")
        grouped[f"{continent(p, countries)}/{kind}"].append({
            "id": f"geography-city-ne-{p['NE_ID']}", "text": text,
            "provenance": CITY_PROVENANCE.format(id=p["NE_ID"]) + correction,
        })
    records = [record for values in grouped.values() for record in values]
    if len(records) != 7289 or len({r["id"] for r in records}) != 7289:
        raise ValueError("Unexpected source count or duplicate Natural Earth ID")
    # Duplicate formatted names expose source errors that require explicit review.
    duplicates = [text for text, count in Counter(r["text"] for r in records).items() if count > 1]
    if duplicates:
        raise ValueError(f"Duplicate city stimulus: {duplicates}")
    for group, values in sorted(grouped.items()):
        write(f"cities/{group}", values)
    # A continent is now a branch; remove only its obsolete flat corpus.
    for region in {group.split("/")[0] for group in grouped}:
        (ROOT / "cities" / region / "records.json").unlink(missing_ok=True)


def pinned_provenance(filename, identifier):
    return (f"Natural Earth, source ID {identifier}; {BASE}{filename}; "
            "public domain: https://www.naturalearthdata.com/about/terms-of-use/. "
            "Reproducible curated transformation; see geography/README.md.")


def slug(text):
    if text == "Seven seas (open ocean)":
        return "ocean-islands"
    return text.lower().replace(" ", "-")


def administrative_geography(source_dir):
    """Country map units and mapped administrative units; no sovereignty claim."""
    country_source = "ne_50m_admin_0_countries.geojson"
    countries = source(country_source, source_dir)
    by_code = {p["ADM0_A3"]: p for p in countries}
    grouped = defaultdict(list)
    for p in countries:
        if p["ADMIN"] == "Antarctica":
            continue
        group = slug(p["CONTINENT"])
        name = COUNTRY_LABELS.get(p["ADMIN"], p["ADMIN"])
        region = p["SUBREGION"]
        if region == "Seven seas (open ocean)":
            region = "ocean islands"
        grouped[group].append({
            "id": f"geography-country-ne-{p['NE_ID']}",
            "text": f"{name} - {region}",
            "provenance": pinned_provenance(country_source, p["NE_ID"]),
        })
    for group, records in sorted(grouped.items()):
        write(f"countries-and-territories/{group}", records)
    filename = "ne_10m_admin_1_states_provinces.geojson"
    grouped = defaultdict(list)
    units = source(filename, source_dir)
    ambiguous = Counter((p["name"], p["type_en"], p["admin"]) for p in units)
    for p in units:
        name = p["name"]
        if not name or not name.strip():
            continue
        if p["adm0_a3"] not in by_code or p["admin"] == "Antarctica":
            continue
        # Some duplicate names describe different units with demonstrably wrong
        # source names/types. Do not invent a correction from coordinates alone.
        if ambiguous[(name, p["type_en"], p["admin"])] > 1:
            continue
        country = COUNTRY_LABELS.get(p["admin"], p["admin"])
        group = slug(by_code[p["adm0_a3"]]["CONTINENT"])
        kind = p["type_en"] or "Administrative unit"
        grouped[group].append({
            "id": f"geography-subdivision-ne-{p['ne_id']}",
            "text": f"{name} - {kind.lower()}, {country}",
            "provenance": pinned_provenance(filename, p["adm1_code"])
                          + f" Natural Earth NE_ID {p['ne_id']}; mapped admin-1 snapshot, not a claim of present-day legal level.",
        })
    for group, records in sorted(grouped.items()):
        write(f"subdivisions/{group}", records)
        (ROOT / "subdivisions" / group / "node.toml").write_text(
            f"weight = {len(records)}\n", encoding="utf-8")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-dir", type=Path, help="Read pinned GeoJSON files locally instead of downloading")
    args = parser.parse_args()
    cities(args.source_dir)
    administrative_geography(args.source_dir)
    import runpy
    runpy.run_path(str(ROOT / "import_physical.py"))["import_physical"](args.source_dir)


if __name__ == "__main__":
    main()
