# Inspiration Suggestions

Candidate leaves and families, one per line. Paths are proposals. *(gen)* marks a
Rust generator; everything else is records from public-domain or open data.

## New top-level families

### physics
- `physics/mechanics/projectiles` *(gen)*: launch speed, angle, and gravity of a real body, with range, apex, and flight time.
- `physics/mechanics/collisions` *(gen)*: two masses before and after an elastic or perfectly inelastic collision.
- `physics/optics/lens-systems` *(gen)*: object distance, focal lengths, and the resulting image position and magnification.
- `physics/circuits/rc-transients` *(gen)*: an RC or RL step response with time constant and a short voltage table.
- `physics/thermodynamics/cycles` *(gen)*: Carnot, Otto, or Rankine state tables with work and efficiency.
- `physics/waves/standing` *(gen)*: string or pipe length, tension, or temperature, and its first harmonics in Hz and note names.
- `physics/constants`: CODATA constants with value, uncertainty, and the experiment that pins them down.
- `physics/units/obsolete`: furlongs, barleycorns, grains, and ells with exact SI conversions and where they were used.
- `physics/thought-experiments`: Maxwell's demon, Newton's bucket, the twin paradox, stated in a few sentences each.

### body
- `body/anatomy/bones`: Gray's Anatomy (1918) entries for single bones and their landmarks.
- `body/anatomy/muscles`: origin, insertion, innervation, and action.
- `body/physiology/reflexes`: stimulus, pathway, and response for named clinical reflexes.
- `body/medicine/historical-remedies`: period pharmacopoeia entries, quoted with source and year.
- `body/medicine/eponymous-signs`: Babinski, Murphy, Trousseau, with what the sign indicates.
- `body/nutrition/foods` *(gen)*: one food item with a USDA FoodData Central nutrient profile.

### society
- `society/law/legal-maxims`: Latin maxims with translation and a classic application.
- `society/law/landmark-cases`: one-paragraph facts and holdings of public-domain court opinions.
- `society/law/constitutions`: single articles from the world's constitutions (Constitute Project texts).
- `society/economics/price-histories`: commodity prices over a decade from public statistical series.
- `society/economics/market-clearing` *(gen)*: linear supply and demand, equilibrium, and the effect of a tax or price ceiling.
- `society/economics/auctions` *(gen)*: bidders' valuations and the outcome under English, Dutch, and Vickrey rules.
- `society/voting/elections` *(gen)*: ranked ballots counted under plurality, IRV, Borda, and Condorcet, where the winners differ.
- `society/occupations/historical`: trades like cooper, fuller, knocker-upper, and lamplighter, with a period description.
- `society/occupations/modern`: O*NET occupation titles with a few concrete daily tasks.
- `society/etiquette/historical`: rules quoted from 19th-century etiquette manuals.
- `society/institutions/charters`: founding clauses of guilds, universities, and companies.

### belief
- `belief/mythology/greek`: episodes from Bulfinch or Apollodorus.
- `belief/mythology/norse`: Prose Edda episodes in public-domain translation.
- `belief/mythology/mesoamerican`: Popol Vuh passages.
- `belief/scripture/wisdom`: single verses from Proverbs, Ecclesiastes, the Dhammapada, and the Tao Te Ching.
- `belief/saints/calendar`: feast day, saint, and one-line legend.
- `belief/divination/i-ching` *(gen)*: a cast hexagram with changing lines and the resulting hexagram, using the Legge text.
- `belief/divination/tarot` *(gen)*: a three-card spread with orientations and Waite's 1911 meanings.

### computing
- `computing/data-structures/heaps` *(gen)*: a sequence of inserts and pops with the array after each step.
- `computing/data-structures/hash-tables` *(gen)*: keys inserted under linear probing or chaining, with the final table.
- `computing/data-structures/red-black` *(gen)*: insertions with rotations and recolorings named.
- `computing/automata/dfa` *(gen)*: a small DFA, its regular expression, and accepted and rejected strings.
- `computing/automata/cellular` *(gen)*: an elementary CA rule number with 16 generations as text.
- `computing/automata/turing` *(gen)*: a busy-beaver-style machine table and its first steps.
- `computing/regex/puzzles` *(gen)*: two string lists and a regex that separates them.
- `computing/sql/queries` *(gen)*: a tiny table, a query, and its result set.
- `computing/bits/tricks` *(gen)*: one integer under popcount, ctz, gray code, and bit reversal.
- `computing/networking/subnets` *(gen)*: a CIDR block split into subnets with ranges and broadcast addresses.
- `computing/encoding/utf8` *(gen)*: one Unicode code point with its name, UTF-8 and UTF-16 bytes, and block.
- `computing/history/rfcs`: memorable opening paragraphs or definitions from early RFCs.
- `computing/history/error-messages`: famous historical error messages and failure reports, such as the first logged bug.
- `computing/esolangs`: short programs in Brainfuck, Befunge, or FRACTRAN with what they compute.

