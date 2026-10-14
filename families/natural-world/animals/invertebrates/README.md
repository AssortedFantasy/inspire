# Invertebrates

Biological groups occupy distinct children: insects, arachnids, crustaceans,
molluscs, cnidarians, echinoderms and annelids. Insects separate `species` names
from `observations`; molluscs separate `species` from `genus-traits`. These
children distinguish evidence kinds: a study observation is not a universal
species property, and a genus trait is not a species measurement. Arachnids
contain only study observations and need no extra single-child branch.
Weights equal record counts at every split within this subtree.

## Marine trait profiles

`import_cefas.py` reads Cefas' 1,025-row trait matrix V2, DOI
[10.14466/CefasDataHub.123](https://doi.org/10.14466/CefasDataHub.123), under
OGL v3.0. The source records trait information at genus level and above for
Northwest European marine benthos. It draws on published literature and expert
judgement, and a strong score can sometimes represent a supported inference
from relatives. Records therefore describe the source's regional genus-level
synthesis, without asserting universal traits for each constituent species.

The importer keeps explicit named genera, excludes sea spiders from crustaceans,
requires repeated rows in a genus to agree on retained confidence codes, and
keeps only code 3 (strong evidence) categories. No missing or zero-score entry
becomes a negative biological claim. The source's positively encoded
`b_None = 3` category is the exception in meaning: it explicitly records strong
evidence for no bioturbation. Body-size and longevity bins are not emitted
without independently verified unit definitions. The broad morphology code
`m_Exoskeleton` becomes "hard skeletal structures"
to avoid calling echinoderm endoskeletons external skeletons.

At most three profiles per family are selected by stable SHA-256 ordering of
genus IDs. Each record includes original source Aphia IDs and required Crown
copyright attribution. The pre-existing Giant Pacific octopus species record
has its own Smithsonian provenance and remains in the separate mollusc species
child, outside this regional synthesis.

The two pinned inputs are cached in `inspire-animal-traits` under the operating
system temporary directory. To replay a marine leaf, for example:

```sh
python -B families/natural-world/animals/invertebrates/import_cefas.py --group echinoderms --output families/natural-world/animals/invertebrates/echinoderms --per-family 3 --replay
```

Every input hash is fixed in the importer and repeated in leaf manifests.
`--replay` is offline and rejects missing or changed bytes. The API may change,
so a lost temporary cache requires review of an intentional snapshot upgrade.
`--cache-dir` selects an existing external cache explicitly when sandbox and
download contexts resolve different temporary directories.

## Terrestrial measured observations

[AnimalTraits v1.0.7](https://doi.org/10.5281/zenodo.6468938) supplies 3,580
study-linked observations, including 336 insect species and 70 arachnid species,
under CC0. `animaltraits_source.json` pins the exact repository revision and
SHA-256 hashes of observations and column documentation; their MD5 values also
match the archived Zenodo v1.0.7 files. Observation CSV is UTF-8, while the small
column-documentation CSV is Latin-1.

Measured values require study/sample context, original units, and available sex
and sample-size metadata. They are not species maxima or universal constants.
Body mass, metabolic rate and brain-size fields have distinct measurement units
and methods; standardized brain mass can be derived from measured volume, so a
volume observation must not be presented as a direct mass measurement. Leaf
importer notes document selection, numerical formatting and any source spelling
corrections. The full CC0 license is embedded alongside CC BY 4.0 at the animal
branch, and full OGL v3.0 accompanies this importer.

`import_animaltraits.py` chooses one study observation per species with at least
two recorded traits, preferring the richest observation and breaking ties by
source row order. It preserves explicit mass ranges, original brain mass versus
volume units, sex, sample size and ant caste when recorded. Body mass is
converted from kg to g; metabolic power explicitly describes the source's
normalization to 25 °C with Q10 = 2, alongside the original measurement
temperature. Numerical observations use three significant figures. Family caps
then spread the collection across taxa; existing NPS butterfly records remain
in the separate species child. Replay, for example:

```sh
python -B families/natural-world/animals/invertebrates/import_animaltraits.py --group insects --output families/natural-world/animals/invertebrates/insects/observations --per-family 3 --replay
```
