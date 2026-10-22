# Amphibian species profiles

212 species across 72 GBIF families: 157 frogs and toads (Anura), 28 salamanders
(Caudata), and 27 caecilians (Gymnophiona). All five original names remain.
Selection uses the shared importer: five 1,000-row pages under accepted GBIF
class Amphibia (key 131), accepted species with English common names, a stable
hash order, and a cap of three per family plus original-name retention. This is
a bounded diversity sample, not a complete checklist. Weight equals record count.

203 exact scientific binomials match AmphiBIO v1; 202 records have at least one
usable trait and 178 have maximum adult size. Nine names lack exact matches;
one matched row has no usable traits. These ten records retain taxonomy only.
No synonym, fuzzy-name, family-name, or inferred trait joins are used. GBIF
classification and the older AmphiBIO classification can differ; matching order
is checked because it determines the length measurement.

## Sources and definitions

- [GBIF Backbone Taxonomy](https://www.gbif.org/dataset/d7dddbf4-2cf0-4f39-9b2a-bb099caae36c),
  GBIF Secretariat, CC BY 4.0. `sources.json` pins every retrieved API response.
- [AmphiBIO v1, Figshare version 5](https://doi.org/10.6084/m9.figshare.4644424.v5),
  Oliveira, São-Pedro, Santos-Barrera, Penone and Costa (2017), CC BY 4.0.
  `trait-sources.json` pins the versioned API metadata, archive, and every archive
  member, including `Metadata.pdf`; it also records both stable source IDs for
  each enriched species. The downloaded CSV contains 6,776 distinct species rows.

Definitions come from the archive's metadata. Adult size is a reported maximum,
in millimeters: snout-to-vent length in frogs and total length in salamanders and
caecilians. Maturity ages and maximum lifespan are years; clutch sizes count
offspring or eggs; reproductive output is the
maximum number of reproductive events per year. Paired maturity and clutch-size
bounds share one range; a lone bound keeps its minimum or maximum label.
Other fields report positive
source categories for foraging strata, adult diet, activity and breeding strategy.
Foraging strata omit seasonal and developmental detail; adult diet may originate
from observation, stomach contents or specialist judgment in the cited literature.
Missing values and binary zeroes are omitted; omission does not assert absence.
Body mass is excluded because sample review found implausible source values
(for example, 15 g for the 15-mm Brachycephalus hermogenesi). Other selected
values are transcribed, without biological corrections or new precision. Source
maxima are compiled observations, not claims about every individual or population.

Both source attributions appear in provenance. The complete CC BY 4.0 notice
is inherited from `../../LICENSE.txt`.

## Rebuild and review

```powershell
python -B families/natural-world/animals/vertebrates/amphibians/enrich_data.py
python -B families/natural-world/animals/vertebrates/amphibians/enrich_data.py --replay
```

Raw files live outside the sampling tree in ignored
`target/import-cache/amphibians/`; taxonomy responses are in its `taxonomy/`
directory. First rebuild needs network access if cached files are missing.
Replay uses only cached bytes and rejects changed hashes, including source
metadata. Pin manifests must accompany the cache for reproducibility. The frozen
shared importer provides taxonomy selection; the leaf importer enriches its
selected records without changing selection.

Review checked unique IDs and texts, required record fields, positive size
measurements, numeric traits, binary coding, order agreement, retention of all
original names, and samples across the three orders. A cached rebuild reproduced
`records.json`, `node.toml`, `sources.json` and `trait-sources.json` byte for byte.