### cryptography
- `cryptography/classical/ciphers` *(gen)*: a proverb under Vigenère, Playfair, or rail-fence with its key.
- `cryptography/classical/frequency`: letter-frequency tables for different languages.
- `cryptography/modern/rsa-toy` *(gen)*: small primes, keys, and one encrypt-decrypt round.
- `cryptography/historical/codebooks`: entries from public-domain commercial telegraph codebooks.

### built-environment
- `built-environment/architecture/orders`: column orders and their proportions from Vitruvius or Vignola.
- `built-environment/architecture/buildings`: notable buildings with architect, year, material, and one distinctive feature.
- `built-environment/architecture/floor-plans` *(gen)*: small house plans in ASCII with room sizes that respect adjacency.
- `built-environment/urban/street-grids`: cities with their founding plan type and date.
- `built-environment/infrastructure/bridges`: span, type, material, and opening year.
- `built-environment/infrastructure/lighthouses`: light characteristic, such as Fl(3) W 15s, with location and height.
- `built-environment/vernacular/dwellings`: yurt, trullo, and longhouse, with construction method and climate.

### transport
- `transport/rail/timetables` *(gen)*: a plausible branch-line timetable with consistent running times.
- `transport/rail/locomotives`: wheel arrangement, builder, and service years.
- `transport/aviation/airports`: IATA code, name, city, elevation, and runway count from OurAirports.
- `transport/aviation/metar` *(gen)*: METAR reports decoded into plain English.
- `transport/maritime/knots`: knot name, ABOK number, use, and tying steps.
- `transport/maritime/flags`: signal flags and their single-flag meanings.
- `transport/maritime/ship-types`: historical rigs such as brigantine, xebec, and junk, with sail plans.

### arts
- `arts/painting/works`: title, artist, year, medium, and museum for public-domain paintings (Met or Rijksmuseum open data).
- `arts/painting/techniques`: grisaille, sfumato, and impasto, with a canonical example.
- `arts/sculpture/works`: works with material and location.
- `arts/film/silent`: plot summaries of public-domain silent films.
- `arts/film/shot-lists` *(gen)*: a short scene broken into shot sizes, angles, and moves that stay consistent.
- `arts/theatre/stage-directions`: notable stage directions from public-domain plays.
- `arts/dance/notation`: ballet positions and steps, or contra dance figures with call sequences.
- `arts/photography/exposure` *(gen)*: a lighting scene with aperture, shutter, and ISO triples of equal exposure.

### typography
- `typography/typefaces`: historical faces with designer, year, classification, and distinguishing letterforms.
- `typography/glyphs`: obscure Unicode punctuation and symbols such as the pilcrow, interrobang, and manicule, with history.
- `typography/scripts`: writing systems with direction, type, and a sample word in script.
- `typography/ciphers/runes` *(gen)*: a short word transliterated into Elder Futhark, Ogham, or Cherokee.

