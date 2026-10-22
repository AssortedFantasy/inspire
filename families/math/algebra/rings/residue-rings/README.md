# Composite residue rings

Exact bounded arithmetic in Z/nZ, uniform over composite moduli 4 through 120.
Each sample selects one mathematical view:

- A nonidentity unit and its inverse.
- A zero divisor and its annihilator.
- The nilradical, with a nonzero nilpotent's powers when available.
- A proper nonzero ideal, its annihilator, and its quotient.
- The Chinese remainder decomposition into prime-power factors, a mapped
  residue, and the corresponding orthogonal idempotents. For prime powers,
  show the chain of ideals and residue field instead.

Long arithmetic progressions use an ellipsis. Samples do not repeat the ring
axioms or report every invariant at once. All choices use the caller's RNG.

Exhaustive bounded tests check units, zero divisors, nilpotence, ideals,
annihilators, quotient operations, CRT reconstruction, and orthogonal
idempotents independently of the generation criteria.

References: Sage's [integer modulus rings](https://doc.sagemath.org/html/en/reference/finite_rings/sage/rings/finite_rings/integer_mod_ring.html)
and [nilpotence criterion](https://doc.sagemath.org/html/en/reference/finite_rings/sage/rings/finite_rings/integer_mod.html#sage.rings.finite_rings.integer_mod.IntegerMod_abstract.is_nilpotent).
No external dataset or source prose is embedded.
