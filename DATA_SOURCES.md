# Dataset sources and redistribution

Inspire embeds transformed source records and procedural generators. Source
versions, download URLs, checksums, filters, and reuse terms are recorded in the
family READMEs and development-time importers. Record provenance remains in the
source JSON; attribution required with a sampled record appears in its text.

Public-domain geographic names, historical texts, and government data coexist
with datasets carrying their own licenses. These licenses apply to the associated
data, not to the entire Inspire project. Source coverage and dates vary; the
collection is not a current or exhaustive reference work.

`inspire --data-licenses` prints this notice followed by the full license files
stored under `families/`. Those files are embedded at build time so the standalone
executable carries the notices even without a source checkout. Keep this document
and the relevant family license files with redistributed source datasets.

## Source families

| Material | Source and reuse terms | Import and editorial notes |
| --- | --- | --- |
| Geographic places and physical features | [Natural Earth](https://www.naturalearthdata.com/about/terms-of-use/), public domain | [Geography](families/geography/README.md) |
| Small graph atlas | [NetworkX 3.4.2](https://github.com/networkx/networkx/tree/networkx-3.4.2), BSD-3-Clause | [Graph atlas](families/math/discrete-mathematics/graph-theory/atlas/README.md) |
| Classical work metadata | [Open Opus](https://www.openopus.org/), public-domain/CC0 data | [Music](families/music/README.md); its API software license is separate |
| Instrument names | [Corpora](https://github.com/dariusk/corpora), CC0 | [Music](families/music/README.md), with documented spelling corrections |
| Worldwide instrument vocabulary | [MIMO via DOREMUS](https://github.com/DOREMUS-ANR/knowledge-base/tree/master/vocabularies), CC BY 4.0 | [Music](families/music/README.md), with source-supported broader families |
| Cartographic color palettes | [ColorBrewer](https://colorbrewer2.org/), its own Apache-style notice | [ColorBrewer license](families/random/colors/palettes/cartographic/LICENSE.txt); every palette carries the required acknowledgment |
| Museum artifact metadata | [Cleveland Museum of Art Open Access](https://www.clevelandart.org/open-access), CC0 | [Objects](families/objects/README.md); no images are included |

Biological and other developing collections document their individual sources
and evidence types in their family and leaf READMEs. Their full required notices
are appended automatically below by the build; the source tree remains the
record-level provenance reference.
