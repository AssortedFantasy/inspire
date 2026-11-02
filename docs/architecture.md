# Architecture

## Directory-defined tree

Directories under `families/` define the sampling tree. Branches contain child
directories; leaves contain `records.json` or `generator.rs`. CLI paths are
directory names joined with `/`. See [family rules](../families/AGENTS.md) for
the format, validation, and weighting examples.

## Build time

`build.rs` calls `build_support.rs` to recursively discover the tree in sorted
directory order. It emits depth-first nodes, packed child indexes, and
precomputed sibling-weight totals. Each branch references a contiguous range in
the child-index array; those indexes need not be adjacent in the node array.

Record texts are packed without separators into chunks of at most 64 KiB of
decoded UTF-8. Records remain whole; oversized records occupy their own chunks.
Chunks can span corpuses. Each chunk is independently compressed as a zlib-wrapped
DEFLATE stream using `miniz_oxide`; all streams share one embedded byte array.
Record IDs and provenance stay in source JSON. TOML and source JSON parsing is
absent from the release runtime; the dashboard uses JSON for its local API.

For each generator, the build emits a module reference to its discovered
`generator.rs` and registers `generate(&mut StdRng) -> String`. Generator unit
tests join the application test suite.

`DATA_SOURCES.md` and family files named `LICENSE`, `LICENSE.txt`, or `LICENSE.md`
are also embedded, in sorted path order. `--data-licenses` prints the resulting
dataset notices without initializing an RNG or accessing source files.

Generated files live in Cargo's `OUT_DIR`. Changes under `families/` trigger
discovery again.

## Embedded layout

`src/layout.rs` defines the shared build/runtime descriptors. All indexes, counts,
and byte offsets are checked at build time to fit `u32`; weight totals use `u64`.

| Array | Contents |
| --- | --- |
| `NODES` | Local weight and branch, corpus, or generator source |
| `CHILDREN` | Node indexes; branches store a first index and count |
| `NAMES`, `NAME_BYTES` | Local name byte ranges, indexed alongside nodes |
| `CORPUSES` | First record index and record count |
| `RECORDS` | Chunk index, decoded byte offset, and byte length |
| `CHUNKS` | Compressed byte offset, compressed length, and decoded length |
| `COMPRESSED_BYTES` | Concatenated independent compressed streams |
| `GENERATORS` | Registered generator functions |

Full paths are not embedded. Explicit family lookup walks path segments using
binary search among sorted child names. Listing constructs paths with one reusable
buffer. Plain-text sampling only reads nodes and child indexes, without accessing
names. JSON output reconstructs the selected leaf path from its ancestors.

## Runtime and probabilities

1. Parse the CLI. Help, version, and listing do not initialize an RNG.
2. Start at the root, or resolve an explicit `--family PATH` to a branch or leaf.
3. Initialize a seeded RNG, or seed it from the operating system.
4. At each branch, draw an integer ticket within the sum of child weights and
   select the corresponding child. Repeat until reaching a leaf.
5. For a corpus, choose a uniform record index and decompress only its chunk.
   Extract the indexed byte range and validate that record's UTF-8. Retrieval
   reuses the decompression buffer for the output string. The decoded-length bound
   limits decompression. For a generator, call its function with the same RNG.
6. Write the stimulus plus a newline and exit.

`--json` uses the same RNG draws and emits `text`, `path`, `kind`, and
`record_index` (zero-based within a corpus, or null for a generator). The external
human review dashboard invokes this mode with a random seed and optional family,
so its distribution and text match ordinary CLI sampling. Source record IDs are
attached by Python only when the source text matches the embedded record; they
are not embedded in the executable.

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
pipes exit quietly; other errors go to stderr with status 1. `--help` prints a
one-line description; `--help-full` shows usage and options.

## Limits

- Retrieval decompresses one whole chunk, including other records packed into it.
- Oversized records exceed the normal chunk limit; retrieval scales with their size.
- Packed arrays and byte buffers must each fit the 32-bit layout limits.
- Calls are independent; repeats are possible.
- Weights are compiled in; edits require rebuilding.
- Moving directories changes CLI paths; old paths are not aliases.

## Content review

`scripts/review_content.py` inventories the source tree, computes effective leaf
and per-record probabilities, detects normalized duplicate texts, and exports
representative records for editorial review. With `--binary PATH`, it starts a
temporary loopback dashboard and samples generator seeds through the existing
API, avoiding one executable launch per output. It closes the server afterward.
The default checks 128 seeds per generator; `--samples` controls how many outputs
are included in the JSON report. Distinct text counts measure exact repetition,
not structural variety or factual quality. Leaf-specific invariant tests and
human review remain necessary.

