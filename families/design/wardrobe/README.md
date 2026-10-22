# Wardrobe

Sixteen outfit generators split by occasion (`everyday`, `office`, `dinner`,
`trail`) and weather (`warm`, `mild`, `rain`, `cold`). Each prints the actual
clothing and accessories, one piece per line.

`outfit.rs` builds separates, dresses, jumpsuits, and matching sets. Twelve
palettes support neutral, tonal, and accent-led outfits. Matching jackets and
trousers share fabric and color. Dresses and jumpsuits take open-front layers;
skirts and dresses gain tights in cool weather. Each outfit has one pair of
shoes and one bag. Warm outfits use light fabrics; rain adds waterproof outerwear
and weather-resistant footwear; cold adds insulation, a middle layer, and gloves.
Trail outfits keep hiking base layers, trousers or shorts, socks, and a daypack.

`import_fabrics.py` verifies nineteen reviewed generic descriptors from the
Corpora fabrics file at revision `2e7adec11a0561696c236ae1fca82e88be173d42`, SHA-256
`6f4314342350c91f1457f3c20c0f37cf0c2149c12306c556302798a3ccae9684`, then
regenerates the leaf adapters. The vocabulary source supplies fabric names;
outfit construction is original. Brands and ambiguous textile classifications
are excluded. See `LICENSE.md` for the source and license.

Run `python -B families/design/wardrobe/import_fabrics.py`, then format touched
Rust files with `rustfmt --edition 2024`. Each leaf checks 500 generated outfits
for garment structure, weather coverage, silhouette variation, and diversity.
