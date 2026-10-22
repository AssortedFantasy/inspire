# Trigonometric antiderivatives

Nine equally likely forms: sine, cosine, their squares and cubes, secant squared,
tangent, and sine times cosine. Each uses the affine angle `a*x + b`, with
integer `a` in `1..=6`, `b` in `-5..=5`, and a nonzero signed coefficient
`c` in `-9..=9`. The positive frequency avoids degenerate constant angles;
the phase and coefficient vary independently.

Only tangent and secant squared require a restriction, printed as
`cos(a*x+b) != 0`. Other forms have the entire real line as their domain.

The formulas were checked against [OpenStax, Calculus Volume 2, Table of
Integrals](https://openstax.org/books/calculus-volume-2/pages/a-table-of-integrals),
entries 5–7 and 18–19. Code and output wording are original; no source prose
is reproduced. Tests differentiate every template across all parameter choices
at 20 points spanning five secant-domain components, and check rendered
coefficients, sampling bounds, domains, and constants.
