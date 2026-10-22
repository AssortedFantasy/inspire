# Quadratic finite fields

Exact constructions of F_p[t]/(t²+bt+c), for p in 2, 3, 5, 7, 11, 13.
Choose p uniformly, then choose uniformly among its monic rootless quadratics.
Nonconstant elements have degree two over F_p.

Each output shows the defining quotient and one sampled view:

- A sum and product of two elements.
- Frobenius conjugation, trace, and norm of one element.
- An inverse and multiplicative order.
- An element's minimal polynomial and its conjugate roots.

Defining polynomials omit zero and unit coefficients. Coefficients use canonical
nonnegative residues, including in characteristic two.

Tests check every field model and element against Frobenius powers, inverse
identities, norms, traces, minimal polynomials, and element orders. Small models
also receive exhaustive associativity and distributivity checks. Arithmetic is
exact and bounded; every model has a multiplicative generator.

Reference context: [SageMath finite-field constructors](https://doc.sagemath.org/html/en/reference/finite_rings/sage/rings/finite_rings/finite_field_constructor.html).
No external dataset or source prose is embedded.
