# Content audit — 2026-10-13

Reviewed the directory tree, all generator implementations, emitted samples, and
corpus wording for category fit, repetition, local probabilities, and instructions
that could be confused with the consuming agent's task. This is an editorial
audit, not a fresh verification of every factual record against its source.

## Corrections

- Seventeen mathematical construction records used commands such as "Construct",
  "Set", or "Complete". They now describe the object and its relations. The tensor
  construction states its commutative-ring assumption and explicit scalar
  relations, consistent with [Stacks Project §10.12](https://stacks.math.columbia.edu/tag/00CV).
- Four poetic-form descriptions gave composition instructions. They now describe
  the form. Grammar examples, idiom examples, and proverbs are quoted and labeled.
  Actual quotations retain their attribution; intrinsic procedure steps remain
  inside named algorithms, kitchen/sewing procedures, or experiment protocols.
- Outfits previously ended with "Use ... for the bag or hat accent". Each now
  names an actual colored accessory. Sudoku states its intrinsic rule
  descriptively; reaction outputs label consumption and production amounts.
- Empirical formulas previously alphabetized elements, producing `H3N` for
  ammonia. Reduction now preserves the catalog's conventional order (`NH3`).
- General counterillumination and bioluminescence records moved from invertebrate
  adaptations to ocean phenomena. The material spans marine taxa, including
  fish; see [NOAA's bioluminescence explanation](https://oceanexplorer.noaa.gov/ocean-fact/bioluminescence/).
- Chess was registered as a generator despite having just 21 fixed outputs. It
  is now a corpus at the same CLI path. The dashboard stops after visiting each
  opening once rather than offering an endless repeated stream. The existing
  lines and labels are preserved; legal replay tests read the JSON itself.

## Sampling policy

Equal sibling weights were mathematically correct but made organizational
folders determine exposure. For example, each inventor was twenty times as
likely as each physicist because six inventors occupied one top-level people
branch, while 24 physicists shared a five-child science branch.

| Collection | Previous accidental per-record advantage | Current policy |
| --- | --- | --- |
| People | Inventor vs physicist: 20× | Each of 132 people has equal share within `people` |
| Cars | BMW vs Porsche: 4× | Each of 81 vehicles has equal share within `objects/cars` |
| Animal names | Amphibian vs mammal: 7.8× | Each of 106 names has equal share within `animals` |
| Subdivisions | Australian vs U.S. entry: 6.25× | Each of 71 names has equal share within `subdivisions` |

These collections use descendant record counts as local weights at every split.
The probability test checks actual conditional record probabilities, including
extra folder levels. Counts must be maintained when records change.

Broad top-level families have weight 2. Music (one historical chart), objects
(only cars), and random values have weight 1. Each broad family receives 7.4074%
of default calls; each narrow family receives 3.7037%. Colors and dates have
weight 2 against UUID's 1, reducing bare UUIDs from 2.2222% to 0.7407% of calls.
Geography gives cities three times the share of subdivisions. City continents
remain equally weighted to keep the larger Asian collection from dominating.
Other branches retain equal topic shares.

These are transparent editorial choices, not measured optimal weights for agent
creativity. Uniform records are appropriate for homogeneous name lists; uniform
topics remain useful for different forms of material. Neither makes the source
coverage representative of the world.

## Validation and remaining limitations

The initial pass sampled seeds 0–99 from each of the 14 former generator leaves
(1,400 outputs), inspected representative outputs, and scanned all 2,113 former
corpus records for normalized exact duplicate texts; none were found. Chess
produced exactly 21 distinct outputs, motivating its conversion. The collection
now contains 2,134 records in 67 corpora and 13 generators.

Generator invariants cover algebraic equalities, integral answers and domains,
balanced chemical reactions, circuit connectivity, legal game histories, unique
Sudoku solutions, valid dates, color syntax, and UUID version/variant bits.
The existing dashboard test checks complete, terminating shuffled passes for
every corpus, including the new chess corpus. Build-tree, CLI, and legal chess
replay tests cover weighting, standalone operation, and corpus conversion.
All 62 tests passed, as did Cargo and generator formatting checks, Clippy with
warnings denied, and the locked release build. Windows fixture canonicalization
required running the checks outside the restricted sandbox.

Coverage is still narrow in several places:

- Music contains only 30 entries from the 1959 year-end chart; objects only cars.
- Literature is public-domain English material, and all ten verse records are
  Shakespeare sonnet openings. A small dialogue corpus repeats six excerpts.
- People favor Nobel-associated scientists and writers. Equal person weights
  remove folder bias but retain that imbalance in the selection itself.
- Animal names largely come from North American inventories; 25 of 26
  invertebrate names are butterflies. Adaptations cluster around a few zoo
  species, and equal topic shares intentionally give small leaves more exposure
  per record. There is no plant branch or fish-name branch.
- Chemistry favors small organic molecules and complete-combustion equations.
  Circuit and integral generators have bounded templates; changing numbers is
  not the same as adding new structures. Algebra can still yield simple
  collection of terms alongside more interesting products.
- Procedures contain real instructions as their subject matter. Labels clarify
  their role but cannot guarantee that a consuming model treats tool output as
  data. Independent CLI calls still repeat; this audit introduces no persistent
  sampling history or claim of prompt-injection immunity.

Future additions should address these coverage gaps with reusable source data
or constrained generators, following [the family workflow](../families/AGENTS.md),
rather than bulk-generating filler or silently claiming broad representation.
