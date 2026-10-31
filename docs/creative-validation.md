# Creative validation: Inspire versus a bare prompt

This experiment tests the current sampler as a creative prompting intervention.
Sampling weights and family contents are unchanged. The intervention is one fresh,
unselected root sample plus an explicit optional-inspiration prefix. The control
receives the same creative task alone. Both receive the same short developer
instruction and start with an empty conversation.

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
