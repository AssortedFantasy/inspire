# Linear algebra

Six leaves balance four topics. Three concept corpora contain 59 concise
definitions, examples and counterexamples; all 14 original record IDs and texts
are preserved. Records are independently written and checked against primary
sources, with individual provenance and source details in leaf READMEs:

- `vector-spaces`: 18 records about bases, field-dependent dimension, direct
  sums, quotients, rank-nullity, duality and infinite-dimensional distinctions.
- `matrices/concepts`: 25 records about matrix identities, spectra, rank,
  invertibility and counterexamples.
- `convex-geometry/concepts`: 16 records about convex sets, affine maps, cones,
  hulls, weak separation and supporting hyperplanes.

`matrices/diagonalizable` emits a 2-by-2 integer matrix diagonalizable over Q,
using distinct eigenvalues in -6..=8 and a determinant-one eigenbasis made from
two shears in -3..=3. It includes exact eigenvectors, inverse, characteristic
polynomial, trace, determinant and square. Positive powers use parenthesized
eigenvalue bases, and the zeroth power is stated separately. Tests verify 2,000
sampled invariant cases, every one of the 10,290 parameter combinations with
powers 1 through 6, and the emitted notation.

`convex-geometry/barycentric-triangles` emits a counterclockwise integer triangle
with area between 2 and 72, a rational interior point, positive barycentric
coordinates, and opposite subtriangle areas. Translation, edge parameters and
shear vary the geometry; positive integer weights in 1..=9 keep the point
strictly interior. Tests independently recover the coordinates from signed
subtriangle determinants across 2,000 samples. This discrete sampling does not
claim uniformity by area.

`inner-product-spaces/orthogonal-projections` emits the exact rational projection
onto a nonzero integer direction in R^3, a projected vector and an independent
kernel basis. Tests check 2,000 internal constructions and independently parse
2,000 emitted certificates for idempotence, rank, symmetry, kernel independence
and residual orthogonality. Additional checks cover the zero-direction fallback
and all 186,732 supported numerator/denominator cases.

The generators use bounded local arithmetic and the supplied RNG; runtime is
offline. Their leaf READMEs document bounds and primary references. No source
prose or code is reproduced. Default sibling weights balance the four topics;
the two-leaf matrices and convex branches split their shares equally.
`review_manifest.json` records the unique reviewer and completed checks for each
leaf.
