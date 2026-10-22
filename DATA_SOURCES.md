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
| Cartographic color palettes | [ColorBrewer](https://colorbrewer2.org/), its own Apache-style notice | [ColorBrewer license](families/random/colors/palettes/cartographic/LICENSE.txt); acknowledgment is bundled with the executable |
| Museum artifact metadata | [Cleveland Museum of Art Open Access](https://www.clevelandart.org/open-access), CC0 | [Objects](families/objects/README.md); no images are included |
| English word senses and antonyms | [Princeton WordNet 3.0](https://wordnet.princeton.edu/), WordNet's redistribution license | [Language](families/language/README.md); copyright and permission notices are bundled |
| Grammar quotations, sayings and word histories | Particular public-domain editions of Baskervill and Sewell, Preston and Weekley, from [Project Gutenberg](https://www.gutenberg.org/) | [Language](families/language/README.md); original wording and attribution replace authored examples and interpretations |
| Chess opening lines | [Lichess opening book](https://github.com/lichess-org/chess-openings), CC0 | [Games](families/games/README.md); complete lines have independent legal replay checks |
| Historical person profiles | [Annamoradnejad and Annamoradnejad's AgeDataset V1](https://github.com/Moradnejad/AgeDataset), CC BY 4.0 data and accompanying MIT repository notice | [People](families/people/README.md); source identities, corrections and attribution are retained |
| Chemical identities and structures | [PubChem](https://pubchem.ncbi.nlm.nih.gov/), selected factual metadata | [Chemistry](families/chemistry/README.md); no blanket license claim for third-party PubChem prose |
| Historical accounts and literary passages | [Project Gutenberg](https://www.gutenberg.org/) and [Standard Ebooks](https://standardebooks.org/), particular public-domain editions | [History](families/history/README.md), [literature](families/literature/README.md); translator, edition, locator and source notices are retained |
| Animal taxonomy and traits | [GBIF](https://www.gbif.org/), AVONET, PanTHERIA, AmphiBIO, FISHMORPH, AnimalTraits and Cefas | [Animals](families/natural-world/animals/README.md); individual sources carry CC BY, CC0 or OGL terms, with separate evidence types and units |
| Plant profiles | [USDA NRCS PLANTS](https://plants.usda.gov/), freely reusable factual database information | [Plants](families/natural-world/plants/README.md); database citation is bundled |
| Astronomical numerical properties | [NASA Exoplanet Archive](https://exoplanetarchive.ipac.caltech.edu/) and [NASA/JPL Solar System Dynamics](https://ssd.jpl.nasa.gov/) | [Astronomy](families/natural-world/astronomy/README.md); literature solutions, missing values and uncertainties remain qualified |
| Weather definitions | [NOAA National Weather Service](https://www.weather.gov/), government glossary material | [Natural phenomena](families/natural-world/phenomena/README.md); source definitions and contiguous excerpts, without authored paraphrases |
| Historical mechanical descriptions | Henry T. Brown, *Five Hundred and Seven Mechanical Movements*, 1908 edition, public domain | [Mechanisms](families/mechanisms/README.md); original book text is separate from modern site animations |
| Historical recipes, needlework and scientific recreations | Particular public-domain editions of Farmer, Higgin, Dillmont and Tissandier, downloaded from [Project Gutenberg](https://www.gutenberg.org/) | [Procedures](families/procedures/SOURCES.md); original source procedures and executable algorithm traces |

Family and leaf READMEs document additional sources, exact snapshots, selection
bounds and evidence types. Full required notices are appended automatically below
by the build; the source tree remains the record-level provenance reference.

Historical additions include Sa'di's *Gulistan* in James Ross's translation,
A. A. Stewart's *Typesetting* (1919), Mattie Phipps Todd's *Hand-Loom Weaving*
(1902), Beeton's *Book of Household Management* (1861), and Vitruvius in Morris
Hicky Morgan's translation (1914). Their exact editions and download checksums
are recorded with the corresponding family importers.

## Dataset credits

Instrument vocabulary is copyright Musical Instrument Museums Online (MIMO),
distributed by DOREMUS-ANR under CC BY 4.0. Selected labels and broader relations
are reformatted and grouped; corrections are documented in the music importer.
WordNet 3.0 is copyright 2006 Princeton University; selected senses and antonyms
are reformatted under the accompanying WordNet permission notice.

Person metadata is adapted from Issa Annamoradnejad and Rahimberdi Annamoradnejad,
*AgeDataset V1* (2022), CC BY 4.0, doi:10.36190/2022.82. Names and descriptions
are selected, grouped, normalized and corrected as documented in the importer.

Animal taxonomy credits GBIF Secretariat, *GBIF Backbone Taxonomy*, CC BY 4.0,
dataset d7dddbf4-2cf0-4f39-9b2a-bb099caae36c. Trait sources include:

- Tobias et al. (2022), *AVONET*, Figshare version 7, CC BY 4.0,
  doi:10.6084/m9.figshare.16586228.v7.
- Jones et al., *PanTHERIA 1.0 WR05* (August 2008), Ecological Archives E090-184,
  CC BY 4.0, https://esapubs.org/archive/ecol/E090/184/.
- Oliveira, São-Pedro, Santos-Barrera, Penone and Costa (2017), *AmphiBIO v1*,
  Figshare version 5, CC BY 4.0, doi:10.6084/m9.figshare.4644424.v5.
- Oskyrko, Mi, Meiri and Du, *ReptTraits v1-2*, Figshare version 4, CC BY 4.0,
  doi:10.6084/m9.figshare.24572683.v4.
- Brosse, Charpin, Su, Toussaint, Herrera-R, Tedesco and Villéger (2021),
  *FISHMORPH v1.1*, CC BY 4.0, doi:10.14278/rodare.1316.
- Hutchings et al. (2012), supplement 1 version 1, CC0,
  doi:10.6084/m9.figshare.3517361.v1; *AnimalTraits v1.0.7*, CC0,
  doi:10.5281/zenodo.6468938.
- Cefas, *Benthic trait matrix V2*, Crown copyright, Open Government Licence v3,
  doi:10.14466/CefasDataHub.123.

Biological data are selected and joined at their documented taxonomic or study
scale, with unit conversions and numerical rounding. Source notes, evidence
context and corrections remain in the associated importers and record provenance.

Plant information credits USDA NRCS, *The PLANTS Database*, National Plant Data
Team, Greensboro, NC 27401-4901 USA, https://plants.usda.gov/. Astronomical data
credit NASA Exoplanet Archive (operated by Caltech under contract with NASA) and
NASA/JPL Solar System Dynamics; published solutions retain units and precision.

This product includes color specifications and designs developed by Cynthia
Brewer (http://colorbrewer.org/). The palettes are selected and reformatted;
the complete ColorBrewer notice follows with the other bundled licenses.
