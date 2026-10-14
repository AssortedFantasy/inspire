# Groups

`concepts` contains 40 concise group-theory facts and counterexamples, retaining
the original eleven records. The
`finite-groups` branch contains separate procedural constructions, so a CLI
selection can target one kind of group instead of drawing from a mixed generator.
Each construction specifies its underlying set, operation, identity, order, and
a computed example. Group laws and structural claims have invariant tests.

The former `math/algebra/groups/finite-groups` generator is now a branch; selecting
that path still samples its whole collection. Its children are `cyclic`,
`direct-products`, `dihedral`, `quaternion`, `symmetric`, `affine`, and `heisenberg`.
The `concepts` path and original record IDs remain available. Equal weights give
concepts and finite constructions equal shares within `groups`; the seven
finite-construction topics also have equal shares, as in the former generator.

The mathematics is original explanatory prose and independently implemented
standard constructions, checked against J. S. Milne's
[Group Theory, version 4.01](https://www.jmilne.org/math/CourseNotes/GT.pdf)
(November 2025; consulted 2026-10-13). The notes are licensed CC BY-NC-SA 4.0;
their prose, exercises, and tables are not copied into the payloads. This branch
uses mathematical constructions and curated facts, rather than claiming to
import a classification dataset. Runtime remains offline.
