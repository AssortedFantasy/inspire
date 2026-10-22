# Integrals

Eight leaves beneath two equally weighted branches contain 38 bounded forms.
Leaves have equal weights among their own siblings, and forms within each leaf
are sampled uniformly. Outputs are evaluated integral identities, followed by
nontrivial domain restrictions where needed.

| Branch | Leaf | Templates |
| --- | --- | --- |
| Indefinite | `polynomials` | Affine powers; sparse polynomial sums |
| Indefinite | `logarithms` | Reciprocal; logarithm by parts; logarithmic powers |
| Indefinite | `exponentials` | Affine exponential; linear and quadratic moments; quadratic phase; oscillating exponentials |
| Indefinite | `trigonometric` | Sine; cosine; their squares and cubes; secant squared; tangent; sine times cosine |
| Indefinite | `inverse-trigonometric` | Reciprocal quadratic; reciprocal root; arctangent by parts |
| Definite | `polynomials` | Monomial; sparse polynomial; factored powers |
| Definite | `beta-integrals` | Unit interval; affine endpoints; odd and even trigonometric powers; rational improper; square-root endpoints |
| Definite | `gamma-integrals` | Ordinary; translated; generalized moments; Gaussian; Laplace; logarithmic moments |

The [indefinite notes](indefinite/README.md) give coefficient, affine, power and
radius bounds. Logarithms use ln|u| on either sign domain; secant poles are
excluded, and inverse sine uses its principal real branch strictly inside its
radius. Constants can differ between disconnected domain components.

The [definite notes](definite/README.md) give polynomial endpoint bounds, beta
parameter ranges and positive gamma decay rates. Values are exact reduced
rationals or rational multiples of pi and square roots. All arithmetic stays
within small integer bounds.

Thirty-three local tests check numerical derivatives, exact polynomial integration,
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
