# Procedure sources and replay

`import_data.py` imports 81 historical recipes, seven kitchen methods,
15 embroidery procedures, 21 sewing/mending procedures and 16 experiments. Every
input has a checked SHA256 in `SOURCES`; raw HTML lives outside the sampling
tree in `target/imports/procedures`. `baseline_records.json` preserves the exact
retired paraphrases and IDs as an unsampled migration archive. The importer uses
Beautiful Soup 4.12.3. Inline markup preserves source spacing; paragraph and
ingredient boundaries remain visible. Output carries author/work credit without
category introductions.
The importer also reproduces descendant record-count weights within each corpus
topic; incoming topic weights and procedural generator branches are preserved.

- Fannie Merritt Farmer, *The Boston Cooking-School Cook Book*, revised 1910
  edition: [Gutenberg 65061](https://www.gutenberg.org/ebooks/65061). Selects
  named recipes with ingredients and textual methods. Ingredient `div.line`
  blocks and h3/h4 boundaries are preserved. Page numbers, captions and wrappers
  are removed. Referenced tea, white-sauce, macaroni and potato foundations are
  appended. Tea's final medical claim and lemonade's storage claim are omitted.
  Preservation, raw-egg sauces, incomplete stock-dependent methods, food
  descriptions, obsolete medical advice and diagram-only material are excluded.
  These are historical excerpts, with period measures, product names and oven
  terms retained; they are not presented as newly tested modern recipes.
- L. Higgin, *Handbook of Embroidery*, 1880 edition:
  [Gutenberg 24964](https://www.gutenberg.org/ebooks/24964). Explicitly selected
  textual needle operations; dependent split/cross stitches include their
  foundations. Illustrations, unsupported botanical explanations and cleaning
  treatments are excluded. The author's historical stitch names are retained.
- Thérèse de Dillmont, *Encyclopedia of Needlework*:
  [Gutenberg 20776](https://www.gutenberg.org/ebooks/20776). Verbatim operations
  for seams, stitches, hems, gathering, thread joins, fastenings and mending.
  Adjacent foundation paragraphs are included where needed. Parenthetical and
  inline figure-navigation references are omitted. Linen darning excludes its
  figure-dependent exception and illustration commentary; the operation remains
  verbatim and intelligible without images.
- Gaston Tissandier, *Popular Scientific Recreations*, English translation and
  enlargement: [Gutenberg 52709](https://www.gutenberg.org/ebooks/52709).
  Self-contained operations and observations, including the walnut-shell ludion,
  cork attraction, coin inertia, water pressure, binocular vision and coloured
  bubbles. Explicit sentence boundaries cut off unrelated apparatus or obsolete
  explanations; no words are paraphrased. Parenthetical and inline figure
  navigation is omitted. Chemistry, fire experiments, and figure-dependent explanations
  are excluded.

The primary texts are public domain. The complete Gutenberg source notice is
retained in `LICENSE.txt` and embedded by the build. No images are imported.
Download URLs and hashes are also in `source_manifest.json`.

```powershell
python -B families/procedures/import_data.py --source-dir target/imports/procedures --check
python -B -m unittest discover -s families/procedures
```

The eleven Rust generators are original bounded implementations, with no
runtime I/O. Sorting traces test every pass, permutations and stable identity
ordering; lower-bound search checks interval invariants against linear scan.
Graph distances are checked against independent Floyd–Warshall matrices, and
reported Dijkstra paths are checked edge by edge. Kruskal trees are compared
with exhaustive spanning-tree enumeration. Every knapsack prefix and capacity
is compared with exhaustive subsets. Edit distance and its alignment are
compared with an independent breadth-first search over the binary edit graph.
Tarjan components are compared with mutual reachability, and Kahn orders and
cycle witnesses are checked over every simple directed four-vertex graph.
Graph instances ensure meaningful reachable subgraphs or rejected cycle edges.
The prose algorithm paraphrases have been retired; no third-party implementation
code or NIST definitions are copied.
