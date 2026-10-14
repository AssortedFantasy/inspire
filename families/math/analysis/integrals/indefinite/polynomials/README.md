# Polynomial antiderivatives

Two equally likely templates: an affine power and a sparse polynomial with two
distinct positive degrees and a nonzero constant. All have domain R.

Affine powers use scale 1–6, shift −5–5, nonzero coefficient −9–9, and power 2–6.
Sparse polynomials use high degree 2–6, low degree 1–(high degree − 1), and
nonzero high, low, and constant coefficients with magnitudes 1–9, 1–6, and 1–5.

Original formulas and code apply the power rule from
[OpenStax's integral table](https://openstax.org/books/calculus-volume-2/pages/a-table-of-integrals),
with affine substitution and linearity.
Tests check both templates by numerical differentiation and exact output examples.
