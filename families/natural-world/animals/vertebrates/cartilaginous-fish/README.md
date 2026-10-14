# Cartilaginous fish species summaries

40 English-named species from 21 GBIF families and 11 orders, including sharks,
skates, stingrays, electric rays, a sawfish, and the Cape elephantfish chimaera.
Every record has a genuine species-level life-history value, rather than an
unrelated taxonomy-only row. Family selection is capped at three species,
preferring more complete unambiguous traits and then a stable hash order.

## Sources and interpretation

- [Hutchings et al., 2012 supplement 1, version 1](https://doi.org/10.6084/m9.figshare.3517361.v1)
  supplies compiled species-specific maximum weight (kg), interbirth interval
  (years), and age at maturity (years). Figshare explicitly licenses this table
  CC0. Its metadata defines the columns; the publisher's MD5 verifies the TSV.
- [GBIF Backbone Taxonomy](https://www.gbif.org/dataset/d7dddbf4-2cf0-4f39-9b2a-bb099caae36c)
  supplies names, classification, and recorded habitat categories under CC BY 4.0.
  GBIF classes Elasmobranchii (121) and Holocephali (120) are independently checked
  for exact accepted names, Animalia membership, rank CLASS, and absence of deletion.
  Four and one 1,000-result pages, respectively, cover the pinned search results.
  Deleted species, synonyms, non-binomial names, unnamed species, and source-flagged
  extinct species are excluded. These source flags are not independent living-status
  certification; none of the retained species is a known extinct species.
- [ICES CM 2001/J:11](https://www.ices.dk/sites/pub/CM%20Doccuments/2001/J/J1101.pdf)
  was downloaded as an independent numeric-plausibility check. Its Table 1 records
  female leafscale gulper shark maturity at 44 years. This supports plausibility
  of late maturity, without replacing the compilation's separate 46-year value.

Joining uses only exact accepted canonical binomials after replacing underscores
in source names. No fuzzy match, synonym migration, or imputation is used. The
table contains repeated species rows; a trait is omitted if its nonmissing values
disagree. Annual mortality, growth rates, and the confusingly labeled maximum
litter-size column are excluded. Values are attributed to the historical compilation,
with an explicit population/sex limitation; they are not universal physiological
constants or claims of current taxonomic consensus. Coverage is deliberately bounded:
35 records have maximum weight, 35 maturity age, and all 40 interbirth interval.

Source-listed clear names are preferred for little skate, kitefin shark, western
stingaree, and small-spotted catshark. This avoids misleading generic or obscure
first-ranked GBIF vernaculars. Required GBIF attribution is included in each output;
full CC BY 4.0 and CC0 notices are inherited from `../../LICENSE.txt`.

## Reproduce and review

```powershell
python -B families/natural-world/animals/vertebrates/cartilaginous-fish/import_data.py
python -B families/natural-world/animals/vertebrates/cartilaginous-fish/import_data.py --replay
```

Raw input stays in ignored `target/import-cache/cartilaginous-fish`, outside the
sampling tree. `sources.json` pins exact URLs, SHA-256 hashes, version, license,
selection rules, and rejected-trait counts. Downloads use Windows system TLS without
certificate bypass. Replay verifies every pinned cache input before producing output
and compares all three generated files byte for byte without rewriting them.

Schema, IDs, exact binomials, family cap, all emitted profiles, and representative
shark/ray/chimaera numeric values were reviewed. The node weight equals its count;
ancestor record-count weights are maintained by the parent branch importer.
