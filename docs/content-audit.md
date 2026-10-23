# Content maintenance - 2026-10-22

The tree contains 77,601 records in 569 corpora and 165 generators across 19
families.

## Suggestion integration

The [suggestion backlog](../INSPIRATION_SUGGESTIONS.md) now distinguishes existing
coverage, this integration and unimplemented extensions. Existing gears, cams,
weaves, group objects, graph atlas and historical manual passages are indexed
under their current paths instead of duplicated.

Nine generators add physical, computational and social-choice artifacts:
collisions, thin-lens systems, standing waves, cellular automata, concurrency
schedules, binary32 arithmetic, elections, stable matchings and normal-form
games. Their family READMEs document model boundaries and primary references.
Regime construction mixes ordinary cases with conservation limits, focal
singularities, changing boundary conditions, majority cycles, competing stable
assignments, deadlocks, buffered reads and rounding transitions. Stable-merge
traces also gain tied, separated, interleaved and unequal input runs.

Transport adds 1,248 public-domain OurAirports sites in six leaves. A pinned
three-file importer verifies checksums, joins landing surfaces by source ID and
balances countries within each site type. It mixes ordinary sites with elevation
extremes, omits unavailable fields, retains source surface codes, and gives every
selected site equal sampling probability. Independent source checks cover every selected
ID, name, elevation, code and local weight; replay reproduces all records.

Dogfooding used the existing CLI during implementation, then targeted each new
leaf. Election seed 3 produces Borda A versus majority/IRV B; matching seed 3 has
four stable assignments; stable-merge seed 3 alternates runs and seed 456 exercises
all-equal ties. These seeds describe this build, not a permanent seed contract.
Contrasting current names, objects, verse and procedural samples helped retain
ordinary material alongside technical constructions.

The final weighted review also exposed Lipkovo mislabeled as a statistical
region. Official-source corrections repair 72 North Macedonia records, recover
Petrovec's name from conflicting source labels, and distinguish four former
municipalities. All source IDs and weights remain unchanged; the geography
importer reproduces the corrected records exactly.

The source scan finds no normalized exact duplicate records. The probability of
two root draws selecting the same corpus record is now 0.007497%; this excludes
generator collisions and does not measure structural variety or correctness.

The final release review exercised 128 seeds for every generator and 128 root
draws; those root draws covered all 19 families. All 351 Rust tests and nine
review-tool tests pass. All 165 generators pass individual rustfmt checks, and
Clippy passes with warnings denied. Airport import
replay and independent source checks pass. The release review command is:

```sh
python -B scripts/review_content.py --binary target/release/inspire.exe --samples 4 --seeds 128 --weighted-samples 128 --seed 20261022 --output target/suggestions-review.json
```

## Earlier maintenance

The preceding pass contained 76,353 records in 563 corpora and 156 generators
across 15 families. It added 1,374 records and 15 generators while merging small
collections and removing duplicate or weak material. The following notes record
that earlier review.

## Output quality

[Human feedback](../reviews/human-feedback.md) guided the pass. Generators now
print their artifacts without rulebooks, repeated category introductions,
defensive definitions, or padded numbers. A week date is `1982-W41-6`; a chemical
reaction is its equation; a circuit is a runnable netlist with a selected analysis.
Relevant assumptions, units, source credits, and mathematical domains remain.

Dogfooding included default weighted draws and targeted samples. The difference
between original poems and rigid generated paragraphs prompted more attention
to structure: recurring musical motifs, varied rhythmic phrases, geometric
trajectories, sparse transition graphs, and whole historical anecdotes.

- Music gains ABC phrases with varied rhythm, meter, form, and contour, plus drum
  grooves, polymeters, chord voicings, and correctly spelled modes.
- Design gains five circuit topologies, focused analyses, more outfit silhouettes,
  and varied weaving drafts. Circuit tests share one independent nodal solver.
- Games gain unique, irredundant Mastermind puzzles and multi-player Hold'em
  showdowns. Dates choose one representation; palettes include separated
  categorical colors. Ordinary game samples omit solution spoilers and rules.
- Mechanisms gain Geneva indexing, thermostat cycles, capillary manifolds, and
  full cam programs. Gear topology and initial oscillator motion also vary.
- Mathematics includes non-coprime and inconsistent congruence systems, sparse
  absorbing chains, varied exact constructions, and compact graph artifacts.
- Algorithms print executions, including new SCC and topological traces.
  Chemistry adds stereochemical comparisons and empirical-formula reductions.

## Source material

Authored examples, interpretations, and paraphrased procedures were replaced by
original source text or concise factual data. Importers and stored records change
together. New source selections are pinned and carry attribution and bundled
notices; source caches stay outside `families/`.

Language now includes 614 Preston sayings, 335 attributed literary quotations,
and 51 Weekley paragraphs. Literature adds 191 complete Gulistan anecdotes.
History restores primary sources and adds manuals, menus, invitations, typesetting,
and Roman building material. Procedures draw on Farmer, Higgin, Dillmont, and
Tissandier. Museum objects gain 295 sourced coin, basket, and writing records;
weather gains 12 original NOAA definitions.

The pass also fixes an incorrect Akira Suzuki identity, chemistry importer branch
collisions, chess importer/tree drift, and broken excerpt boundaries. Biological
and astronomical records use readable fields while retaining measurement context.
Instrument deduplication keeps 2,135 MIMO and 11 additional Corpora names.

## Sampling

The animal branch accidentally had incoming weight 2,604, overwhelming the other
natural-world topics. Its incoming weight is now 1; all five topics have equal
shares. Count weights remain below the animal and plant topic roots.

Other changes reduce disproportionate exposure of tiny collections while retaining
deliberate broad topic shares. Geographic topics, museum artifacts, chemical
identities, reaction catalogs, instruments, and source procedures use count weights internally.
Language has explicit 50/25/20/5 shares for lexicon, sayings, grammar, and histories.
Literature favors larger bodies of actual text; topic leaves within individual
historical works use record counts.

The probability of two independent root draws selecting the same corpus record
falls from 0.024996% to 0.011823%, about 53%. This excludes generator collisions
and cross-leaf duplicates; it measures repetition exposure, not quality. The final
source scan finds no normalized exact duplicate outputs.

## Verification

Use `scripts/review_content.py --binary target/release/inspire.exe --samples 4 --seeds 128 --weighted-samples 512 --seed 20261022 --output target/content-after.json`
for both per-leaf inspection and ordinary weighted dogfooding. Seeds reproduce
within this build; changed content and tree paths change historical seed results.

The integrated Rust suite passes all 323 tests; the review tools pass nine tests.
All 156 generators pass individual formatting checks, and Clippy passes with
warnings denied. Source replay where caches are available, record schemas, weight
calculations, duplicate checks, and a standalone release build also pass.
Circuit checks use independent
modified nodal analysis; ngspice is not installed. Some animal, plant, and astronomy
source caches were unavailable, so their format changes also use reconstructed
importer comparisons. Source coverage remains curated and inherits historical and
geographic biases.

Historical review manifests are records of earlier work; they are not fabricated
or expanded into new reviewer histories. See [family guidance](../families/AGENTS.md)
and [architecture](architecture.md) for maintenance and runtime details.
