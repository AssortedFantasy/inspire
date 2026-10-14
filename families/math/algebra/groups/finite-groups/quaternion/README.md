# Generalized quaternion groups

Original procedural examples of groups of order 8, 16, 32, 64, 128, or 256.
The parameter `m` is a power of two, and `Q_(4m)` labels the group by its
order. Coordinates use the normal form `r^i s^j`; the multiplication law
includes the carry `mjl` because `s²=r^m`.

Reference checked by the finite-groups curator: J. S. Milne,
[Group Theory, version 4.01, Example 2.7(b), printed page 35](https://www.jmilne.org/math/CourseNotes/GT.pdf).
Milne uses `Q_n` for the group of order `2^n`; this generator instead places
the order itself in the subscript and states that convention in each output.
The coordinate law and accompanying examples are original computations;
no source text or external dataset is redistributed.

Tests exhaustively check associativity and closure for orders 8 and 16,
and check both inverse identities, computed element orders, the center,
the unique involution, and presentation relations throughout the parameter range.
Fixed-seed samples verify reproducibility and coverage of all six group orders.
