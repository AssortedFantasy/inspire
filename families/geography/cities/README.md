# Settlements

7,289 geographic name records imported from pinned, public-domain Natural Earth
1:10m populated places v5.1.2. These include cities, towns, and small settlements;
they do not assert legal city status. See [source and maintenance notes](../README.md).

Each continent contains three source-based groups:

- `national-centres`: `Admin-0 capital` and `Admin-0 capital alt` source rows.
- `administrative-centres`: `Admin-0 region capital`, `Admin-1 capital`, and
  `Admin-1 region capital` source rows.
- `settlements`: other populated-place source rows.

These are dated source classifications, **not claims of current capital status**.
The source includes former capitals, alternate seats, and classification errors
(for example Johannesburg). The emitted material is a geographic name, without a
political-role claim. Source significance determines inclusion; this is not a
population ranking or an exhaustive gazetteer.

Child weights match descendant record counts, so every settlement has the same
share across continents and source groups. Record IDs retain Natural Earth's `NE_ID`, including all 1,209
IDs from the previous 1:50m corpus. The importer excludes
scientific and meteorological stations, historic places, a mislocated Niamey row,
and repeated Bandar Lampung, Kasserine, and Aktau source identities. Nord and
Timmiarmiut are excluded as stations despite their generic source classification.
Demonstrated name, country, and region errors use explicit primary-source
corrections in `CITY_CORRECTIONS` and `CITY_REGION_CORRECTIONS`.

Treat `NAME_EN` as an alternate, not an automatic correction: some entries name a
different place (for example Guaíra, Mazatlán, and Kuta). The curated corrections
retain precise primary-source references; review actual names as well as schema.

Rebuild using `python families/geography/import_data.py`; no network is needed at
runtime. Review new source revisions before changing the pinned input or filters.
