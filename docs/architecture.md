# Architecture

## Directory-defined tree

Directories under `families/` define the sampling tree. Branches contain child
directories; leaves contain `records.json` or `generator.rs`. CLI paths are
directory names joined with `/`. See [family rules](../families/AGENTS.md) for
the format, validation, and weighting examples.

## Build time

`build.rs` calls `build_support.rs` to recursively discover the tree in sorted
directory order. It emits a flat node array with branch child indexes and
precomputed sibling-weight totals.

For each dataset, the build joins UTF-8 record texts with NUL separators and
compresses them into one zlib-wrapped DEFLATE stream using `miniz_oxide`.
Record IDs and provenance stay in readable source JSON. Each leaf has its own
stream; the registry embeds its compressed bytes, record count, and decoded
length. TOML/JSON parsing is absent from the release runtime.

For each generator, the build emits a module reference to its discovered
`generator.rs` and registers `generate(&mut StdRng) -> String`. Generator unit
tests join the application test suite.

Generated files live in Cargo's `OUT_DIR`. Changes under `families/` trigger
discovery again.

## Runtime and probabilities

1. Parse the CLI. Help, version, and listing do not initialize an RNG.
2. Start at the root, or resolve an explicit `--family PATH` to a branch or leaf.
3. Initialize a seeded RNG, or seed it from the operating system.
4. At each branch, draw an integer ticket within the sum of child weights and
   select the corresponding child. Repeat until reaching a leaf.
5. For a dataset, choose a uniform record index, decompress only that dataset,
   and extract its text. The decoded-length bound limits decompression.
   For a generator, call its function with the same RNG.
6. Write the stimulus plus a newline and exit.

For a node c under parent p:

```text
P(c | p) = weight(c) / sum(weights of p's children)
P(c)     = P(p) * P(c | p)
```

Each child's weight defaults to 1. Weights are independent of subtree size.
`--list` shows paths, node kinds, local weights, and effective root probabilities.
Percentages are rounded to four decimal places; small nonzero values may display
as zero. Branch probabilities include all their descendants.

A zero-weight node cannot be selected from its parent. Explicit selection bypasses
the chosen node's incoming weight and all ancestors; it still obeys weights below
the chosen node.

The executable runs offline without source or configuration files. Broken stdout
pipes exit quietly; other errors go to stderr with status 1. `--help` prints
`Generates a bit of inspiration`; `--help-full` shows usage and options.

## Limits

- Retrieval decompresses the entire selected dataset and scans to the record.
- Calls are independent; repeats are possible.
- Weights are compiled in; edits require rebuilding.
- Moving directories changes CLI paths; old paths are not aliases.

## Release builds

The release profile uses size optimization, LTO, one codegen unit, stripped
symbols, and abort-on-panic. `.cargo/config.toml` statically links the C runtime on
Windows MSVC targets.

Measure launch-to-exit time using the release executable directly. Record the
platform, binary size, invocation count, and whether launches are warm or cold.
Check both corpus and generator paths; startup and antivirus can dominate runtime.