### food
- `food/ingredients/spices`: origin, flavor compounds, and classic pairings.
- `food/ingredients/cultivars`: apple, potato, and rice varieties with traits.
- `food/dishes/regional`: named regional dishes with core ingredients and origin.
- `food/fermentation`: process, organism, time, and temperature for kimchi, miso, kefir, and garum.
- `food/menus/historical`: items with prices from digitized historical menus (NYPL What's on the Menu).
- `food/drinks/cocktails`: IBA cocktails with specs and glassware.
- `food/baking/ratios` *(gen)*: a bread or pastry at baker's percentages, scaled to a given flour weight.

### sport
- `sport/rules/oddities`: strange real rules from sports rulebooks.
- `sport/records/historical`: progression of a world record with athletes and dates.
- `sport/tournaments/brackets` *(gen)*: a seeded single-elimination bracket with plausible upsets.
- `sport/climbing/routes`: famous routes with grade, first ascent, and style.

### mind
- `mind/cognitive-biases`: the bias with a concrete classic experiment.
- `mind/illusions`: perceptual illusions with how to construct them.
- `mind/emotions/lexicon`: untranslatable emotion words with language and a usage sentence.
- `mind/psychology/experiments`: classic studies with design, n, and result.
- `mind/dreams/historical`: dream reports from public-domain dream books and diaries.

### philosophy
- `philosophy/arguments`: famous arguments in numbered premise form, such as the ontological argument or the Chinese room.
- `philosophy/paradoxes`: Sorites, Newcomb, and Ship of Theseus, each in two or three sentences.
- `philosophy/dilemmas` *(gen)*: trolley-style variants built from orthogonal factors, not template swaps.
- `philosophy/logic/syllogisms` *(gen)*: a categorical syllogism with mood, figure, and validity.
- `philosophy/logic/truth-tables` *(gen)*: a propositional formula, its table, and whether it is a tautology.
- `philosophy/fallacies`: a named fallacy with a real historical instance.

### earth
- `earth/minerals`: Mohs hardness, crystal system, luster, and streak (Mindat or public USGS data).
- `earth/rocks`: igneous, sedimentary, and metamorphic types with formation conditions.
- `earth/earthquakes`: historical quakes with magnitude, depth, and effects.
- `earth/volcanoes`: Smithsonian GVP volcanoes with type and last eruption.
- `earth/soils`: USDA soil orders with properties and where they occur.
- `earth/stratigraphy/geologic-time`: periods and stages with dates and a marker event.
- `earth/climate/koppen`: a location with its Köppen class and monthly temperature and precipitation table.

## Additions to existing families

### chemistry
- `chemistry/reactions/organic/named`: Diels-Alder, Grignard, and Wittig with reactants, products, and conditions as SMILES.
- `chemistry/spectroscopy/nmr` *(gen)*: a small molecule and its predicted proton NMR peaks with splitting.
- `chemistry/stoichiometry/limiting` *(gen)*: given masses, the limiting reagent and theoretical yield.
- `chemistry/lab/glassware`: apparatus with purpose and common misuse.
- `chemistry/history/alchemy`: alchemical symbols and their modern substances.

### design
- `design/typography-pairings` *(gen)*: a heading and body face pair with a reason drawn from their classification.
- `design/graphic/grids` *(gen)*: page size, margins, columns, and gutters that add up exactly.
- `design/heraldry/blazons` *(gen)*: a valid blazon that respects the rule of tincture, with a plain description.
- `design/flags/vexillology`: national and historic flags with their official proportions and blazon.
- `design/interiors/rooms` *(gen)*: a room with dimensions, furniture placements that fit, and a palette.
- `design/textiles/knitting` *(gen)*: knitting charts for small lace or cable repeats.
- `design/textiles/quilting`: traditional quilt block names with grid and construction.
- `design/industrial/objects`: iconic industrial designs with designer, year, and material.

### games
- `games/board/go-openings`: joseki with sequences.
- `games/board/shogi` *(gen)*: tsume problems.
- `games/board/backgammon` *(gen)*: positions with a dice roll and the best move.
- `games/puzzles/crosswords` *(gen)*: a 5x5 mini crossword with clues from the lexicon leaves.
- `games/puzzles/kakuro` *(gen)*: Kakuro puzzles with unique solutions.
- `games/puzzles/logic-grids` *(gen)*: Zebra-style puzzles with a unique solution and minimal clues.
- `games/puzzles/mazes` *(gen)*: ASCII mazes built by different algorithms with visibly different textures.
- `games/puzzles/rebus`: historical rebuses and riddles from public-domain collections.
- `games/rpg/encounters` *(gen)*: a dungeon room with exits, contents, and a hazard that interlock.
- `games/folk/playground`: traditional children's games with rules (Gomme's Traditional Games).
- `games/cards/solitaire` *(gen)*: a Klondike or FreeCell deal labeled solvable or not.

### geography
- `geography/physical/islands`: area, population, and the nearest neighboring landmass.
- `geography/physical/deserts-and-ice`: deserts, glaciers, and ice shelves with size and character.
- `geography/toponymy/etymologies`: place names and what they originally meant.
- `geography/maps/projections`: map projections with properties and distortion.
- `geography/coordinates/antipodes` *(gen)*: a real city, its antipode, and what lies there.
- `geography/borders/oddities`: enclaves, tripoints, and exclaves with a short description.
- `geography/time-zones`: zones with UTC offsets and historical oddities.

