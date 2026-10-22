# Mathematics

Mathematics is organized by subject, with deeper nodes distinguishing actual
constructions. Generators emit exact objects, equations and focused computed
results. They vary mathematical structure as well as parameters; individual
outputs show one useful view rather than a full catalogue of properties.
Concept corpora retain concise definitions, examples and counterexamples.

- `algebra`: finite groups, rings, modules and polynomials; outputs include
  permutations, subgroup and quotient calculations, CRT idempotents, tensor
  images, finite-field arithmetic and expression-tree polynomial identities.
- `analysis`: convergence, continuity, differentiation, series and integrals.
  Definite integration is separate from five kinds of real antiderivatives.
- `linear-algebra`: exact eigenbases, powers, line and plane projections, and
  interior, edge and exterior barycentric combinations.
- `topology`: arbitrary labeled weighted trees with distances, centers or balls;
  partial-order topologies with open sets or exact subset interiors/closures.
- `number-theory`: continued fractions, primitive Pythagorean triples and Chinese
  remainder systems with shared factors, exact periods and incompatibility
  witnesses.
- `probability`: finite mass functions and absorbing Markov chains, with complete
  distributions and exact moments; sparse absorbing chains vary their state
  counts and transition graphs and give absorption probabilities or hitting times.
- `discrete-mathematics/graph-theory/atlas`: 1,249 small graph objects imported
  from the pinned BSD-licensed NetworkX 3.4.2 atlas, partitioned into forests,
  cyclic bipartite graphs and nonbipartite graphs. Records include exact graph
  invariants in three compact views, and a checksum-pinned importer reproduces them.

Local weights balance subjects and construction classes; corpus sizes do not
determine their shares. Mathematical generators implement bounded exact
constructions; reference links in branch documentation explain their source
context. Source identifiers, links and notices remain in provenance and license
files. The graph atlas retains its full BSD notice.

Generators use only the supplied RNG and bounded local computation. Their tests
check mathematical content: laws, eigenpairs, idempotence, coprimality, exact
congruences, stochastic first-step equations, topology axioms and metric
properties. See each branch README for parameter bounds and references.

Path migrations: former `math/algebra` identities now live at
`math/algebra/polynomials/identities`; `math/finite-groups` moved under
`math/algebra/groups/finite-groups`; and `math/integrals` became the
`math/analysis/integrals` subtree. Former broad corpus records now live in
subject-specific corpus leaves; `concepts` children distinguish them where a
topic also has procedural constructions.
