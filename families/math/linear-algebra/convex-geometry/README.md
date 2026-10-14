# Convex geometry

`concepts` contains 16 original concise explanations, examples and counterexamples
about real convex sets and their hulls. Its record provenance distinguishes
independent paraphrases from source text; the two original record IDs and texts
are preserved. Primary references actually read during the leaf review were
[Lebl, Basic Analysis, Section 8.1.4](https://www.jirka.org/ra/html/sec_vectorspaces.html)
and [Boyd and Vandenberghe, Convex Optimization lecture slides](https://web.stanford.edu/~boyd/cvxbook/bv_cvxslides.pdf),
Sections 2.2-2.10, 2.14, and 2.22-2.23. See the leaf README for source topics and
the records' provenance for individual checks. No source prose is reproduced.

`barycentric-triangles` generates a counterclockwise triangle with integer vertices,
an exact rational interior point, its barycentric coordinates, and total and
opposite subtriangle areas. The anchor coordinates lie in -6..=6. Relative edge
parameters are a,c in 2..=12 and b in -8..=8, with shear in -3..=3:
A=(x,y), B=(x+a,y+shear*a), C=(x+b,y+shear*b+c).
The signed doubled area is a*c, so the area is between 2 and 72. Three integer
weights in 1..=9 are divided by their sum (3..=27); positivity places the point
strictly inside the triangle. All fractions are reduced with positive denominators.
The largest absolute vertex coordinate is 42, and weighted arithmetic uses i64.

The generator's invariant test independently recovers all three barycentric
weights from signed subtriangle determinants across 2,000 samples. The leaf
review also inspected eight actual outputs and reduced rational formatting;
the isolated test and rustfmt check passed. The geometric relations were checked
against [Alec Jacobson, Barycentric coordinates and point-triangle queries](https://www.cs.toronto.edu/~jacobson/weblog/1596.html).
Geometry and arithmetic are original bounded constructions, with no source prose
or code reproduced. This discrete sampling does not claim uniformity by area.
`review_manifest.json` records each completed independent leaf review.
