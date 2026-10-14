# Trigonometric antiderivatives

Five equally likely templates: sine, cosine, sine squared, cosine squared,
and secant squared. Each uses the affine angle `a*x + b` in radians, with
integer `a` in `1..=6`, `b` in `-5..=5`, and a nonzero signed coefficient
`c` in `-9..=9`. The positive frequency avoids degenerate constant angles;
the phase and coefficient vary independently.

Sine, cosine, and their squared templates have the entire real line as their domain. Secant
squared excludes `a*x + b = pi/2 + k*pi` for every integer `k`. Integration
constants are independent on connected domain intervals.

The formulas were checked against [OpenStax, Calculus Volume 2, Table of
Integrals](https://openstax.org/books/calculus-volume-2/pages/a-table-of-integrals),
entries 5–7 and 18–19. Code and output wording are original; no source prose
is reproduced. Tests differentiate every template across all parameter choices
at 20 points spanning five secant-domain components, and check rendered
coefficients, sampling bounds, domains, and constants.
