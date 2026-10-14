# Reptile species and trait profiles

237 English-named, accepted, non-extinct-flagged GBIF species across 87 families;
219 have exact scientific-binomial joins to the downloaded ReptTraits workbook.
Provenance and bundled notices credit GBIF and ReptTraits.
Both sources use CC BY 4.0; the full notice is in `../../LICENSE.txt`.

The traditional reptile group is represented by GBIF's accepted classes
Squamata, Testudines, Crocodylia and Sphenodontia. GBIF key 358, formerly
Reptilia, is a pro-parte synonym with no descendants and cannot provide the
sampling scope. Lineage labels preserve the downloaded classification rather
than silently replacing those classes with historical orders.

The shared importer retrieves at most five 1,000-row pages per class and chooses
up to three species per family in deterministic ID-hash order. Original seed
names can add records above that cap. Targeted searches recover rubber boa,
prairie rattlesnake and bullsnake. All seven original animals are represented:
bullsnake is an English GBIF alias of *Pituophis catenifer*, displayed as Gopher
Snake; other names may similarly use the source's preferred English variant.
This is a diverse bounded sample, not a complete reptile inventory.

ReptTraits source: Oskyrko, Mi, Meiri and Du,
[Figshare article version 4](https://doi.org/10.6084/m9.figshare.24572683.v4),
file `ReptTraits dataset v1-2.xlsx` (file ID 45408133). The workbook contains
12,060 species, trait definitions, references and per-cell source citations.
v1-2 corrects microhabitat and habitat entries for several snake groups.

| Emitted source-defined trait | Species |
| --- | ---: |
| Reproductive mode | 214 |
| Activity period | 139 |
| Diet category | 122 |
| Maximum total length, mm | 83 |
| Maximum reported longevity, years | 74 |
| Maximum straight carapace length, mm | 21 |

Missing cells and cells without individual citations are omitted. No fuzzy
names, synonym joins, inferred traits or inferred unit conversions are used.
Longevity is a reported maximum and may include captive observations. Categories
are literal source classifications, not universal claims about every individual.
18 species remain taxonomy-only because their exact names or usable trait cells
are absent from this source version.

Quality review excluded the entire microhabitat column after finding
*Crocodylus porosus* categorized as arboreal/saxicolous/terrestrial. It also
excluded all non-turtle SVL and sex-specific lengths after finding
*Sphenodon punctatus* at 800 mm SVL, apparently a total-length value. Turtle
straight carapace length stays separate from total length. Body mass is omitted:
the unusually precise heterogeneous entries were not independently validated as
measured maxima. Broad numeric validity bounds and categorical allowlists are
documented in the importer; they do not repair individual source errors.

## Rebuild

From the repository root, with Python 3 and `openpyxl` installed:

```sh
python -B families/natural-world/animals/vertebrates/reptiles/enrich_data.py
python -B families/natural-world/animals/vertebrates/reptiles/enrich_data.py --replay
```

`sources.json` pins taxonomy responses; `trait_sources.json` pins the actual
Figshare metadata and workbook, retains used definitions, per-species citations
and exclusions. Raw inputs stay outside the sampling tree under
`target/import-cache/reptiles/`; taxonomy responses occupy its
`inspire-animal-taxonomy/` directory. `--replay` uses only those cached bytes,
rejects changed SHA-256 inputs, and reproduces the four payload/manifest files
byte for byte. Published workbook MD5 is checked as well.

Spot checks covered a crocodile, turtle, lizard, snakes and tuatara. The source's
taxonomic name mismatches are retained as missing coverage rather than corrected
without evidence (for example GBIF *Crocodylus johnsoni* versus a different
ReptTraits spelling).
