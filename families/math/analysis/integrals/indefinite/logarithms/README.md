# Logarithmic antiderivatives

Three equally sampled templates use `u = a*x + b`: reciprocal `c/u`,
logarithm by parts `c*ln|u|`, and logarithmic power `c*(ln|u|)^n/u`.
Integer bounds are `a = 1..6`, `b = -5..5`, nonzero `c = -9..9`, and
`n = -1` or `1..4` for the power template. Within that template, `n = -1`
has probability 1/3 and each positive power 1/6. The exceptional power gives
`ln(|ln|u||)` instead of dividing by `n + 1`, and opens domain holes at
`u = -1, 1` as well as zero. Coefficients are reduced rational numbers;
the affine scale contributes the chain-rule factor and the shift moves the pole.

All formulas hold on each interval of their printed real domain.
The integration constant is independent on each connected interval.
Tests numerically differentiate every parameter combination on both sign domains,
including `|u|` below and above one, and equal to one when the integrand exists.

These are original bounded constructions using the reciprocal rule and
integration by parts from OpenStax *Calculus Volume 2*:
[table of integrals](https://openstax.org/books/calculus-volume-2/pages/a-table-of-integrals)
and [integration by parts](https://openstax.org/books/calculus-volume-2/pages/3-1-integration-by-parts).
The logarithmic-power template follows by substituting `v = ln|u|`.
No source prose or dataset is embedded.
