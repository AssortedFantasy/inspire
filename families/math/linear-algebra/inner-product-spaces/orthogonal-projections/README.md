# Orthogonal projections

Exact orthogonal projections onto a line or its perpendicular plane in R^3,
selected with equal probability. Each output contains the target subspace, the
exact decomposition of an integer vector into its projection and orthogonal
residual. The matrix stays internal rather than repeating the same map in
another representation.

Direction coordinates start in [-6,6]; an all-zero draw gets a nonzero pivot.
The direction is reduced to a primitive integer vector with positive first
nonzero coordinate. Example coordinates lie in [-8,8]. The denominator u dot u
is at most 108. The line projection is `uu^T/(u^Tu)` and the plane projection
is its complement `I-P`. All fractions are reduced, including integer results.
All arithmetic fits in i64.

Tests cover exact complementary projections, ranks, symmetry, idempotence,
residual orthogonality, Pythagoras, the zero-direction fallback, an independent
kernel basis, and every numerator in [-1728,1728] with denominators in [1,108].

Reference: [MIT 18.06, Spring 2022 problem set 5 solutions, problems 4–6](https://web.mit.edu/~linalg/webpage/Spring2022/pset5sol.html).
