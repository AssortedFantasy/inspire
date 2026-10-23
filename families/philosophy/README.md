# Philosophy

`logic` contains three equally weighted generators. The family has local weight
2. Outputs are generated formal artifacts, with no attributed quotations or
synthetic reconstructions of historical arguments.

## Categorical syllogisms

`logic/syllogisms` covers every A/E/I/O mood in each of the four standard
figures: 256 forms. Major premise, minor premise and conclusion appear in that
order. S is the conclusion's subject, P its predicate, and M the middle term.
Figures use MP/SM, PM/SM, MP/MS, and PM/MS respectively.

The domain is nonempty. A(X,Y) means X is a subset of Y; E means disjointness;
I means nonempty intersection; O means nonempty set difference X minus Y. Universal
sentences have no existential import. Sampling first chooses equally among
the 15 forms valid with empty extensions allowed, the nine additional forms
valid when S, P and M are all nonempty, and the other 232 forms. Forms are uniform
within each class, so the distribution deliberately favors validity boundaries.

Validity is decided over all 255 nonempty occupancy patterns of the eight
S/P/M membership regions. One representative per occupied region is complete
for these sentences, regardless of domain size. Conditional forms print every
minimal sufficient nonemptiness assumption; alternatives joined by “or” are
separate sufficient assumptions. Invalid cases print a smallest countermodel
by domain cardinality. For the final class the countermodel keeps all three
term extensions nonempty. Tests compare every form and every subset of
nonemptiness assumptions against independent individual-based quantification
over every interpretation on domains of one, two and three objects.

The three figures in Aristotle's [Prior Analytics, Book I, chapters 4–6](https://classics.mit.edu/Aristotle/prior.1.i.html)
provide the historical starting point; this generator also includes the
standard fourth figure. Philip Stark's author-maintained
[Categorical Logic](https://www.stat.berkeley.edu/~stark/SticiGui/Text/categoricalLogic.htm)
gives the set interpretation and the distinction between unconditional and
existentially dependent forms. “All terms nonempty” is the explicit modern
condition here, rather than an unspecified historical interpretation.

## Propositional tables

`logic/truth-tables` uses classical two-valued semantics with negation,
conjunction, disjunction, material implication, biconditional and exclusive or.
Every displayed variable receives a column and every valuation receives a row.
The generator chooses equally among tautologies, contradictions, formulas with
one satisfying valuation, formulas with one falsifying valuation, and arbitrary
implication/equivalence/parity expressions. The final class is classified from
its computed table and can fall in any semantic class.

There are at most four variables and 16 rows. Random subexpressions have depth
at most two; identity constructions and equivalent conjunction constructions
can produce deeper final trees. Tautologies vary across excluded middle,
Peirce's law, contraposition, De Morgan, distributivity, biconditional/parity
duality and hypothetical syllogism, with sampled subexpressions. Unique-model
constructions vary literal polarities, order, association and connective
representation. Contingent outputs include a sampled satisfying valuation and
a sampled falsifying valuation. Tests compare scalar recursion against an
independent 16-bit truth-vector evaluator and check the intended regimes.

The author-maintained [forall x: Calgary](https://openlogicproject.org/wp-content/uploads/2017/10/forallxyyc.pdf),
chapters 8–12, supplies the classical truth-table semantics.

## Kripke frames

`logic/modal-kripke` has two to four worlds, a directed accessibility relation
including explicit self-loops, and arbitrary propositional valuations. Frames
mix arbitrary relations, empty relations, directed cycles, equivalence
relations, reflexive partial orders, serial irreflexive graphs, reflexive
nontransitive paths, symmetric stars, strict partial orders and nonreflexive
Euclidean successor cliques. Labels are permuted. The properties printed are
computed from the resulting graph, including coincidences between regimes.

Half the outputs evaluate a sampled modal formula in a sampled model, giving
its complete truth set and its truth value at one world. Formula construction
depth is at most three. The other half checks a modal axiom on every world and
every valuation on the frame, favoring failures when available. It prints a
countervaluation and world when invalid. K uses both p and q (at most 256
valuations); the other formulas use p (at most 16 valuations).

| Formula | Frame condition |
| --- | --- |
| K: □(p → q) → (□p → □q) | Every frame |
| D: □p → ◇p | Serial |
| T: □p → p | Reflexive |
| B: p → □◇p | Symmetric |
| 4: □p → □□p | Transitive |
| 5: ◇p → □◇p | Right Euclidean |
| S4: conjunction of T and 4 | Reflexive and transitive |
| S5: conjunction of T and 5 | Reflexive and Euclidean |

Euclidean means wRv and wRu imply vRu, allowing repeated worlds. Box is
universally quantified over successors, so it is true at dead ends; diamond
is false there. A model's truth set is separate from frame validity. Frame
validity here is exact for the displayed finite relation; sampled models make
no claim about unrestricted satisfiability or validity over arbitrary frames.
The S4 and S5 checks show their characteristic axioms, not derivations in a
proof calculus. Reflexivity plus Euclideanness makes the relation an
equivalence relation.

Edward Zalta's author-hosted [Basic Concepts in Modal Logic](https://mally.stanford.edu/notes.pdf),
printed pages 31–33, gives these predicates, axiom schemata and
correspondences. Tests exhaust every directed frame with one to three worlds,
every required p/q valuation and every world using an independent recursive
world evaluator. They verify all eight displayed checks against the graph
conditions. At three worlds the serial/reflexive/symmetric/transitive/Euclidean
frame counts are respectively 343/64/64/171/39 out of 512. Further tests compare
arbitrary nested formulas and preserve dead-end vacuity.
