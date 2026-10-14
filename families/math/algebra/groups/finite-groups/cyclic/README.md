# Finite cyclic groups

Original procedural implementation, not a downloaded dataset. Samples additive
cyclic groups C_n for 2 <= n <= 96, two residues, and a divisor subgroup. Every
output gives the concrete law, identity, sum, inverse, element order, generated
subgroup, generator count, and one subgroup with its cyclic quotient.

Reference: J. S. Milne, [Group Theory, version 4.01 (2025)](https://www.jmilne.org/math/CourseNotes/GT.pdf),
sections on groups and subgroups. The reference is mathematical background;
the generated wording and Rust implementation are original and do not reproduce
the reference text. No dataset is imported. The reference's stated license
is CC BY-NC-SA 4.0.

Tests exhaustively check group laws through n=16, and order formulas against
enumerated orbits, divisor subgroup closure, quotient kernels and addition
through n=96.
A fixed-seed test checks deterministic sampling and output variety.
