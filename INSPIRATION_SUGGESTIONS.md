# Inspiration Suggestions

Candidate leaves and families. Paths are proposals. *(gen)* marks a Rust
generator; everything else is records from public-domain or open data. Each
entry outlines how to achieve **within-niche structural diversity** so samples
vary in topology, regime, mechanism, or perspective rather than just swapping
numbers or names into a rigid template.

## Integration status

Existing coverage sometimes lives at a different path. Extend these implementations
instead of creating duplicate branches. The proposals below retain ideas that go
beyond their current scope.

| Proposal | Existing implementation and remaining scope |
| --- | --- |
| `mechanisms/gears/trains` | `mechanisms/mechanical/transmission/gears/{compound-trains,epicyclic-trains}` |
| `mechanisms/cams/profiles` | `mechanisms/linkages/cams/{motion-programs,cycloidal-rise}`; pressure-angle bounds remain open |
| `procedures/weaving/drafts` | `design/textiles/weaving/{plain,basket,twill,satin}`; huck and overshot remain open |
| `procedures/household/historical` | `history/material-practices/households/britain/beeton-1861` |
| `procedures/printing/letterpress` | `history/material-practices/printing/united-states/stewart-1919` |
| `random/graphs/small-topologies` | `math/discrete-mathematics/graph-theory/atlas/{forests,bipartite-cyclic,nonbipartite}`; 3–7 vertices, including disconnected graphs |
| `math/abstract-algebra/cayley-tables` | `math/algebra/groups/finite-groups/`; focused views of seven group classes, without complete subgroup lattices |
| `random/permutations/cycles` | `math/algebra/groups/finite-groups/symmetric`; inversion-table view remains open |
| `procedures/algorithms/sorting/traces` | `procedures/algorithms/sequences/sorting/`; stable merge, insertion and selection traces; quicksort and radix remain open |
| `procedures/algorithms/strings` | `procedures/algorithms/sequences/alignment/levenshtein-distance`; KMP and Huffman remain open |
| `language/word-histories/doublets` | `language/word-histories` includes sourced Weekley passages on doublets |

New implementations and their exact model limits are documented in the
[physics](families/physics/README.md), [society](families/society/README.md),
[computing](families/computing/README.md) and [transport](families/transport/README.md)
family READMEs. A leaf's presence does not imply every proposed variant is implemented.

This integration adds:

| Implemented leaf | Coverage |
| --- | --- |
| `physics/mechanics/collisions` | Collinear and oblique smooth impacts, grazing contact, coalescence, lab/COM frames |
| `physics/optics/lens-systems` | Single lenses, relays, telephoto pairs, afocal telescopes and focal boundaries |
| `physics/waves/standing` | Strings, light-touch harmonics, open and stopped pipes, spectra and mode shapes |
| `society/voting/elections` | Plurality, Borda, tie-aware IRV and pairwise comparisons, including cycles and center squeeze |
| `society/economics/matching` | Both deferred-acceptance extremes and enumerated stable assignments |
| `society/economics/game-theory` | Five 2×2 regimes and weighted cyclic zero-sum games, exact equilibria and pure-cell Pareto sets |
| `computing/automata/cellular` | Elementary rules, finite boundaries, contrasting starts and perturbation comparisons |
| `computing/concurrency/schedules` | Split-increment lost updates, lock-order deadlocks, SC/TSO store-buffer litmus witnesses |
| `computing/floating-point/ieee754` | Binary32 fields, neighbors, rounding ties, underflow, association and special encodings |
| `transport/aviation/airports/*` | 1,248 pinned OurAirports sites in six type leaves, with source codes, elevation and longest listed surface |

Existing stable-merge traces now mix tied, separated, interleaved and unequal
runs. The backlog's remaining variants are proposals, not claims about current output.

### Expanded coverage

The larger pass adds 57 generators and 12,294 source records. Current family
READMEs define each implementation's exact scope; the detailed proposals below
still include extensions beyond those models.

| Area | Added coverage |
| --- | --- |
| [Physics](families/physics/README.md) | Projectiles, RC/RL switching, ideal heat cycles, room modes, orbital transfers, Buckingham Pi |
| [Computing](families/computing/README.md) | Heaps, hash tables, DFA, checked SQL, IPv4 subnetting and UTF encodings |
| [Cryptography](families/cryptography/README.md) | Four classical cipher architectures, RSA specimens, Shamir interpolation, Hamming/SECDED |
| [Society](families/society/README.md) | Explicit-bid auctions, EF1/utilitarian allocations, five apportionment methods, market boundaries |
| [Math](families/math/README.md) | Triangle centers, partitions, magic squares, higher finite fields, Latin squares, equal-summary datasets |
| [Systems](families/systems/README.md) | Queue regimes, ecological dynamics, shared-event fault trees, project slack, max-flow/min-cut |
| [Games](families/games/README.md) | Mazes, unique/minimal logic grids, unique Kakuro, exact cribbage, Kayles, normal/misère Nim |
| [Algorithms](families/procedures/algorithms/README.md) | Quicksort, radix sort, KMP, Huffman, Newton and quadrature traces |
| [Random](families/random/README.md) | Checked identifiers, Haar/Hadamard signals, contingency tables, finite orders, simple polygons |
| [Design](families/design/README.md) | Exact page grids, knitting repeats, verified two-up imposition, constrained blazons |
| [Philosophy](families/philosophy/README.md) | All syllogistic forms, propositional tables, modal frames and countermodels |
| [Transport](families/transport/README.md) | Consistent railway timetables, ideal navigation legs, synthetic IALA mark configurations |
| [Food](families/food/README.md) | 7,793 USDA profiles plus 1,587 culinary names, including 1,000 apple cultivars |
| [Materials](families/materials/README.md) | 483 sourced material and textile labels |
| [Belief](families/belief/README.md) | 566 complete mythology and wisdom-text units from four attributed editions |
| [Arts](families/arts/README.md) | 1,800 painting/drawing/print catalogue works and 65 original stage-direction blocks |

## New top-level families

### physics
- `physics/mechanics/projectiles` *(gen)*: launch speed, angle, and gravity of a real body, with range, apex, and flight time.
  - Contrast flat-ground parabolas against elevated cliff launches ($y_0 > 0$), inclined target planes, and trajectories on low-gravity bodies (Moon, Titan, Ceres).
  - Mix vacuum trajectories with linear Stokes drag regimes where trajectory asymmetry, terminal velocity, and range suppression appear.
- `physics/mechanics/collisions` *(gen)*: two masses before and after an elastic or perfectly inelastic collision.
  - Vary regimes: equal-mass velocity swap ($m_1 = m_2, e=1$), heavy-on-light slingshot ($m_1 \gg m_2$), glancing 2D scattering, and perfectly inelastic coalescence ($e=0$).
  - Alternate between lab-frame and center-of-mass-frame momentum and energy tables.
- `physics/optics/lens-systems` *(gen)*: object distance, focal lengths, and the resulting image position and magnification.
  - Cover distinct optical architectures: single thin lens (real inverted vs. virtual magnifier), telephoto pair, Galilean negative-eyepiece system, and afocal beam expander.
  - Include boundary regimes alongside standard conjugates: object near the focal point ($s \approx f$) and symmetric $2f$ relays ($M = -1$).
- `physics/circuits/rc-transients` *(gen)*: an RC or RL step response with time constant and a short voltage table.
  - Vary topologies: capacitor step charge, stored-charge shunt decay, RL current rise, and inductive break / flyback spike.
  - Span sub-$\mu\text{s}$ switching to multi-second thermal/filtering transients across non-uniform $t/\tau$ points ($0.5\tau, \tau, 3\tau, 5\tau$).
- `physics/thermodynamics/cycles` *(gen)*: Carnot, Otto, or Rankine state tables with work and efficiency.
  - Rotate across Carnot, Otto, Diesel (cutoff ratio), Brayton, and subcritical Rankine (with two-phase steam quality $x$) cycles.
  - Tabulate state vertices $(P, V, T, s)$ alongside net work and second-law reversible efficiency limits.
- `physics/waves/standing` *(gen)*: string or pipe length, tension, or temperature, and its first harmonics in Hz and note names.
  - Vary boundary topologies: fixed-fixed string, open-open bore, and stopped pipe (quarter-wave resonator producing odd harmonics $1f, 3f, 5f$ only).
  - Vary linear mass density ($\mu$) or ambient gas temperature $c(T)$, and show node/antinode positions.
- `physics/acoustics/room-modes` *(gen)*: a rectangular room's dimensions and its lowest axial, tangential, and oblique resonant frequencies.
  - Contrast well-proportioned rooms (Sepmeyer/Bolt ratios) against degenerate cubes ($L = W = H$ modal clustering) and narrow corridors.
  - Distinguish 1D axial, 2D tangential, and 3D oblique modes below the Schroeder frequency.
- `physics/orbital/transfers` *(gen)*: two circular orbits around a parent body and the Hohmann transfer delta-v budget and transit time.
  - Contrast standard coplanar Hohmann transfers, bi-elliptic three-impulse transfers at high radius ratios ($r_2/r_1 > 11.94$), and combined plane-change transfers at apogee.
  - Span LEO-to-GEO ascent, lunar capture, and heliocentric planetary transfers.
- `physics/dimensional/buckingham-pi` *(gen)*: a physical relation reduced to its dimensionless Pi groups from a nullspace basis of the unit matrix.
  - Span fluid drag (Reynolds/Mach), hull resistance (Froude/Weber), natural convection (Nusselt/Rayleigh/Prandtl), and blast scaling (Taylor-Sedov).
  - Show the $[M, L, T, \Theta]$ dimensional matrix, repeating variables, and resulting independent $\Pi$-groups.
- `physics/constants`: CODATA constants with value, uncertainty, and measurement or unit-realization context.
  - Span exact SI defining constants ($c, h, e, k_B$), low-uncertainty QED constants ($\alpha, R_\infty$), and macroscopic invariants ($G$).
  - Pair measured values with the primary apparatus (cold-atom recoil, Penning trap, torsion pendulum). For exact SI defining constants, describe unit realization (such as a Kibble balance), not a new measurement of a fixed numerical value.
- `physics/units/obsolete`: furlongs, barleycorns, grains, and ells with exact SI conversions and where they were used.
  - Cover textile commerce (Flemish/Scottish ells), surveyor chains and rods, apothecary weights (scruples, drachms), maritime fathoms, and regional systems (French *toise*, Russian *verst/pood*, German mining *Lachter*).
