# Barycentric triangles

Integer triangles and exact affine combinations. Anchor coordinates range
from -6 to 6. Edge vectors are `(a,s*a)` and `(b,s*b+c)`, where a,c range
from 2 to 12, b from -8 to 8, and s from -3 to 3. The determinant is a*c>0;
area ranges from 2 to 72 and vertex coordinates have absolute value at most 42.

Three initial integer weights range from 1 to 9. Half the draws keep all three
positive. A quarter replace one with zero, placing P on an open edge. A quarter
negate one weight and add 9 to a different weight; their total stays positive,
placing P outside the triangle. Dividing by the total gives exact coordinates
summing to 1. Terms with zero coefficients are omitted. The sampling is discrete.

Tests independently recover all three signed weights from subtriangle
determinants over 2,000 samples, covering interior, edge and exterior points.

Reference: [Alec Jacobson, Barycentric coordinates and point-triangle queries](https://www.cs.toronto.edu/~jacobson/weblog/1596.html).
