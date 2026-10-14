# Matrices

`concepts` contains 25 definitions, examples and counterexamples. Its original ten
record IDs and texts are preserved; additional records are original paraphrases
checked against the primary course notes listed in the leaf README.

`diagonalizable` emits a 2-by-2 integer matrix diagonalizable over Q, together
with its distinct integer eigenvalues, eigenvectors, a determinant-one integer
eigenbasis and inverse, characteristic polynomial, trace, determinant and
square. Its power identity includes the separate zeroth-power case.

The eigenvalues range from -6 through 8. The two shear parameters range from -3
through 3, so the basis entries have absolute value at most 10 and the emitted
matrix entries at most 420. Runtime uses bounded integer arithmetic and only
the supplied RNG. Tests check 2,000 sampled invariant cases, all 10,290 parameter
combinations and powers 1 through 6, and displayed field and exponent notation.
The leaf README documents sources and substantive tests.

The two default-weight leaves balance concepts and constructions equally.
`review_manifest.json` records the distinct leaf reviewers and completed checks.
