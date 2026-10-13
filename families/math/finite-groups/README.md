# Finite groups

Samples cyclic groups, products of two cyclic groups, dihedral groups,
quaternion groups, symmetric groups, affine groups over prime fields, and
three-dimensional finite Heisenberg groups. Every sample specifies the set,
operation, order, identity, commutativity, and a computed product. Dihedral
notation always means the symmetry group of an n-gon with order 2n.

No classification database is needed. Sizes are bounded; permutation sampling
uses a shuffle and never enumerates the whole group during generation. Exhaustive
small-instance tests check closure, associativity, both identity laws, inverses,
orders, and claimed commutativity. Additional tests pin multiplication conventions.

Original implementations of standard constructions. Reference:
[J. S. Milne, Group Theory](https://www.jmilne.org/math/CourseNotes/GT.pdf),
especially groups, permutation groups, direct products, and semidirect products.
The Heisenberg law follows by multiplying the displayed matrices; affine laws
follow by composing the displayed maps. No prose is copied from the notes.
