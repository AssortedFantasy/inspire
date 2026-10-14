# Finite groups

Seven separate procedural leaves describe cyclic groups, direct products of
cyclic groups, dihedral groups, generalized quaternion groups, symmetric groups,
affine groups over prime fields, and finite Heisenberg groups. Every sample
specifies its set, operation, group order, identity, a computed product, and an
inverse or element order. Dihedral notation D_n always denotes the symmetry
group of an n-gon with order 2n. Each child documents its parameter bounds and
additional invariants.

These are bounded algebraic constructions, not imported classification data.
Permutation sampling uses a shuffle and never enumerates the whole group during
generation. Leaf tests check algebraic invariants, including multiplication
conventions, inverses, associativity, and displayed element orders.

Original implementations of standard constructions. Reference:
[J. S. Milne, Group Theory](https://www.jmilne.org/math/CourseNotes/GT.pdf),
version 4.01 (November 2025), especially groups (pp. 7–14), direct products
(p. 23 onward), semidirect products, and permutation groups (p. 64 onward).
The Heisenberg law follows by multiplying the displayed matrices; affine laws
follow by composing the displayed maps. No source dataset or prose is copied;
the linked notes are a reference for the standard mathematics. Their source
license is CC BY-NC-SA 4.0; the original generator implementations do not embed
the notes.
