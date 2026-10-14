# Arachnid study observations

44 observations across 29 source-listed families and six orders, selected from
57 eligible species in [AnimalTraits v1.0.7](https://doi.org/10.5281/zenodo.6468938)
(McLean, 2022; CC0 1.0). These include spiders, mites, ticks and a whip spider.
Exact input URLs, encodings and hashes are in [sources.json](sources.json).
The full CC0 notice is in the animals branch's `LICENSE.txt`.

Each species contributes its richest single study observation with at least two
traits; the first source row breaks ties. A stable ID-hash selection caps each
source-listed family at three species. These values describe sampled individuals
or groups, not species maxima or universal constants. Source order/family names
may be historical; the corpus does not claim a current taxonomic backbone.

Body mass is converted from kg to g. Metabolic values retain source normalization
to 25 °C using Q10=2, with original temperature, sex, sample size and available
measurement type. Values use three significant figures. All 40 metabolic
conversions were independently recalculated: oxygen uses 20 J/ml; carbon dioxide
is divided by the reported respiratory quotient. Molar CO2 measurements use the
pinned R constants 44.01 g/mol and 1.977 kg/m³ before conversion to oxygen volume,
with mass-specific measurements multiplied by the same observation's body mass.
Original neural volumes retain their dimension and explicit multiplier; no brain
tissue density assumption is applied.

The four Quesada et al. (2011) neural measurements are labeled CNS neuropil-only
volumes: supra- and sub-oesophageal ganglia, excluding cortex. Each value was
checked against the N column of the primary paper's supplementary Table 2.
The Mysmenopsis tengellacompa record also notes that AnimalTraits lists a sample
size of one whereas that table reports three females; the source field is retained.

Three source spelling errors are corrected with evidence in record provenance:
`Pholcus phalangioideus` → `Pholcus phalangioides`, `Alaskozeres antarcticus` →
`Alaskozetes antarcticus`, and `Misumenoidea formosipes` → `Misumenoides formosipes`.
The Damon annulatipes observation retains the explicit 21 °C measurement column
and its source-normalized rate while noting that the metadata comment says 25 °C.
No measurement temperature is inferred to resolve that contradiction.

Rebuild from the complete pinned cache outside `families/`:

```sh
python -B families/natural-world/animals/invertebrates/import_animaltraits.py --group arachnids --output families/natural-world/animals/invertebrates/arachnids --per-family 3 --replay
```

Use `--cache-dir` for an alternate cache. All 44 outputs passed source, schema,
unit and context checks. Complete-cache replay with networking blocked reproduces
payload, weight and source manifest byte-for-byte. Corruption of either pinned
input and a missing cache are rejected before those files change. The review
ledger records the actual checks and hashes. Leaf weight equals the record count.
