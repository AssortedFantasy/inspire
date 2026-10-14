# Polynomials

Separate leaves sample distinct polynomial objects:

- `math/algebra/polynomials/identities`: exact identities from expression trees,
  including collected and factored forms in one or two variables.
- `math/algebra/polynomials/rational-factorizations`: integer polynomials with
  complete factorizations over the rationals and root multiplicities.

The leaves use bounded original constructions, exact integer arithmetic, and the
supplied RNG. Each README documents its coefficient and degree bounds, source
checks, and substantive invariant tests. These mathematical generators need no
runtime datasets or computer-algebra installation. Sibling weights balance
objects rather than the number of possible instances.
