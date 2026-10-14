# Dataset sources and redistribution

Inspire embeds transformed source records and procedural generators. Source
versions, download URLs, checksums, filters, and reuse terms are recorded in the
family READMEs and development-time importers. Record provenance remains in the
source JSON. Dataset credit, modification notes and full notices travel with the
executable through `--data-licenses`; ordinary sampled text focuses on the material.

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
| English word senses and antonyms | [Princeton WordNet 3.0](https://wordnet.princeton.edu/), WordNet's redistribution license | [Language](families/language/README.md); emitted definitions retain the required copyright notice |
| Chess opening lines | [Lichess opening book](https://github.com/lichess-org/chess-openings), CC0 | [Games](families/games/README.md); complete lines have independent legal replay checks |
| Historical person profiles | [Annamoradnejad and Annamoradnejad's AgeDataset V1](https://github.com/Moradnejad/AgeDataset), CC BY 4.0 data and accompanying MIT repository notice | [People](families/people/README.md); source identities, corrections and attribution are retained |
| Chemical identities and structures | [PubChem](https://pubchem.ncbi.nlm.nih.gov/), selected factual metadata | [Chemistry](families/chemistry/README.md); no blanket license claim for third-party PubChem prose |
| Historical accounts and literary passages | [Project Gutenberg](https://www.gutenberg.org/) and [Standard Ebooks](https://standardebooks.org/), particular public-domain editions | [History](families/history/README.md), [literature](families/literature/README.md); translator, edition, locator and source notices are retained |
| Animal taxonomy and traits | [GBIF](https://www.gbif.org/), AVONET, PanTHERIA, AmphiBIO, FISHMORPH, AnimalTraits and Cefas | [Animals](families/natural-world/animals/README.md); individual sources carry CC BY, CC0 or OGL terms, with separate evidence types and units |
| Plant profiles | [USDA NRCS PLANTS](https://plants.usda.gov/), freely reusable factual database information | [Plants](families/natural-world/plants/README.md); requested database citation accompanies each record |
| Astronomical numerical properties | [NASA Exoplanet Archive](https://exoplanetarchive.ipac.caltech.edu/) and [NASA/JPL Solar System Dynamics](https://ssd.jpl.nasa.gov/) | [Astronomy](families/natural-world/astronomy/README.md); literature solutions, missing values and uncertainties remain qualified |
| Weather definitions | [NOAA National Weather Service](https://www.weather.gov/), government glossary material | [Natural phenomena](families/natural-world/phenomena/README.md); downloaded definitions and editorial paraphrases are distinguished |
| Historical mechanical descriptions | Henry T. Brown, *Five Hundred and Seven Mechanical Movements*, 1908 edition, public domain | [Mechanisms](families/mechanisms/README.md); original book text is separate from modern site animations |
| Historical recipes and embroidery procedures | Particular public-domain editions of Farmer and Higgin, downloaded from [Project Gutenberg](https://www.gutenberg.org/) | [Procedures](families/procedures/SOURCES.md); historical source procedures and original algorithm traces remain separate |

Family and leaf READMEs document additional sources, exact snapshots, selection
bounds and evidence types. Full required notices are appended automatically below
by the build; the source tree remains the record-level provenance reference.
