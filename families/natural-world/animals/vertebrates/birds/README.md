# Bird species profiles

453 accepted, English-named species in 167 GBIF families. Of these, 413 have
exact-name AVONET profiles containing species-average mass, beak culmen length,
unflattened wing length, primary habitat, diet category, and migration category.
The GBIF classification and AVONET ecological classification are named separately
because they come from different datasets and taxonomic snapshots.

## Sources and selection

- GBIF Backbone Taxonomy, GBIF Secretariat, dataset
  `d7dddbf4-2cf0-4f39-9b2a-bb099caae36c`, retrieved 2026-10-13, CC BY 4.0.
  The shared importer selects up to three species per family from the first
  5,000 API search results for Aves, plus exact common-name matches to the
  original seed collection. This is a bounded sample, not a bird checklist.
- AVONET: Tobias et al. (2022), *Ecology Letters*,
  [doi:10.1111/ele.13898](https://doi.org/10.1111/ele.13898),
  [Figshare version 7](https://doi.org/10.6084/m9.figshare.16586228.v7),
  CC BY 4.0. Downloaded supplementary dataset 1, file `34480856`;
  sheet `AVONET1_BirdLife` uses HBW–BirdLife taxonomy version 5 (2020).
  The workbook's Metadata sheet defines the units and ecological categories.

The enrichment importer also queries exact English common-name matches for
original seed birds missing from the bounded GBIF sample. All 29 original names
are retained, including names represented by an alternative primary common name.
These recovery queries add 12 records. `sources.json` describes the shared
importer's 441-record selection; `traits-sources.json` describes the final
453-record enriched corpus and pins the additional recovery queries.

Trait joins use an exact scientific binomial only. No fuzzy matches, synonym
substitution, or traits borrowed from relatives are used. Eleven matched species
are excluded from enrichment because AVONET marks at least some traits as
inferred; another 29 have no exact BirdLife-sheet binomial match. Their GBIF
profiles remain available. Taxonomic splits and historical combinations can
explain unmatched names; a match is not forced.

Mass is a species average in grams, incorporating male and female measurements.
Beak culmen length runs from the beak tip to the skull base in millimetres.
Wing length runs from the carpal joint to the longest primary on an unflattened
wing in millimetres; it is not wingspan. Morphology values are species averages,
rounded to three significant figures, not bounds on an individual. Ecological
category labels are retained from AVONET.
Migration codes map directly to sedentary, partially migratory, and migratory.
The source's definition of partial migration includes nomadic or altitudinal
movements. Adjacent metadata includes the original variable definitions.

## Reproduce

From the repository root, with Python and `openpyxl` installed:

```sh
python families/natural-world/animals/import_data.py --taxon Aves --output families/natural-world/animals/vertebrates/birds --pages 5 --per-family 3
python families/natural-world/animals/vertebrates/birds/enrich_data.py
```

Add `--replay` to both commands to use only pinned cached responses. The shared
GBIF cache is under the system temporary directory; the AVONET XLSX and article
metadata are under ignored `target/import-cache/birds`. Input checksums are
verified on replay. No runtime downloads or Python dependencies are introduced.
The inherited `animals/LICENSE.txt` contains CC BY 4.0; provenance identifies the exact species and dataset versions.
