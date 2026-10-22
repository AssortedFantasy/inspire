# Bony fishes

Named, accepted species in a finite set of 36 GBIF Backbone fish orders,
including ray-finned fishes, lungfishes and coelacanths. This is a breadth sample,
not a complete fish inventory. The importer rejects synonyms, higher-rank
fallback matches, deleted order nodes, species flagged extinct and unnamed species.
It uses orders because GBIF's historical Actinopterygii and Sarcopterygii class
nodes were deleted; the latter name also cannot safely delimit fish from
tetrapods. Cichliformes does not match this backbone snapshot; its cichlid
species appear under the accepted Perciformes source classification.
The branch separates two kinds of source evidence:

- [Species](species/README.md): **599 species in 229 families**, with **289**
  compiled maximum body-length profiles.
- [Image measurements](image-measurements/README.md): **287** measurements
  associated with source-selected photographs or drawings: **278** body ratios
  and **9** caudal-fin/peduncle ratios for flatfishes lacking a body ratio.

The branch weight is its **886** descendant records; child weights are their
record counts. Species and depiction measurements have different stable IDs.

The first API page (at most 1,000 rows) is downloaded per order. Named species are
grouped by source family, capped at three per family. Exact, unique scientific
binomial matches to FISHMORPH take priority, then SHA-256 of the stable GBIF ID
breaks ties. The taxonomy has no asserted class when the source lacks one.
Taxonomy and English-name coverage are bounded by the downloaded pages and
GBIF's constituent datasets; common names and classifications are source
claims rather than a current taxonomic consensus.
Chinese paddlefish (`Psephurus gladius`) is additionally excluded despite its
missing GBIF extinction flag, following the downloaded and pinned
[primary extinction study](https://pubmed.ncbi.nlm.nih.gov/31911255/).
Other absent extinction flags are not a guarantee that a species survives.

## Traits and reuse

The downloaded author-deposited [FISHMORPH v1.1](https://rodare.hzdr.de/record/1316)
contains 8,342 rows. Its README retains an older 10,230-species description;
we validate the actual v1.1 file count, publisher MD5 checksums, metadata license,
and SHA-256 pins. Attribution: Brosse, Charpin, Su, Toussaint, Herrera-R,
Tedesco and Villéger (2021), doi:10.14278/rodare.1316; associated paper
doi:10.1111/geb.13395. Both FISHMORPH and GBIF Backbone are CC BY 4.0; the
complete license is inherited from `../../LICENSE.txt`.

Only exact binomial joins are used, without synonym guessing or imputation.
`MBl` is maximum body length in **centimetres**, compiled by FISHMORPH from
FishBase (Froese & Pauly 2020). The source does not specify standard, total or
fork length; provenance keeps that limitation and the compilation vintage.
These values are not measurements of the illustrations or current maxima.

The [primary paper](https://aureletoussaint.github.io/homepage/data/2021_Brosse_GEB.pdf),
sections 2.1.2 and 2.2.2, explains that ImageJ pixel measurements were taken
from one selected lateral depiction per species. Section 4 states that the
dataset does not account for variation within species. `BEl` divides body
length (Figure 1: standard length, snout to caudal-fin basis) by maximum body
depth. `CPt` divides maximum caudal-fin depth by minimum caudal-peduncle depth.
Both are dimensionless ratios. Drawings and photographs remain explicitly
distinguished; records identify the species association and source citation,
without asserting an identified individual, sex, invariant species proportion
or behavioral interpretation.

Of 291 exact matched source rows, 13 have missing `BEl`. Nine retain positive,
defined `CPt` and appear in the image child using that measurement. Four image
records are omitted: three tongue soles have zero-coded `CPt`, and starry
flounder has missing image ratios. No missing/zero code is turned into a
physical measurement. These species remain in the species child.
Illustration references and source attribution remain in provenance; images and
the paper are not redistributed. Children
inherit the full CC BY 4.0 license from `../../../LICENSE.txt`.

## Reproduce

From the repository root:

```powershell
python -B families/natural-world/animals/vertebrates/bony-fish/import_data.py
python -B families/natural-world/animals/vertebrates/bony-fish/import_data.py --replay
```

Inputs are cached under ignored `target/import-cache/bony-fish/`, outside the
sampling tree. `sources.json` pins every GBIF response, including accepted
order validation. `traits-sources.json` pins the trait CSV, metadata, units
README, primary methods paper and extinction-study metadata. These manifests
and `import_data.py` are shared by both children, which reference their parent
paths in provenance. Replay regenerates both child payloads and weights without
downloads, and rejects missing or changed inputs before writing outputs.
On Windows, the initial trait download uses PowerShell's system certificate
trust with normal TLS verification. Runtime uses only embedded records.
