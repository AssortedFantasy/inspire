# Content improvement pass — 2026-11-02

Fifteen GPT-6.1-sol agents implemented a bounded expansion and cleanup pass from
the [GPT, Gemini and Opus findings](vibe-check-2026-11-02.md). This did not rerun
the broad vibe exercise. Findings were checked against source and implementation;
reviewer preferences were not treated as universal quality rules.

## Expansion

205 attributed source records were added in 16 leaves. Each collection has a
deterministic importer, pinned input hashes, record-level provenance, source
locators and bundled redistribution notices.

| Collection | Added records | Material |
| --- | ---: | --- |
| [Everyday voices](../../families/history/everyday-voices/README.md) | 42 | Sancho and Austen letters, Dorothy Wordsworth journals: requests, interruptions, domestic arrangements, humor and sensory observation. |
| [Work observations](../../families/history/work-observations/README.md) | 30 | Original NASA Apollo 12 exchanges: equipment trouble, fatigue, cross-checking, improvisation and gaps in procedures. |
| [Institutional documents](../../families/society/institutions/README.md) | 46 | Court decisions, guild obligations, commons rules, borough complaints and reciprocal toll arrangements. |
| [Philosophical sources](../../families/philosophy/primary-texts/README.md) | 44 | Hume, Wollstonecraft and James on inference, education, authority, sensation and experience, retaining their different positions. |
| [Bookbinding and pottery](../../families/procedures/craft/README.md) | 43 | Repair, construction, shaping and finishing operations from Cockerell and White. |

The Apollo selection uses the original NASA transcript, not the separately
copyrighted corrected Journal. Its speaker turns were checked against scanned
pages; OCR corrections and extraction offsets are explicit. The other additions
use verified public-domain editions. Source words are not paraphrased into an
LLM voice. Figures, page furniture and unsuitable source-dependent selections
are excluded or handled by documented extraction rules.

The resulting tree has 90,270 corpus records in 638 corpus leaves and 222
generator leaves: 860 leaves total. Cleanup removed 21 records net elsewhere,
including consolidation of repetitive empirical-formula examples, so the total
record increase is 184.

These additions broaden the modes of material, but do not constitute global or
contemporary coverage. Most new prose remains historical English-language text;
the operational collection reaches 1969.

## Generator improvements

Fifteen generator leaves changed.

- **Logic:** bounded construction avoids accidental identical random operands
  and collapsed law substitutions. Deliberate logical identities remain. Kripke
  relations use compact successor sets.
- **Flows and schedules:** effective capacity interventions are preferred when
  available; blocked cut-edge upgrades retain an explicit competing-cut witness.
  Fully serial projects print a compact chain instead of a redundant slack table.
- **Subtraction games:** exact N/P outcomes, every winning removal, and eventual
  outcome cycles proved by recurrence-state repetition are emitted.
- **Cryptography:** singular share wording is fixed. Clean Hamming cases remain
  at their existing 1/12 share with a shorter representation.
- **Design and geometry:** identical consecutive knitting rounds are grouped
  losslessly; singleton grid ranges use one index. Triangle reflex counts are
  omitted, thin triangles are bounded, and star tips avoid extreme needles.
- **Math:** projections expose the exact complementary decomposition; probability
  draws include mean-preserving spreads and mode/support boundaries; logarithmic
  integration includes the exceptional exponent and its domain changes. Graph
  atlas degree sequences are explicitly labelled as sorted.
- **Chemistry:** weak/strong titrations compare buffering, equivalence and excess
  titrant under an ideal mass/charge-balance model. Silver-halide comparisons
  include tied and reversed precipitation onsets. Both leaves retain simple
  equations in 35% of draws. Empirical compositions retain 17 plain reductions
  and four groups showing shared ratios, replacing 34 repetitive reductions.

The flow agent's fixed 2,000-seed diagnostic found effective interventions in
65.8% of draws, versus 42.85% before. This checks the intended selection change;
it is not a measured improvement in downstream creativity.

