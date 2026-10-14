# Real analysis

Four concept corpora contain ten records each, distinguishing continuity,
convergence, differentiation and series. Each record states a definition,
theorem with hypotheses, or a concrete example or counterexample. The original
fourteen concept IDs are retained within the forty records.
Domains, endpoint behavior and quantifiers are part of the mathematical content.

- `continuity`: metric quantifiers, separate continuity, square-root regularity,
  bounded oscillation, intermediate and extreme values, Thomae's function,
  preimages and continuous endpoint extension.
- `convergence`: pointwise versus uniform convergence, exact uniform error,
  finite suprema, Cauchy tails, monotone limits, limsup and liminf, and continuity
  of uniform limits.
- `differentiation`: linear approximation, flat nonanalytic functions,
  discontinuous derivatives, Darboux, mean value and inverse rules, stationary
  points and uniform convergence of derivatives with an anchoring value.
- `series`: exact geometric and telescoping tails, alternating remainder bounds,
  ratio-test limitations, p-series, endpoint behavior, rearrangements and Cauchy
  products with their convergence hypotheses.

`integrals` has eight generator leaves containing twenty-four bounded templates:
sixteen real antiderivatives and eight definite forms spanning polynomials,
beta integrals and gamma integrals. Results are evaluated, with real domains and
constants stated explicitly; outputs do not ask the reader to solve a problem.
Thirty local tests check derivatives, exact polynomial arithmetic, quadrature,
substitution factors, endpoint behavior and convergent gamma tails. See its
README for parameter bounds and tests.
Each generator contains only its integrand class and leaf-local helpers and
invariant tests, so an entire leaf can be moved without breaking source paths.

The corpus prose and generators are original mathematical explanations and
constructions. Primary references used for checking are linked in record
provenance and integral documentation. These are source-grounded formulations,
not a downloaded textbook dataset. No source prose is reproduced. Local weights
balance topics, rather than corpus record counts.

Primary reference context: Jiri Lebl's [Basic Analysis](https://www.jirka.org/ra/html/)
for real-function examples and theorem hypotheses; OpenStax's
[integration table](https://openstax.org/books/calculus-volume-2/pages/a-table-of-integrals)
for elementary antiderivatives; NIST's [beta integral](https://dlmf.nist.gov/5.12.E1)
and [gamma integral](https://dlmf.nist.gov/5.2.E1) for the two special-function
families. Individual corpus provenance identifies the section actually checked.

Earlier broad mathematical corpora were reorganized into these topic leaves;
the former `math/integrals` node is now `math/analysis/integrals`. Every leaf is
reviewed separately, including emitted examples and substantive invariants.
