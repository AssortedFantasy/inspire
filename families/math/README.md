# Mathematics

Mathematics is organized by subject, with deeper nodes distinguishing actual
constructions. Every procedural leaf emits an object and computed properties;
concept corpora state definitions, examples and counterexamples together within
a specific topic. Mixed definition/construction/counterexample collections have
been retired, while their original 64 stable record IDs are preserved.

- `algebra`: groups, rings, modules and polynomials; finite group constructions
  have individual leaves with explicit operation conventions and examples.
- `analysis`: convergence, continuity, differentiation, series and integrals.
  Definite integration is separate from five kinds of real antiderivatives.
- `linear-algebra`: vector spaces, matrices, convex geometry and inner products;
  exact eigenbases and rational orthogonal projections include witnesses.
- `topology`: metric spaces and topological spaces; finite weighted tree metrics
  and partial-order topologies include complete distance/open-set data.
- `number-theory`: continued fractions, primitive Pythagorean triples and Chinese
  remainder systems, with exact integer certificates.
- `probability`: finite mass functions and absorbing Markov chains, with complete
  distributions and exact moments, absorption probabilities and hitting times.
- `discrete-mathematics/graph-theory/atlas`: 1,249 small graph objects imported
  from the pinned BSD-licensed NetworkX 3.4.2 atlas, partitioned into forests,
  cyclic bipartite graphs and nonbipartite graphs. Records include exact graph
  invariants, and a checksum-pinned importer reproduces them.

Local weights balance subjects and construction classes; corpus sizes do not
determine their shares. Mathematical explanations are newly worded unless a
leaf explicitly states an imported origin. A source reference documents fact
checking and does not imply that source prose or its license was copied.
The graph atlas retains its full BSD notice and short emitted attribution.

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
