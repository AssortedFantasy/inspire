# Exponential antiderivatives

Six equally likely forms: affine exponential `c*exp(a*x+b)`, linear
exponential moment `c*x*exp(a*x+b)`, and quadratic exponential moment
`c*x^2*exp(a*x+b)`, quadratic substitution `c*x*exp(a*x^2+b)`, and
`c*exp(u)*sin(u)` or `c*exp(u)*cos(u)` with `u=a*x+b`.
The moments use integration by parts; the quadratic phase uses substitution.
All are real integral identities.

Independent parameters are `a` in `{-6,...,-1,1,...,6}`, `b` in `-5..=5`, and
`c` in `{-9,...,-1,1,...,9}`. Fractions are reduced with positive denominators.
Tests differentiate all 14,256 parameter/form combinations at five points,
and check phase shifts and rendered signs.

These are original bounded constructions using the exponential integral and
integration-by-parts rules in OpenStax *Calculus Volume 2*: [table of
integrals](https://openstax.org/books/calculus-volume-2/pages/a-table-of-integrals)
and [integration by parts](https://openstax.org/books/calculus-volume-2/pages/3-1-integration-by-parts).
No source prose or dataset is embedded.