### history
- `history/medieval/chronicles`: entries from the Anglo-Saxon Chronicle.
- `history/medieval/charters`: Magna Carta clauses with translation.
- `history/early-modern/broadsides`: public-domain broadside ballads and news sheets.
- `history/science/notebooks`: entries from da Vinci's or Darwin's notebooks.
- `history/disasters`: shipwrecks, fires, and floods with date, toll, and cause.
- `history/inventions/patents`: early patent abstracts with drawings described.
- `history/exploration/logbooks`: ship log entries with position and weather.
- `history/china/classics`: passages from the Analects, Sun Tzu, and Zhuangzi in Legge or Giles translation.
- `history/africa/chronicles`: passages from Ibn Battuta and the Kebra Nagast.
- `history/americas/codices`: descriptions of individual Aztec and Maya codex pages.

### language
- `language/languages/profiles`: word order, phoneme count, speakers, and family (WALS data).
- `language/phonology/ipa` *(gen)*: a word, its IPA, and a feature breakdown for one segment.
- `language/morphology/paradigms`: verb conjugation tables for Latin, Finnish, Swahili, and Turkish.
- `language/conlang/words` *(gen)*: words from a consistent invented phonotactics with derivations.
- `language/word-histories/doublets`: etymological doublets such as guard and ward or chef and chief.
- `language/lexicon/collective-nouns`: real attested collective nouns with source.
- `language/lexicon/obsolete`: obsolete English words with definitions and quotations.
- `language/translation/false-friends`: false-friend pairs across languages.
- `language/palindromes-and-constraints`: famous palindromes, lipograms, and pangrams.

### literature
- `literature/drama/soliloquies`: public-domain dramatic monologues.
- `literature/letters/authors`: letters by Keats, Austen, and Chekhov.
- `literature/openings/first-lines`: first lines of public-domain novels with author and year.
- `literature/epigraphs`: epigraphs with the work they head.
- `literature/sermons`: excerpts from Donne and Edwards.
- `literature/children/nursery-rhymes`: rhymes with variants.
- `literature/riddles/exeter-book`: Anglo-Saxon riddles in translation.
- `literature/verse/haiku`: Basho, Buson, and Issa in public-domain translation.
- `literature/verse/ghazals`: Hafez and Ghalib in translation.
- `literature/verse/african`: public-domain translations of praise poetry.
- `literature/criticism`: short passages from Johnson, Coleridge, and Hazlitt.
- `literature/poetic-forms/exemplars` *(gen)*: a meter or rhyme scheme scanned against a real public-domain stanza.

### math
- `math/geometry/constructions`: compass-and-straightedge constructions as step lists.
- `math/geometry/polyhedra`: vertices, edges, faces, symmetry group, and dual.
- `math/geometry/tilings`: vertex configurations such as 3.4.6.4 with names.
- `math/geometry/triangle-centers` *(gen)*: a triangle with its centroid, circumcenter, and Euler line.
- `math/combinatorics/partitions` *(gen)*: an integer's partitions with a Young diagram.
- `math/combinatorics/sequences`: OEIS-style entries with first terms and a definition.
- `math/combinatorics/knots`: knots with crossing number, notation, and invariants.
- `math/recreational/magic-squares` *(gen)*: magic squares built by different methods.
- `math/recreational/problems`: classic problems from Dudeney or Loyd.
- `math/statistics/datasets` *(gen)*: small datasets with summary statistics and a fitted regression.
- `math/statistics/paradoxes`: Simpson's paradox and Berkson's bias with real data.
- `math/calculus/odes` *(gen)*: an ODE with initial conditions and its closed form.
- `math/fractals`: fractals with dimension and construction rule.
- `math/history/problems`: Rhind Papyrus and Nine Chapters problems.

### mechanisms
- `mechanisms/horology/complications`: tourbillon, perpetual calendar, and minute repeater with how each works.
- `mechanisms/locks`: pin tumbler, warded, and lever locks with the mechanism.
- `mechanisms/automata/historical`: Jaquet-Droz and Vaucanson automata with what they did.
- `mechanisms/gears/trains` *(gen)*: a compound gear train with tooth counts that produce a target ratio.
- `mechanisms/engines/heat`: Stirling, Newcomen, and Wankel with operating cycles.

