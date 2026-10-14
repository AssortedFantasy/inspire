# Procedure sources and replay

`import_data.py` rebuilds the 60 original records in their new locations and
imports 81 historical recipes and 15 textual embroidery procedures. Every
input has a checked SHA256 in `SOURCES`; raw HTML lives outside the sampling
tree in `target/imports/procedures`. `baseline_records.json` preserves the exact
original records and IDs. The importer uses Beautiful Soup 4.12.3.

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
  [Gutenberg 20776](https://www.gutenberg.org/ebooks/20776). Existing twelve
  original sewing paraphrases are preserved; the downloaded edition is pinned
  for subsequent section reviews.

The primary texts are public domain. The complete Gutenberg source notice is
retained in `LICENSE.txt` and embedded by the build. No images are imported.
Download URLs and hashes are also in `source_manifest.json`.

```powershell
python -B families/procedures/import_data.py --source-dir target/imports/procedures --check
python -B -m unittest discover -s families/procedures
```

The nine new Rust generators are original bounded implementations, with no
runtime I/O. Sorting traces test every pass, permutations and stable identity
ordering; lower-bound search checks interval invariants against linear scan.
Graph distances are checked against independent Floyd–Warshall matrices, and
reported Dijkstra paths are checked edge by edge. Kruskal trees are compared
with exhaustive spanning-tree enumeration. Every knapsack prefix and capacity
is compared with exhaustive subsets. Edit distance and its alignment are
compared with an independent breadth-first search over the binary edit graph.
The original algorithm paraphrases retain their Princeton companion references;
no third-party implementation code or NIST definitions are copied.
