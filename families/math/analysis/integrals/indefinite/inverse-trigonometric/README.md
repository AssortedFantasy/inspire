# Inverse-trigonometric antiderivatives

Three equally likely templates use `u = a*x + b`: reciprocal quadratic
`c/(r² + u²)`, reciprocal root `c/sqrt(r² - u²)`, and `c*atan(u/r)` by parts.
Parameters are `a = 1..6`, `b = -5..5`, nonzero `c = -9..9`, and `r = 1..5`.
Fractions in answers are reduced. Arctangent uses its principal real branch on
all real x; arcsine uses its principal real branch only where `-r < u < r`.
The arbitrary constant is independent on each connected domain interval.

These are original bounded constructions, checked against rows 15 and 16 of
[OpenStax's table of integrals](https://openstax.org/books/calculus-volume-2/pages/a-table-of-integrals).
The third template follows by integration by parts. Differentiating its bracket
gives `a*atan(u/r)`: the derivatives of `u*atan(u/r)` and the logarithmic term
cancel their `a*r*u/(r² + u²)` contributions. The other answers differentiate to
`c/(r² + u²)` and `c/sqrt(r² - u²)` by the affine chain rule with `r > 0`.
No source prose or dataset is embedded. Tests differentiate all parameter
combinations at interior arcsine points and real arctangent points, and check
the rendered factors and domains.
