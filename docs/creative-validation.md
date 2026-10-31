# Creative validation: Inspire versus a bare prompt

This experiment tests the current sampler as a creative prompting intervention.
Sampling weights and family contents are unchanged. The intervention is one fresh,
unselected root sample plus an explicit optional-inspiration prefix. The control
receives the same creative task alone. Both receive the same short developer
instruction and start with an empty conversation.

## Measured result

**Inspire won 11 of 12 independent batch comparisons for diversity.** It won
9 of 12 on the separate quality score, with 3 ties and no aggregate quality
losses. Each comparison contains ten works per condition and averages two
fresh, oppositely ordered judge calls.

| Task | Diversity wins / losses / ties | Mean diversity | Quality wins / losses / ties | Mean quality |
| --- | --- | ---: | --- | ---: |
| Poem | 3 / 0 / 0 | +2.33 | 3 / 0 / 0 | +1.00 |
| Video-game pitch | 3 / 0 / 0 | +2.33 | 2 / 0 / 1 | +0.67 |
| SVG artwork | 3 / 0 / 0 | +3.00 | 3 / 0 / 0 | +0.67 |
| Microfiction | 2 / 1 / 0 | +0.83 | 1 / 0 / 2 | +0.17 |

Scores are ordinal, from −3 to +3, with positive favoring Inspire. These means
describe the judgments; they are not percentage improvements. The overall means
are +2.125 for diversity and +0.625 for quality. Diversity preference directions
agree under A/B reversal in all 12 pairs; exact score magnitudes agree in 11.
Quality direction agrees in 9 pairs; the remaining 3 combine a slight Inspire
preference with a tie. Both randomly selected identical-batch controls tied on
both scores. Both controls happened to use microfiction.

Browse the complete, initially blinded comparison at
`target/creative-validation/review.html`. It contains all 240 works, rendered
SVGs, optional condition and inspiration reveals, and both judge assessments.
Retained [results and verdicts](data/creative-validation-results.json) and the
[independent audit](data/creative-validation-audit.json) record the evidence.

### What changed

The following observations explain the frozen results; they are subsequent
inspection rather than additional independent judgments.

- Baseline poems repeatedly use laundromats, bakers, buses, rain, and small acts
  of care. In replicate 1, baseline items 01, 06, and 10 all feature a folding
  woman and revolving laundry. Inspired items introduce a vacuum demonstration
  (06), competing electoral outcomes (08), and beaver physiology (05), while
  often retaining an elegiac, domestic voice.
- Baseline game pitches repeatedly use drowned cities, disappearing districts,
  and memories spent as fuel. Replicate 1 baseline items 04, 07, and 08 share
  the title "The Last Lighthouse." Inspired items add railway dispatching (04),
  a Ragnarok cobbler (07), floating-point rounding (08), and cryptographic
  vulnerabilities (10). Some still use the drowned-city framework.
- SVGs show the strongest judged effect. Baselines cluster around nocturnal
  lighthouses, observatories, and mountain lakes. Inspired replicate 2 includes
  a monochord (01), amulet (03), cooking pot (07), and coat (10). Teal/gold
  palettes, celestial disks, and decorative framing often persist: subject
  breadth improves more than stylistic breadth.
- Microfiction replicate 2 is the counterexample: both judges slightly prefer
  baseline diversity. Inspired stories about a harbor radio (06), lamp chimney
  (05), and speaking radiators (01) repeat sacrificing contact with a dead
  relative to protect someone living. Baseline alternatives include a communal
  dragon story (06), dementia rescue (07), and ecological fable (03). New props
  do not reliably produce new plot structures.

For inspection, start with replicate 1 of each task, then microfiction replicate
2 for the counterexample. Replicate 1 is a fixed first-replicate choice, not a
selection of the largest effect. IDs above are original generation positions;
the review reveals these IDs because judge-visible item order was shuffled.

### Integrity and cost

All 240 generations completed without refusals, truncation, or blank outputs.
All 180 text works met the requested word range under whitespace counting,
including titles. All 60 SVGs rendered and passed the separate source/viewBox
checks. All 26 judge responses were valid; none reported task failures.

