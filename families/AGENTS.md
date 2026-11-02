# Adding and maintaining the family tree

Add or move directories under `families/` and rebuild to change the sampling tree.
See [human feedback](../docs/reviews/human-feedback.md) for concrete review notes.

## Output quality (no AI slop)

Focus on the **entropic core**—the actual artifact, fact, or quotation—and strip away scaffolding, defensive clarifications, and GPT prose patterns.

Keep each output within **512 `o200k_base` tokens**. Audit with
`scripts/token_count.py` as described in the root agent instructions. For
generators, bound the artifact's dimensions and remove redundant traces or
representations. For corpora, choose complete, distinctive passages or stanzas
from the human source, retaining attribution. Do not fit the cap by arbitrary
truncation or LLM paraphrasing.

- **No CYA definitions or rulebook explanations.** Assume a competent reader. Do not explain how Sudoku or Nim works, that poker uses a 52-card deck with no jokers, what an atomic number means (`29 protons per nucleus and 29 electrons in a neutral atom`), that ionic solids form lattices, that MIDI puts `C4 = 60` in 12-TET, or that a SPICE netlist uses ideal sources and SI units. Print the puzzle, hand, reaction, or netlist directly.
- **No tree-path or category scaffolding.** Do not leak the directory taxonomy into the output as a header (`Experiment protocol: Blind spot card`, `Word history: apron`, `Poetic form: Shakespearean sonnet`, `Algorithm: Kruskal tree`, `Type: Drums`). And never double-introduce an item (`Algorithm: Kruskal tree` followed by `Kruskal's minimum spanning tree:`). Start directly with the material.
- **No semicolon/colon key-value sludge.** Avoid the GPT habit of chaining terse clauses with colons and semicolons (`Ideal hydraulic press: incompressible liquid... Input piston diameter 17 mm; output diameter 68 mm...`). When presenting multiple fields (like biological taxonomy and trait estimates), use clean line breaks instead of a semicolon wall.
- **Vary structure, not just numbers—and never print every representation at once.** Instead of printing `ISO week date: 1982-W41-6; Gregorian date: 1982-10-16 (Saturday).`, just print `1982-W41-6`. Avoid fake variation like multiplying a chemical reaction by a random integer (`For a forward extent of 1 mol...`) or plugging numbers into a rigid paragraph template. If erasing the numbers makes every output identical, the generator needs structural or topological variety.
- **Sample on the boundary (and mix regimes).** Uniform random parameters usually produce bland textbook exercises. Select a distinct structural regime first, then construct a valid instance—elections where voting rules disagree alongside clean Condorcet winners, games with mixed-strategy tension, circuits at resonance or critical damping, or matchings where proposer- and receiver-optimal solutions diverge. Construct valid instances directly rather than relying on unbounded rejection sampling.
- **Trim noisy float precision and trailing zeros.** Write `9` or `2.2e3`, not `9.00000e0` or `4800.000 N`.
- **Ban LLM paraphrasing.** Paraphrasing a good human source collapses its natural texture into flat ChatGPT cadence (`A packet that already sinks or floats too strongly may not respond as a useful diver`). Use verbatim public-domain text (with author/work credit) or concise factual data—never LLM-authored paraphrases or out-of-context figure/animation captions.
- **Emit the artifact, not a prompt.** Never print instructions telling the consuming agent to "Design an outfit" or "Construct a group." Keep URLs, checksums, record IDs, and license text in `provenance` or `LICENSE*` files.

## Corpora (`records.json`)

A corpus leaf contains `records.json`:

```json
[
  {
    "id": "stable-record-id",
    "text": "The exact stimulus to print.\nLine breaks are preserved.",
    "provenance": "Origin, source URL, and redistribution terms."
  }
]
```

- Records are sampled uniformly within a leaf.
- `id`, `text`, and `provenance` are required and must be nonblank. `id` values must be unique within the file, and `text` cannot contain NUL (`\u0000`). Unknown fields fail the build.
- Import real datasets or verified public-domain texts rather than having an LLM author or paraphrase records. Keep download caches outside `families/` (for example under `target/`) and run helper scripts with `python -B` so `__pycache__` directories do not pollute the tree.
- Place required dataset notices in `LICENSE`, `LICENSE.txt`, or `LICENSE.md` inside the branch so the build embeds them for `inspire --data-licenses`. See [DATA_SOURCES.md](../DATA_SOURCES.md).
- Update both the importer script and `records.json` together when changing formatting.

## Procedural generators (`generator.rs`)

A generator leaf contains `generator.rs`:

```rust
use rand::{rngs::StdRng, Rng};

pub fn generate(rng: &mut StdRng) -> String {
    let number = rng.random_range(2..=9);
    format!("{number} + 0 = {number}")
}
```

- The build discovers and compiles `generator.rs` automatically. Format touched generators with `rustfmt --edition 2024 <path>`.
- Return a nonblank string with no trailing newline. Use only the provided `StdRng` and bounded local computation (no I/O or external state). Helper `.rs` files may live beside `generator.rs`.
- Put tests for substantive mathematical or structural invariants in `#[cfg(test)] mod tests`. Do not write tests that lock in boilerplate wording or disclaimer strings.

## Tree structure and weights

- **Branches** contain child directories. **Leaves** contain either `records.json` or `generator.rs` (never both, and never child directories). Empty directories and symlinks are rejected.
- Directory names must use lowercase ASCII letters, digits, and hyphens (`[a-z0-9-]`), starting with a letter or digit.
- Every node defaults to weight `1` among its siblings. Override a node's local weight with an optional `node.toml`:
  ```toml
  weight = 2
  ```
- A node's effective sampling probability is the product of sibling weight ratios along its path (`inspire --list` shows all weights and probabilities). Weight `0` disables selection through the parent while still allowing direct selection via `--family`.
- Watch out for **small-leaf repeat collisions**: a leaf with only a few records (or a tiny generator space) sitting on a shallow, high-weight branch will repeat single items far too often. Scale weights so tiny leaves do not dominate sampling. In homogeneous collections (`people`, `objects/cars`, `natural-world/animals`, `geography/subdivisions`), weights match descendant record counts so every record has equal probability.
