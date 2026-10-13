# Working on inspire

Read `README.md`, then `docs/architecture.md`. For any family-tree change, also
read `families/AGENTS.md`. `REASEARCH.md` motivates experiments; it is not a
settled implementation specification.

## Project map

- `families/`: the sampling tree; directories are branches or payload leaves.
- `families/**/node.toml`: optional local sibling weight; defaults to 1.
- `build.rs`: invokes tree discovery and code generation.
- `build_support.rs`: validates the tree, compresses datasets, emits registration.
- `src/tree.rs`: hierarchical sampling and developer-facing tree inspection.
- `src/main.rs`: selected-leaf rendering, selective decompression, and output.
- `src/cli.rs`: argument handling; keep the CLI small and predictable.
- `tests/build_tree.rs`: discovery, relocation, and invalid-input coverage.
- `tests/cli.rs`: executable-level behavior and standalone operation.
- `docs/architecture.md`: architectural choices and explicit limitations.

## Engineering rules

- Preserve the single-executable, offline runtime. Do preparation at build time.
- The directory layout is the source of truth. Derive paths using slash separators.
  Do not introduce a central registry, global weight file, or duplicated path IDs.
- Each directory is a branch, a corpus leaf, or a generator leaf. Reject ambiguous
  combinations rather than silently ignoring a payload or descendants.
- Weights are local to siblings. Counts of records or descendants must not
  implicitly increase a branch's probability. Moving a subtree carries its local
  configuration with it; effective probabilities can change.
- Keep source corpora readable and retain provenance. Do not commit compressed
  artifacts or hand-edit anything under `target/`.
- Use the supplied RNG inside generators. Do not read clocks, create another RNG,
  access the network, launch subprocesses, or depend on the working directory.
- Keep generated material bounded in size and computation. No unbounded grammar
  expansion or retry loops.
- Normal stdout is only the stimulus. Diagnostics belong on stderr. Avoid adding
  a repeated explanation or interpreting every stimulus for the consuming agent.
- Keep `--help` and `-h` to `Generates a bit of inspiration`. Detailed usage lives
  behind `--help-full`; do not advertise it in normal help or error messages.
- Preserve record IDs when editing content. Moving or renaming a node changes its
  CLI path. Seeds are reproducible within the same build and arguments, not a
  compatibility promise across tree, corpus, or policy changes.
- Keep dependencies modest. Build-only parsers belong in `[build-dependencies]`;
  tests may use the same parsers without linking them into the shipped executable.
- Update nearby docs when changing the extension contract or CLI behavior.
- Do not claim the placeholder families establish broad cultural coverage or
  measurable creative benefit.

## Validation

Run the checks in `README.md`. Format every changed generator explicitly with
`rustfmt --edition 2024 <path>`; `cargo fmt` alone does not find generated modules.
For generators, test meaningful invariants such as mathematical correctness.
Verify local weights, subtree selection, and effective probabilities when changing
the sampler. Smoke-test the release executable from outside the repository.
Do not change or regenerate `REASEARCH.md` incidentally.