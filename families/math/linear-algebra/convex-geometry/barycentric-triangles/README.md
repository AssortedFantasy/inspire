# Barycentric triangles

Original bounded examples of an integer triangle and a rational interior point.
The anchor coordinates range from -6 to 6. Edge vectors are `(a, s*a)` and
`(b, s*b+c)`, where `a,c` range from 2 to 12, `b` from -8 to 8, and the shear
`s` from -3 to 3. Their determinant is `a*c > 0`, so every triangle is
counterclockwise and has area from 2 to 72. Vertex coordinates have absolute
value at most 42.

Three integer weights range from 1 to 9; their total ranges from 3 to 27.
Normalizing them gives strictly positive barycentric coordinates summing to 1.
Coordinates and opposite subtriangle areas are emitted as reduced exact
fractions, with integers printed without a denominator. This discrete sampling
does not claim to sample points uniformly by triangle area.

The geometric relations were checked against Alec Jacobson's
[Barycentric coordinates and point-triangle queries](https://www.cs.toronto.edu/~jacobson/weblog/1596.html)
(2011): positive barycentric coordinates characterize the interior, and each
coordinate is the area of its opposite subtriangle divided by the whole area.
No source text or dataset is embedded.
