# Content maintenance - 2026-10-22

The tree contains 76,353 records in 563 corpora and 156 generators across 15
families. This pass adds 1,374 records and 15 generators overall while merging
small collections and removing duplicate or weak material.

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
identities, instruments, and source procedures use count weights internally.
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

Validation includes Rust invariant tests, source replay where caches are available,
record schemas, weight calculations, duplicate checks, individual generator
formatting, Clippy, and a standalone release build. Circuit checks use independent
modified nodal analysis; ngspice is not installed. Some animal, plant, and astronomy
source caches were unavailable, so their format changes also use reconstructed
importer comparisons. Source coverage remains curated and inherits historical and
geographic biases.

Historical review manifests are records of earlier work; they are not fabricated
or expanded into new reviewer histories. See [family guidance](../families/AGENTS.md)
and [architecture](architecture.md) for maintenance and runtime details.
