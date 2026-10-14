# Finite Heisenberg groups

Original procedural descriptions of the upper unitriangular 3×3 matrix groups
over the prime fields of orders 2, 3, 5, 7, 11, and 13. The prime and each matrix
coordinate are sampled uniformly. Outputs include explicit multiplication,
inverse, element order, an oriented commutator, center, quotient, exponent, and
a noncommuting pair. Characteristic two is handled separately: some elements
have order four.

The coordinate formulas are derived from matrix multiplication; no source data
or prose is imported. Background reference: J. S. Milne, *Group Theory*, v4.01,
[upper unitriangular example, printed p. 93](https://www.jmilne.org/math/CourseNotes/GT.pdf).
Tests exhaust the group axioms and independently cross-check matrix
multiplication for p=2,3. Every supported element is checked for its inverse,
order, and center membership; commutator formulas are checked against their
defining products, and every central element is exhibited as a commutator.
Fixed-seed samples check reproducibility and coverage of all six prime fields.
