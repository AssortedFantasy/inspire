# Orthogonal projections

Generated rank-one orthogonal projections in R^3 with exact reduced fractions,
a projected example vector, and an explicit two-vector kernel basis. The
direction has coordinates in [-6,6]; an all-zero draw is replaced by a positive
coordinate in [1,6]. Example coordinates lie in [-8,8]. There are 2,196 nonzero
direction vectors and 4,913 example vectors; proportional directions can
describe the same line.

The denominator u dot u lies in [1,108]. Matrix numerators have absolute value
at most 36; example numerators at most 864. Kernel coordinates have absolute
value at most 6. All calculations fit comfortably in i64.

Primary reference checked: [MIT 18.06, Spring 2022 problem set 5 solutions,
problems 4–6](https://web.mit.edu/~linalg/webpage/Spring2022/pset5sol.html).
The generator applies P=uu^T/(u^Tu) and constructs a basis of the perpendicular
plane using a nonzero pivot. Outputs are original generated examples.

Tests check 2,000 internal constructions and independently parse 2,000 emitted
matrices, vectors, and kernel certificates. They verify exact symmetry,
idempotence, rank one, trace, the fixed direction, and perpendicular residuals.
Separate tests cover the zero-direction fallback and every numerator in
[-864,864] with every denominator in [1,108], including signs, zero, and reduction.
