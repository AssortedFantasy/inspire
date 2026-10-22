# Mammal species

English-named species from the pinned GBIF Backbone Taxonomy, balanced by
taxonomic family with a deterministic hash-based cap. Taxonomy, known habitat
categories and names come from downloaded API records; this frozen backbone
is not a statement of current taxonomic consensus. Fossil taxa marked extinct
are excluded. Absence of an extinction flag does not certify living status.

The current import selects 425 species from 13,000 downloaded taxonomy rows,
covering 156 families and 29 orders. Each family contributes at most three
hash-selected records plus retained original seed names. Original species
remain represented under source-preferred names; the former generic "Pocket
gopher" name is replaced by precise species-level gopher profiles.

Available traits are joined by **exact scientific binomial** to Jones et al.'s
[PanTHERIA 1.0 WR05, August 2008](https://esapubs.org/archive/ecol/E090/184/),
a 5,416-row species dataset. The selected columns contain adult body mass,
gestation, litter size and maximum recorded longevity. These are source
species estimates, which can incorporate model adjustment and literature
extrapolation, rather than measurements of a particular animal. Missing
values (`-999`) are omitted; no imputation, synonym remapping or EXT columns
are used. Numbers are rounded to three significant figures; longevity of at least
12 source months is shown in years. Maximum recorded
longevity can include captivity and is not life expectancy.

312 profiles have at least one selected trait: 308 body masses, 193 gestation
estimates, 249 litter sizes and 174 maximum longevity records. 46 scientific
binomials have no exact WR05 match; other unenriched profiles lack values in
these four columns. `traits-sources.json` records coverage and unmatched names.

Names with changed species concepts can remain unmatched even when a related
older name exists; matching species names cannot eliminate every change in
taxonomic circumscription since 2008. Traits do not describe all populations,
sexes or individuals. Provenance identifies the source and vintage.

GBIF and the current Ecological Archives default are CC BY 4.0. PanTHERIA's
dataset-specific metadata additionally states no copyright or proprietary
restrictions. Attribution is retained in provenance; `LICENSE.txt` records
the source-specific terms. Raw downloads stay outside the sampling tree.
`sources.json` and `traits-sources.json` pin exact URLs and SHA-256 checksums.

From the repository root:

```sh
python families/natural-world/animals/import_data.py --taxon Mammalia --output families/natural-world/animals/vertebrates/mammals --pages 13 --per-family 3
python families/natural-world/animals/vertebrates/mammals/enrich_data.py
```

Replay both commands with `--replay` using the original outside-tree caches.
For an alternate trait cache, add `--cache /path/outside/families` to the second
command. Replaying the taxonomy alone replaces the enriched output; run the
trait enrichment afterwards. Enrichment is idempotent and checks source count,
unique scientific names, input hashes and finite positive trait values.
