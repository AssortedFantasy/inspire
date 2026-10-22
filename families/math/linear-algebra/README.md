# Linear algebra

Six leaves cover vector spaces, matrices, convex geometry and inner products.
The three concept corpora contain 59 records of canonical equations and concrete
examples, with individual source provenance.

- `matrices/diagonalizable`: exact integer matrices, paired with eigenvectors,
  a computed power, or an eigenbasis decomposition. Tests verify eigenpairs,
  Cayley–Hamilton and powers 1–6 for every bounded parameter combination.
- `convex-geometry/barycentric-triangles`: integer triangles and exact affine
  combinations. Points lie inside, on an edge, or outside the triangle; signed
  subtriangle determinants independently verify their coordinates.
- `inner-product-spaces/orthogonal-projections`: exact rational projections onto
  a line or its perpendicular plane in R^3, with a projected vector. Tests check
  complementary ranks, symmetry, idempotence and orthogonal decompositions.

Generator outputs contain the objects and computations directly. Each generator
uses bounded integer arithmetic and only the supplied RNG. Leaf READMEs document
sampling and mathematical references.