- `physics/thought-experiments`: Maxwell's demon, Newton's bucket, the twin paradox, stated in a few sentences each.
  - Span relativity (twin paradox, Bell's spaceships), thermodynamics (Maxwell's demon, Brownian ratchet), and quantum mechanics (Wigner's friend, Elitzur-Vaidman bomb tester).

### body
- `body/anatomy/bones`: Gray's Anatomy (1918) entries for single bones and their landmarks.
  - Sample across axial cranium (ethmoid, sphenoid), irregular vertebrae, appendicular girdle, carpal/tarsal clusters, and sesamoid bones, retaining articulations and landmarks (sulci, foramina, processes).
- `body/anatomy/muscles`: origin, insertion, innervation, and action.
  - Balance intrinsic ocular/laryngeal muscles, deep spinal stabilizers, forearm flexors/extensors, and large prime movers.
- `body/physiology/reflexes`: stimulus, pathway, and response for named clinical reflexes.
  - Cover monosynaptic deep tendon (patellar), polysynaptic cutaneous (corneal, cremasteric), cranial autonomic (pupillary light, vestibulo-ocular), and primitive release reflexes.
- `body/medicine/historical-remedies`: period pharmacopoeia entries, quoted with source and year.
  - Draw verbatim from Dioscorides, Avicenna, Culpeper, *Pharmacopoeia Londinensis* (1618), and 19th-century dispensatories, preserving preparation methods (decoction, calcination, electuary).
- `body/medicine/eponymous-signs`: Babinski, Murphy, Trousseau, with what the sign indicates.
  - Span surgical abdomen (Murphy, Rovsing), neuromotor (Babinski, Hoffmann), metabolic/tetany (Chvostek, Trousseau), and cardiovascular signs, pairing the bedside maneuver with the response.
- `body/nutrition/foods` *(gen)*: one food item with a USDA FoodData Central nutrient profile.
  - Sample across culinary staples, leafy brassicas, oily fish, legumes, spices, and organ meats, varying between raw, fermented, and cooked preparations.

### society
- `society/law/legal-maxims`: Latin maxims with translation and a classic application.
  - Span equity (*vigilantibus non dormientibus*), contract (*pacta sunt servanda*), tort/property boundaries, and statutory interpretation (*expressio unius*) from Roman Digest, Coke, and Blackstone.
- `society/law/landmark-cases`: one-paragraph facts and holdings of public-domain court opinions.
  - Mix everyday private-law disputes (contract remoteness in *Hadley*, escaping reservoirs in *Rylands*, chimney-sweep bailment in *Armory*) with constitutional and admiralty holdings.
- `society/law/constitutions`: single articles from the world's constitutions (Constitute Project texts).
  - Sample across presidential, parliamentary, and federal charters worldwide, mixing ordinary procedural mechanics (quorums, bill readings) with emergency powers and rights clauses.
- `society/economics/price-histories`: commodity prices over a decade from public statistical series.
  - Span medieval English grain (Beveridge), 16th-century Spanish silver inflation (Hamilton), Edo-period Osaka rice, and 19th-century industrial staples, including both stable decades and famine/blockade shocks.
- `society/economics/market-clearing` *(gen)*: linear supply and demand, equilibrium, and the effect of a tax or price ceiling.
  - Vary regimes: undisturbed clearing, per-unit tax incidence across inelastic vs. elastic curves, binding price ceilings (shortages), and non-binding vs. binding price floors.
- `society/economics/auctions` *(gen)*: bidder valuations, submitted bids or an explicit strategy and information model, and outcomes under English, Dutch, and Vickrey rules.
  - Contrast independent private values against common-value auctions with the winner's curse, and include all-pay or reserve-price edge cases where a high-valuation item goes unsold.
- `society/economics/game-theory` *(gen)*: a 2x2 or 3x3 normal-form payoff matrix with its pure and mixed Nash equilibria and Pareto frontier.
  - Rotate across Prisoner's Dilemma, Stag Hunt (payoff- vs. risk-dominant equilibria), Hawk-Dove, Battle of the Sexes, and zero-sum cyclic games with unique mixed equilibria.
- `society/economics/matching` *(gen)*: two-sided preference lists and the Gale-Shapley proposer-optimal and receiver-optimal stable matchings.
  - Mix aligned preference profiles (unique stable matching) with conflicted/cyclic preferences where proposer-optimal and receiver-optimal matchings diverge across multiple pairs.
- `society/economics/fair-division` *(gen)*: heterogeneous item valuations across agents and an envy-free or utilitarian allocation.
  - Vary between divisible cake-cutting (Selfridge-Conway) and indivisible goods (Adjusted Winner, EF1), including instances where utilitarian efficiency and envy-freeness conflict.
- `society/voting/elections` *(gen)*: ranked ballots counted under plurality, IRV, Borda, and Condorcet, where the winners differ.
  - Mix clean Condorcet-winner elections with Condorcet cycles ($A \succ B \succ C \succ A$), IRV center-squeeze eliminations, and Borda-vs-majority inversions.
- `society/voting/apportionment` *(gen)*: state or party populations allocated seats under Hamilton, Jefferson, Webster, and Huntington-Hill, highlighting seat flips.
  - Include instances of the Alabama paradox (adding a seat reduces a small state's allocation) alongside divisor-method splits between large-state (Jefferson/d'Hondt) and small-state (Adams) bias.
- `society/kinship/systems`: Eskimo, Hawaiian, Iroquois, Crow, Omaha, and Sudanese kinship terminologies with cousin-merging rules.
  - Contrast parallel- vs. cross-cousin mergers and matrilineal/patrilineal generational skewing across concrete ethnographic examples.
- `society/law/sumptuary`: historical sumptuary laws specifying permitted fabrics, colors, and ornaments by rank.
  - Draw from Tokugawa Edo edicts, Elizabethan Statutes of Apparel, Renaissance Italian communes, and Roman *Lex Oppia*, listing specific forbidden materials and penalties.
- `society/occupations/historical`: trades like cooper, fuller, knocker-upper, and lamplighter, with a period description.
  - Mix rural agricultural crafts, urban maintenance/sanitation trades, maritime dock work, and workshop manufacturing, retaining period tools and materials.
- `society/occupations/modern`: O*NET occupation titles with a few concrete daily tasks.
  - Balance ordinary service, clerical, and municipal jobs alongside specialized industrial, medical, and field-technician roles.
- `society/etiquette/historical`: rules quoted from 19th-century etiquette manuals.
  - Sample table manners, calling-card corner folds, street salutations, mourning dress gradations, and correspondence rules verbatim from period manuals.
- `society/institutions/charters`: founding clauses of guilds, universities, and companies.
  - Span medieval craft guilds, early universities, municipal borough grants, and chartered trading companies.

### belief
- `belief/mythology/greek`: episodes from Bulfinch or Apollodorus.
  - Mix major Olympian myths with minor local metamorphoses, genealogical oddities, foundation legends, and chthonic rituals from Apollodorus and Pausanias.
- `belief/mythology/norse`: Prose Edda episodes in public-domain translation.
  - Span cosmogony, dwarfs' forging contests, Utgarda-Loki's illusions, skaldic kenning origins, and Ragnarök from Brodeur's translation of Snorri.
- `belief/mythology/mesoamerican`: Popol Vuh passages.
  - Sample the failed mud/wood creation cycles, Xibalba's underworld trial houses, Hero Twins' ballgames, and early Highland migration passages.
- `belief/scripture/wisdom`: single verses from Proverbs, Ecclesiastes, the Dhammapada, and the Tao Te Ching.
  - Contrast pragmatic household proverbs, existential laments, monastic mindfulness verses, and Daoist reversals in public-domain translations.
- `belief/saints/calendar`: feast day, saint, and one-line legend.
  - Mix well-known martyrs and doctors of the church with obscure local patron saints, desert anchorites, and strange iconographic emblems.
- `belief/divination/i-ching` *(gen)*: a cast hexagram with changing lines and the resulting hexagram, using the Legge text.
  - Use true yarrow-stalk or three-coin probabilities (including static 0-moving-line casts alongside multi-line transitions) with Legge's 1882 judgment and line texts.
- `belief/divination/tarot` *(gen)*: a three-card spread with orientations and Waite's 1911 meanings.
  - Vary spread roles (Past/Present/Future, Situation/Action/Outcome) and mix Major Arcana trumps with ordinary Minor Arcana pip and court cards, upright and reversed.

### computing
- `computing/data-structures/heaps` *(gen)*: a sequence of inserts and pops with the array after each step.
  - Contrast min vs. max heaps, root-to-leaf sift-down cascades, duplicate-key ties, and Floyd bottom-up bulk heapify vs. incremental insertion.
- `computing/data-structures/hash-tables` *(gen)*: keys inserted under linear probing or chaining, with the final table.
  - Contrast separate chaining against linear/quadratic probing and Robin Hood hashing, including primary clustering runs and tombstone reuse.
- `computing/data-structures/red-black` *(gen)*: insertions with rotations and recolorings named.
  - Mix single rotations (zig-zig), double rotations (zig-zag), and red-uncle cascading recolorings propagating to the root.
- `computing/automata/dfa` *(gen)*: a small DFA, its regular expression, and accepted and rejected strings.
  - Vary topologies: acyclic finite languages, modular counting cycles, trap-state substring filters, and minimal vs. unminimized equivalent automata.
- `computing/automata/cellular` *(gen)*: an elementary CA rule number with 16 generations as text.
  - Sample across Wolfram Classes 1–4 (uniform extinction, periodic stripes, chaotic Rule 30 triangles, and glider-supporting Rule 110) from impulse or random seeds.
- `computing/automata/turing` *(gen)*: a busy-beaver-style machine table and its first steps.
  - Contrast short halting Busy Beaver contenders, binary tape counters, and oscillating head shuttles.
- `computing/regex/puzzles` *(gen)*: two string lists and a regex that separates them.
  - Vary character-class parity, anchored prefixes/suffixes, alternating repeat counts, and forbidden substring patterns.
- `computing/sql/queries` *(gen)*: a tiny table, a query, and its result set.
  - Mix simple joins and `GROUP BY` rollups with self-joins, window functions (`LAG`, `ROW_NUMBER`), and three-valued `NULL` logic edge cases (`NOT IN` with `NULL`).
- `computing/bits/tricks` *(gen)*: one integer under popcount, ctz, gray code, and bit reversal.
  - Mix ordinary integers with structural bit masks (`0x5555...`), powers of two minus one, isolated high/low bits, and two's-complement boundaries (`INT_MIN`, `-1`).
- `computing/floating-point/ieee754` *(gen)*: an exact float value decomposed into sign, exponent, mantissa bits, hex representation, and ULP step.
  - Mix ordinary normalized numbers with `-0.0`, subnormals, $\pm\infty$, quiet/signaling NaNs, and precision boundaries ($2^{24}$ in `f32`, $0.1 + 0.2$).
- `computing/concurrency/schedules` *(gen)*: a two-thread instruction interleaving that exposes a race condition, lost update, or store-buffer litmus anomaly.
  - Cover read-modify-write lost updates, lock-free ABA interleavings, deadlock lock-order inversions, and store-buffer (`SB`) or message-passing (`MP`) weak-memory reorderings.
- `computing/compilers/ssa` *(gen)*: a short branching basic-block sequence lowered into Static Single Assignment form with phi nodes.
  - Vary control flow across if-else diamonds, while-loop header $\phi$-nodes with loop-carried dependencies, and early-exit branches.
- `computing/type-systems/inference` *(gen)*: a lambda term, its generated Hindley-Milner type constraints, and the principal type via unification.
  - Cover standard combinators (`S`, `K`, `B`, `C`), polymorphic `let`-generalization, and self-application terms (`\x -> x x`) that fail the occurs check.
- `computing/distributed/consensus` *(gen)*: a vector clock causal history or Lamport timestamp trace across nodes with concurrent events marked.
  - Mix clean causal chains with concurrent divergent writes ($a \parallel b$), out-of-order message delivery, and majority-quorum intersections.
- `computing/networking/subnets` *(gen)*: a CIDR block split into subnets with ranges and broadcast addresses.
  - Vary between uniform subnet splits, variable-length subnet masking (VLSM), `/31` point-to-point links, and non-byte-boundary masks (`/19`, `/27`).
- `computing/encoding/utf8` *(gen)*: one Unicode code point with its name, UTF-8 and UTF-16 bytes, and block.
  - Sample across 1-, 2-, 3-, and 4-byte UTF-8 widths (including UTF-16 surrogate pairs), combining marks, zero-width joiners, and mathematical/technical symbols.
- `computing/history/rfcs`: memorable opening paragraphs or definitions from early RFCs.
  - Mix foundational protocol specs (IPv4, TCP, DNS, HTTP), Postel's robustness principle, and classic April 1 RFCs (IP over Avian Carriers).
- `computing/history/error-messages`: famous historical error messages and failure reports, such as the first logged bug.
  - Span hardware incident logs (Mark II moth, Apollo 1202 alarm), OS kernel panics (`lp0 on fire`, Guru Meditation), and terse Unix/compiler diagnostics.
- `computing/esolangs`: short programs in Brainfuck, Befunge, or FRACTRAN with what they compute.
  - Contrast 1D tape loops (Brainfuck), 2D directional grid routing (Befunge), and prime-exponent register machines (FRACTRAN).

### cryptography
- `cryptography/classical/ciphers` *(gen)*: a proverb under Vigenère, Playfair, or rail-fence with its key.
  - Contrast polyalphabetic substitution (Vigenère), digraph grid substitution (Playfair with `I/J` merge and double-letter padding), and rail-fence/columnar transpositions.
- `cryptography/classical/frequency`: letter-frequency tables for different languages.
  - Contrast vowel-heavy, consonant-cluster-heavy, and diacritic-rich orthographies alongside English unigram and digraph distributions.
- `cryptography/modern/rsa-toy` *(gen)*: small primes, keys, and one encrypt-decrypt round.
  - Show $p, q$, $\phi(n)$ or $\lambda(n)$, extended GCD for $d \equiv e^{-1}$, and modular exponentiation, including edge cases like close primes (Fermat factorization) or small $e=3$.
- `cryptography/secret-sharing/shamir` *(gen)*: a secret and threshold polynomial over a small prime field with k-of-n shares and Lagrange reconstruction.
  - Vary thresholds $(k, n)$ over $\text{GF}(p)$, showing the share points and exact Lagrange interpolation weights at $x = 0$.
- `cryptography/error-correction/hamming` *(gen)*: a message encoded as a Hamming(7,4) or Reed-Solomon codeword, a single-error corruption, and its syndrome decoding.
  - Contrast clean zero-syndrome codewords, single-bit flips pinpointed by $S = H r^T$, and SECDED Hamming(8,4) double-bit errors detected by overall parity.
- `cryptography/historical/codebooks`: entries from public-domain commercial telegraph codebooks.
  - Sample shipping, banking, commodity, and personal travel phrases mapped to single 5-letter codewords from 19th- and early-20th-century cable codes.

### built-environment
- `built-environment/architecture/orders`: column orders and their proportions from Vitruvius or Vignola.
  - Cover Tuscan, Doric, Ionic, Corinthian, and Composite module ratios, intercolumniation spacings (pycnostyle to araeostyle), and entablature divisions.
- `built-environment/architecture/buildings`: notable buildings with architect, year, material, and one distinctive feature.
  - Mix civic landmarks, industrial warehouses, sacred structures, and domestic houses across timber, masonry, cast iron, and reinforced concrete.
- `built-environment/architecture/floor-plans` *(gen)*: small house plans in ASCII with room sizes that respect adjacency.
  - Vary circulation topologies: central-hall double-pile, axial enfilade, courtyard peristyle, and linear shotgun layouts.
- `built-environment/urban/street-grids`: cities with their founding plan type and date.
  - Contrast Roman *cardo/decumanus* castra, Spanish *Leyes de Indias* plazas, baroque radial plans, Jeffersonian grids, and organic hilltop towns.
- `built-environment/infrastructure/bridges`: span, type, material, and opening year.
  - Span stone voussoir arches, timber covered trusses, cast-iron ribs, suspension spans, cantilevers, and cable-stayed bridges.
- `built-environment/infrastructure/lighthouses`: light characteristic, such as Fl(3) W 15s, with location and height.
  - Contrast wave-swept offshore rock towers, screw-pile shoal lights, and coastal headland beacons across Fresnel orders and flash rhythms.
- `built-environment/vernacular/dwellings`: yurt, trullo, and longhouse, with construction method and climate.
  - Span arid courtyard houses, dry-stone corbelled domes, nomadic felt lattices, stilted tropical houses, and turf/timber northern dwellings.
- `built-environment/fortifications/vauban`: bastion trace geometry, ravelins, glacis, and enfilade angles from early modern siege treatises.
  - Cover star-fort polygons, ravelins, hornworks, covered ways, and siege sapping parallels.
- `built-environment/surveying/metes-and-bounds`: historical property descriptions in chains, rods, links, and compass bearings.
  - Mix riparian meander descriptions, colonial tree-and-cairn boundary calls, and township-and-range section corners.
- `built-environment/acoustics/theatres`: classical and modern hall proportions, reverberation times, and sightline geometry.
  - Contrast stepped ancient amphitheatres, Italian horseshoe opera houses, shoebox concert halls, and surround vineyard halls.

### materials
- `materials/alloys/compositions`: classic bronzes, brasses, tool steels, and solders with weight-percent composition, phase structure, and working properties.
  - Cover bell and gunmetal bronzes, alpha/duplex brasses, carbon and high-speed tool steels, pewters, and eutectic solders.
- `materials/metallurgy/heat-treatment`: quench media, tempering temperatures, temper colors (straw, peacock, blue), and resulting hardness.
  - Span annealing, normalizing, water/oil/air quenching, and oxide temper-color schedules from pale straw ($220^\circ\text{C}$) to blue ($300^\circ\text{C}$).
- `materials/woods/species`: Janka hardness, specific gravity, grain character, shrinkage ratios, and traditional uses from USDA Forest Service Wood Handbook data.
  - Contrast softwoods, ring-porous temperate hardwoods (oak, ash, hickory), diffuse-porous carving woods (basswood, boxwood), and dense tropical heartwoods.
- `materials/stone/masonry`: freestone, ashlar, rubble, and coursing bonds with quarry dressing tools.
  - Span limestone freestones, sandstones, granites, slates, and marbles alongside ashlar, rusticated, and rubble coursing bonds.
- `materials/pigments/historical`: vermilion, ultramarine, mummy brown, lead white, and Prussian blue with chemical formula, refractive index, and permanence.
  - Mix natural earth ochres, ground minerals (lapis, malachite, cinnabar), lake dyes, and synthetic breakthroughs (Prussian blue, cobalt, viridian).
- `materials/glass/batches`: soda-lime, borosilicate, flint, and crown glass batch recipes and softening points.
  - Contrast potash forest glass, Venetian cristallo, leaded optical flint, low-expansion borosilicate, and metallic-oxide colored glasses.
- `materials/fibers/natural`: bast, leaf, and animal fibers (flax, ramie, coir, mohair) with staple length and tensile behavior.
  - Span seed hairs (cotton, kapok), bast stems (flax, hemp, ramie), leaf fibers (sisal, abaca), and protein fleeces/silks.
- `materials/aging-and-failure`: observed wear, cracking, spalling, galvanic corrosion, and patina from conservation records, with material and environment.
  - Cover galvanic couples, bronze disease, glass weeping/crizzling, stone salt efflorescence, wood checks, and textile photodegradation.

### transport
- `transport/rail/timetables` *(gen)*: a plausible branch-line timetable with consistent running times.
  - Vary operational structures: single-track passing-loop meets, express overtaking a local freight, clock-face commuter intervals, and branch-junction timed transfers.
- `transport/rail/locomotives`: wheel arrangement, builder, and service years.
  - Span early 19th-century steam, articulated mountain freight engines (Mallet, Garratt), streamlined passenger Pacifics/Hudsons, and classic diesel/electric locomotives.
- `transport/aviation/airports`: IATA code, name, city, elevation, and runway count from OurAirports.
  - Mix major international hubs, high-altitude mountain strips, island airstrips, and ordinary regional fields.
- `transport/aviation/metar` *(gen)*: METAR reports decoded into plain English.
  - Vary weather regimes from calm CAVOK and radiation fog to gusting crosswinds, winter freezing rain (`FZRA`), and convective squalls (`+TSRA`).
- `transport/maritime/knots`: knot name, ABOK number, use, and tying steps.
  - Cover fixed loops, binding knots, bends joining two ropes of unequal diameter, friction hitches, and decorative stopper knots from Ashley.
- `transport/maritime/flags`: signal flags and their single-flag meanings.
  - Cover alphabet flags, numeral pennants, substitutes/repeaters, and urgent single- or two-letter International Code signals.
- `transport/maritime/ship-types`: historical rigs such as brigantine, xebec, and junk, with sail plans.
  - Contrast full-rigged square ships, fore-and-aft schooners/cutters, Mediterranean lateen vessels, Arab dhows, and battened lug-sail junks.
- `transport/maritime/beaufort-and-sea-state`: Beaufort wind scale numbers paired with sea-surface appearance and sail-handling orders.
  - Span Force 0 (flat calm) through Force 6 (reef topsails) to Force 12 (hurricane, bare poles).
- `transport/maritime/buoyage` *(gen)*: IALA lateral, cardinal, and isolated-danger marks with topmark, color pattern, and light rhythm.
  - Vary across Region A vs. Region B lateral marks, North/East/South/West clock-face cardinal flashes, safe-water marks, and isolated-danger beacons.
- `transport/navigation/dead-reckoning` *(gen)*: a rhumb-line or great-circle leg with course, leeway, current set and drift, and corrected position.
  - Mix coastal tidal-current vector triangles, windward leeway corrections, and high-latitude legs where rhumb-line and great-circle courses diverge.

### arts
- `arts/painting/works`: title, artist, year, medium, and museum for public-domain paintings (Met or Rijksmuseum open data).
  - Span Renaissance panels, Dutch Golden Age genre scenes, Edo ukiyo-e, Mughal miniatures, and 19th-century landscapes, noting support/ground and dimensions.
- `arts/painting/techniques`: grisaille, sfumato, and impasto, with a canonical example.
  - Contrast optical glazing, alla prima impasto, verdaccio underpainting, encaustic, and buon fresco, noting binder chemistry and working time.
- `arts/sculpture/works`: works with material and location.
  - Span subtractive carving (marble, porphyry, limewood), lost-wax bronze casting, terracotta, and assemblage across classical, non-Western, and modern periods.
- `arts/film/silent`: plot summaries of public-domain silent films.
  - Mix early actualities and slapstick shorts with German expressionism, Soviet montage, and French impressionist features.
- `arts/film/shot-lists` *(gen)*: a short scene broken into shot sizes, angles, and moves that stay consistent.
  - Vary visual grammar: dialogue reverse-angles preserving the 180-degree line, continuous tracking blocking, cross-cut parallel action, and silent reaction sequences.
- `arts/theatre/stage-directions`: notable stage directions from public-domain plays.
  - Contrast Shakespearean exits and dumbshows, Jacobean trapdoor/machinery cues, Ibsen's naturalistic room descriptions, and Shaw's character entrances.
- `arts/dance/notation`: ballet positions and steps, or contra dance figures with call sequences.
  - Mix classical ballet *enchaînements*, baroque Feuillet floor paths, and 32-bar contra/square dance call progressions with beat counts.
- `arts/photography/exposure` *(gen)*: a lighting scene with aperture, shutter, and ISO triples of equal exposure.
  - Sample from EV -2 (starlight) to EV 16 (snow in sun), contrasting motion-freeze, deep-focus hyperfocal, and wide-open shallow-depth-of-field equivalents.
- `arts/stagecraft/lighting-gels`: theatrical filter numbers, transmission spectra, and psychological or period lighting uses.
  - Cover saturated color primaries, CTO/CTB color-temperature shifts, pale cosmetic tints, and frost diffusions.
- `arts/printmaking/intaglio-and-relief`: etching, mezzotint, drypoint, aquatint, and woodblock processes with tool marks and ink behavior.
  - Contrast relief (woodcut, linocut), intaglio (burin engraving, acid etching, mezzotint rocker), and planographic lithography.
- `arts/perspective/projections`: one-point, two-point, isometric, cavalier, and cabinet projection rules with vanishing-point or foreshortening ratios.
  - Contrast converging linear perspective (1-, 2-, and 3-point) with parallel axonometric and oblique projections (isometric, dimetric, cavalier, cabinet).

### typography
- `typography/typefaces`: historical faces with designer, year, classification, and distinguishing letterforms.
  - Span blackletter, humanist/Garalde old-style, transitional, Didone, slab serif, and grotesque/geometric sans, noting stress axis and diagnostic glyphs (`g`, `Q`, `a`).
- `typography/glyphs`: obscure Unicode punctuation and symbols such as the pilcrow, interrobang, and manicule, with history.
  - Cover scribal marks (obelus, diple), section delimiters (pilcrow, hedera, fleuron), editorial pointers (manicule), and commercial/alchemical ligatures.
- `typography/scripts`: writing systems with direction, type, and a sample word in script.
  - Contrast abjads, abugidas, alphabets, syllabaries, and featural scripts across LTR, RTL, vertical, and boustrophedon directions.
- `typography/ciphers/runes` *(gen)*: a short word transliterated into Elder Futhark, Ogham, or Cherokee.
  - Respect each script's orthographic rules: Younger/Elder Futhark double-consonant collapse, Ogham stem-notch *aicme* groups, or Cherokee syllabary CV mapping.

### food
- `food/ingredients/spices`: origin, flavor compounds, and classic pairings.
  - Span barks, buds, umbellifer seeds, rhizomes, and dried chilies, pairing primary volatile compounds (eugenol, cuminaldehyde, piperine) with fat/water solubility.
- `food/ingredients/cultivars`: apple, potato, and rice varieties with traits.
  - Contrast high-tannin cider vs. dessert apples, waxy vs. floury potatoes, and glutinous vs. long-grain aromatic rice varieties.
- `food/dishes/regional`: named regional dishes with core ingredients and origin.
  - Balance everyday homestyle porridges, flatbreads, and street snacks alongside complex regional braises and festival dishes worldwide.
- `food/fermentation`: process, organism, time, and temperature for kimchi, miso, kefir, and garum.
  - Contrast lacto-fermentation, *Aspergillus* koji saccharification, SCOBY/kefir cultures, and enzymatic autolysis, including salt %, pH, and temperature ranges.
- `food/menus/historical`: items with prices from digitized historical menus (NYPL What's on the Menu).
  - Mix ordinary tavern, diner, and railway lunch-counter bills of fare with ocean-liner and civic banquet menus.
- `food/drinks/cocktails`: IBA cocktails with specs and glassware.
  - Cover sours, spirit-forward stirred drinks, highballs/fizzes, flips, and punches with proportions, ice method, and glassware.
- `food/baking/ratios` *(gen)*: a bread or pastry at baker's percentages, scaled to a given flour weight.
  - Vary dough regimes: high-hydration lean sourdough, stiff bagel dough, enriched brioche, laminated pastry, shortcrust, and high-moisture choux paste.

### sport
- `sport/rules/oddities`: strange real rules from sports rulebooks.
  - Sample edge-case clauses across cricket, baseball, soccer, golf, sumo, and athletics, citing the trigger condition and ruling.
- `sport/records/historical`: progression of a world record with athletes and dates.
  - Track chronological progressions showing both incremental improvements and sudden leaps from technique or equipment changes (Fosbury flop, fiberglass pole, klap skate).
- `sport/tournaments/brackets` *(gen)*: a seeded single-elimination bracket with plausible upsets.
  - Vary between chalk-heavy brackets, early Cinderella upsets (12-over-5, 15-over-2), and unseeded random draws.
- `sport/climbing/routes`: famous routes with grade, first ascent, and style.
  - Span gritstone boulders, Yosemite granite splitters, Dolomite limestone big walls, and sport testpieces across grading systems.

### mind
- `mind/cognitive-biases`: the bias with a concrete classic experiment.
  - Cover anchoring, availability, base-rate neglect, framing, and confirmation bias through the specific experimental setup and observed choices.
- `mind/illusions`: perceptual illusions with how to construct them.
  - Contrast geometric line/size illusions, luminance/contrast effects (Mach bands, Hermann grid), multistable figures, and motion/color afterimages.
- `mind/emotions/lexicon`: untranslatable emotion words with language and a usage sentence.
  - Span affect terms across diverse language families, pairing morphological literal meaning with the social or sensory situation it names.
- `mind/psychology/experiments`: classic studies with design, n, and result.
  - Span perception, memory reconstruction, conditioning, infant attachment, and group conformity studies.
- `mind/dreams/historical`: dream reports from public-domain dream books and diaries.
  - Draw verbatim accounts from antiquity (Artemidorus), early modern diaries, and 19th-century journals, keeping mundane and bizarre dreams alike.

### philosophy
- `philosophy/arguments`: famous arguments in numbered premise form, such as the ontological argument or the Chinese room.
  - Span metaphysics, epistemology, ethics, and philosophy of mind in clean numbered premise-and-conclusion form.
- `philosophy/paradoxes`: Sorites, Newcomb, and Ship of Theseus, each in two or three sentences.
  - Cover semantic self-reference, vagueness, decision theory, epistemic surprise (Unexpected Hanging), and persistence/identity puzzles.
- `philosophy/dilemmas` *(gen)*: trolley-style variants built from orthogonal factors, not template swaps.
  - Vary agency (doing vs. allowing), physical proximity (remote switch vs. direct contact), causal role (harm as means vs. side effect), and epistemic certainty.
- `philosophy/logic/syllogisms` *(gen)*: a categorical syllogism with mood, figure, and validity.
  - Mix unconditionally valid moods (Barbara, Celarent), moods valid only under Aristotelian existential import (Barbari, Darapti), and classic formal fallacies (undistributed middle).
- `philosophy/logic/truth-tables` *(gen)*: a propositional formula, its table, and whether it is a tautology.
  - Mix classical tautologies (Peirce's law, contraposition), contradictions, and contingent formulas with a single satisfying or falsifying row.
- `philosophy/logic/modal-kripke` *(gen)*: a small Kripke frame (worlds and accessibility relations) and modal formulas that hold or fail at a world.
  - Construct 2–4 world frames that separate modal axioms ($K, D, T, B, S4, S5$) by breaking reflexivity, transitivity, symmetry, or Euclidean closure.
- `philosophy/epistemology/indian-pramanas`: perception, inference, comparison, postulation, and non-apprehension schools and counterexamples from classical Nyaya and Mimamsa texts.
  - Contrast Nyaya, Buddhist, Mimamsa, and Carvaka views through classic debate examples (smoke and fire on the hill, rope-snake illusion, Devadatta not at home).
- `philosophy/fallacies`: a named fallacy with a real historical instance.
  - Pair formal and informal fallacies with real historical arguments from speeches, trials, or pamphlets rather than synthetic textbook sentences.

### earth
- `earth/minerals`: Mohs hardness, crystal system, luster, and streak (Mindat or public USGS data).
  - Span silicates, carbonates, oxides, sulfides, halides, and native elements across all seven crystal systems.
- `earth/rocks`: igneous, sedimentary, and metamorphic types with formation conditions.
  - Contrast plutonic vs. volcanic igneous, clastic vs. chemical/biogenic sedimentary, and contact vs. regional high-pressure metamorphic rocks.
- `earth/earthquakes`: historical quakes with magnitude, depth, and effects.
  - Cover subduction megathrusts, continental strike-slip faults, deep-focus slab events, and intraplate earthquakes.
- `earth/volcanoes`: Smithsonian GVP volcanoes with type and last eruption.
  - Span shield volcanoes, stratovolcanoes, calderas, cinder cones, and submarine ridges across VEI scales.
- `earth/soils`: USDA soil orders with properties and where they occur.
  - Cover all 12 USDA soil orders from permafrost Gelisols and volcanic Andisols to prairie Mollisols and tropical Oxisols.
- `earth/stratigraphy/geologic-time`: periods and stages with dates and a marker event.
  - Span Precambrian eons to Quaternary epochs, noting GSSP boundary markers, index fossils, or isotopic excursions.
- `earth/oceanography/tides` *(gen)*: harmonic tidal constituents (M2, S2, K1, O1) summed into a high/low water table and spring-neap state.
  - Vary constituent amplitude ratios across semidiurnal, mixed, and diurnal regimes, including spring, neap, and king-tide alignments.
- `earth/meteorology/clouds`: WMO cloud genera, species, and varieties with altitude band and synoptic association.
  - Span high cirriform, mid-level altocumulus/lenticularis, low stratiform, and deep convective cumulonimbus types.
- `earth/meteorology/soundings` *(gen)*: surface temperature and dewpoint with lifted condensation level and dry/moist adiabatic lapse profile.
  - Contrast dry well-mixed boundary layers, radiation inversions, moist marine profiles, and unstable convective soundings.
- `earth/climate/koppen`: a location with its Köppen class and monthly temperature and precipitation table.
  - Span tropical rainforest/monsoon, hot/cold deserts, Mediterranean, humid continental, steppe, and polar tundra stations.

### systems
- `systems/queues/kendall` *(gen)*: an M/M/1 or M/M/c queue with arrival and service rates, utilization, mean wait time, and queue length distribution.
  - Contrast light traffic, heavy-traffic near-saturation ($\rho \to 1$), multi-server pooling ($M/M/c$ vs. $c$ separate $M/M/1$ queues), and finite-buffer $M/M/1/K$ loss regimes.
- `systems/ecology/lotka-volterra` *(gen)*: predator-prey or competing-species parameters with equilibrium points, Jacobian eigenvalues, and phase-portrait regime.
  - Keep classic Lotka–Volterra predator-prey neutral cycles distinct from competitive exclusion and bistability in competition models. State logistic-growth and saturating-response extensions explicitly when generating damped spirals or Hopf limit cycles; distinguish stable nodes from stable foci.
- `systems/epidemiology/compartments` *(gen)*: an SIR or SEIR model with R0, herd-immunity threshold, peak infectious fraction, and final epidemic size.
  - Contrast subcritical fadeout ($R_0 < 1$), sharp high-$R_0$ outbreaks, long-latent SEIR waves, and endemic SIRS equilibria with waning immunity.
- `systems/cybernetics/feedback-archetypes`: limits to growth, shifting the burden, tragedy of the commons, and bullwhip effect as causal loop structures.
  - Detail the reinforcing and balancing loops, delay links, and where local optimization destabilizes the whole system.
- `systems/reliability/fault-trees` *(gen)*: an AND/OR fault tree of component failure rates with minimal cut sets and top-event probability.
  - Contrast series single-point-of-failure chains, parallel active redundancy, $k$-out-of-$n$ voting gates, and shared common-cause failure nodes.
- `systems/operations/critical-path` *(gen)*: a small project DAG with task durations, earliest/latest start times, slack, and critical path.
  - Vary DAG topologies between single-bottleneck chains, diamond joins with near-zero slack on secondary paths, and co-critical parallel branches.
- `systems/flows/bottlenecks` *(gen)*: a small capacitated network with a checked max flow and min-cut bottleneck; vary topology and redundancy.
  - Include Braess-like routing topologies, single-bridge bottlenecks, and multi-edge min-cuts where local capacity increases do not raise total throughput.

## Additions to existing families

### chemistry
- `chemistry/reactions/organic/named`: Diels-Alder, Grignard, and Wittig with reactants, products, and conditions as SMILES.
  - Span cycloadditions, rearrangements, carbonyl condensations, and cross-couplings, noting stereochemical selectivity (endo/exo, E/Z) and reaction conditions.
- `chemistry/spectroscopy/nmr` *(gen)*: a small molecule and its predicted proton NMR peaks with splitting.
  - Vary coupling regimes: clean first-order $n+1$ multiplets, diastereotopic $\text{CH}_2$ geminal splitting, roofed AB quartets, exchangeable broad OH/NH singlets, and para-disubstituted aromatics.
- `chemistry/stoichiometry/limiting` *(gen)*: given masses, the limiting reagent and theoretical yield.
  - Mix clean stoichiometric ratios with near-equimolar boundary cases (where a small mass shift flips the limiting reagent) and multi-reactant excesses.
- `chemistry/lab/glassware`: apparatus with purpose and common misuse.
  - Cover distillation, extraction, air-free Schlenk, and volumetric apparatus (Soxhlet, Dean-Stark, Vigreux, Büchner).
- `chemistry/history/alchemy`: alchemical symbols and their modern substances.
  - Span classical planetary metals, acids/salts (aqua regia, vitriol, sal ammoniac), and operations (calcination, sublimation, putrefaction).

### design
- `design/typography-pairings` *(gen)*: a heading and body face pair with a reason drawn from their classification.
  - Vary pairing principles: shared historical period, shared x-height/skeleton across serif and sans, high-contrast display with neutral text, and monospace-plus-humanist combinations.
- `design/graphic/grids` *(gen)*: page size, margins, columns, and gutters that add up exactly.
  - Vary across classical manuscript proportions (2:3:4:6 margins), Swiss modular grids, asymmetric editorial layouts with marginalia columns, and poster grids.
- `design/heraldry/blazons` *(gen)*: a valid blazon that respects the rule of tincture, with a plain description.
  - Mix simple single-ordinary shields with divisions of the field, counterchanged charges, furs (ermine, vair), and canting arms.
- `design/flags/vexillology`: national and historic flags with their official proportions and blazon.
  - Mix classic triband and Nordic cross proportions with unusual aspect ratios (Qatar 11:28, Switzerland 1:1, Nepal's double pennant) and historic maritime jacks.
- `design/interiors/rooms` *(gen)*: a room with dimensions, furniture placements that fit, and a palette.
  - Vary room shapes and uses (narrow galley kitchen, square study, L-shaped living room), checking door swings and clearance paths.
- `design/textiles/knitting` *(gen)*: knitting charts for small lace or cable repeats.
  - Contrast balanced yarn-over/decrease lace motifs, 4- and 6-stitch cable crossings, and textured knit-purl gansey repeats.
- `design/textiles/quilting`: traditional quilt block names with grid and construction.
  - Span 4-patch, 9-patch, 8-point star, Log Cabin, and applique blocks across regional traditions.
- `design/industrial/objects`: iconic industrial designs with designer, year, and material.
  - Balance everyday anonymous utility objects (Mason jar, paperclip, Duralex tumbler) alongside named Bauhaus, Braun, and mid-century industrial designs.
- `design/book-arts/imposition` *(gen)*: folio, quarto, octavo, and duodecimo signature folding schemes with sheet-side page layouts.
  - Contrast sheetwise and work-and-turn layouts across 4-page folio, 8-page quarto, 16-page octavo, and 12mo offcut gatherings.
- `design/wayfinding/pictograms`: public-domain international transit, hazard, and map symbols with geometric construction ratios.
  - Span transit icons, ISO hazard triangles/circles, topographic map symbols, and hobo/trail markers.
- `design/origami/tessellations` *(gen)*: Kawasaki and Maekawa flat-foldability vertex assignments on a small grid.
  - Vary between Miura-ori rigid folds, waterbomb bases, square-twist tessellations, and single-vertex mountain/valley parity assignments.
  - Distinguish local vertex conditions from global flat-foldability and nonintersection. Use verified constructions for complete tessellations; local angle/parity checks alone do not certify a whole grid.

### games
- `games/board/go-openings`: joseki with sequences.
  - Contrast 3-4 (*komoku*), 4-4 (*hoshi*), and 3-3 (*san-san*) corner sequences balancing territory, outer influence, and *sente*.
- `games/board/shogi` *(gen)*: tsume problems.
  - Vary between 1-move, 3-move, and 5-move mates using piece drops, promotion sacrifices, and pinned defenders.
- `games/board/backgammon` *(gen)*: positions with a dice roll and the best move.
  - Contrast opening rolls, blitz vs. priming choices, back-game holding anchors, and late bear-off races.
- `games/puzzles/crosswords` *(gen)*: a 5x5 mini crossword with clues from the lexicon leaves.
  - Vary black-square symmetry patterns (open 5x5, corner steps, diagonal bars, central cross).
- `games/puzzles/kakuro` *(gen)*: Kakuro puzzles with unique solutions.
  - Mix forced Magic-block sums (e.g., 2 cells summing to 3 or 17; 3 cells summing to 6 or 24) with intersecting parity deductions.
- `games/puzzles/logic-grids` *(gen)*: Zebra-style puzzles with a unique solution and minimal clues.
  - Mix direct exclusions, relative ordering clues, and transitive implications so no single clue type dominates.
- `games/puzzles/mazes` *(gen)*: ASCII mazes built by different algorithms with visibly different textures.
  - Rotate across recursive backtracker (long winding passages), Kruskal's/Prim's (many short cul-de-sacs), Sidewinder, and braided loop mazes.
- `games/puzzles/rebus`: historical rebuses and riddles from public-domain collections.
  - Span positional typography puzzles (e.g., `STAND / I`), phonetic charades, and 18th/19th-century parlor riddles.
- `games/rpg/encounters` *(gen)*: a dungeon room with exits, contents, and a hazard that interlock.
  - Vary room functions (guardroom, cistern, scriptorium, forge, natural cavern) and pair environmental features with non-combat or mechanical interactions.
- `games/folk/playground`: traditional children's games with rules (Gomme's Traditional Games).
  - Span counting-out rhymes, chase/tag variants, hopscotch diagrams, knucklebones/jacks, and singing circle games.
- `games/cards/solitaire` *(gen)*: a Klondike or FreeCell deal labeled solvable or not.
  - Include both solvable deals (with move count) and provably deadlocked deals where mutual suit-rank blocks prevent progress.
- `games/cards/cribbage-hands` *(gen)*: four cards and a starter card with exact fifteen, pair, run, flush, and nobs point breakdown.
  - Mix ordinary mid-scoring hands with double/triple runs, high fifteens (28 and 29 hands), and zero-point nineteen hands.
- `games/tile/mahjong-waits` *(gen)*: a 13-tile tenpai hand in suit notation with its winning wait tiles and yaku.
  - Contrast simple *ryanmen* (two-sided) and *kanchan* (closed) waits with complex single-suit multi-sided (*chinitsu*) waits and Seven Pairs / Thirteen Orphans.
- `games/combinatorial/nim-and-kayles` *(gen)*: a multi-heap impartial game state, its Sprague-Grundy nim-value, and all winning moves.
  - Mix standard Nim, Misère Nim (endgame flip when heaps reach size 1), Dawson's chess, and Kayles bowling-pin splits across P- and N-positions.

### geography
- `geography/physical/islands`: area, population, and the nearest neighboring landmass.
  - Mix major archipelagos, river/lake islands, coral atolls, volcanic oceanic outposts, and ordinary coastal islands.
- `geography/physical/deserts-and-ice`: deserts, glaciers, and ice shelves with size and character.
  - Span subtropical sand seas, cold rain-shadow basins, valley glaciers, polar ice caps, and floating ice shelves.
- `geography/toponymy/etymologies`: place names and what they originally meant.
  - Cover topographical descriptions, river/ford crossings, personal names, substrate survivals, and bilingual tautologies across languages.
- `geography/maps/projections`: map projections with properties and distortion.
  - Contrast cylindrical, conic, azimuthal, pseudocylindrical, and interrupted projections across conformal, equal-area, and gnomonic properties.
- `geography/coordinates/antipodes` *(gen)*: a real city, its antipode, and what lies there.
  - Mix typical land-to-ocean antipodes with rare land-to-land antipodal pairs (Spain/New Zealand, Chile/China, Southeast Asia/South America).
- `geography/borders/oddities`: enclaves, tripoints, and exclaves with a short description.
  - Cover counter-enclaves, river-meander cutoffs, condominiums, quadripoints, and treaty corridors.
- `geography/time-zones`: zones with UTC offsets and historical oddities.
  - Mix standard hourly zones with 30- and 45-minute offsets, wide single-zone countries, and International Date Line salients.

### history
- `history/medieval/chronicles`: entries from the Anglo-Saxon Chronicle.
  - Mix sparse ordinary annal years (bishops consecrated, eclipses, hard frosts) with major campaigns and succession crises.
- `history/medieval/charters`: Magna Carta clauses with translation.
  - Balance constitutional due-process clauses alongside practical provisions on fish-weirs, standard cloth widths, wardship, and debt interest.
- `history/early-modern/broadsides`: public-domain broadside ballads and news sheets.
  - Span street ballads, execution confessions, market proclamations, and accounts of storms or comets.
- `history/science/notebooks`: entries from da Vinci's or Darwin's notebooks.
  - Mix quick observational jottings and shopping/reading lists with mechanical sketches, geological notes, and species-transmutation musings.
- `history/disasters`: shipwrecks, fires, and floods with date, toll, and cause.
  - Span maritime wrecks, urban fires, dam/bridge failures, mine collapses, and river floods.
- `history/inventions/patents`: early patent abstracts with drawings described.
  - Mix everyday workshop and household devices (fasteners, churns, lamps) with telegraph, steam, and textile machinery.
- `history/exploration/logbooks`: ship log entries with position and weather.
  - Mix routine blue-water watches (sail handling, noon sights, cask repairs) with landfalls, gales, and ice encounters.
- `history/china/classics`: passages from the Analects, Sun Tzu, and Zhuangzi in Legge or Giles translation.
  - Balance terse conversational exchanges in the *Analects*, strategic maxims in *Sun Tzu*, and craftsmen/animal parables in *Zhuangzi*.
- `history/africa/chronicles`: passages from Ibn Battuta and the Kebra Nagast.
  - Span West African and Swahili coast travel observations, Ethiopian royal narratives, and Timbuktu/Sudanic chronicle excerpts.
- `history/americas/codices`: descriptions of individual Aztec and Maya codex pages.
  - Cover tonalpohualli divination almanacs, Venus/eclipse tables, dynastic marriage/conquest histories, and provincial tribute lists.
- `history/everyday/letters-and-petitions`: complete short requests, complaints, and negotiations from archives, retaining the writer's words.
  - Span oxyrhynchus papyri, Vindolanda tablets, Cairo Genizah trade notes, and early modern parish or guild petitions.
- `history/commerce/reuse-and-substitution`: sourced instances of packaging, buildings, tools, or materials acquiring a second use.
  - Cover architectural spolia, ballast stones, feed-sack textiles, ship-timber framing, and melted-bell or coin recasting.

### language
- `language/languages/profiles`: word order, phoneme count, speakers, and family (WALS data).
  - Sample across major families and isolates, varying word order (SOV, SVO, VSO, VOS), tone, and morphological type.
- `language/phonology/ipa` *(gen)*: a word, its IPA, and a feature breakdown for one segment.
  - Mix common stops, fricatives, and vowels with ejectives, clicks, retroflexes, lateral fricatives, and secondary articulations.
- `language/morphology/paradigms`: verb conjugation tables for Latin, Finnish, Swahili, and Turkish.
  - Contrast fusional Latin stems, agglutinative vowel-harmony chains (Turkish/Finnish), and Bantu noun-class prefix slots (Swahili).
- `language/conlang/words` *(gen)*: words from a consistent invented phonotactics with derivations.
  - Vary syllable templates, vowel harmony or triconsonantal root rules, and derivational compounding patterns.
- `language/word-histories/doublets`: etymological doublets such as guard and ward or chef and chief.
  - Cover Old English/Old Norse pairs, Norman/Central French splits, and inherited vernacular vs. learned Latinate borrowings.
- `language/lexicon/collective-nouns`: real attested collective nouns with source.
  - Mix everyday herd/flock terms with 15th-century *Book of Saint Albans* terms of venery and occupational company nouns.
- `language/lexicon/obsolete`: obsolete English words with definitions and quotations.
  - Mix homely dialectal and agricultural words with 17th-century inkhorn coinages from Blount and Cockeram.
- `language/translation/false-friends`: false-friend pairs across languages.
  - Span Romance, Germanic, and Slavic pairs where a shared root diverged into contrasting everyday meanings.
- `language/palindromes-and-constraints`: famous palindromes, lipograms, and pangrams.
  - Cover letter-level and word-level palindromes, compact pangrams, univocalics, and acrostics across languages.
- `language/rhetoric/figures`: classical schemes and tropes (chiasmus, zeugma, hendiadys, litotes, aposiopesis) with a verbatim classical or early-modern example.
  - Contrast syntactic word-order schemes (chiasmus, hyperbaton, isocolon) with semantic tropes (zeugma, metonymy, litotes).
- `language/pragmatics/gricean-maxims`: conversational implicatures flouting quantity, quality, relation, or manner.
  - Cover understatement, irony, pointed topic shifts, and deliberate ambiguity in brief dialogue exchanges.
- `language/historical/sound-laws`: Grimm's, Verner's, and the Great Vowel Shift correspondences with cognate triples.
  - Show conditioning environments (such as PIE accent position in Verner's law) alongside clean three-language cognate sets.
- `language/writing-systems/numerals` *(gen)*: a number formatted in Babylonian sexagesimal, Mayan vigesimal, Attic Greek, and Cistercian cipher staves.
  - Include base-60 and base-20 zero-placeholder positions alongside additive acrophonic/alphabetic numerals.
- `language/correspondence/forms`: actual salutations, requests, refusals, and closings in attributed letters across languages and periods.
  - Contrast intimate family openings, formal business courtesies, diplomatic protocol, and terse refusals.

### literature
- `literature/drama/soliloquies`: public-domain dramatic monologues.
  - Span Greek tragedy, Elizabethan/Jacobean verse, French neoclassicism, and 19th-century realist prose plays.
- `literature/letters/authors`: letters by Keats, Austen, and Chekhov.
  - Mix everyday travel and domestic gossip with reflections on craft, revision, and observation.
- `literature/openings/first-lines`: first lines of public-domain novels with author and year.
  - Mix quiet situational openings, direct address, aphoristic hooks, and weather/landscape settings.
- `literature/epigraphs`: epigraphs with the work they head.
  - Pair the quoted epigraph text and its original source with the work or chapter it introduces.
- `literature/sermons`: excerpts from Donne and Edwards.
  - Contrast metaphysical conceits and meditative prose with plain-style homilies and funeral orations.
- `literature/children/nursery-rhymes`: rhymes with variants.
  - Cover lullabies, finger-plays, cumulative rhymes, and riddle verses from Halliwell and Opie-era sources.
- `literature/riddles/exeter-book`: Anglo-Saxon riddles in translation.
  - Mix everyday workshop and farm tools (plow, bellows, key, book-moth) with natural phenomena and weapons.
- `literature/verse/haiku`: Basho, Buson, and Issa in public-domain translation.
  - Span all four seasons (*kigo*) and contrast Basho's spare travel verses, Buson's visual tableaux, and Issa's humor toward small creatures.
- `literature/verse/ghazals`: Hafez and Ghalib in translation.
  - Preserve the autonomous couplet (*sher*) structure, rhyme (*qafia*), and refrain (*radif*).
- `literature/verse/african`: public-domain translations of praise poetry.
  - Span Yoruba *oriki*, Zulu *izibongo*, Hausa praise songs, and Somali *gabay* in public-domain ethnographic translations.
- `literature/criticism`: short passages from Johnson, Coleridge, and Hazlitt.
  - Mix close line-by-line practical criticism with broader observations on diction, dramatic illusion, and wit.
- `literature/poetic-forms/exemplars` *(gen)*: a meter or rhyme scheme scanned against a real public-domain stanza.
  - Cover iambic pentameter (with trochaic inversions and feminine endings), ballad meter, sapphics, Spenserian stanzas, and blank verse.
- `literature/narratology/structures`: kishotenketsu, ring composition (chiasmic structure), frame tale, and in medias res mapped onto a classic work.
  - Contrast conflict-driven three-act arcs against conflict-free four-part *kishōtenketsu* and symmetric ring compositions.
- `literature/oulipo/constraints`: N+7, snowball, univocalism, and sestina permutation tables with a worked specimen.
  - Show the explicit generative rule (e.g., spiral sestina permutation $6,1,5,2,4,3$) alongside the resulting text.

### math
- `math/geometry/constructions`: compass-and-straightedge constructions as step lists.
  - Span elementary bisectors/perpendiculars, Apollonian tangencies, regular polygons (pentagon, 17-gon), and Mohr-Mascheroni compass-only constructions.
- `math/geometry/polyhedra`: vertices, edges, faces, symmetry group, and dual.
  - Cover Platonic, Archimedean, Catalan dual, Kepler-Poinsot star, and Johnson solids.
- `math/geometry/tilings`: vertex configurations such as 3.4.6.4 with names.
  - Contrast regular and semiregular Euclidean tilings, $k$-uniform tilings, hyperbolic `{7,3}` tilings, and aperiodic Penrose/monotile tilings.
- `math/geometry/triangle-centers` *(gen)*: a triangle with its centroid, circumcenter, and Euler line.
  - Mix acute triangles, obtuse triangles where the circumcenter and orthocenter lie outside the triangle, right-triangle degeneracies, and Heronian integer triangles.
- `math/combinatorics/partitions` *(gen)*: an integer's partitions with a Young diagram.
  - Highlight self-conjugate partitions, distinct-parts vs. odd-parts bijections (Euler's theorem), Durfee squares, and hook lengths.
- `math/combinatorics/sequences`: OEIS-style entries with first terms and a definition.
  - Balance well-known combinatorial sequences (Catalan, Motzkin, Bell) with number-theoretic, cellular-automaton, and self-referential sequences.
- `math/combinatorics/knots`: knots with crossing number, notation, and invariants.
  - Span alternating prime knots, non-alternating knots, torus knots, and chiral vs. amphichiral knots with Dowker notation and Alexander/Jones polynomials.
- `math/recreational/magic-squares` *(gen)*: magic squares built by different methods.
  - Vary across odd Siamese steps, doubly-even ($4k$) diagonal swaps, singly-even ($4k+2$) LUX constructions, and pandiagonal squares.
- `math/recreational/problems`: classic problems from Dudeney or Loyd.
  - Span geometric dissections, balance-scale weighings, river crossings, and rate/clock riddles.
- `math/statistics/datasets` *(gen)*: small datasets with summary statistics and a fitted regression.
  - Mix well-behaved linear samples with Anscombe-like datasets (curved residuals, single high-leverage outlier, clustered points) sharing identical summary statistics.
- `math/statistics/paradoxes`: Simpson's paradox and Berkson's bias with real data.
  - Cover Simpson's trend reversal across subgroups, Berkson's selection bias, Will Rogers phenomenon, and the inspection/waiting-time paradox.
- `math/calculus/odes` *(gen)*: an ODE with initial conditions and its closed form.
  - Contrast overdamped, critically damped ($t e^{-\lambda t}$), underdamped oscillatory, and resonant driven regimes alongside separable nonlinear ODEs.
- `math/fractals`: fractals with dimension and construction rule.
  - Span L-system curves, iterated function systems, Julia/Mandelbrot boundaries, and space-filling curves (Hilbert, Peano).
- `math/history/problems`: Rhind Papyrus and Nine Chapters problems.
  - Span Egyptian unit-fraction grain problems, Babylonian Plimpton/quadratic tablets, Han *Jiuzhang Suanshu* linear systems, and Sangaku temple geometry tablets.
- `math/abstract-algebra/cayley-tables` *(gen)*: a small group (D4, Q8, Z2×Z4) with its Cayley table, conjugacy classes, and lattice of subgroups.
  - Contrast cyclic vs. non-cyclic abelian groups ($Z_4$ vs. $V_4$) and non-abelian dihedral, quaternion ($Q_8$), and alternating ($A_4$) groups.
- `math/algebra/galois-fields` *(gen)*: arithmetic tables and primitive polynomials in GF(2^n) or GF(p^2).
  - Vary extension degrees and irreducible polynomials, highlighting primitive vs. non-primitive generators.
- `math/geometry/wallpaper-groups`: the 17 planar crystallographic groups (p1, p4m, p6m, cmm) with orbifold signature, generators, and fundamental domain.
  - Contrast groups with pure rotations ($p3, p4, p6$), glide reflections without mirrors ($pg, pgg$), and dihedral mirror lattices ($p4m, p6m$).
- `math/combinatorics/latin-squares` *(gen)*: mutually orthogonal Latin squares (Graeco-Latin squares) for experimental block designs.
  - Vary prime-power field constructions and cyclic vs. non-cyclic Latin squares.
- `math/topology/braids` *(gen)*: a braid word in Artin generators reduced to handle form and closed into a link with component count.
  - Mix pure braids, single-component knot closures, multi-component links, and non-trivial words that reduce to the unbraid.

### mechanisms
- `mechanisms/horology/complications`: tourbillon, perpetual calendar, and minute repeater with how each works.
  - Cover escapements (anchor, detent, coaxial, tourbillon), striking racks/snails, equation-of-time kidneys, and leap-year program wheels.
- `mechanisms/locks`: pin tumbler, warded, and lever locks with the mechanism.
  - Span warded locks, Chubb detector levers, pin tumblers with security spool/serrated pins, and rotating disc detainers.
- `mechanisms/automata/historical`: Jaquet-Droz and Vaucanson automata with what they did.
  - Span Hellenistic pneumatic temple devices (Hero of Alexandria), Islamic water-clock automata (Al-Jazari), European cam-stack writers, and Japanese *karakuri ningyō*.
- `mechanisms/gears/trains` *(gen)*: a compound gear train with tooth counts that produce a target ratio.
  - Vary across simple idlers, revertive coaxial compound trains, planetary epicyclic ratios (sun/ring/carrier locked), and hunting-tooth prime pairs.
- `mechanisms/engines/heat`: Stirling, Newcomen, and Wankel with operating cycles.
  - Contrast atmospheric condensing steam (Newcomen), Watt separate condenser, Stirling regenerators, hit-and-miss governed engines, and rotary Wankel geometry.
- `mechanisms/cams/profiles` *(gen)*: a cam follower motion schedule (dwell-rise-dwell-return) with cycloidal or harmonic displacement, velocity, and pressure-angle bounds.
  - Contrast uniform velocity, simple harmonic, cycloidal (zero jerk at dwell transitions), and polynomial motion laws across roller and flat-faced followers.
- `mechanisms/pneumatics-and-hydraulics/valves`: spool valves, shuttle valves, and fluidic logic gates (Coanda bistable flip-flops) with port states.
  - Cover 3/2 and 5/2 directional spools, shuttle OR / two-pressure AND valves, pilot-operated checks, and no-moving-part Coanda fluidic bistables.
- `mechanisms/compliant/flexures`: living hinges, cross-strip pivots, and bistable buckled-beam mechanisms with degrees of freedom.
  - Contrast notch hinges, parallel leaf-spring stages, cross-strip flexural pivots, and snap-through bistable arches.
- `mechanisms/joinery/japanese`: kanawa-tsugi, shihou-kamo-tsugi, and ari-tsugi joints with interlocking axes and draw-pin directions.
  - Span longitudinal scarf splices (*tsugite*), right-angle mortise/dovetail corners (*shiguchi*), and blind wedged tenons.

### music
- `music/notation/abc` *(gen)*: short tunes in ABC notation with modal or idiomatic constraints.
  - Vary modes (Dorian, Mixolydian, Ionian, Aeolian) and meters (6/8 jig, 4/4 reel, 3/4 waltz, 9/8 slip jig, 5/4 asymmetric).
- `music/forms`: sonata, rondo, and passacaglia with the section layout of an exemplar.
  - Span binary/rounded binary dances, sonata-allegro, 5- and 7-part rondos, ground-bass passacaglias, and fugal expositions.
- `music/folk/tunes`: traditional tunes in ABC from public collections.
  - Draw from O'Neill's Irish dance tunes, Playford's *English Dancing Master*, Scottish strathspeys, and Sacred Harp shape-note hymns.
- `music/tuning/temperaments`: historical temperaments with cents tables.
  - Contrast 5-limit just intonation, Pythagorean tuning, quarter-comma meantone (with wolf fifth), Werckmeister/Kirnberger well-temperaments, and 19-TET/31-TET.
- `music/rhythm/tala`: Indian talas with beat structure and bols.
  - Span symmetric (Teental 4+4+4+4) and asymmetric (Jhaptal 2+3+2+3, Rupak 3+2+2) cycles with *tali*, *khali*, and *theka* bols.
- `music/rhythm/clave` *(gen)*: Euclidean rhythms with their traditional names.
  - Sample across coprime and non-coprime $E(k, n)$ necklaces (3:8 tresillo, 5:8 cinquillo, 5:12, 5:16, 7:12, 7:16) and their cyclic rotations.
- `music/lyrics/hymns`: public-domain hymn texts with meter, such as 8.6.8.6 CM.
  - Span Common Meter (8.6.8.6), Long Meter (8.8.8.8), Short Meter (6.6.8.6), and Hallelujah Meter (6.6.6.6.8.8).
- `music/orchestration/ranges`: instrument ranges and sweet spots.
  - Contrast low/middle/high register timbral shifts, woodwind break crossings, brass pedal tones, and string harmonics.
- `music/counterpoint/species` *(gen)*: a short cantus firmus paired with a valid first-, second-, or fourth-species counterpoint line and interval figures.
  - Vary between 1st species (contrary/oblique motion), 2nd species (passing tones), and 4th species (7–6 and 4–3 tied suspensions above, 2–3 below).
- `music/harmony/neo-riemannian` *(gen)*: a triad walked through P, L, and R parsimonious voice-leading operations on the Tonnetz.
  - Contrast hexatonic `PLPLPL` cycles, octatonic `PRPRPR` cycles, and `LPR` loops around a shared vertex.
- `music/synthesis/patches` *(gen)*: an FM operator algorithm or subtractive signal chain with carrier:modulator ratios, modulation indices, and envelope stages.
  - Vary between harmonic integer $C:M$ ratios, inharmonic bell/metallic ratios ($1:1.414$, $1:3.14$), stacked 4-op cascades, and resonant filter sweeps.
- `music/bells/change-ringing` *(gen)*: Plain Bob, Grandsire, or Stedman method hunting courses on 5 to 8 bells.
  - Vary across plain courses, bob/single calls at lead ends, and even-bell vs. odd-bell methods.

### natural-world
- `natural-world/fungi`: species with spore print, habitat, and edibility.
  - Mix everyday edible mushrooms (chanterelles, morels, porcini), wood-decay brackets, mycorrhizal agarics, and toxic amanitas.
- `natural-world/birds/calls`: Peterson-style mnemonics for bird songs.
  - Span warblers, sparrows, owls, shorebirds, and thrushes, pairing phonetic phrases with cadence and habitat.
- `natural-world/ecology/food-webs` *(gen)*: a small consistent food web for a biome.
  - Vary biomes (kelp forest, desert scrub, boreal bog, hydrothermal vent, prairie stream) and include detrital loops or keystone predators.
- `natural-world/ecology/succession`: stages for a habitat.
  - Contrast primary succession (bare lava, glacial till, coastal dune) with secondary succession (old-field, post-fire forest, whale fall).
- `natural-world/astronomy/constellations`: constellations with their brightest stars and myth.
  - Mix zodiacal, northern circumpolar, and southern sky constellations alongside non-Western lunar mansions or asterisms.
- `natural-world/astronomy/sky-tonight` *(gen)*: for a date and latitude, the moon phase and visible planets.
  - Vary across equatorial, mid-latitude, and polar latitudes (including midnight sun and polar night) and lunar phases.
- `natural-world/astronomy/deep-sky`: Messier objects with type, distance, and magnitude.
  - Span open clusters, globular clusters, planetary nebulae, supernova remnants, and spiral/elliptical galaxies.
- `natural-world/microbes`: organisms with morphology and niche.
  - Mix ordinary soil, gut, and fermentation microbes (*Lactobacillus*, *Saccharomyces*, *Rhizobium*) with extremophiles and marine phytoplankton.
- `natural-world/phenology`: seasonal events with latitude and date ranges.
  - Cover first frost, ice-out, budburst, insect emergence, spawning runs, and bird migration dates across latitudes.
- `natural-world/naturalists/journals`: passages from Thoreau, White's Selborne, and Audubon.
  - Mix quiet ordinary weather and garden notes with detailed behavioral observations of birds, insects, and plants.
- `natural-world/genetics/crosses` *(gen)*: Punnett squares for dihybrid or sex-linked traits with ratios.
  - Vary across Mendelian dominant/recessive, incomplete dominance, codominance (ABO blood types), epistasis ($9:3:4$, $9:7$), and X-linked traits.
- `natural-world/ecology/interactions`: specific documented mutualisms, parasitisms, and mimicry rings with behavior and ecological triggers.
  - Cover pollination syndromes, ant-plant myrmecophytes, cleaner-station symbioses, brood parasitism, and Batesian/Müllerian mimicry.

### objects
- `objects/tools/hand-tools`: tools with use, and regional names where they exist.
  - Mix common bench tools (planes, chisels, saws) with specialized coopering, wheelwright, masonry, leatherworking, and gardening tools.
- `objects/instruments/scientific`: astrolabe, sextant, and orrery with how each is used.
  - Span surveying, celestial navigation, weights-and-measures, optical, and electrical demonstration instruments.
- `objects/everyday/patents`: everyday objects and their first patents.
  - Cover ordinary fasteners, kitchen tools, stationery, closures, and containers.
- `objects/toys/historical`: toys with era and mechanism.
  - Mix simple folk and street toys (tops, hoops, knucklebones, cup-and-ball) with optical toys (zoetrope, thaumatrope) and tinplate wind-ups.
- `objects/clothing/historical-garments`: doublet, chiton, and hanbok with era and construction.
  - Contrast draped uncut lengths, rectangular loom-width garments, and curved tailored/boned constructions across regions and classes.
- `objects/cars/europe`: Italian, French, and British cars.
  - Mix ordinary post-war economy cars (2CV, Fiat 500, Mini, Morris Minor) with grand tourers, rally homologations, and pre-war coachbuilt models.
- `objects/aircraft`: types with role, first flight, and notable trait.
  - Span bush planes, gliders, flying boats, utility helicopters, trainers, airliners, and experimental aircraft.
- `objects/stamps`: stamps with country, year, and design.
  - Mix ordinary definitive postage issues and parcel/postage-due stamps with classic 19th-century engravings, airmails, and overprints.
- `objects/repair/visible-mends`: documented repairs (kintsugi, darning, Dutchman patches, riveted boiler plates) with material, join, and surviving part.
  - Cover ceramic staple/rivet and lacquer repairs, textile *boro*/darning, timber butterfly keys and scarf patches, and sheet-metal patches.

### people
- `people/society/inventors`: underrecognized inventors.
  - Span workshop mechanics, agricultural innovators, textile/printing inventors, and materials/chemical pioneers across regions.
- `people/society/rulers`: rulers with reign and one notable act.
  - Mix well-known monarchs with regional sovereigns, city-state doges/archons, and short-reigned reformers across eras.
- `people/society/activists`: activists outside Europe and North America.
  - Span labor, agrarian, anti-colonial, educational, and environmental organizers across Asia, Africa, Latin America, and Oceania.
- `people/arts/architecture`: architects.
  - Span ancient and medieval master masons, Renaissance treatise writers, vernacular builders, and modern architects worldwide.
- `people/arts/craft`: potters, weavers, and smiths.
  - Cover named and workshop artisans across ceramics, textiles, metalwork, furniture, typefounding, and lutherie.
- `people/sport/athletes`: historical athletes.
  - Span ancient Olympians, 19th-century pedestrians/rowers/pugilists, and 20th-century pioneers across global sports.
- `people/medicine/physicians`: physicians.
  - Span antiquity, medieval Islamic and East Asian medicine, early modern anatomists, epidemiologists, and country doctors.
- `people/everyday/diarists`: ordinary diarists such as Pepys, Ruth Hayes, and Anne Lister.
  - Mix clerks, farmers, midwives, shopkeepers, sailors, and country parsons recording weather, meals, accounts, and neighborhood life.

### procedures
- `procedures/craft/woodworking/joints`: joints with cutting steps.
  - Span simple butt, rabbet, and dado joints alongside mortise-and-tenon, dovetail, bridle, and scarf joints.
- `procedures/craft/bookbinding`: binding methods with steps.
  - Contrast pamphlet stitch, Japanese stab binding, Coptic link stitch, limp vellum, and rounded-and-backed case binding.
- `procedures/craft/ceramics/glazes`: cone-rated glaze recipes in percentages.
  - Span earthenware (Cone 04), mid-fire stoneware (Cone 6), and high-fire reduction (Cone 10 celadon, tenmoku, shino, ash) recipes.
- `procedures/craft/dyeing`: natural dye recipes with mordants.
  - Contrast substantives, alum/tartar mordanted reds and yellows (madder, weld, cochineal), iron/copper modifiers, and indigo reduction vats.
- `procedures/craft/origami` *(gen)*: crease patterns for simple bases, or written folding sequences.
  - Cover kite, fish, bird, frog, waterbomb, and windmill bases alongside simple traditional folds.
- `procedures/first-aid`: historical and modern first-aid steps, clearly sourced.
  - Contrast 19th-century Esmarch/St. John manuals with modern field protocols across splinting, bandaging, and triage.
- `procedures/gardening/companion`: companion-planting pairs with the reason.
  - Cover nitrogen fixation, trap cropping, structural trellising, shade nurse crops, and pest confusion.
- `procedures/household/historical`: tasks from Mrs Beeton.
  - Mix ordinary kitchen, laundry, lamp-trimming, and pantry preservation routines from 19th-century domestic manuals.
- `procedures/algorithms/sorting/traces` *(gen)*: sort traces for quicksort, merge, and radix with intermediate arrays.
  - Mix random permutations with nearly sorted, reverse-sorted, and Dutch-National-Flag many-duplicate inputs.
- `procedures/algorithms/strings` *(gen)*: KMP failure tables, edit-distance matrices, and Huffman codes.
  - Include periodic/self-overlapping pattern strings for KMP, transposition/indel alignments for edit distance, and skewed vs. near-uniform symbol counts for Huffman trees.
- `procedures/algorithms/numerical` *(gen)*: Newton's method and Simpson's rule iteration tables.
  - Contrast fast quadratic convergence with multiple-root slow convergence, inflection overshoot, and oscillatory integrals.
- `procedures/experiments/chemistry`: classic demonstrations with materials and observations.
  - Span gas generation, crystallization, acid-base indicators, clock reactions, and electroplating from period laboratory manuals.
- `procedures/experiments/electricity`: Faraday's or Franklin's experiments in their own words.
  - Span static charge and Leyden jars, voltaic piles, Oersted's compass deflection, and Faraday's induction coils.
- `procedures/navigation/celestial` *(gen)*: a sextant sight reduction (body, index error, dip, refraction, GHA/Dec, assumed position, and Sumner line intercept).
  - Mix noon meridian sights for latitude with morning/evening star fixes, sun-run-sun advances, and high- vs. low-altitude refraction regimes.
- `procedures/metrology/vernier-and-micrometer` *(gen)*: a main-scale and vernier or thimble coincidence reading with least count and total measurement.
  - Vary across metric ($0.02\text{ mm}, 0.05\text{ mm}$) and inch ($0.001\text{ in}$, fractional $1/128\text{ in}$) verniers, micrometers, and zero-error offsets.
- `procedures/weaving/drafts` *(gen)*: a 4-shaft or 8-shaft loom threading, tie-up, and treadling matrix with the resulting ASCII drawdown.
  - Vary across plain weave, straight/point/broken twills, basketweave, huck lace, and overshot patterns.
- `procedures/glassblowing/steps`: marvering, punty transfer, jacking, and annealing schedules from studio glass manuals.
  - Cover gathering, blocking, marvering, optic molding, punty transfer, lip opening, and annealing soak/cooldown stages.
- `procedures/printing/letterpress`: composing-stick justification, furniture lockup in a chase, and platen makeready steps.
  - Cover hand-setting spaces in the stick, leading, imposing in the chase with quoins, and tympan/packing makeready.
- `procedures/alchemy-and-assay/cupellation`: Agricola's De Re Metallica or Biringuccio's Pirotechnia fire-assaying and smelting procedures.
  - Cover ore roasting, crucible fluxing, bone-ash cupellation of silver/gold, and nitric-acid parting.

### random
- `random/names/personal` *(gen)*: culturally coherent full names from real name-frequency data.
  - Span diverse global naming structures (given+surname, double surnames, patronymics, mononyms) and balance common everyday names with rarer regional names.
- `random/coordinates` *(gen)*: a land-weighted point with its nearest named place.
  - Mix populated agricultural valleys and towns with remote desert, tundra, island, and mountain coordinates.
- `random/identifiers` *(gen)*: valid ISBNs, IBANs, and Luhn-valid numbers with check digit shown.
  - Vary across ISBN-10 (mod 11 with `X` check digit), ISBN-13, ISSN, ISMN, IBAN (mod 97), and Luhn check strings.
- `random/passwords/diceware` *(gen)*: a diceware passphrase with the dice rolls.
  - Vary word counts and wordlists (standard 5-dice 7776-word list vs. compact 4-dice 1296-word list).
- `random/times/schedules` *(gen)*: a week calendar with non-overlapping events.
  - Vary density and structure: sparse weeks, fragmented days, back-to-back blocks, and multi-timezone or shift-work schedules.
- `random/geometry/shapes` *(gen)*: random simple polygons as SVG paths with area and perimeter.
  - Contrast convex polygons, star-shaped polygons, rectilinear orthogonal polygons, and deep-bay non-convex polygons.
- `random/sounds/frequencies` *(gen)*: a chord or frequency set given in Hz, cents, and MIDI.
  - Mix standard 12-TET triads/voicings with harmonic-series overtone clusters, sub-bass intervals, and microtonal dyads.
- `random/graphs/small-topologies` *(gen)*: an unlabeled connected graph on 5–7 vertices from the existing atlas, with adjacency list and selected invariants. Eight-vertex coverage needs a separate source or generation method.
  - Sample uniformly across non-isomorphic graph atlas classes (trees, unicyclic graphs, bipartite graphs, planar cages, and dense cliques).
- `random/orders/posets` *(gen)*: a finite partially ordered set given by covering relations, its linear extensions count, and its antichains.
  - Contrast chains, flat antichains, Boolean lattices, series-parallel posets, and N-free vs. crown posets.
- `random/permutations/cycles` *(gen)*: an element of S_n in one-line, cycle, and inversion-table notation with sign and order.
  - Mix single-cycle derangements, involutions (order 2), high-order elements (Landau's function), and identity-adjacent transpositions.
- `random/distributions/contingency` *(gen)*: a 2x2 or 3x3 integer contingency table with fixed marginals and its exact Fisher/chi-square statistic.
  - Mix independent proportional tables with zero-cell boundary tables, strong diagonal associations, and small-count Fisher exact cases.
- `random/signals/waveforms` *(gen)*: a short discrete signal (8 or 16 samples) with its exact Walsh-Hadamard or Haar wavelet transform coefficients.
  - Contrast step edges, single impulses, alternating Nyquist oscillations, ramp trends, and sparse Haar-basis signals.
