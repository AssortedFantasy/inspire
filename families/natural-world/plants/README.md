# Plants

713 accepted species from actual USDA NRCS PLANTS profiles, divided first into
vascular and nonvascular plants. Vascular plants separate flowering plants,
gymnosperms, and spore-bearing groups. Flowering plants separate woody forms,
forbs, graminoids, and climbers; ferns, lycophytes, horsetails, and whisk ferns
remain distinct. Nonvascular plants separate mosses, liverworts, and hornworts.

Records give a scientific binomial, a common name where supplied, botanical
family, recorded growth forms and life cycles, and up to two explicitly named
native-status jurisdictions. They retain the USDA taxonomy. "Native" means
native somewhere within that USDA jurisdiction; populations elsewhere in that
jurisdiction may be introduced. The North America status is used only for
nonvascular plants. These botanical profiles describe recorded traits and
statuses. Missing traits remain absent. Generic common names
that repeat a printed botanical group or growth form are printed only once.

Sampling within plants is uniform per retained species: every child weight is
its descendant record count. The incoming plants weight remains the default
topic share. This prevents a tiny botanical clade from receiving as much total
probability as hundreds of flowering species. The import is deliberately
family-diverse rather than a representative abundance survey.

## Source and reuse

- [USDA PLANTS checklist](https://plants.sc.egov.usda.gov/DocumentLibrary/Txt/plantlst.txt),
  downloaded 13 October 2026; SHA256
  `2a08310dfc2fd33dd441e935ed169ab680bcea367e7bb5fb6c11d450d0c0d0bf`.
- [Profile API](https://plantsservices.sc.egov.usda.gov/api/PlantProfile?symbol=EQTE).
  `sources.json` pins every retained species' stable USDA symbol, exact URL,
  and original response SHA256. The API does not offer a versioned snapshot.
- [PLANTS Help, Conditions of Image and Data Use, page 3](https://plants.usda.gov/assets/docs/PLANTS_Help_Document_2022.pdf)
  states that the plant information is not copyrighted and free for any use,
  and requests citation. The database citation is bundled in `--data-licenses`
  and retained in provenance. No images, plant guides, or external prose are used.
- [Federal dataset metadata](https://catalog.data.gov/dataset/plant-list-of-accepted-nomenclature-taxonomy-and-symbols-plants-database)
  identifies the database as CC0.

The source selection accepts named binomials, omits synonyms, hybrids and
infraspecific taxa, and normally samples two distinct genera per botanical
family using a stable SHA256 symbol order. The selection adds up to 40 grass,
20 sedge and 10 rush genera. Eleven smaller spore-plant families allow up to
12 species each, including species without vernacular names, so a one-genus
clade is not reduced to a constant output. Cached lichen and algal profiles are
excluded. 72 flowering profiles without growth-form metadata are excluded
rather than given invented traits. 713 retained species span 361 families.
All source growth forms are printed. For browsing, vine takes precedence over
graminoid, then woody growth forms, then herbaceous growth; ambiguous woody
herbs therefore retain their original multiple forms rather than losing detail.

## Reproduce

Raw responses remain in a cache outside the sampling tree. Python's `-B`
prevents accidental cache directories becoming sampling nodes.

```sh
python -B families/natural-world/plants/import_data.py --cache <cache> --download
python -B families/natural-world/plants/import_data.py --cache <cache> --check
```

The importer checks source bytes, accepted identity, species rank, strict
binomials, one botanical family, known source enums, unique IDs, complete
payload fields, and exact output/weight reproduction. It fails on changed live
responses rather than silently rewriting pinned data. To deliberately select
and review a new snapshot, run `prepare_sources.py --cache <cache>` with `-B`,
then the importer. Use a new cache to request fresh checklist and profile bytes.
