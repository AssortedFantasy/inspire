# Geography

Names and physical features with geographic context, imported from public-domain
Natural Earth data. Runtime remains fully offline.

| Branch | Design | Records |
| --- | --- | ---: |
| `cities` | Six continents, then national centres, administrative centres, and other settlements | 7,289 |
| `subdivisions` | Continental groups of mapped administrative units; country and type accompany every name | 4,550 |
| `countries-and-territories` | Continental groups and ocean islands, with source subregions | 241 |
| `physical` | Land and water, then distinct kinds of relief, landscape, coasts, islands, and inland/marine water | 3,765 |

This replaces 1,280 names across nine leaves. Mountains and ranges are separate;
natural lakes and reservoirs are separate; islands and archipelagos are separate.
Settlements retain three times the incoming weight of each other immediate
geography branch. Within each of those four branches, child weights match
descendant record counts. Each settlement, subdivision, country/territory, or
physical feature has the same conditional share as its peers, regardless of its
browsing depth. Small capital, island, or landform collections cannot make a few
names recur disproportionately.

## Sources and reproducibility

All source GeoJSON files are pinned to Natural Earth repository commit
`f1890d9f152c896d250a77557a5751a93d494776`. [The source repository](https://github.com/nvkelso/natural-earth-vector/tree/f1890d9f152c896d250a77557a5751a93d494776/geojson)
and individual exact URLs appear in every record's provenance. SHA-256 checksums
are verified by [the importer](import_data.py); physical transformations live in
[their separate importer](import_physical.py), with
[source audits and detailed concept notes](physical/README.md). Records retain Natural Earth
`NE_ID`; administrative provenance also preserves `adm1_code`. River records
preserve the source `dissolve` identifier, which includes its map river number.

Settlements use 1:10m populated places v5.1.2, rather than the former 1:50m
selection. Administrative units and physical features use 1:10m datasets;
country/territory map units use the pinned 1:50m admin-0 dataset. These are
cartographic selections, not exhaustive geographic catalogues.
Natural Earth declares [all its map data public domain](https://www.naturalearthdata.com/about/terms-of-use/),
with no permission or attribution required. No population figures, GDP estimates,
or descriptive source prose are imported. [LICENSE.md](LICENSE.md) records the
data reuse notice.

```sh
python -B families/geography/import_data.py
```

For an offline rebuild, download files listed in `SOURCES` to a directory outside
`families/` and pass `--source-dir PATH`. Raw downloads, caches, and temporary
scripts must stay outside the sampling tree.
`-B` prevents Python from creating invalid cache nodes there.

## Scope and quality

Settlement leaves contain towns and other inhabited places and do not assert
legal city status. Their source-derived centre groups are dated classifications,
including former capitals and erroneous capital flags; emitted names make no
claim about current political roles. All 1,209 original settlement IDs remain in
the expanded corpus; 123 outputs carry primary-source-backed corrections.
Scientific, historic, and meteorological stations, two misclassified stations,
and demonstrated mistaken identities are filtered. Explicit,
primary-source-verified name and country corrections are recorded in
`CITY_CORRECTIONS`; alternate English labels are not blindly substituted. See
[the settlement design](cities/README.md).

The administrative dataset contains mapped admin-1 units at mixed legal levels
(for example French departments and British local authorities). It is not a
definitive list of contemporary first-level subdivisions. Seven unnamed units,
16 units absent from the country-map grouping (including military bases and
small offshore map units), Antarctica's single unit, and all 22 rows with
ambiguous repeated name/type/country combinations are excluded. Some repeated
names revealed source errors; they are omitted rather than renamed from
coordinates alone. Political labels and administrative boundaries remain those
of the pinned source and may be dated. Common country/territory names normalize
[North Macedonia](https://www.un.org/en/node/123222),
[Czechia](https://mzv.gov.cz/jnp/en/czechia.html), and
[Serbia](https://www.mfa.rs/en/republic-serbia), and
[Macao](https://www.macaotourism.gov.mo/en/article/about-macao/location-and-time)
against primary references.

North Macedonia's 84 pinned features follow the older municipality framework;
72 incorrectly carry Natural Earth's `Statistical Region` type. The
[official regional and municipal breakdown](https://www.stat.gov.mk/publikacii/2020/Regionite-Regions-2020.pdf)
supports their municipal classification. Drugovo, Oslomej, Vraneštica and Zajas
are labeled former municipalities following the
[statistical office's merger note](https://makstat.stat.gov.mk/PXWeb/pxweb/en/MakStat/MakStat__Naselenie__Vitalna/685_VitStat_Umr_Pol_ops_ml.px/).
The feature mislabeled Skopje (`NE_ID 1159314409`) has explicit Petrovec municipal
identifiers; its name is corrected against [Petrovec's municipal profile](https://petrovec.gov.mk/profil/).
All feature IDs and dated geometry remain intact. The eight actual statistical
regions are not separate features in this snapshot.

Country and continent labels follow source mapping conventions, not statements
of recognition or territorial claims. Ocean-island grouping comes from Natural
Earth's `Seven seas (open ocean)` category. Settlement grouping separately uses
physical-location overrides for overseas places and coarse transcontinental
boundaries, including Russian settlements across the dateline and Turkey's
East Thrace settlements; it is a sampling aid, not precise continent geometry.

Physical imports keep named features in explicitly mapped classes, omit unnamed
features, and suppress repeated source IDs. Generic marine areas, lake
centerlines, historical/pluvial lakes, unnamed elevation marks, stations, and
unclear mixed polygon classes are excluded. The mixed peninsula/cape class
is accepted only when a source label explicitly identifies a peninsula. Eight
ambiguous desert labels are omitted rather than mislabelling mountains,
vegetation regions, salt pans, and broad geographic regions as deserts. This
includes [Caatinga](https://www.gov.br/insa/pt-br/semiarido-brasileiro),
[Soudah](https://www.visitsaudi.com/en/see-do/destinations/abha), and
[Makgadikgadi](https://www.botswanatourism.co.bw/index.php/explore/makgadikgadi-and-nxai-pans).
Coordinates for point features are
rounded to 0.1°; trailing zeroes are omitted. Inland-water locations are centers of
mapped bounding boxes, labelled accordingly; they are not centroids, sources,
mouths, navigational positions, or precise surveyed locations. Rivers with
absent geometry or identifiers are excluded.

## Path migrations

- `cities/<continent>` became `cities/<continent>/{national-centres,administrative-centres,settlements}`.
- The three manual subdivision leaves became continental source-imported leaves.
  Scope includes mapped districts and territories rather than only U.S. states,
  Canadian provinces/territories, and Australian mainland states/territories.
- `countries-and-territories` and `physical` are new geography branches.
