# inspire

A small, offline CLI that prints one stimulus before a creative decision.

This is an engineering prototype, not a demonstrated improvement to LLM
creativity. It ships with five original AI-generated haikus and a generator of
integer algebra identities using `+`, `-`, and `*`.

## Build and use

Install a current stable Rust toolchain, then run:

```sh
cargo build --release --locked
cargo run --release --locked -- --list
cargo run --release --locked -- --family poems/haiku --seed 42
cargo run --release --locked -- --family math/algebra --seed 42
```

The standalone executable is `target/release/inspire` on Unix or
`target/release/inspire.exe` on Windows. It needs no corpus files, Rust
installation, network connection, or configuration files at runtime.

To install it into Cargo's bin directory:

```sh
cargo install --path . --locked
```

With that directory on `PATH`:

```sh
inspire
inspire --family poems
inspire --family math/algebra --seed 42
inspire --list
```

For an agent, add this to the consuming project's `AGENTS.md`:

```text
Use `inspire` before making a creative decision. Treat its output as optional
inspiration; preserve the user's requirements and do not force a connection.
```

Normal output contains only the stimulus, followed by a newline. Errors go to
stderr and exit with status 1. `--help` (or `-h`) prints only
`Generates a bit of inspiration`. Detailed usage is available through
`--help-full`, which is not advertised by normal help or error messages.

`--family PATH` selects a branch or leaf. Paths come directly from directories
under `families/` and use `/` on all platforms. `--list` shows the hierarchy,
local weights, and effective probabilities from the root. Branch probabilities
represent their entire subtrees; do not add them to their descendants.

`--seed` takes an unsigned 64-bit integer and reproduces a sample with the same
build and arguments. Unseeded runs use operating-system randomness. Calls are
independent; there is no saved history or no-repeat rule.

## Extend and rebalance

The directory tree is the sampling tree:

```text
families/
├── math/
│   └── algebra/
│       └── generator.rs
└── poems/
    └── haiku/
        └── records.json
```

Directories with children are branches. A leaf contains either `records.json`
or `generator.rs`. Every node defaults to weight 1 among its siblings. To give
mathematics twice the probability of poetry, create `families/math/node.toml`:

```toml
weight = 2
```

Move directories to regroup them, or add child directories to expand a branch,
then rebuild. There is no global weight file, required manifest, or registry to
maintain. A node's local weight moves with its directory. Regrouping can change
effective probabilities: each branch allocates its share among its own children,
independently of their numbers of descendants or records.

Weight 0 excludes a node's entire subtree from its parent's random selection.
Explicit `--family PATH` bypasses incoming weights, including those of ancestors.
Sampling inside a selected branch still respects its children's weights. Every
branch must have at least one positive-weight child, even when disabled.

- Start with [AGENTS.md](AGENTS.md) for the project map and contribution rules.
- Follow [families/AGENTS.md](families/AGENTS.md) to add, move, or weight nodes.
- Read [docs/architecture.md](docs/architecture.md) for build/runtime details.
- [REASEARCH.md](REASEARCH.md) contains research context and possible experiments.

## Checks

```sh
cargo fmt --all --check
rustfmt --edition 2024 --check families/math/algebra/generator.rs
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
```

Run `rustfmt` on each added `generator.rs` too: Cargo's formatter does not
discover modules referenced only through build-generated source. CI checks all
generator files recursively. Commit `Cargo.lock` when dependencies change.

Release builds favor compact code. Corpus JSON and local TOML parsing happen at
build time; only the selected dataset is decompressed at runtime.