# Objects

`artifacts/` contains 4,427 physical objects from the Cleveland Museum of Art's
CC0 Open Access catalog. Function, form, and material define 54 separate leaves:
vessels subdivide into ceramic, glass, and metal, then into drinking vessels,
pouring vessels, storage vessels, vases, bowls, plates, or serving vessels.
Furnishings, adornment, textiles, equipment, arms, and ritual objects have their
own children. Jewelry separates body ornaments, clothing fasteners, head ornaments,
necklaces, pendants, beads, and complete sets; those branches separate individual
forms such as finger rings, earrings, brooches, buckles, and fibulae.
The catalog also supplies 183 coins, 80 fiber baskets and related vessels, and
32 pieces of writing equipment, including inkstones, brush holders and writing
boxes. Coin face records yield to complete parent accessions, and electrotype
replicas are excluded.
A cup does not acquire a random material, culture, or date: every
record describes one actual catalog object, retaining its stable museum ID.

Emitted text preserves the catalog's title, date, culture, and technique.
Museum credit remains in provenance and the bundled source notice.
Uncertainty, alternative origins, historical place names, and date ranges remain
as catalogued. Records contain catalog metadata only.
The catalog's `culture` field can include place, period, dynasty, or attribution;
it is labelled literally rather than presented as a precise ethnic identity.
Only metadata is embedded; no images, interpretive essays, or ownership histories.

`import_data.py` reproducibly transforms the pinned official dataset:

```sh
python -B families/objects/import_data.py
python -B families/objects/import_data.py --check
```

The default cache is `target/imports/cleveland`; `--cache` accepts another directory.
The importer verifies SHA-256 before processing. It excludes missing factual
fields, unclassified object forms, catalogued forgeries, fragments, covers, and
separately catalogued numeric or letter components when an eligible parent
accession is present. Excluded generic amulet aggregates yield to their more
informative named children. Reviewed catalog-type mistakes route complete watches
to timepieces, protective ornaments to amulets, an inkstand to writing equipment,
and an ear stud to ear ornaments.
Objects lacking evidence for their jewelry form, modern replacement beads, and
records containing internal editorial notes are excluded. Exact
title/culture/date/technique duplicates collapse to the smallest stable museum ID.
Each leaf is capped at 300 objects, selected by round-robin across the first
component of the source culture field, with IDs ascending within each component.
This keeps a large porcelain or lace collection from filling every catalog slot.
The selection is still a museum collection, not a representative survey of all
world objects or cultures. Individual armor components remain useful objects;
weapon guards, hilts, and scabbards are excluded from the weapons leaf.

The source is [Cleveland Museum of Art Open Access](https://github.com/ClevelandMuseumArt/openaccess)
revision `d8dd744bc29e7986df428341133c14e18eb56b3f`, downloaded from its
[pinned Git LFS CSV](https://media.githubusercontent.com/media/ClevelandMuseumArt/openaccess/d8dd744bc29e7986df428341133c14e18eb56b3f/data.csv).
Input SHA-256: `137ddfab3264555d52c0211f804544cab1c62170ce3d753d924c9d464d74e449`.
Metadata reuse terms are CC0 1.0, retained in `artifacts/LICENSE` for
`--data-licenses`. Object URLs remain in record provenance.

Within artifacts, child weights match descendant record counts. Every catalog
object has the same share, so a singleton jewelry form cannot dominate a large
collection through a short browsing path. Cars likewise retain equal per-record
probability within their branch. The incoming artifacts/cars shares remain equal.

`cars/` contains 73 documented historical make, model and year combinations, grouped
by manufacturer origin. Nissan's branch further separates the Bluebird family,
compacts and sports cars. Manufacturer origin does not imply an individual car's assembly
location or sales market.
Within `cars`, local weights follow descendant record counts so each catalogued
vehicle has the same share, including within Nissan's extra grouping level.

Years are kept with their documented meaning in each record's provenance:
heritage collection vehicle year, calendar-year sales introduction or explicit
model year. They are not interchangeable across markets. A generation's unveiling
date is not treated as its sales date. Model, trim and chassis codes are retained
when they identify the catalogued vehicle. Prototypes and competition-only
variants are excluded. No record independently combines a model with a random year.
Sampled labels contain the year, make, and model without repeating year caveats.
The source review removed six competition-only Nissan entries and two entries
whose model or trim could not be established reliably from their catalog records.

Sources are manufacturer heritage catalogs, historical accounts and releases.
These are compact factual labels, not reproductions of source prose or images.
The corpus is selective and does not list every offered year, trim or market.