The independent audit replayed all 120 sampled CLI payloads exactly, verified
all 1,898 frozen source files, checked fresh contexts and exact model/reasoning
settings, recomputed all costs, and reconstructed the pair assignments and
score orientations. Sampling weights remain unchanged.

Returned token usage gives **$1.9587325 for generation + $0.7468335 for judging
= $2.705566 estimated API cost**, matching the local ledger. No API request was
retried. A Windows file-lock error interrupted local bookkeeping after 46
completed calls. A recorded implementation-only amendment added atomic-write
retries and released one reservation proven to precede HTTP dispatch. All 46
completed records and all 240 frozen request bodies remained byte-identical;
none was regenerated. The incident and hash chain are retained with the run.

## Frozen protocol

- Generator: `gpt-6.1-terra`, high reasoning, standard service tier, no tools.
- Tasks: poem (80–140 words), video-game pitch (120–180 words), standalone SVG,
  and microfiction (120–180 words). Exact prompts are in the frozen manifest.
- Three independent replicates per task; ten independent completions per arm in
  each replicate: **240 completions, 12 batch pairs**. Requests are interleaved
  using a fixed random schedule. A completion never sees the other nine works.
- One uncurated current-weight Inspire sample per treatment completion; no
  repairs, quality-based retries, or selection of favorable samples.
- Judge: `gpt-6.1-sol`, high reasoning. Each pair receives two fresh judgments
  with A/B order reversed. Within-batch order is shuffled, then preserved under
  reversal. Two identical-collection controls check judge consistency.
- Judges receive only the task and finished works, with neutral item labels;
  they do not receive the condition, inspiration, source path, or hypothesis.
- Primary score: meaningful within-batch diversity, −3 to +3. Quality and task
  failures are assessed separately. Analysis orients scores toward Inspire and
  averages the two mirrored judgments within each independent batch pair.
- SVGs are judged as uniformly rendered 400×400 images on white. Source text,
  renderer failures, and requested-viewBox compliance are retained separately.
  No SVG is repaired; a failed rendering occupies its original position.
- Both models have a 16,384-token output cap including reasoning. Truncations
  remain outcomes. Transport/API failures are missing observations, not creative
  failures, and prevent silently judging incomplete collections.
- The runner reserves worst-case call costs under a $40 cap, including $8 held
  for judging during generation. Ambiguous network outcomes retain their full
  reservation and are not automatically retried.

The complete frozen protocol, request/response records, token ledger, samples,
images, judgments, and standalone comparison viewer live in
`target/creative-validation/`. The key is read from `OPENAI_KEY` in `.env` and is
never written into the artifacts.

## Interpretation

The unit of comparison is a pair of ten-work batches, not an individual work or
a mirrored judge call. Three pairs per task support an exploratory result about
these prompts and models. The experiment estimates the full prefix intervention
against the bare prompt; it does not establish superiority to other sources of
random context. Condition labels are hidden, but judges may infer the treatment
from the works. Model judging is evidence about the chosen rubric, not proof of
human preference or general creative usefulness.

## Reproduction

Use Python with `requests` and Pillow, and Node with `sharp` available through
normal module resolution or `NODE_PATH`.

```text
python -B scripts/creative_validation.py prepare
python -B scripts/creative_validation.py generate --limit 2
python -B scripts/creative_validation.py generate
python -B scripts/creative_render.py
python -B scripts/creative_validation.py cases
python -B scripts/creative_validation.py judge
python -B scripts/creative_validation.py aggregate
python -B scripts/creative_review.py
```

If another process holds the ordinary release executable open on Windows, set
`CARGO_TARGET_DIR` to a separate directory and pass its executable with
`prepare --binary`. Generation resumes completed requests without paying again;
saved outputs, rather than deterministic API sampling, make analysis replayable.

Pricing and API settings were checked against the official
[pricing table](https://developers.openai.com/api/docs/pricing) and
[model parameters](https://developers.openai.com/api/docs/guides/latest-model#gpt-6-astra-update-api-and-model-parameters).
Standard rates used are $1/$5 per million input/output tokens for Terra and
$2/$10 for Sol, with cached-read and cache-write adjustments from returned usage.
