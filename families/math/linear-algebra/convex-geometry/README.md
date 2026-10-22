# Convex geometry

`concepts` contains 16 formulas and concrete sets covering convex hulls, affine
maps, cones and hyperplanes, with individual source provenance.

`barycentric-triangles` generates a counterclockwise integer triangle and an
exact affine combination. Half the points have three positive weights; a
quarter lie on an edge, and a quarter have one negative coordinate and lie
outside. Coordinates and coefficients are reduced fractions. Zero terms are
omitted from the affine combination.

Tests independently recover the signed coordinates using subtriangle
determinants across 2,000 samples. See the leaf README for bounds and sampling.