### music
- `music/notation/abc` *(gen)*: short tunes in ABC notation with modal or idiomatic constraints.
- `music/forms`: sonata, rondo, and passacaglia with the section layout of an exemplar.
- `music/folk/tunes`: traditional tunes in ABC from public collections.
- `music/tuning/temperaments`: historical temperaments with cents tables.
- `music/rhythm/tala`: Indian talas with beat structure and bols.
- `music/rhythm/clave` *(gen)*: Euclidean rhythms with their traditional names.
- `music/lyrics/hymns`: public-domain hymn texts with meter, such as 8.6.8.6 CM.
- `music/orchestration/ranges`: instrument ranges and sweet spots.

### natural-world
- `natural-world/fungi`: species with spore print, habitat, and edibility.
- `natural-world/birds/calls`: Peterson-style mnemonics for bird songs.
- `natural-world/ecology/food-webs` *(gen)*: a small consistent food web for a biome.
- `natural-world/ecology/succession`: stages for a habitat.
- `natural-world/astronomy/constellations`: constellations with their brightest stars and myth.
- `natural-world/astronomy/sky-tonight` *(gen)*: for a date and latitude, the moon phase and visible planets.
- `natural-world/astronomy/deep-sky`: Messier objects with type, distance, and magnitude.
- `natural-world/microbes`: organisms with morphology and niche.
- `natural-world/phenology`: seasonal events with latitude and date ranges.
- `natural-world/naturalists/journals`: passages from Thoreau, White's Selborne, and Audubon.
- `natural-world/genetics/crosses` *(gen)*: Punnett squares for dihybrid or sex-linked traits with ratios.

### objects
- `objects/tools/hand-tools`: tools with use, and regional names where they exist.
- `objects/instruments/scientific`: astrolabe, sextant, and orrery with how each is used.
- `objects/everyday/patents`: everyday objects and their first patents.
- `objects/toys/historical`: toys with era and mechanism.
- `objects/clothing/historical-garments`: doublet, chiton, and hanbok with era and construction.
- `objects/cars/europe`: Italian, French, and British cars.
- `objects/aircraft`: types with role, first flight, and notable trait.
- `objects/stamps`: stamps with country, year, and design.

### people
- `people/society/inventors`: underrecognized inventors.
- `people/society/rulers`: rulers with reign and one notable act.
- `people/society/activists`: activists outside Europe and North America.
- `people/arts/architecture`: architects.
- `people/arts/craft`: potters, weavers, and smiths.
- `people/sport/athletes`: historical athletes.
- `people/medicine/physicians`: physicians.
- `people/everyday/diarists`: ordinary diarists such as Pepys, Ruth Hayes, and Anne Lister.

### procedures
- `procedures/craft/woodworking/joints`: joints with cutting steps.
- `procedures/craft/bookbinding`: binding methods with steps.
- `procedures/craft/ceramics/glazes`: cone-rated glaze recipes in percentages.
- `procedures/craft/dyeing`: natural dye recipes with mordants.
- `procedures/craft/origami` *(gen)*: crease patterns for simple bases, or written folding sequences.
- `procedures/first-aid`: historical and modern first-aid steps, clearly sourced.
- `procedures/gardening/companion`: companion-planting pairs with the reason.
- `procedures/household/historical`: tasks from Mrs Beeton.
- `procedures/algorithms/sorting/traces` *(gen)*: sort traces for quicksort, merge, and radix with intermediate arrays.
- `procedures/algorithms/strings` *(gen)*: KMP failure tables, edit-distance matrices, and Huffman codes.
- `procedures/algorithms/numerical` *(gen)*: Newton's method and Simpson's rule iteration tables.
- `procedures/experiments/chemistry`: classic demonstrations with materials and observations.
- `procedures/experiments/electricity`: Faraday's or Franklin's experiments in their own words.

### random
- `random/names/personal` *(gen)*: culturally coherent full names from real name-frequency data.
- `random/coordinates` *(gen)*: a land-weighted point with its nearest named place.
- `random/identifiers` *(gen)*: valid ISBNs, IBANs, and Luhn-valid numbers with check digit shown.
- `random/passwords/diceware` *(gen)*: a diceware passphrase with the dice rolls.
- `random/times/schedules` *(gen)*: a week calendar with non-overlapping events.
- `random/geometry/shapes` *(gen)*: random simple polygons as SVG paths with area and perimeter.
- `random/sounds/frequencies` *(gen)*: a chord or frequency set given in Hz, cents, and MIDI.
