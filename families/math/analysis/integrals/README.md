# Integrals

Eight leaves beneath two equally weighted branches contain 24 original bounded
templates. Leaves have equal weights among their own siblings, and templates
within each leaf are sampled uniformly. Every output gives an integrand, its
evaluated result, and the real domain or convergence conditions.

| Branch | Leaf | Templates |
| --- | --- | --- |
| Indefinite | `polynomials` | Affine powers; sparse polynomial sums |
| Indefinite | `logarithms` | Reciprocal; logarithm by parts; logarithmic powers |
| Indefinite | `exponentials` | Affine exponential; linear and quadratic moments |
| Indefinite | `trigonometric` | Sine; cosine; sine squared; cosine squared; secant squared |
| Indefinite | `inverse-trigonometric` | Reciprocal quadratic; reciprocal root; arctangent by parts |
| Definite | `polynomials` | Monomial; three-term polynomial |
| Definite | `beta-integrals` | Unit interval; affine endpoints; trigonometric |
| Definite | `gamma-integrals` | Ordinary; translated; generalized moments |

The [indefinite notes](indefinite/README.md) give coefficient, affine, power and
radius bounds. Logarithms use ln|u| on either sign domain; secant poles are
excluded, and inverse sine uses its principal real branch strictly inside its
radius. Constants can differ between disconnected domain components.

The [definite notes](definite/README.md) give polynomial endpoint bounds, beta
parameter ranges and positive gamma decay rates. Values are reduced rationals.
Beta substitutions include their Jacobians; gamma cases explain convergence of
the infinite tail. All arithmetic stays within small integer bounds.

Thirty local tests check numerical derivatives, exact polynomial integration,
quadrature, symmetry cancellation, substitution factors, endpoint behavior,
gamma-tail bounds, rendered fractions and parameter coverage. Beta and gamma
invariants cover every parameter combination. Reviewers also inspected actual
emitted text and independently checked rendered definite answers.

Every generator keeps its helpers, tests and source notes inside its own leaf.
There is no symbolic solver or runtime I/O. The former mixed `math/integrals`
generator now lives beneath `math/analysis`.

Code and wording are original bounded constructions, not downloaded prose.
Primary references checked during review are the
[OpenStax integral table](https://openstax.org/books/calculus-volume-2/pages/a-table-of-integrals),
[integration by parts](https://openstax.org/books/calculus-volume-2/pages/3-1-integration-by-parts),
[NIST DLMF beta identities](https://dlmf.nist.gov/5.12),
[Euler's gamma integral](https://dlmf.nist.gov/5.2.E1), and
[generalized gamma moments](https://dlmf.nist.gov/5.9.E1).
