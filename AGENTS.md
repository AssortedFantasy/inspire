Inspire is a CLI for generating entropy for agents.

It runs offline as a single executable. Directories under `families/` define the
sampling tree; leaves contain records or a Rust generator. The build embeds the
tree and payloads, so runtime needs no source files. Weights are local to siblings.

Try to be clever about random generators so that the outputs look "good". Prefer sampling on boundaries, regime transitions, and nontrivial edge cases (where voting methods disagree, dynamics bifurcate, or constraints barely close) alongside clean, ordinary cases rather than uniform textbook homework problems. Not every leaf needs to be a grand spectacle—simple names, places, and ordinary things are rich in their own right. Vibe check that outputs aren't garbage in some obvious way. Cut GPT-style CYA slop: no semicolon/colon glossary chains, no explaining basic conventions, and no fake variation from trivial arithmetic or rigid mad-libs templates.

# Agent Rules

Keep documentation and code up to date without being asked.
Refactor as needed implicitly.
Commit often without being asked.
Keep documentation and code concise.
Keep scratch output and agent session state under ignored `target/`.

# Project Map

Keep this up to date; list only important files and directories.

- `families/`: sampling tree; see [family rules](families/AGENTS.md) for extensions.
- `build.rs`, `build_support.rs`: tree validation and code generation.
- `src/`: CLI, packed layouts, sampling, and chunk retrieval.
- `src/dashboard.rs`, `src/dashboard/`: embedded local server and output explorer.
- `scripts/review_dashboard.py`, `scripts/review-dashboard/`: human review tool; feedback appends to `docs/reviews/human-feedback.md`.
- [docs/reviews/vibe-check-2026-11-02.md](docs/reviews/vibe-check-2026-11-02.md): multi-model sample reviews, strengths, gaps, and cleanup candidates.
- [docs/reviews/improvement-pass-2026-11-02.md](docs/reviews/improvement-pass-2026-11-02.md): implemented review fixes, sourced expansions and validation.
- `tests/`: build-tree and CLI integration tests.
- [docs/examples.md](docs/examples.md), [docs/examples-details.md](docs/examples-details.md): poems and skateboarding SVGs with and without Inspire; prompts and run details.
- `scripts/review_content.py`: counts, probabilities, duplicate checks, and output samples.
- `scripts/token_count.py`: fast text/JSONL token counts and per-leaf length audits; [cleanup results](docs/token-length-audit.md).
- `scripts/balance_*.py`, [docs/balance-study.md](docs/balance-study.md): counterfactual sampling experiments and measured results.
- `scripts/balance_blind_trial.py`, [docs/blind-stream-study.md](docs/blind-stream-study.md): frozen blinded stream comparisons and judge audit.
- `scripts/creative_validation.py`, `scripts/creative_render.py`, `scripts/creative_review.py`, [docs/creative-validation.md](docs/creative-validation.md): paid creative-task validation, SVG rendering, and blinded review.
- `scripts/import_catalogues.py`, `scripts/catalogue_sources.json`: pinned food and material imports.
- [DATA_SOURCES.md](DATA_SOURCES.md): embedded dataset redistribution notices.
- `docs/archive/`: historical research, planning, and agent review artifacts; not current maintenance instructions.
- [docs/architecture.md](docs/architecture.md): build and runtime details.

# Tips

Run checks at integration points after a coherent batch, not after each leaf.
Commit finished leaves promptly; avoid reopening completed reviews for wording edits.

Audit sample lengths after adding or substantially expanding content (build first):

```sh
uv run --cache-dir target/uv-cache scripts/token_count.py audit --binary target/release/inspire.exe --family PATH --output target/token-audit.json
```

Omit `--family` for the whole tree; on Unix use `target/release/inspire`.
This checks every corpus record and 128 seeds per generator. The default
`--max-tokens 512` is the sample length cap. Simplify overlong generators to
their structural core; use complete, distinctive excerpts for corpora. Do not
truncate outputs mechanically or paraphrase human sources to fit.
Exit 1 means over-limit samples were found; the JSON report retains their texts
and record IDs or reproduction seeds. Use `--seeds N` for deeper generator review.
For one text file use `count FILE`; for CLI JSON or sample banks use
`count --jsonl FILE` (omit FILE to read stdin). Counts use `o200k_base`; override
with `--encoding NAME`. Initial setup downloads tiktoken and encoding data;
subsequent runs work offline from cache. No API key is needed.

```sh
cargo fmt --all --check
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
```

`cargo fmt` misses build-discovered generators. Format each `generator.rs` with
`rustfmt --edition 2024 <path>`; add `--check` to verify. CI checks all generators.
Keep `Cargo.lock` committed.
