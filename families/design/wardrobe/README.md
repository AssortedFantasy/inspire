# Wardrobe

Sixteen original outfit generators split first by occasion (`everyday`, `office`,
`dinner`, `trail`), then by weather (`warm`, `mild`, `rain`, `cold`). The former
`design/wardrobe` selector remains a branch selecting among those contexts.

Ten coordinated palettes assign neutral tops, dark bottoms, compatible shoes,
and an accent on an actual middle layer or accessory. Clothing weight and
formality follow the scenario; trousers and skirts with seasonal coverage are
available outside the trail contexts. Warm outfits omit insulation. Rain outfits
have waterproof outer layers and appropriate footwear. Cold outfits include a
middle layer, an insulated outer layer, full-length coverage, and gloves. Layers
are explicitly sized to fit together. These are styling scenarios, not thermal
comfort guarantees or equipment prescriptions for technical terrain.

`import_fabrics.py` reproducibly builds the leaf source. It checks the downloaded
Corpora fabrics file at revision `2e7adec11a0561696c236ae1fca82e88be173d42`, SHA-256
`6f4314342350c91f1457f3c20c0f37cf0c2149c12306c556302798a3ccae9684`:
[exact vocabulary source](https://raw.githubusercontent.com/dariusk/corpora/2e7adec11a0561696c236ae1fca82e88be173d42/data/materials/fabrics.json).
Nineteen reviewed generic descriptors are accepted. Brands, historical/regional
textile names, processes, and ambiguous classifications are excluded. The source
supplies names only; garment pairings, performance descriptors, seasonal rules,
palettes, and tests are original. No insulating or waterproof behavior is inferred
from a source name. The unrelated `fibers` and `technical-fabrics` lists were
reviewed and rejected because they mix materials, processes, products, and brands.

Run `python -B families/design/wardrobe/import_fabrics.py`, then format every
generated `generator.rs` with `rustfmt --edition 2024`. Each leaf tests 500 samples
for its weather/formality constraints, realized palette, and output diversity.
