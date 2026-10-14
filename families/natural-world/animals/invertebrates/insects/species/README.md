# Named insect species

66 source-grounded records: 63 GBIF names with classification and three retained
NPS butterfly names. The GBIF records span 19 orders and 45 families. All 25
original butterfly taxa remain represented exactly once; 22 receive source-backed
classification. This is a species-name collection, without invented descriptions,
range, conservation status, or behavior.

## Sources and selection

[GBIF Backbone Taxonomy](https://www.gbif.org/dataset/d7dddbf4-2cf0-4f39-9b2a-bb099caae36c),
GBIF Secretariat, CC BY 4.0. The legacy backbone is frozen and obsolete; its
classification is a pinned historical source, not current taxonomic consensus.
The full license is in [the animal branch](../../../LICENSE.txt), and emitted GBIF
records carry attribution.

The importer verifies [legacy class key 216](https://api.gbif.org/v1/species/216)
as canonical `Insecta`, accepted, in `Animalia`, and not deleted. A separate strict
`species/match?name=Insecta&strict=true&rank=CLASS&kingdom=Animalia` check returned
the same key on 2026-10-13. Searches explicitly use the legacy dataset and key.
Every exact request URL, response SHA-256, byte length, query, and retrieval count
is pinned in `sources.json` (48 responses retrieved 2026-10-13).

Each vernacular query reads at most its first 1,000 results. Accepted binomial
species require source class `Insecta`, class key 216, an order, a family, and a
usable English name. Deleted entries and entries flagged extinct are excluded;
absence of an extinct flag does not establish that a species is living. Generic
family-level labels and isolated scientific names are excluded. Two additional
distinct families per order are selected by SHA-256 ordering of stable GBIF IDs.
Explicit family coverage also preserves ants, bees, aphids, cicadas, damselflies,
and mosquitoes. Common-name and scientific-name duplicates are excluded.

Keys 1031743 and 1420243 are excluded because their supplied English labels
(minute pirate bug and leaf-miner fly) conflict with their own orders (Psocodea
and Thysanoptera). Coarse habitat categories are omitted because they can conflate
different insect life stages. Selection never invents a replacement common name.

`legacy-records.json` preserves the exact original 25-record NPS corpus (SHA-256
`d914ed86f6b2b74b4260a955b2ec9afa734eae93d1cf56cf3f680fbf4fc166c8`). Its source is
[NPS's North Coast and Cascades butterfly inventory](https://www.nps.gov/articles/butterflies-of-the-north-coast-and-cascades.htm).
Scientific-name or source English-name matches are enriched; the remaining brown
elfin, purplish copper, and western pine elfin retain their original records and
provenance. The archive is an ordinary file, not a second runtime corpus.

## Reproduce

Run from the repository root; raw downloads stay outside `families/`:

```powershell
python -B families/natural-world/animals/invertebrates/insects/species/import_data.py --cache-dir C:/Users/jehan/AppData/Local/Temp/inspire-animal-taxonomy
python -B families/natural-world/animals/invertebrates/insects/species/import_data.py --replay --cache-dir C:/Users/jehan/AppData/Local/Temp/inspire-animal-taxonomy
```

The first command downloads missing pinned inputs. Replay performs no network
requests and rejects missing or changed cached bytes before payload writes.
The local importer reuses the animal branch's name and attribution formatter.

Reviewed all emitted texts and validated schema, nonblank fields, NUL exclusion,
unique IDs/texts/scientific names, all 25 retained taxa, source classification,
English-name membership, and weight 66. Replay reproduced `records.json`,
`sources.json`, and `node.toml` byte for byte. A deliberately changed class response
in an isolated temporary cache failed its checksum before any payload changed.
