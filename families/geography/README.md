# Geography

Factual place names, printed as `City — Country` or
`City — State/Province, Country`. This branch has 1,280 records:

| Leaf | Records |
| --- | ---: |
| `cities/africa` | 226 |
| `cities/asia` | 399 |
| `cities/europe` | 172 |
| `cities/north-america` | 220 |
| `cities/oceania` | 66 |
| `cities/south-america` | 126 |
| `subdivisions/us-states` | 50 |
| `subdivisions/canada-provinces-and-territories` | 13 |
| `subdivisions/australia-states-and-mainland-territories` | 8 |

The city leaves include towns and other inhabited settlements; they do not
assert legal city status. Natural Earth's selection emphasizes regional
significance, rather than being a population ranking or exhaustive gazetteer.
Normal sibling weights apply, so continent leaves receive equal shares when
sampling `geography/cities`, regardless of their record counts.
Sampling `geography` gives cities three-quarters of calls and subdivisions
one-quarter. Within subdivisions, each name has the same share: the three
country lists have weights 50, 13, and 8.

## City sources and maintenance

Imported from [Natural Earth 1:50m populated places v5.1.2](https://www.naturalearthdata.com/downloads/50m-cultural-vectors/50m-populated-places/).
Continental grouping starts with its matching admin-0 country dataset.
Both files are pinned to repository commit
`f1890d9f152c896d250a77557a5751a93d494776`; the importer verifies SHA-256
checksums. Natural Earth declares its map data
[public domain, with no permission or attribution required](https://www.naturalearthdata.com/about/terms-of-use/).
Each record keeps the exact source URL and stable Natural Earth `NE_ID`.
Only names are imported; descriptions, population estimates, and geometry
are not stimuli.

The 1,251 source rows become 1,209 records after excluding 40 Antarctic
scientific stations, the historic place Ambarchik, and a second Niamey row
whose coordinates and region instead indicate Maradi. A small explicit list
uses the source's alternate English labels to fix misspellings and ambiguous
abbreviations. The importer deliberately does not use `NAME_EN` wholesale:
some alternatives incorrectly name a different city (for example Cuttack,
Oujda, and Mazatlán). U.S. and Canadian records carry states/provinces;
two distinct Suzhous and two distinct Yulins carry Chinese provinces.
Fort Smith's province is corrected to Northwest Territories against the
[town's own address](https://www.fortsmith.ca/contact-us).

Continents group physical locations, including French overseas settlements,
Hawaii, Jayapura, the Canary Islands, Mauritius, Seychelles, and Maldives.
Russia uses a coarse 60°E split, with Vorkuta in Europe and Nizhny Tagil in
Asia. Istanbul and Magnitogorsk span the Europe/Asia boundary and are grouped
in Europe. These categories are sampling aids, not precise continental
boundary geometry. Country/territory labels mostly follow Natural Earth's
conventions with readable aliases; Sevastopol uses the regional label
`Crimea`. Labels do not express recognition of sovereignty or territorial
claims. The dated source may retain older names or administrative conventions.

Rebuild these corpora with Python's standard library:

```sh
python families/geography/import_data.py
```

This downloads the two pinned source files into memory. For an offline import,
place `ne_50m_populated_places.geojson` and `ne_50m_admin_0_countries.geojson`
outside the sampling tree and pass `--source-dir PATH`. The generated records
remain embedded at build time; runtime never downloads data or runs Python.

## Subdivision sources

The subdivision leaves are independently assembled factual name lists,
checked against these official references on 2026-10-13:

- [U.S. Census Bureau ANSI codes for states](https://www.census.gov/library/reference/code-lists/ansi/ansi-codes-for-states.html):
  all 50 states, excluding the District of Columbia and territories.
- [Government of Canada province and territory abbreviations](https://our-languages.canada.ca/en/writing-tips-plus/abbreviations-canadian-provinces-and-territories):
  all ten provinces and three territories.
- [Australian Bureau of Statistics ASGS state/territory structure](https://www.abs.gov.au/statistics/standards/australian-statistical-geography-standard-asgs/edition-3-july-2021-june-2026/main-structure-and-greater-capital-city-statistical-areas/australia-and-stateterritory):
  six states and two mainland territories; external territories and the
  statistical `Other Territories` aggregate are outside this leaf's scope.

These records contain independently formatted geographic name facts, not
copied prose, descriptions, or page layouts. No blanket public-domain claim
is made for the reference websites. Canada's
[website terms](https://www.canada.ca/en/transparency/terms.html) distinguish
noncommercial reproduction from commercial redistribution; the Census
Bureau's [public-access policy](https://www2.census.gov/foia/ds_policies/ds027.pdf)
describes the U.S. copyright status of employee-created data and cautions
that foreign copyright rules can differ. ABS's
[website terms](https://www.abs.gov.au/website-privacy-copyright-and-disclaimer)
generally license its material under CC BY 4.0, subject to listed exceptions.
The factual name lists do not reproduce those sites' expressive material.
