# Number theory

Three independent exact constructions separate rational approximation,
Diophantine geometry, and modular arithmetic. Each leaf emits an actual object
with computed certificates, uses bounded integer arithmetic and the supplied
RNG, and has substantive invariant tests. No source prose is reproduced.

- `continued-fractions/convergents`: finite simple continued fractions of length
  3–8, entries bounded by 9, exact rational values and all reduced convergents.
  Tests independently evaluate backward and check coprimality and adjacent
  determinant identities across 2,000 samples.
- `diophantine/primitive-pythagorean-triples`: Euclid parameter pairs with
  `2 <= m <= 48`, `0 < n < m`, opposite parity and gcd 1. Uniform over eligible
  parameter pairs; sorted integer sides, area, perimeter and exact radii.
  Tests exhaust all allowed pairs for primitiveness and geometric identities.
- `modular-arithmetic/chinese-remainders`: 2–4 pairwise coprime moduli in 2–40,
  with canonical residues, least nonnegative solution and explicit inverse
  reconstruction. Tests cover modular inverses, 2,000 systems and independent
  enumeration of small complete solution periods.

Primary reference context, checked during development:
[NIST DLMF §1.12, continued fractions](https://dlmf.nist.gov/1.12),
[Keith Conrad, Pythagorean Triples](https://kconrad.math.uconn.edu/blurbs/ugradnumthy/pythagtriple.pdf),
and [Victor Shoup, A Computational Introduction to Number Theory and Algebra,
version 2.5, §2.4](https://www.shoup.net/ntb/ntb-v2_5.pdf).
These sources establish the mathematics; the generators and rendered wording
are original, and are not imported datasets.
