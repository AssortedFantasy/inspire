# Definite integrals

Three equally weighted leaves emit exact integral identities. Each leaf samples
its structural forms equally.

- `polynomials`: monomials, sparse sums with distinct degrees, and products of
  distinct linear factors. Degree is at most 6; integer limits lie in [-3,3].
  Factored forms keep their roots visible rather than expanding every product.
- `beta-integrals`: unit-interval and affine endpoint powers, odd and even
  trigonometric powers, rational integrals over [0,infinity), and integrable
  square-root endpoint singularities. Answers are reduced rationals or rational
  multiples of pi.
- `gamma-integrals`: exponential and translated moments, generalized Gaussian
  moments, whole-line Gaussian and Laplace moments, and logarithmic moments on
  [0,1]. Answers are reduced rationals or rational multiples of `sqrt(pi/a)`.

Eighteen local tests check independent quadrature, exact polynomial arithmetic,
factored/expanded agreement, symmetry, substitution Jacobians, endpoint behavior,
fraction reduction, parameter bounds, and tail convergence. Quadrature covers
every parameter combination in the beta and gamma forms.

These are bounded mathematical constructions. Beta identities follow
[NIST DLMF 5.12.1](https://dlmf.nist.gov/5.12.E1) and
[5.12.2](https://dlmf.nist.gov/5.12.E2). Gamma moments follow
[Euler's integral](https://dlmf.nist.gov/5.2.E1), integration by parts, and
[DLMF 5.9.1](https://dlmf.nist.gov/5.9.E1). Rational beta integrals use
`t=x/(1+x)`; root endpoints use `x=1-t^2`; logarithmic moments use `x=exp(-t)`.
