# Adding and maintaining the family tree

Add or move directories under `families/` and rebuild to change the sampling tree.

## Output quality (no AI slop)

Inspire outputs should read like crisp, high-entropy artifacts or authentic primary material, not LLM textbook exercises or defensive spec sheets.

- **No CYA definitions or convention footnotes.** Assume a competent reader. Do not explain what an atomic number is (`71 protons per nucleus and 71 electrons in a neutral atom`), that MIDI puts `C4 = 60` in 12-TET, that a SPICE netlist uses SI units and an ideal source, or that a free oscillator has "No external forcing." If the notation already carries the meaning, let it stand on its own.
- **No semicolon/colon key-value sludge.** Avoid the robotic GPT cadence of chaining terse labels, colons, and semicolons to pack in minor clarifications (`Label: X; condition: Y; convention: Z.`). Write natural prose when prose is needed, or just emit the object itself.
- **No fake variation.** Multiplying a balanced chemical equation by a random integer (`For a forward extent of 2 mol: consume...`), counting atoms in $k$ molecules (`2 molecules contain 24 atoms`), or slotting a 10-row color lookup table into a rigid `Top: / Bottom: / Footwear: / Accent:` checklist is padding, not entropy. Every random choice in a generator should produce genuine structural variety.
- **Prefer verbatim primary text over LLM paraphrases.** Real human writing has texture and variation that LLM summaries flatten out. When pulling from historical, technical, or literary public-domain sources, quote the text directly (with author/work credit) instead of paraphrasing it into "X describes Y, turning A into B."
- **Emit the artifact, not a prompt.** Print the mathematical object, recipe, circuit, game position, or quotation itself. Never print instructions telling the reader to "Design an outfit," "Construct a group," or "Imagine a position."
- **Keep metadata out of sampled text.** Put download URLs, checksums, snapshot dates, internal record IDs, and license boilerplate in `provenance` or `LICENSE*` files, not in the printed output.

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
- Prefer importing real datasets or verified public-domain texts over having an LLM bulk-author records. Keep download caches and raw files outside `families/` (for example under `target/`) and use `python -B` for helper scripts so `__pycache__` directories do not pollute the tree.
- Place required dataset notices in `LICENSE`, `LICENSE.txt`, or `LICENSE.md` inside the branch so the build embeds them for `inspire --data-licenses`. See [DATA_SOURCES.md](../DATA_SOURCES.md).
- When changing output formatting for an imported corpus, update both the importer script and `records.json` together.

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
- Put tests for real mathematical or structural invariants (valid moves, unique puzzle solutions, balanced equations, circuit nodal analysis) in `#[cfg(test)] mod tests`. Avoid brittle tests that only lock in boilerplate wording.

## Tree structure and weights

- **Branches** contain child directories. **Leaves** contain either `records.json` or `generator.rs` (never both, and never child directories). Empty directories and symlinks are rejected.
- Directory names must use lowercase ASCII letters, digits, and hyphens (`[a-z0-9-]`), starting with a letter or digit.
- Every node defaults to weight `1` among its siblings. Override a node's local weight with an optional `node.toml`:
  ```toml
  weight = 2
  ```
- A node's effective sampling probability is the product of sibling weight ratios along its path (`inspire --list` shows all weights and probabilities). Weight `0` disables selection through the parent while still allowing direct selection via `--family`.
- In homogeneous name collections (`people`, `objects/cars`, `natural-world/animals`, and `geography/subdivisions`), node weights match descendant record counts so every record has equal probability within the collection. Other branches weight by topic.
