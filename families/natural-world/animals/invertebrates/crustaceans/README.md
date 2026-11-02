# Crustacean genus traits

181 regional genus profiles from [Cefas trait matrix V2](https://doi.org/10.14466/CefasDataHub.123),
selected from 204 named crustacean genera in 123 source family groups: 122 resolved
families and one unresolved placement, `Paratanaoidea incertae sedis`. Four source
classes and 16 named orders are represented; two genera have no source order.
Crabs, shrimps, amphipods, isopods, cumaceans, mysids, tanaids, ostracods, barnacles
and parasitic copepods provide distinct ecological profiles. Six sea-spider rows
are excluded because Pycnogonida are chelicerates.

The [primary methods paper](https://www.nature.com/articles/s41597-022-01442-y)
describes genus-level and higher syntheses for Northwest Europe, mainly the UK
shelf, compiled from literature and professional judgement. Strong scores can
include supported inference from relatives. Taxonomy follows the source's WoRMS
snapshot of 21 January 2022. These profiles report the source's categories;
they are not species measurements or universal biological assertions. The source
does not identify the original supporting reference for each taxon-trait entry.
Multiple categories can reflect species or environmental differences.

Only confidence code 3 is emitted, with at least two retained categories per
genus. Zero and missing scores never become absence claims. Three profiles
explicitly retain positive `b_None = 3` evidence for no bioturbation. Body-size
and longevity bins are omitted; morphology `m_Exoskeleton` is conservatively
rendered as "hard skeletal structures". The unresolved placement is labeled
honestly rather than presented as an established family. At most three genera
per source family group are selected by SHA-256 order of stable genus IDs;
repeated rows agree on every trait code before deduplication. Original Aphia IDs
remain in provenance. Crown attribution and full OGL v3.0 are
embedded at the parent branch.

Replay from the repository root uses the two pinned inputs in an external cache:

```sh
python -B families/natural-world/animals/invertebrates/import_cefas.py --group crustaceans --output families/natural-world/animals/invertebrates/crustaceans --per-family 3 --replay --cache-dir C:/Users/jehan/AppData/Local/Temp/inspire-animal-traits
```

`sources.json` pins the CSV and metadata URLs, exact byte counts and hashes.
All 181 emitted profiles were reviewed and matched to source classifications,
positive confidence codes and Aphia IDs; independent family selection agrees.
Replay with networking blocked reproduces all output files byte-for-byte.
Corrupting or removing either pinned input fails before output mutation.
