# Adding and maintaining the family tree

Add or move directories and rebuild to change the sampling tree.

## Output preferences

Prioritize the content printed by Inspire: variety, clarity, and interest.
See [human feedback](../reviews/human-feedback.md) for concrete examples. Its
sample frequencies reflect the review dashboard's selection method; do not infer
runtime weights or rebalance the tree from those frequencies.

- Lead with the material. Avoid introductions, repeated explanations, and generic
  boilerplate. A compact recipe, object, or construction is better than a
  description of the dataset that supplied it.
- Keep download URLs, repository links, checksums, retrieval dates, snapshot
  labels, license boilerplate, and internal record/paragraph/poem IDs in
  `provenance`, source notes, or bundled notices. Keep meaningful author/work
  credit for quotations and historical context where it helps understand them.
- Do not add editorial "this is X, not Y" or "these are X, not a claim of Y"
  framing. State the fact or assumption directly. Preserve mathematical negation,
  intrinsic rules, and wording inside authentic quotations.
- Include units, defining relations, and conditions needed to interpret the
  output. Integrate qualifications briefly instead of appending disclaimers.
- Use practical precision for approximate quantities; keep exact integers and
  fractions exact. Avoid long decimal dumps.
- Prefer ordinary `-` separators to repeated em dashes in generated framing.
  Preserve the wording and punctuation of literary quotations.

## Efficient work

- Use `gpt-6-luna` with high reasoning for delegated leaf implementation. Assign
  bounded tasks; one agent may apply a shared formatting fix to several leaves.
- Fix shared boilerplate in its common formatter, then pass over affected leaves
  for content-specific problems. Update records and their producer together so
  regeneration preserves the change.
- Commit completed leaves or small coherent batches promptly. Do not wait for
  an entire family or a separate chain of reviewers.
- Test at integration points after a coherent batch. Avoid separate Cargo builds,
  exhaustive seed sweeps, and repeated source audits for each leaf. Add or adapt
  substantive invariant tests when behavior changes, not for wording alone.
- Inspect representative outputs and feedback examples, fix concrete defects,
  and finish. Review manifests and a dedicated reviewer per leaf are not required
  for routine maintenance. Do not restart historical review queues unless asked.
- Preserve provenance and bundled notices. Output cleanup does not require new
  downloads or reopening settled sourcing decisions.

## Content rules

Emit self-contained material: a fact, name, quotation, mathematical object,
position, puzzle, procedure, or generated artifact. Do not write prompts,
assignments, creative challenges, or instructions directing the consuming agent's
work. Do not substitute a request to create something for the thing itself.

Mathematical constructions describe an object and its defining relations; they
do not tell the reader to construct, prove, or calculate it. For example, replace
"Construct G x H using componentwise multiplication" with "The direct product
G x H has multiplication (g,h)(g',h')=(gg',hh') and identity (e_G,e_H)."
Apply the same distinction across families: emit an outfit, not "Design an
outfit"; emit a concrete game position, not "Imagine a game position."

A procedure's steps or a puzzle's intrinsic rules can be part of the material.
Include the actual procedure or puzzle, without adding an unrelated task for
the consuming agent. Review meaning, not just imperative verbs; keyword checks
cannot reliably distinguish these cases.

## Recommended corpus workflow

Prefer downloading an existing, reusable dataset and transforming it into the
record format over having an LLM bulk-author a list. Use agents to find sources,
write importers, and review results. Procedural generators remain appropriate
for material that benefits from constrained random construction.

1. Inspect the source data and its reuse terms. Record the exact download URL,
   version or revision, and applicable attribution requirements.
2. Download at development time and use a reproducible transformation. Preserve
   stable source IDs, pin the input version or checksum where possible, and
   document filtering, corrections, and category mappings. See
   [the geography importer](geography/import_data.py) for an example.
3. Keep raw downloads and temporary files outside the sampling tree. Commit the
   resulting records and useful importer/source notes; runtime stays offline.
   Use `python -B` when importing helper modules so `__pycache__` directories do
   not become invalid sampling nodes. Put download caches under `target/` or the
   system temporary directory.
4. Validate the schema, then spot-check the actual emitted text across categories.
   Check for prompt-like framing, errors, duplicates, misleading labels, and
   repetitive filler. A large record count or valid JSON does not establish quality.

When no suitable dataset exists, use verified primary material or a bounded
generator and document that choice. A reference URL alone is not evidence that
an LLM-authored record was downloaded from, or checked against, that source.

Store required full dataset notices in a file named `LICENSE`, `LICENSE.txt`, or
`LICENSE.md` within the associated branch. The build embeds those files for
`--data-licenses`. Keep source credit and modification notes there and in
provenance rather than repeating long notices in sampled text.
See [dataset redistribution notes](../DATA_SOURCES.md).

