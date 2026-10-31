# Blind stream trial — 2026-10-30

**Current sampling won this pilot.** Judges preferred current over both optimized
methods in every comparison. Embedding improvements did not translate into lower
perceived redundancy in 32-output collections.

The three frozen methods were current, RBF + exact (`λ=.03`), and the balanced
RBF + exact + KL candidate (`λ=.03`, `τ=.03`). Each matchup had four independent
pairs of fresh 32-output streams. Each pair was judged twice by separate fresh
`gpt-6-sol` agents, with A/B sides exchanged. Two additional agents received
identical-stream controls: 26 agents total, fresh contexts, reasoning effort not
overridden, no follow-up judgments or preference revisions. Instructions asked
which collection felt less redundant, allowed ties, and imposed no preferred
topic or length. Judges saw full texts and neutral item labels, without method
names, paths, weights, or scores.

| Matchup | Independent pair wins | Mean score favoring second method | Mirrored direction agreement |
| --- | --- | ---: | ---: |
| Current vs RBF + exact | Current 4, optimized 0 | −2.125 | 4/4 |
| Current vs balanced | Current 4, balanced 0 | −2.000 | 4/4 |
| RBF + exact vs balanced | Optimized 1, balanced 3 | +1.500 | 4/4 |

Scores range from −3 to +3. Mirrored scores are averaged **within each pair**;
eight judgments per matchup are four independent pairs, not eight independent
trials. Both identical controls tied. Mirror direction agreed in 12/12 pairs;
exact score magnitude agreed in 9/12.

Reasons identified real clusters rather than merely literal duplicates. Cases
001/002 cited seven brief biographies and three sparse place entries in the
optimized stream. Cases 007/008 cited three breadth-first-search traces. Cases
011/012 cited three outfit lists in balanced. Cases 017/018 correctly identified the repeated
“Guinea-Bissau - Western Africa” entry, plus repeated BFS and dice formats.
Some cited groups contain meaningful differences despite a shared format or
source; those are judges' subjective redundancy judgments. The audit found no
invented item references. A loose “artist biographies” gloss in case014 included
non-artists; the underlying biographical group was real. No votes were rewritten.

A post-judgment diagnostic embedded the streams using the original
model and bandwidths, without fitting. In all four current-versus-balanced pairs,
mean cosine, all three mean RBF affinities, nearest-neighbor cosine, and maximum
pair cosine favored balanced, while judges favored current. Mean fine affinity
fell from 0.158516 to 0.144628; neither method produced an exact duplicate in these
four pairs. Current covered 18.0 families per stream versus balanced's 15.75.
Thus the metric disagreement also occurred on the exact judged samples; it was
not just a mismatch between the earlier sample bank and new draws.

Independent audit reconstructed all 768 leaf selections and seeds, then replayed
all 768 real CLI outputs with exact full-text matches. Source, binary, and method
weight hashes matched; frozen key SHA-256 remained
`0c7bec0d42856eddfb5219da43050f5f8570d0c7d368066a97dd53da74d3f17b`.
Blinding, mirrors, text preservation, aggregator orientation, and paired counting
passed inspection and focused synthetic tests.

This is a small model-judge pilot: four pairs per matchup, one horizon, and one
judge model family. Side reversal and controls help diagnose consistency, not
human validity or a universal ranking. Keep current weights; these results do
not support adopting either candidate as less redundant in ordinary use.

[Summary CSV](data/blind-stream-summary.csv) preserves votes and actual-stream
diagnostics. [Raw verdicts and unblinded mapping](data/blind-stream-verdicts.json)
preserve all 26 judgments, case hashes, model protocol, and audit notes in 28 KB.
Full stimuli remain under ignored `target/blind-study/`:

```sh
python -B scripts/balance_blind_trial.py prepare
python -B scripts/balance_blind_trial.py aggregate
python -B -m unittest discover -s scripts -p test_balance_blind_trial.py
```

Preparation seed is `202610300132`; reuse the original weights and binary to
reproduce stimuli. Preserved verdicts reproduce aggregation, not future model judgments.
