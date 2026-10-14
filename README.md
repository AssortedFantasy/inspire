# Inspire

A small, offline CLI that creates a bit of entropy for agents.

Samples come from a categorized tree of factual records, public-domain excerpts,
and procedural generators. The tree and data are embedded in one executable;
sampling needs no network or source files.

## Build and use

Install with Rust:

```sh
cargo install --path . --locked
inspire
inspire --help-full
inspire --list
inspire --data-licenses
inspire --family games/chance/dice --seed 42
inspire --family geography
```

Cargo's bin directory must be on `PATH`.
Build only: `cargo build --release --locked`.

## Output dashboard

Run `inspire --dashboard`, then open the printed URL (normally
`http://127.0.0.1:8765/`). Use `--port 0` to choose an available port, or
`--port INTEGER` to choose one yourself. Stop the server with Ctrl+C.

Each row splits a branch into rectangles proportional to sibling weights.
Hover or focus a rectangle to preview its children; click to expand the next
row and scroll to it. Ancestors remain visible above. The leaf selector jumps
directly to an output stream. Narrow screens can scroll rows horizontally.

At a leaf, the dashboard shows the full path's weighted probability and appends
outputs as you scroll. Datasets appear in shuffled order without repeats and
stop after every record; **Reshuffle** starts another pass. Generators continue
indefinitely. Copy a generator's CLI command to reproduce an output. Zero-weight
nodes remain inspectable, with their probability shown as zero.

The dashboard, tree, and outputs are embedded in the same executable. It serves
only on loopback and needs no internet, source files, or frontend installation.

## Human review dashboard

Run `python scripts/review_dashboard.py` from the checkout. This separate Python
tool rebuilds the current dataset and opens `http://127.0.0.1:8766/`. It requires
Python 3 and Cargo, with no additional Python or frontend dependencies.

Read the sample, optionally write notes, and click **LGTM** or **JANK** to save
and load another random sample. The family selector narrows the review. Sampling
gives each leaf an equal chance, then picks a random record or generator seed;
this exposes small leaves as well as large corpora, independently of CLI weights.

Feedback appends to `reviews/human-feedback.md`, including the full text, source
path, record ID and provenance, or generator seed and binary hash. Notes stay
literal Markdown code blocks for later AI review. Feedback survives restarts;
the on-screen count covers the current session. The tool reads source records
and invokes the CLI only for generated samples; it is not embedded in Inspire.

Use `--feedback PATH` for a different file, `--port 0` for an available port,
`--no-open` to skip opening the browser, or `--binary target/release/inspire.exe`
to use an existing build without rebuilding. Stop with Ctrl+C.
Check the review tool with `python -m unittest discover -s scripts -p test_review_dashboard.py`.

For an agent, add this to the consuming project's `AGENTS.md`, or your global
`AGENTS.md`:

```text
Use `inspire` before making a decision.
```

## Dataset

Fifteen top-level families combine downloaded corpora with bounded procedural
generators. `inspire --list` shows the tree embedded in the current build;
`python scripts/review_content.py` reports source record and generator counts.

Select any branch or leaf with `--family`. Omit it to sample the whole tree.

| Family | Material |
| --- | --- |
| `chemistry` | Chemical compounds and balanced equations |
| `design` | Coordinated wardrobes, SPICE circuits, and textile weave drafts |
| `games` | Legal board positions, opening books, card deals, dice, and puzzles |
| `geography` | Settlements, administrative regions, countries, landforms, and waters |
| `history` | Customs, inventions, and everyday practices |
| `language` | Idioms, proverbs, etymologies, grammatical constructions |
| `literature` | Public-domain quotations and passages, literary forms |
| `math` | Exact algebra, analysis, geometry, graph objects, topology, and probability |
| `mechanisms` | Mechanical, fluid, and control mechanisms |
| `music` | Classical works, instruments, scales, harmony, rhythms, and melodic phrases |
| `natural-world` | Species and biological traits, adaptations, plants, astronomical bodies, and natural processes |
| `objects` | Historical museum artifacts and actual car make, model, and year combinations |
| `people` | Notable people grouped by field |
| `procedures` | Algorithms, crafts, cooking, and experimental methods |
| `random` | Dates and times, colors, and UUIDs |

Weights are local to siblings: a larger corpus does not automatically receive
more samples. Broad top-level families have weight 2; raw random values have
weight 1. Names within `people`,
`objects/cars`, `natural-world/animals`, and `geography/subdivisions` have equal
per-record shares across their folders. Other branches balance subjects rather
than corpus size; family READMEs explain their local choices. Colors and dates
each have twice UUID's share. `--list` shows the
effective probabilities. Calls are independent, so repeats are possible.

Source URLs and reuse notes live in each record's `provenance` field and in
family READMEs. Record metadata is not printed or embedded; required attribution
belongs in the emitted text. Factual explanations are original paraphrases;
licensed glossary text is identified by source, and literary excerpts identify
their authors and works. Coverage is curated rather than
exhaustive, and random constructions are not uniform over every possible object.
Dataset license files are embedded and printed by `--data-licenses`; see
[dataset redistribution notes](DATA_SOURCES.md).

Seeds reproduce samples within a given build. Changing the tree, records,
generators, or RNG version can change the output for an existing seed.

Samples should be self-contained material, not prompts or assignments for the
consuming agent. For new corpora, prefer downloading reusable source datasets
and transforming them with reproducible importers; spot-check the emitted text.
Procedures label their intrinsic steps as algorithms, kitchen/sewing procedures,
or experiment protocols; language examples and proverbs are explicitly quoted.

See [the content audit](docs/content-audit.md) for the weighting rationale,
corrected issues, and remaining coverage limitations.

See [family rules and sourcing workflow](families/AGENTS.md) to extend the dataset and
[architecture](docs/architecture.md) for embedding and sampling details.
