# Dihedral groups

Original procedural descriptions of the regular n-gon symmetry group D_n,
3 <= n <= 48, always explicitly of order 2n. Each output gives the presentation,
coordinate law, two sampled elements, both multiplication orders, inverses,
element orders, a noncommuting witness, the center, and the rotation subgroup.
The coordinates and finite bounds make all claims directly checkable.

Reference checked at development time: J. S. Milne, *Group Theory*, v4.01
(November 2025), pp. 13–14 and 35, examples 1.17 and 2.7:
https://www.jmilne.org/math/CourseNotes/GT.pdf. The reference supplies the
standard polygon interpretation and presentation. Center and element-order
formulas are derived from the implemented multiplication law. No source prose
or downloaded dataset is redistributed; outputs and implementation are original.
The reference is published under CC BY-NC-SA 4.0.

Tests exhaust group axioms for n=3..8 and verify inverses, minimal element orders,
and centers across the full n=3..48 range, plus seeded repeatability and diversity.
