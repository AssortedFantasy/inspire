# Sample length audit

The subsequent [content improvement pass](reviews/improvement-pass-2026-11-02.md)
adds sourced material and changes 15 generators. Its rebuilt release audit
checks 90,270 corpus records plus 128 seeds for each of 222 generators:
118,686 samples across 860 leaves, none above 512 tokens (maximum 509).
The earlier deeper generator study below describes the pre-expansion revision.

Content is capped at 512 `o200k_base` tokens, including source credits. The
2026-11-02 cleanup used tiktoken 0.12.0 and reviewed all 844 leaves with nine
GPT-6.1-sol agents at medium reasoning effort.

The initial audit found 281 over-limit samples across 19 leaves: 119 corpus
records and 162 generator outputs from 128 seeds per leaf. After cleanup:

| Coverage | Checked | Above 512 | Maximum tokens |
| --- | ---: | ---: | ---: |
| All 622 corpora | 90,086 records | 0 | 509 |
| All 222 generators | 454,656 outputs | 0 | 405 |

The integrated generator audit checked 2,048 seeds per leaf: 0–1,023,
`u64::MAX`, and 1,023 deterministic random unsigned 64-bit seeds. Source review
also checked output dimensions and potentially expanding traces. These are
observed maxima, not an exhaustive enumeration of every RNG state.
The rebuilt release executable's ordinary audit also passed: 118,502 samples
across all 844 leaves, with no output above 512 tokens.

## Generator changes

Nineteen generators changed. Dense traces now retain initial/final state and
consequential operations rather than repeated arrays, matrices, path prefixes,
or equivalent representations. Automata use smaller complete state structures;
Latin squares use one symbol legend and an exact formula for the orthogonal
mate. Puzzle fallbacks bound the number of clues while retaining uniqueness
and deletion-minimality. Unchanged generators already had compact, bounded
artifacts.

| Initially over-limit generator | Before, 128 seeds | After, 2,048 seeds |
| --- | ---: | ---: |
| Hash tables | 1,033 | 334 |
| Cellular automata | 822 | 392 |
| Heaps | 819 | 337 |
| Latin squares | 808 | 317 |
| DFA | 543 | 270 |
| Levenshtein alignment | 514 | 42 |

## Corpus changes

The 119 long originals became coherent excerpts at reviewed paragraph,
sentence, stanza, or verse-sentence boundaries. Mythology, poems, fables, and
Shaw settings retain their source wording and attribution. Equiano's collision
excerpt drops its preamble and aftermath; one mythology bridge to the next
section is omitted. No text was paraphrased or mechanically token-truncated.

The corpus count increased from 89,895 to 90,086. Frozen excerpt maps and source
selection offsets reproduce the changes without a tokenizer dependency in the
importers. Count-based stage-direction weights were updated. The belief and
literature importers retain their existing topic weights.

## Recheck

Build first, then run the explicit maintenance audit:

```sh
cargo build --release --locked
uv run --cache-dir target/uv-cache scripts/token_count.py audit --binary target/release/inspire.exe --output target/token-audit.json
```

Use `target/release/inspire` on Unix. Add `--family PATH` for a subtree or
`--seeds N --seed INTEGER` for a different generator range. Every corpus record
is checked on each audit; generator output coverage is sampled. The audit
remains separate from normal Rust tests and CI.

Verification included Cargo formatting, tests, clippy with warnings denied,
the release build, generator formatting, importer replay, source-preservation
tests, and the exhaustive corpus/deeper generator audits above. Scratch reports
and sample harnesses remain under ignored `target/`.