## Node rules

- A directory containing child directories is a branch.
- A directory containing `records.json` is a corpus leaf.
- A directory containing `generator.rs` is a procedural generator leaf.
- A leaf cannot contain child directories, and cannot contain both payload types.
- Empty nodes fail the build. Ordinary files such as `README.md` do not create a
  payload; extra helper Rust files may live beside a generator.
- Directory names start with a lowercase ASCII letter or digit and contain only
  lowercase ASCII letters, digits, and hyphens. CLI paths use `/` on every OS.
- The root `families/` is only a container for child nodes. It has no incoming
  weight, so do not put `node.toml` or a payload directly at the root.
- Symlinks are rejected to keep discovery unambiguous and avoid cycles.

## Add a corpus

Create a leaf directory anywhere under a branch and add `records.json`:

```json
[
  {
    "id": "stable-record-id",
    "text": "The exact stimulus to print.\nLine breaks are preserved.",
    "provenance": "Origin, source URL and redistribution terms where applicable."
  }
]
```

The array must be nonempty. IDs must be nonblank and unique within this dataset.
Text must be nonblank and cannot contain NUL (`\u0000`).
Provenance must be nonblank. All three fields are required; unknown fields fail
the build. Metadata stays in source; sampled text focuses on the material.
Label invented material honestly in its provenance.

See the corpora under `geography/` for examples. Records are sampled uniformly.
Rebuild and try `inspire --family geography --seed 42`.

Prefer factual names, original source-grounded explanations, and verified
public-domain quotations. Record precise source URLs and relevant reuse terms;
do not label paraphrases as quotations or unsupported material as verified.
Keep song metadata separate from copyrighted lyrics. Generated artifacts should
have useful constraints and tests for their substantive invariants (legal moves,
unique puzzle solutions, balanced reactions), not just output formatting.

## Add a procedural generator

Create a leaf directory and add `generator.rs` with this entry point:

```rust
use rand::{rngs::StdRng, Rng};

pub fn generate(rng: &mut StdRng) -> String {
    let number = rng.random_range(2..=9);
    format!("{number} + 0 = {number}")
}
```

The build compiles and registers it automatically. Substantive invariant tests
belong in `#[cfg(test)] mod tests`; run them at integration points under `cargo test`.
See `math/algebra/polynomials/identities/generator.rs` for equality checks.

Return a nonblank stimulus without a trailing newline. Use only the supplied RNG
and bounded local computation. Do not print, perform runtime I/O, mutate external
state, or depend on the source directory being present. Keep helper modules as
files beside `generator.rs`, so moving the directory preserves relative references.
See [root AGENTS.md](../AGENTS.md#tips) for formatting and checks.

## Move or subdivide a node

To group multiple nodes, create a parent directory and move them inside it.
Move each entire node directory so its local configuration and helper files follow.
The build derives new paths automatically; update any documentation and tests that
refer to the old paths.

To turn a leaf into a branch, first move its payload into a new child directory.
For example, move `literature/quotations/records.json` into
`literature/quotations/classical/records.json` before adding other sibling datasets.
Decide whether an existing local weight should remain on the new branch or follow
the old payload.

## Tune weights

Every node defaults to weight 1 among its siblings. To override it, add an optional
`node.toml` inside that node's directory:

```toml
weight = 2
```

Weights are nonnegative 32-bit integers. Unknown settings are errors. An empty
configuration file also uses the default weight.

The default probability of reaching a node is the product of the sibling selection
probabilities along its path. Descendant counts and dataset sizes do not affect a
branch's share. With equal-weight `literature` and `math`, mathematics receives half
the calls even if it has many more leaves. Giving `math` weight 2 changes its
share to two-thirds.

Weight 0 excludes a subtree from selection through its parent. Explicit selection
of that branch or a descendant bypasses incoming weights; sampling within the
chosen branch still honors its children's weights. Every branch must have at least
one positive-weight child, including disabled branches.

Regrouping changes probabilities. Three equal top-level leaves get one-third each.
Moving two under a new default-weight parent gives that parent half the calls and
its children one-quarter each. Give the new parent weight 2 to preserve thirds
when the other top-level node has weight 1.

After rebuilding, `inspire --list` displays local weights and effective root
probabilities. Branch rows include their whole subtree; leaf probabilities sum
to 100%, with displayed rounding.

The current editorial policy is documented in [the content audit](../docs/content-audit.md).
In homogeneous name collections (`people`, `objects/cars`, `natural-world/animals`,
and `geography/subdivisions`), weights equal descendant record counts at every
split. Update the affected leaf and ancestor weights when adding or moving
records; an integration test checks equal conditional per-record probability.
Other branches balance topics rather than records. Check the resulting leaf and
per-record probabilities before treating a folder reorganization as cosmetic.
