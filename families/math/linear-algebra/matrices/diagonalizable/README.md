# Diagonalizable matrices

Bounded integer examples of `A=S diag(l,m) S^-1`, with
`S=[[1,t],[u,1+tu]]`. Its determinant is 1, so the inverse is integral.
Parameters `t,u` lie in -3..=3; distinct eigenvalues `l,m` lie in -6..=8.
Basis and inverse entries have absolute value at most 10; entries of A at most
420. Powers up to 6 fit comfortably in i64.

Each output contains A and its eigenpairs, a computed power A^k
(k=2..6), or its eigenbasis decomposition. No general identities or property
lists are appended.

Tests cover exact eigenpairs, trace, determinant, inverse and Cayley–Hamilton
on 2,000 seeded constructions, plus bounds and powers 1–6 on all 10,290
parameter combinations.

Reference: [MIT 18.06SC, Diagonalization and Powers of A](https://ocw.mit.edu/courses/18-06sc-linear-algebra-fall-2011/d05795355f5cbe35356a4a1b39658336_MIT18_06SCF11_Ses2.9sum.pdf).
