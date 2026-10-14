# Logarithmic antiderivatives

Three equally sampled templates use `u = a*x + b`: reciprocal `c/u`,
logarithm by parts `c*ln|u|`, and logarithmic power `c*(ln|u|)^n/u`.
Integer bounds are `a = 1..6`, `b = -5..5`, nonzero `c = -9..9`, and
`n = 1..4` for the power template. Coefficients are reduced rational numbers;
the affine scale contributes the chain-rule factor and the shift moves the pole.

All formulas hold for either sign of `u` on intervals avoiding `u = 0`.
The integration constant is independent on each connected interval.
Tests numerically differentiate every parameter combination on both sign domains,
including `|u|` below, equal to, and above one.

These are original bounded constructions using the reciprocal rule and
integration by parts from OpenStax *Calculus Volume 2*:
[table of integrals](https://openstax.org/books/calculus-volume-2/pages/a-table-of-integrals)
and [integration by parts](https://openstax.org/books/calculus-volume-2/pages/3-1-integration-by-parts).
The logarithmic-power template follows by substituting `v = ln|u|`.
No source prose or dataset is embedded.
