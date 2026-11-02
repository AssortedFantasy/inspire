# Usage

`inspire` runs offline as a standalone executable with all corpora and procedural generators compiled in.

## CLI Options

```sh
inspire                             # Sample from the full tree
inspire --family games/chance/dice  # Sample from a specific branch or leaf
inspire --seed 42                   # Reproduce a sample with a 64-bit seed
inspire --json                      # Emit text, leaf path, and record index as JSON
inspire --list                      # List all branches, leaves, weights, and probabilities
inspire --data-licenses             # Print embedded dataset attributions and licenses
inspire --dashboard                 # Launch local web explorer at http://127.0.0.1:8765/
inspire --help-full                 # Show all options
```

## Families

26 top-level families combine curated corpora with procedural generators. Pass any branch or leaf path to `--family`, or omit it to sample the full tree.

| Family | Material |
| --- | --- |
| `arts` | Painting, drawing and print catalogues, and dramatic stage directions |
| `belief` | Mythology episodes and wisdom-text passages |
| `chemistry` | Chemical elements, compounds, and balanced equations |
| `computing` | Data structures, automata, SQL, networking, encodings, concurrency, and binary32 |
| `cryptography` | Classical ciphers, modular arithmetic, secret shares, and error-correcting codes |
| `design` | Wardrobes, circuits, weave and knitting drafts, page grids, imposition, and blazons |
| `food` | Food composition, ingredients, cultivar names, and culinary vocabulary |
| `games` | Legal board positions, opening books, card deals, dice, and puzzles |
| `geography` | Settlements, administrative regions, countries, landforms, and waters |
| `history` | Customs, inventions, and everyday material practices |
| `language` | Idioms, proverbs, etymologies, word senses, and grammatical constructions |
| `literature` | Public-domain quotations, aphorisms, and literary passages |
| `math` | Exact algebra, analysis, geometry, graph objects, topology, and probability |
| `materials` | Building materials, stones, textiles, metals, fibres, and packaging |
| `mechanisms` | Mechanical, fluid, and control mechanisms |
| `music` | Classical works, instruments, scales, harmony, rhythms, and melodic phrases |
| `natural-world` | Species and biological traits, adaptations, plants, astronomical bodies, and phenomena |
| `objects` | Historical museum artifacts and car make, model, and year combinations |
| `people` | Notable people grouped by field |
| `philosophy` | Syllogisms, truth tables, modal frames, and countermodels |
| `physics` | Mechanics, optics, waves, circuits, heat cycles, acoustics, orbits, and scaling |
| `procedures` | Algorithms, crafts, cooking, and historical experimental methods |
| `random` | Dates, colors, identifiers, signals, contingency tables, orders, and polygons |
| `society` | Elections, matchings, games, auctions, allocations, and market regimes |
| `systems` | Queues, ecology, reliability, project schedules, and network flows |
| `transport` | Airfields, railway timetables, navigation legs, and maritime marks |

## Weighting and Reproducibility

Weights are local to siblings so large datasets do not drown out smaller topics. Use `inspire --list` to inspect local weights and effective root probabilities.

`--seed` reproduces samples within a build. Changing the tree, records, generators, or RNG version can change which output a seed selects.

For internal layout and build details, see [docs/architecture.md](architecture.md). For creative benchmark results, see [docs/creative-validation.md](creative-validation.md).
