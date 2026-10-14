# Diagonalizable matrices

Original bounded integer examples of `A=S diag(l,m) S^-1`, diagonalizable over
`Q`. The columns of `S=[[1,t],[u,1+tu]]` are the emitted eigenvectors;
`det(S)=1`, so its inverse is also integral. Parameters `t,u` lie in `-3..=3`,
and distinct ordered eigenvalues `l,m` lie in `-6..=8`. Both eigenvalues are
sampled uniformly with the second excluding the first. This construction keeps
every entry of `S` and `S^-1` within absolute value 10 and every entry of `A`
within 420. The displayed square uses at most two products of entries bounded
by 420, safely within `i64`.

The power formula applies to positive integer exponents, including singular
matrices with a zero eigenvalue. `A^0=I` is stated separately. Parenthesized
bases distinguish negative eigenvalues from unary minus outside a power.

Checked against [MIT 18.06SC: Diagonalization and Powers of A, lecture summary](https://ocw.mit.edu/courses/18-06sc-linear-algebra-fall-2011/d05795355f5cbe35356a4a1b39658336_MIT18_06SCF11_Ses2.9sum.pdf).
The source explains the independent eigenvector criterion and similarity
formula; examples and wording are generated here, without copied course text.

Tests cover exact eigenpairs, trace, determinant, inverse, and Cayley-Hamilton
on 2,000 seeded samples; bounds and powers 1 through 6 on all 10,290 parameter
tuples; and emitted field, negative-base, zero-eigenvalue, and exponent-domain
notation. Isolated `rustc --test`, Clippy with warnings denied, and Rust 2024
formatting checks verify this leaf.
