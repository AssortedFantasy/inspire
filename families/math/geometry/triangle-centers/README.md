# Triangle centers

Exact rational circumcenters, centroids, orthocenters and nine-point circles.
Outputs select an Euler-line, circumcircle or nine-point-circle view.

Seven equally likely constructions cover acute scalene, obtuse isosceles,
right, one integer step either side of a right angle, integer-sided Heronian,
thin obtuse and exactly collinear triples. The collinear boundary has a centroid
and no circumcircle. The four Heronian models have side lengths (13,14,15),
(13,20,21), (17,25,26) and (25,25,30). Quarter turns, reflections and translations
preserve these regimes. Integer coordinates stay within magnitude 60. No
equilateral triangles or symbolic radicals are generated.

Tests independently check equal circumradii, perpendicular altitudes, Euler's
2:1 center ratio, side midpoints on the nine-point circle and all three sides of
the right-angle boundary. The 13–14–15 case checks its integer sides and area.

Reference: H. S. M. Coxeter and S. L. Greitzer,
[*Geometry Revisited*, §§1.5–1.6](https://www.math.unipd.it/~legovini/Coxeter_Greitzer_Geometry_revisited.pdf).
The constructions and implementation are original; no source prose is embedded.
