# Indefinite integrals

Five equally weighted leaves contain 16 original bounded templates. Each leaf
keeps its generator, formatting helpers, tests, and source notes locally.
Templates within a leaf are sampled uniformly; every output gives the integrand,
antiderivative, and real domain.

| Leaf | Templates |
| --- | --- |
| `polynomials` | Affine powers; sparse polynomial sums |
| `logarithms` | Reciprocal; logarithm by parts; logarithmic powers divided by the affine argument |
| `exponentials` | Affine exponential; linear and quadratic exponential moments |
| `trigonometric` | Sine; cosine; sine squared; cosine squared; secant squared |
| `inverse-trigonometric` | Reciprocal quadratic; reciprocal square root; arctangent by parts |

Affine shifts range from -5 through 5, coefficient magnitudes from 1 through 9,
and slopes from 1 through 6; exponential slopes also include the negative values.
Polynomial powers range from 2 through 6. Sparse polynomial parameters are
specified in their leaf README. Logarithmic powers range from 1 through 4;
inverse-trigonometric radii range from 1 through 5.

Logarithms use ln|u| on either sign domain. Secant poles are excluded; inverse
sine uses its principal real branch strictly inside its radius. Integration
constants can differ between disconnected domain components. Tests check
numerical derivatives, rendered formulas, fractions, domains, and output bounds.

The generators apply standard identities from the
[OpenStax integral table](https://openstax.org/books/calculus-volume-2/pages/a-table-of-integrals)
and [integration by parts](https://openstax.org/books/calculus-volume-2/pages/3-1-integration-by-parts).
Code and wording are original; no source prose or dataset is embedded.
