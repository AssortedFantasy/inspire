# Prepared fungal sources

No fungal payload has been added yet. The following downloaded sources are ready
for a reproducible import and distinct leaf reviews. Cache used during development:
`C:/Users/jehan/AppData/Local/Temp/inspire-natural-sources` (outside the tree).

## Funfun: measured traits and study metadata

Primary repository: <https://github.com/traitecoevo/fungaltraits>.
Pinned revision: `2a3671647afc82f101bb00b3ff3d99da0630388a`; R package version 0.0.3.
Citation requested by authors: Flores-Moreno et al. (2019), *fungaltraits aka
funfun: a dynamic functional trait database for the world's fungi*.

Exact inputs from
`https://raw.githubusercontent.com/traitecoevo/fungaltraits/2a3671647afc82f101bb00b3ff3d99da0630388a/`:

| Source filename | Cache filename | SHA-256 |
| --- | --- | --- |
| `funtothefun.csv` | `fungal-funfun.csv` | `6abbd9a370aadda6c23b6145bb738d832a6a5d57b25a584531aab3f3a4a8e10c` |
| `fttf.zip` | `fungal-metadata.zip` | `9c46dd2111f05f7708d40f0018356da8688bb1f7ff014e9dfc097582ae6ed64b` |
| `DESCRIPTION` | `fungal-funfun-DESCRIPTION` | `480258310b3a08d0ce62a176e9e0d5417afb398c572391f3b69b425e0f92b8a2` |
| `LICENSE` | `fungal-funfun-LICENSE` | `c76faf609384c53fd05bae1489f0705e3316ea6d5bbae765906f070028209201` |
| `README.md` | `fungal-funfun-README.md` | `699a99e656b57691bba2cea43dfc32450b4b6b960b49a1b1f837f971c96f38fa` |

The repository/package declares MIT plus its license file (2017, William
Cornwell). Retain the full applicable MIT notice and the dataset's requested
attribution; inspect the precise scope before asserting a separate data license.
This is not Põlme's similarly named FungalTraits dataset.

The CSV contains 51,555 long-form trait entries, 5,866 object IDs, and 3,104
matched taxon labels. Join traits by **obj_id**, not species name: different
studies or observations of the same name must retain their contexts. `studyName`,
matched scientific name, UUID, and Index Fungorum number identify the source.
Metadata ZIP includes `output/metadata/{dictionary,metadata,methods,references}.csv`.

Promising separate leaves:

- Spore dimensions: 690 published Bässler et al. (2015) profiles; length and width
  in micrometres. Source metadata describes mean dimensions and an ellipsoid
  volume calculation. Another 100 spore records come from unpublished Schlüssler
  material: do not silently merge them with the published measurements.
- Sac-fungus reproductive morphology: 361 Schoch et al. (2009) object profiles.
  `ascoma_type`, development, and ascus-release states are categorical; explain
  specialist terms from primary definitions. Ascomata are fruiting bodies.
- Ectomycorrhizal exploration: 162 Tedersoo/Smith (2013) rows. Codes C/SD/MD/LD
  have explicit definitions in the dictionary. Many labels end in `sp.` and are
  genus-level compilations; never call them precise species properties.
- Ecological guild assignments: 1,116 objects have FUNGuild annotations, of which
  386 are "Highly Probable". These are assignments with explicit confidence,
  not independent observations. Rank codes include 20/13/7; verify their primary
  meanings before translating them. Do not assume a genus assignment applies
  universally to every species or strain.

`fruiting_body_size` is **area in mm²**, not diameter. Metadata's tissue-N and
tissue-P descriptions contain repeated carbon-label text: variable meaning and
original studies must resolve these before using them. Missing/NULL values,
unresolved labels, conflicting object fields, unpublished sources, and source
category interpretations need explicit filtering or qualifications.

## USDA lichen-forming taxa

The prepared `usda-profiles/` cache includes 159 profiles with `Group=Lichen`;
`usda-profile-sources.json` contains their URLs and SHA-256 hashes. Plants
deliberately excluded them. These can form a separate factual lichen-taxon leaf
with common/scientific names, family, and qualified native status, preserving the
USDA acknowledgement and public factual-data terms already documented by plants.
Do not repeat USDA's historical Plantae ancestry as current fungal phylogeny.

## Optional Põlme FungalTraits genus table

Downloaded cache `fungal-data`, SHA-256
`e4e75c5bc4e915b3a1ab76de1ec22f7f29f10d705a1059942e36f58a3b09f3e6`, from
`polme2020-s1-fungal-traits-genera.csv` in
<https://github.com/globalbioticinteractions/fungaltraits>, revision
`edac5137e67e2a30b0ae18881248bcd14cf78f0a`.
The original paper <https://doi.org/10.1007/s13225-020-00466-2> states its attached
data are available for public use. Secondary packages call it CC BY 4.0, but
primary license scope must be checked before labeling it that way. Its table
includes fungus-like stramenopiles as well as fungi: these are different lineages
and must not be silently grouped as true fungi. This source is optional because
the measured Funfun corpus and lichen profiles already offer substantial breadth.
