# Matrices

`concepts` contains 25 canonical identities and concrete matrix examples with
source provenance.

`diagonalizable` constructs a 2-by-2 integer matrix with distinct integer
eigenvalues in -6..=8 and an integral eigenbasis made from two shears in -3..=3.
Outputs pair the matrix with eigenvectors, a computed power, or the eigenbasis
and diagonalization. Basis entries have absolute value at most 10; matrix entries
at most 420.

Tests verify sampled eigenpairs, inverses and Cayley–Hamilton, plus powers 1–6
for all 10,290 parameter tuples. See the leaf README for the construction.
