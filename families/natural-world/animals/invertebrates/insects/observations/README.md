# Insect study observations

59 observations across 28 source-listed families, selected from 218 eligible
species in [AnimalTraits v1.0.7](https://doi.org/10.5281/zenodo.6468938)
(McLean, 2022; CC0 1.0). The 3,580-row source CSV and column documentation
are pinned to revision `06c872793e83d448538a12f6f98dbad389a86703`; exact URLs,
encodings and SHA-256 hashes are in [sources.json](sources.json).

Each species contributes its richest single study observation with at least two
traits; the first source row breaks ties. A stable ID-hash selection caps each
family at three species. These are sampled individuals or groups, not species
maxima or universal constants. Order and family labels follow the dataset and
may be historical; this collection is not a current taxonomic backbone.

Body mass is converted from kg to g, retaining reported ranges. Metabolic values
retain the source's conversion to W and normalization to 25 °C using Q10=2;
original temperature, sex, sample size and measurement type remain when supplied.
Brain size retains original mass or volume units and explicit multipliers, without
assuming tissue density. Measurements use three significant figures.

Documented spelling repairs are `Reticlitermes` → `Reticulitermes`,
`Orthoperta` → `Orthoptera`, `Aeschnidae` → `Aeshnidae`, and
`Tettigonildae` → `Tettigoniidae`; evidence accompanies affected records and the
manifest. `Lygus kalmii` and `Multareoides bifuractus` are omitted because their
source identities remain unverified; no replacement identity is inferred.

Rebuild from the pinned cache outside `families/`:

```sh
python -B families/natural-world/animals/invertebrates/import_animaltraits.py --group insects --output families/natural-world/animals/invertebrates/insects/observations --per-family 3 --replay
```

Use `--cache-dir` for an alternate cache. Review checked every record against its
CSV row, independently recalculated all 54 metabolic conversions, and checked the
five brain observations and two body-mass ranges. Cached replay preserves payload,
weight and manifest byte-for-byte. Corrupting either pinned input is rejected
before those files change. Leaf weight equals the record count.
