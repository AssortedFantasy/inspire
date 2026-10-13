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
inspire --family games/dice --seed 42
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

For an agent, add this to the consuming project's `AGENTS.md`, or your global
`AGENTS.md`:

```text
Use `inspire` before making a decision.
```

## Dataset

The initial collection contains 2,113 records in 66 corpora and 14 procedural
generators, organized under 15 top-level families.

Select any branch or leaf with `--family`. Omit it to sample the whole tree.

| Family | Material |
| --- | --- |
| `chemistry` | Chemical compounds and balanced equations |
| `design` | Coordinated wardrobes and SPICE circuits |
| `games` | Dice, Sudoku, tic-tac-toe positions, chess openings |
| `geography` | Real cities and administrative subdivisions |
| `history` | Customs, inventions, and everyday practices |
| `language` | Idioms, proverbs, etymologies, grammatical constructions |
| `literature` | Public-domain quotations and passages, literary forms |
| `math` | Algebraic identities, integrals, finite groups, mathematical concepts |
| `mechanisms` | Mechanical, fluid, and control mechanisms |
| `music` | Song titles and artists from a historical year-end chart |
| `natural-world` | Animals, adaptations, geology, and weather |
| `objects` | Actual car make, model, and year combinations |
| `people` | Notable people grouped by field |
| `procedures` | Algorithms, crafts, cooking, and experimental methods |
| `random` | Dates and times, colors, and UUIDs |

Weights are local to siblings: a larger corpus does not automatically receive
more samples. With default weights, each top-level family receives an equal
share, then each child receives an equal share within its parent. `--list` shows
the effective probabilities. Calls are independent, so repeats are possible.

Source URLs and reuse notes live in each record's `provenance` field and in
family READMEs. Record metadata is not printed or embedded; required attribution
belongs in the emitted text. Factual explanations are original paraphrases;
excerpts identify their authors and works. Coverage is curated rather than
exhaustive, and random constructions are not uniform over every possible object.

Seeds reproduce samples within a given build. Changing the tree, records,
generators, or RNG version can change the output for an existing seed.

Samples should be self-contained material, not prompts or assignments for the
consuming agent. For new corpora, prefer downloading reusable source datasets
and transforming them with reproducible importers; spot-check the emitted text.

See [family rules and sourcing workflow](families/AGENTS.md) to extend the dataset and
[architecture](docs/architecture.md) for embedding and sampling details.
