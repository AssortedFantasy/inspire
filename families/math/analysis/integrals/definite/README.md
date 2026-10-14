# Definite integrals

Three self-contained leaves sample eight bounded templates. Each prints its
integrand, limits, exact reduced rational value, and domain or convergence facts.
Templates within each leaf have equal probability; leaves retain equal weights.

- `polynomials`: a monomial or a three-term polynomial. Monomial degrees are
  0–6 with nonzero coefficients of magnitude 1–9. Sums use degrees 2–4 and
  nonzero leading, linear, and constant coefficients of magnitude at most
  5, 6, and 9. Integer endpoints lie in [-3,3], with interval width 1–3.
- `beta-integrals`: unit-interval endpoint powers, affine endpoint powers,
  or odd powers of sine and cosine on [0,pi/2]. Unit and trigonometric beta
  parameters are 2–8; affine parameters are 2–5, left endpoints 1–3, and
  widths 2–3. Substitution factors appear in the output.
- `gamma-integrals`: exponential moments, translated moments, or generalized
  Gaussian moments. Ordinary powers are 0–8 and positive rates 1–6. Translated
  cases use powers 0–5, rates 1–4, and nonzero shifts -3–3. Generalized cases
  use powers `p*(n+1)-1`, with `n=0..5`, rates 1–4, and `p=2..3`.

Fifteen local tests check quadrature, exact polynomial arithmetic, symmetry
cancellation, substitution Jacobians, endpoint behavior, reduced fractions,
parameter bounds, and gamma-tail convergence. Beta and gamma tests cover every
parameter combination. Review also independently checked actual rendered answers
for 80 polynomial samples, all 194 beta variants, and 120 gamma samples.

These are original bounded constructions, not downloaded prose. The polynomial
cases use the power rule and linearity. Beta identities follow
[NIST DLMF 5.12.1](https://dlmf.nist.gov/5.12.E1) and
[5.12.2](https://dlmf.nist.gov/5.12.E2). Gamma moments follow
[Euler's integral](https://dlmf.nist.gov/5.2.E1), integration by parts, and
[DLMF 5.9.1](https://dlmf.nist.gov/5.9.E1), with positive real parameters.