The report also ranks corpus leaves by per-record exposure and computes the
probability that two independent root draws select the same corpus record. This
excludes generator collisions and duplicate text across leaves. With
`--weighted-samples N --seed INTEGER --binary PATH`, it includes actual default
CLI draws and their exact reproduction seeds alongside the per-leaf samples.
Use both views: leaf review finds local problems, while weighted draws reveal
what ordinary use repeatedly exposes.

Historical agent review ledgers and their checker live under `docs/archive/`.
Current maintenance uses representative output inspection and checks at
integration points. The human review tool in `scripts/review_dashboard.py`
serves assets from `scripts/review-dashboard/` and appends feedback to
`docs/reviews/human-feedback.md` by default.

`scripts/token_count.py` uses pinned [tiktoken](https://github.com/openai/tiktoken)
for local token counts. Run it with `uv run --cache-dir target/uv-cache` to install
its inline dependency. `count [FILE]` counts a UTF-8 file or stdin as one text;
`count --jsonl [FILE]` adds `tokens` to each object's decoded `text`, preserving
other fields. It preserves whitespace and treats special-token spellings as
ordinary text. Counts cover sample text only, excluding chat framing and JSON
metadata. `--encoding` defaults to `o200k_base`; counts differ across encodings.

`audit --binary PATH [--family PATH] --output target/token-audit.json` checks
every source corpus record and samples 128 adjacent seeds per generator through
one temporary dashboard process. `--seeds` and `--seed` set the generator range.
Build first: corpora come from the current source tree, generators from the
binary. Leaf paths and corpus counts are checked for mismatch; this does not
detect stale content with unchanged paths/counts. Disabled leaves are included.
The report includes exact token-count frequencies, ranks leaves by maximum
length, and retains all over-limit texts with source IDs or CLI reproduction
seeds. Generator maxima describe the sampled
seeds, not a proven bound. `--max-tokens` defaults to a review threshold of 256.
Exit codes are 0 within the threshold, 1 when exceeded, and 2 for invalid input
or operational errors. For `count`, the threshold is optional.

Initial dependency and encoding downloads need network access; cached runs need
no network or API key. Encoding data stays in `target/tiktoken-cache` unless
`TIKTOKEN_CACHE_DIR` is set. To run the focused tests:

```sh
uv run --cache-dir target/uv-cache --with tiktoken==0.12.0 python -B -m unittest discover -s scripts -p test_token_count.py
```

Chess opening integration tests use the `shakmaty` development dependency for
complete legal, canonical SAN move histories. This dependency is confined to
tests and does not enter the release executable.

## Dashboard

`--dashboard [--port INTEGER]` starts a loopback HTTP server in
`src/dashboard.rs`. Its HTML, CSS, and JavaScript under `src/dashboard/` are embedded
with `include_str!`; there is no separate build or runtime dependency. The server
prints its URL, accepts GET requests at that loopback address, and closes each
connection after a response. Requests have bounded headers and socket timeouts.

`/api/tree` exposes node IDs, names, paths, source kinds, local weights, record
counts, and effective probabilities. The UI sizes sibling rectangles by local
weight, previews children on hover or keyboard focus, and keeps the chosen path
above the output stream. Zero-weight children are separate inspectable controls.
Dashboard percentages use at most one decimal place; positive values below
0.1% display as `<0.1%`. Calculations and rectangle widths retain full precision.

`/api/outputs?node=ID&seed=U64&offset=U64` returns up to 12 outputs and the next
offset. Datasets shuffle record indexes from the stream seed, then page through
that permutation without replacement. They terminate at the record count.
Generators use `seed.wrapping_add(offset + batch_position)` for each output and
initialize the same RNG used by explicit CLI leaf sampling. Each output includes
its seed as a decimal string to avoid JavaScript integer precision loss and allow
CLI reproduction. Generator streams have no dataset-style endpoint.

Each dataset batch decodes a selected chunk once and reuses it for all records
in that batch. The cache lives only for the request, is bounded by its 12 selected
records, and stores compact copies of the selected texts. Ordinary CLI sampling
still decodes only one chunk and reuses that buffer for its output string.

The browser appends outputs in normal document flow using an intersection
observer, with a manual load/retry button. Changing the path or restarting a
stream cancels pending fetches and discards stale responses. The server processes
one request at a time to bound simultaneous generation and decompression.

## Release builds

The release profile uses size optimization, LTO, one codegen unit, stripped
symbols, and abort-on-panic. `.cargo/config.toml` statically links the C runtime on
Windows MSVC targets.

Measure launch-to-exit time using the release executable directly. Record the
platform, binary size, invocation count, and whether launches are warm or cold.
Check both corpus and generator paths; startup and antivirus can dominate runtime.
