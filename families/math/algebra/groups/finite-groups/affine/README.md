# Affine groups over prime fields

Original procedural material describing AGL(1,p) for
p ∈ {3,5,7,11,13,17,19,23,29,31}. Each sample specifies the field,
composition convention, group order, identity, two concrete maps, their
product, an inverse, an element order, fixed points, and the translation–dilation
semidirect product. The noncommuting witness uses 2≠1 in every chosen field.

The formulas are derived from composing t↦at+b. A nonidentity translation
has order p. An affine map with a≠1 has a unique fixed point; shifting that
point to zero makes its order the multiplicative order of a.

Mathematical background: J. S. Milne, *Group Theory*, version 4.01,
https://www.jmilne.org/math/CourseNotes/GT.pdf (semidirect products).
No source text or online dataset is copied; finite constrained construction
provides varied, independently checkable objects without a lookup corpus.

Tests exhaustively check the group axioms and map composition for p=3,5.
For all supported primes and every element they verify both inverse laws,
minimal element order, fixed points, and composition with representative maps.
Fixed-seed outputs check reproducibility and coverage of all ten prime fields.
