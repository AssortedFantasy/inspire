# Formula and structure relationships

`composition/empirical-formulas` contains 17 ordinary molecular/empirical formula
comparisons and four shared-composition groups. The latter replace repetitive
reductions with named molecules that have the same element ratios at different
molecular sizes, including distinct structures at the same size. Source formulas
are grouped by exact atom ratios, independently checked by cross multiplication;
no random molecule counts inflate the output space.

`isomers/constitutional` has 15 comparisons of source structures sharing a formula,
including three-member groups. `isomers/stereochemical` has 26 pairs distinguishing
geometry, chirality and epimerism. Open-chain aldose pairs use complete source R/S
assignments on identical backbones. Cyclic sugar records with unspecified anomers
are excluded.

All three corpora regenerate offline with `python -B families/chemistry/import_isomers.py`.
Add `--check` to verify source pins, structural relationships and emitted records
without rewriting them. Provenance retains PubChem CIDs and the relevant manifests.
