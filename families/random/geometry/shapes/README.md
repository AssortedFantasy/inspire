# Polygon paths

Five equally weighted constructions: the convex hull of bounded integer points,
an eight- or twelve-ray star-shaped polygon, an orthogonal histogram with two
to six bars, a deep rectangular bay, and a triangle. Triangles mix ordinary
proportions with thin cases, with base-to-height ratio at most six. Stars multiply
outer ray directions by 12–18 and inner directions by 4–5.
Half the bays leave a one-unit neck. Every construction is bounded, with
no rejection loop. Rotations and reflections preserve integer coordinates;
the result is translated to nonnegative coordinates and oriented counterclockwise
in Cartesian coordinates. SVG's usual downward y-axis reverses that visual sense.

Outputs are an SVG path element, exact integer/half-integer area, perimeter rounded
to two decimal places, and the number of reflex vertices for nontriangles. The
path contains one simple closed boundary and no holes. It has no stroke, fill,
unit, or viewport assumption; callers can insert the element into their own SVG.
This leaf focuses
on arbitrary polygon paths rather than barycentric triangle geometry already in
`math/linear-algebra/convex-geometry`.

References: [CGAL polygon operations and simplicity](https://doc.cgal.org/Manual/3.1/doc_html/cgal_manual/Polygon/Chapter_main.html)
and [signed area](https://doc.cgal.org/Manual/3.2/doc_pdf/cgal_manual.pdf).
Tests check 10,000 constructions for all nonadjacent edge intersections, repeated
and flat vertices, positive orientation, and area agreement with independent
horizontal slab integration. Triangle and star tests bound acute tips below by
atan(1/6), while retaining thin triangles and concave stars. Hull tests check
interior and collinear removal.
