# Architecture

## Directory-defined tree

The directories under `families/` define a rooted sampling tree. Each non-root
node optionally contains `node.toml` with a sibling-relative `weight` (default 1).
A node with child directories is a branch; `records.json` and `generator.rs`
identify the two leaf kinds. Required manifests and central weight files are not
part of the format. Paths are derived from directory names, joined with `/`.

The root is a branch with no user-configurable incoming weight. Ambiguous
payloads, a leaf with child directories, empty branches, symlinks, invalid directory
names, invalid records, malformed or unknown settings, and branches without a
positive-weight child fail the build. Disabled branches must still be valid
because explicit subtree selection can enter them.

## Build time

`build.rs` calls `build_support.rs` to recursively discover the tree in sorted
directory order. It emits a flat node array with branch child indexes and
precomputed sibling-weight totals. No maintained registry is required when
adding, moving, or renaming nodes.

For each dataset, the build joins UTF-8 record texts with NUL separators and
compresses them into one zlib-wrapped DEFLATE stream using `miniz_oxide`.
Record IDs and provenance stay in readable source JSON. Each leaf has its own
stream; the registry embeds its compressed bytes, record count, and decoded
length. TOML/JSON parsing is absent from the release runtime.

For each generator, the build emits a module reference to its discovered
`generator.rs` and registers `generate(&mut StdRng) -> String`. Rust checks the
signature; generator unit tests join the application test suite. Moving a
self-contained generator directory updates registration without code edits.

Generated files live in Cargo's `OUT_DIR`. Cargo tracks the entire family
directory recursively so additions, moves, removals, data changes, and local
weight edits trigger discovery again.

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

Weights are independent of subtree size. Regrouping nodes can change effective
probabilities even when all local weights remain unchanged. `--list` recursively
shows full paths, node kinds, local weights, and effective probabilities from the
root. Percentages are rounded to four decimal places; small nonzero values may
display as zero. Branch probabilities include all their descendants.

A zero-weight node cannot be selected from its parent. Explicit selection bypasses
the chosen node's incoming weight and all ancestors; it still obeys weights below
the chosen node. Thus a disabled subtree remains inspectable with `--family`.

The runtime reads no source or configuration files. It has no network requests,
persistent caches, history files, services, or interpreter dependency. Broken
stdout pipes exit quietly; other errors go to stderr with status 1. Normal help
remains the single line `Generates a bit of inspiration`; detailed usage is behind
the unadvertised `--help-full` flag.

## Limits and extension points

- There is one compressed block per dataset. Retrieval decompresses and scans
  that selected dataset. Large datasets may eventually need independently
  compressed, indexed blocks; select records before blocks to avoid size bias.
- Five haikus are too little data to establish an optimal codec. Compare complete
  executable size and startup time against uncompressed storage and other codecs
  as representative data becomes available.
- Calls are independent. Local weighting controls probabilities, not guaranteed
  sequence coverage. History and shuffled bags would be separate policy features.
- Weights are compiled in. There are no runtime plugins, per-record weights,
  motif annotations, or automatic rebalancing.
- Paths change when directories move; old paths are not aliases. Seeded outputs
  can change with tree structure, ordering, dependencies, weights, or content.
  Cross-version/platform RNG compatibility is not promised.
- The two initial leaves demonstrate the extension mechanism. Correct algebra
  and varied strings do not establish useful downstream creative effects.

## Validation and performance

Build-tree tests exercise recursive discovery, moving a subtree without metadata
edits, default and zero weights, and invalid layouts and records. Sampler tests
check weighted ticket intervals, subtree boundaries, disabled-node behavior, and
effective probabilities. CLI tests cover branch/leaf selection, seeded runs,
minimal help, and operation from outside the repository.

Use a release executable when measuring performance, not `cargo run`. Record
platform, binary size, invocation count, and whether launches are warm or cold.
Measure process launch-to-exit time for both dataset and generator paths; host
startup and antivirus can dominate tiny workloads.

The release profile uses size-oriented optimization, LTO, one codegen unit,
stripped symbols, and abort-on-panic. `.cargo/config.toml` statically links the C
runtime on Windows MSVC targets so no separate VC++ runtime installation is
required; operating-system libraries are still used. Keep `Cargo.lock` committed.