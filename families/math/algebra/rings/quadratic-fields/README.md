# Quadratic finite fields

Original bounded constructions of F_p[t]/(t²+bt+c), for p in
2, 3, 5, 7, 11, 13. Enumerate all monic rootless quadratics and sample one
uniformly after choosing p uniformly. A degree-two polynomial is irreducible
exactly when it has no root in its coefficient field. Examples use nonconstant
elements and report addition, product, inverse, Frobenius conjugate, trace,
norm, and multiplicative order.

Printed defining polynomials omit zero and unit coefficients. Frobenius images
use canonical nonnegative residues, including in characteristic two.

Tests check every sampled field model and element against Frobenius powers,
inverse identities, norms, traces, and element orders, plus exhaustive small
field associativity and distributivity. All arithmetic is exact and bounded.
Every model has an element of order p²-1, checking the cyclic group statement.

Reference context checked online (SageMath 10.8 manual, 2026-10-13):
[SageMath finite-field constructors](https://doc.sagemath.org/html/en/reference/finite_rings/sage/rings/finite_rings/finite_field_constructor.html),
which documents prime-power cardinalities and irreducible defining moduli.
This is original arithmetic, with no copied source prose or imported dataset.
