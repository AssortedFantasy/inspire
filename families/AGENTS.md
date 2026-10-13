# Adding and maintaining the family tree

The filesystem defines the tree. Add or move directories and rebuild; there is
no required manifest, path ID field, or central registry to synchronize.

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

For example, `poems/haiku/author-name/records.json` creates three nested nodes.
Only the final node owns a dataset. The words `poems`, `haiku`, and the author
name have no special meaning to the sampler.

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
Text must be nonblank and cannot contain NUL (`\u0000`), the internal separator.
Provenance must be nonblank. All three fields are required; unknown fields fail
the build. Metadata stays in source; place any attribution that must accompany
the stimulus in `text`. Label invented material honestly.

See `poems/haiku/records.json` for the complete example. No configuration is
needed to use default weights. Records inside a dataset are sampled uniformly.
Rebuild and try `inspire --family poems/haiku --seed 42`.

## Add a procedural generator

Create a leaf directory and add `generator.rs` with this entry point:

```rust
use rand::{rngs::StdRng, Rng};

pub fn generate(rng: &mut StdRng) -> String {
    let number = rng.random_range(2..=9);
    format!("{number} + 0 = {number}")
}
```

The build compiles and registers it automatically. A wrong signature is a compiler
error. Put meaningful invariant tests in a `#[cfg(test)] mod tests` in this file;
they run under `cargo test`. See `math/algebra/generator.rs` for equality checks.

Return a nonblank stimulus without a trailing newline. Use only the supplied RNG
and bounded local computation. Do not print, perform runtime I/O, mutate external
state, or depend on the source directory being present. Keep helper modules as
files beside `generator.rs`, so moving the directory preserves relative references.
Format generator files explicitly and run the root README's checks.

## Move or subdivide a node

To group multiple nodes, create a parent directory and move them inside it.
Move each entire node directory so its local configuration and helper files follow.
The build derives new paths automatically; update any documentation and tests that
intentionally refer to the old paths. There are no compatibility aliases.

To turn a leaf into a branch, first move its payload into a new child directory.
For example, move `poems/haiku/records.json` into
`poems/haiku/generated/records.json` before adding author-specific sibling datasets.
Decide whether an existing local weight should remain on the new branch or follow
the old payload. A directory cannot be both a leaf and a branch.

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
branch's share. With equal-weight `poems` and `math`, mathematics receives half
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
to 100%, with displayed rounding. This is inspection data for maintainers, not
extra text added to ordinary inspiration output.

There are no per-record weights, history, motif tags, or automatic motif detection.
Curate repetitive records or split them into meaningful weighted subtrees. More
combinatorial outputs do not establish semantic variety.