## Corpus repairs and decisions

- Cleaned embedded missing creator roles and dangling predicate roles in museum
  credits. Preserved attribution uncertainty and documented the narrow
  `Itay` → `Italy` editorial spelling correction.
- Made combined geography types readable and retained the precise meaning of
  bounding-box center coordinates. Specialist landing facilities now identify
  their kind when the name alone does not.
- Corrected HAT-P-47 b's discovery year to 2016 from the
  [discovering team's paper](https://arxiv.org/abs/1606.04556v1), retaining the
  original snapshot value and the unchanged numerical solution in provenance.
- Repaired twelve documented Bulfinch transcription artifacts, joined the Edda
  question/verse response correctly, removed numeric fable footnote callouts,
  and restored the source paragraph needed by Weekley's comparison.
- Replaced weak Brown escapement selections with source passages that explain
  operation. Some redundant figure references remain where the prose itself is
  sufficient; all 25 historical mechanism leaves remain populated.
- Removed Zorbeez as a finished cleaning-product brand and six specifically
  reviewed trivial numeric WordNet senses. The confusing `tingling` sense now
  displays its native synonym `tickling`, with the original gloss unchanged.

Several proposed changes were deliberately narrowed. The `unfair` gloss has a
substantive continuation and was retained. Existing Hammurabi material prevented
a duplicate institutional import. Figure-dependent woodworking sources were
rejected in favor of usable bookbinding and pottery. Ordinary objects, clean
technical baselines, specialist notation and unfamiliar names remain legitimate
outputs. Attribution styles were not mechanically rewritten across all sources.

## Sampling

All root family weights remain unchanged. Local changes give new material a
controlled share:

- Philosophy keeps 75% logic and assigns 25% to the 44 source units.
- Society keeps economics/voting weights 7/3 and adds institutions at 2;
  generated models retain 5/6 of the family share.
- The five previous history branches each use weight 4; everyday voices and
  work observations each use 1. Each new branch receives 1/22 of history draws.
- Craft keeps its existing family share and samples uniformly across its 79
  needlework, bookbinding and pottery records.

The historical balance-study cap was never deployed; the review's follow-up
claim implying otherwise has been corrected. No embedding optimization or
global distribution retuning was performed.

## Validation

Source agents ran pinned importer replay, source/selection integrity checks and
targeted structural tests. The history and procedures tests now compose their
independent importers, checking the complete family rather than silently omitting
new branches.

- Cargo formatting and explicit formatting checks for all generator files pass.
- All 532 unit tests and 21 integration tests pass. The dashboard integration
  test initially hit the sandbox's loopback-socket restriction (Windows 10013);
  its targeted rerun with socket access passed.
- `cargo clippy --all-targets --locked -- -D warnings` and
  `cargo build --release --locked` pass. Cargo emitted only its environment
  warning about copying incremental cache files instead of hard-linking them.
- Combined history replay tests (3) and procedures integrity tests (7) pass,
  alongside the agents' focused importer/source and mathematical tests.
- All 90,270 corpus texts are exactly distinct.
- The rebuilt release audit checks all 90,270 corpus records and 28,416
  generated outputs (128 seeds per generator): **118,686 samples across 860
  leaves, zero above 512 `o200k_base` tokens**. Maximum observed length is 509.

These are source/correctness/length checks, not a new model-judged vibe trial or
proof of downstream creative gains. The full audit is reproducible with:

```sh
cargo build --release --locked
uv run --offline --cache-dir target/uv-cache scripts/token_count.py audit --binary target/release/inspire.exe --output target/improvement-pass-2026-11-02/token-audit.json
```

Raw source downloads, agent handoffs, focused harnesses and verification logs
remain under ignored `target/improvement-pass-2026-11-02/` and source-specific
`target/imports/` caches. Existing review batches remain unchanged.
