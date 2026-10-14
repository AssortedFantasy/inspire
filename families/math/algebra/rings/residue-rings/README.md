# Composite residue rings

Original bounded arithmetic generator for `Z/nZ`, uniform over composite
moduli 4 through 120. Each sample supplies operations, a nonidentity unit and
its inverse, a nonzero zero divisor with a nonzero annihilating witness,
nilpotents, a proper nonzero ideal, its annihilator, and its quotient map.
All choices use the caller's seeded RNG. No downloaded dataset or Sage code
is embedded.

Reference checked online on 2026-10-13: Sage's official
[integer modulus ring documentation](https://doc.sagemath.org/html/en/reference/finite_rings/sage/rings/finite_rings/integer_mod_ring.html)
for the ring construction and
[element documentation](https://doc.sagemath.org/html/en/reference/finite_rings/sage/rings/finite_rings/integer_mod.html#sage.rings.finite_rings.integer_mod.IntegerMod_abstract.is_nilpotent)
for the prime-divisor criterion for nilpotence. The prose and arithmetic are
original. Exhaustive bounded tests use multiplication to check units, zero
divisors, nilpotence, ideals, annihilators, and quotient operations independently
of the number-theoretic criteria.
