# Integrals

Ten bounded templates randomize integer coefficients, affine substitutions,
powers, and endpoints. Nine produce real antiderivatives with explicit domains;
one gives an exact reduced rational value for a definite polynomial integral.
The constant may differ between disconnected domain components. No symbolic
solver or runtime I/O is used.

Tests differentiate every antiderivative numerically and independently integrate
the definite cases with composite Simpson quadrature. Coefficients and powers
are small enough to avoid integer overflow.

Original parameterized integral identities based on standard calculus, checked
against [OpenStax, Calculus Volume 2, Table of Integrals](https://openstax.org/books/calculus-volume-2/pages/a-table-of-integrals)
and [integration by parts](https://openstax.org/books/calculus-volume-2/pages/3-1-integration-by-parts).
Each output includes its evaluated result and domain, with no request to solve it.
