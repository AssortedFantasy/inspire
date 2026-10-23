# Food

9,380 records combine 7,793 actual food profiles from USDA FoodData Central's
April 2018 SR Legacy release with 1,587 names from the CC0 Corpora collection.
The nutrition branch has weight 2 and catalogues weight 1. Nutrition categories
use record-count weights, giving every food profile the same conditional share.
Catalogue leaves also use record counts, except the thousand apple cultivars
have incoming weight 100 to keep them from dominating ordinary food names.

Profiles preserve the source description, including raw, cooked, canned or
branded preparation. Energy, protein, fat and carbohydrate appear per 100 g
edible portion; fibre and sodium appear when provided. Missing values remain
missing. Nutrients use source units and values with numeric trailing zeros
removed. Values are historical compositional data, not current product labels.
The source includes analyses, calculations and published literature; output
does not imply every listed nutrient was directly measured in the same sample.
See the [USDA documentation](https://fdc.nal.usda.gov/data-documentation/).

Catalogue leaves contain herbs, spices, seasoning mixtures, fruits, vegetables,
breads, pastries, condiments, apple cultivars and cocktail names. These are
literal source labels, not invented recipes or descriptions. Topic membership
follows the source's broad culinary usage, including botanical fruits eaten as
vegetables and overlapping regional names. Duplicate labels go to the first
listed topic; cultivar names gain the noun `apple`. The cocktail list is a pinned
historical vocabulary and makes no claim about current IBA recognition.

## Rebuild

```sh
python -B scripts/import_catalogues.py --family food --download
python -B scripts/import_catalogues.py --family food --check
```

The standard-library importer reads the ZIP tables directly, joins by FDC ID
and nutrient ID, checks source units and rejects repeated IDs or amounts.
All downloads are checksum-pinned in `scripts/catalogue_sources.json`.
Caches live under `target/imports/{usda,corpora}`. `--download` fetches missing
files only; changed upstream content fails checksum validation.

Sources: [USDA SR Legacy download](https://fdc.nal.usda.gov/download-datasets/)
and [Corpora](https://github.com/dariusk/corpora/tree/2e7adec11a0561696c236ae1fca82e88be173d42/data/foods),
revision `2e7adec11a0561696c236ae1fca82e88be173d42`. Record provenance retains
the source FDC ID or original list/index. Reuse notices are bundled in `LICENSE.md`.
