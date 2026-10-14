# Content maintenance - 2026-10-13

The current tree contains 74,979 records in 595 corpora and 141 generators across
15 families. Earlier expansion work grew the tree from 2,134 records, 67 corpora,
and 13 generators. Historical review manifests are snapshots rather than a
requirement for routine maintenance.

## Human feedback applied

[Human feedback](../reviews/human-feedback.md) favors useful material with compact
framing: recipes, examples, objects, poems, and concrete mathematical or technical
artifacts. Repeated source and dataset descriptions obscured that material.

- Instrument and person records omit repeated source URLs, license paragraphs,
  transformation notes, and administrative identifiers.
- Chemistry records focus on identity, formulas, mass, and structure. Reaction
  extents state consumption and production directly, with relevant conditions.
- WordNet retains senses and antonyms; copyright and permission notices are bundled.
- Literary credits retain author, work, translator, and meaningful printed
  locations. Invented poem ordinals, paragraph counters, and HTML anchors stay in
  provenance. Authentic excerpt wording and punctuation are preserved.
- Museum records retain object, date, culture, and material. Geographic labels
  prefer ordinary hyphens. Biological data retain units, measurement method, and
  species, genus, regional, or study context with shorter framing.
- Circuit displays use six significant digits. Exact mathematical values,
  combinatorial drafts, and intrinsic game/procedure rules retain their meaning.

Producer formatting and stored records change together. Source provenance and
full notices remain available; `inspire --data-licenses` includes dataset credit,
modification notes, and complete license text without crowding ordinary samples.

The corpus scan found no normalized exact duplicate texts and no printed source
URL, snapshot-date, license-footer, or invented poem-ID remnants. Authentic
quotations retain contrast and negation; mathematical negation and defining
assumptions remain part of the content.

## Sampling policy

The feedback dashboard's sample frequencies do not measure default CLI exposure.
This cleanup preserves topic weights. Broad top-level families have weight 2;
raw random values have weight 1. `inspire --list` shows effective probabilities.

Within `people`, `objects/cars`, `natural-world/animals`, and
`geography/subdivisions`, child weights equal descendant record counts, giving
records equal conditional shares across folder depths. Animal weights were
synchronized to the current 2,604 records. Other branches balance subjects rather
than corpus size; family notes describe additional local choices.

## Maintenance workflow

Leaf implementation uses GPT-6 Luna with high reasoning. Shared formatting fixes
are applied in bulk, followed by a bounded pass over affected leaves. Completed
leaves or coherent batches are committed promptly. Tests run at integration
points, with substantive invariants covering behavior changes. Routine wording
edits do not start another source audit or exhaustive review chain.

Coverage remains curated and inherits source geographic, historical, and language
biases. Generators cover bounded constructions. Cached inputs can be needed to
reproduce mutable API snapshots; runtime remains offline.

The integrated tree passes all 279 Rust tests, Cargo and generator formatting,
Clippy with warnings denied, and the standalone release build. Corpus scanning
and cached importer checks, including all four literature replay tests, cover
the updated output format without reopening historical review queues.

See [family guidance](../families/AGENTS.md) for content preferences and the
[architecture](architecture.md) for packing, sampling, and verification tools.
