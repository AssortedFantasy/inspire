# Contingency tables

Two-by-two tables receive 60% of samples, three-by-three 40%. Every row and
column marginal is positive; individual cells may be zero. Margins are selected
or constructed first and preserved by the table construction. This is a mixture
of interesting tables, not a null-distribution sampler or a uniform sampler of
all fixed-margin tables.

Two-by-two regimes are equally weighted: exact proportional independence,
an endpoint of the feasible fixed-margin interval, the nearest integer to the
independent expectation, and a uniform feasible upper-left cell. Three-by-three
regimes are exact rank-one independence, fixed-margin checkerboard transfers,
strong diagonal association with sparse cyclic spillover, and greedy boundary
transportation tables. Row and column permutations conceal the construction.

The Pearson statistic is an exact reduced rational, without Yates correction.
Degrees of freedom are `(r-1)*(c-1)`. No asymptotic p-value is printed. For 2×2,
Fisher's two-sided p sums every fixed-margin hypergeometric weight at most the
observed weight; comparison and summation use exact `u128` integers. It is not
twice the smaller one-sided tail. The result is printed as a reduced rational.
Fisher totals are at most 64, and 3×3 totals at most 81; larger tables are absent.

References: [NIST Pearson statistic](https://itl.nist.gov/div898/handbook/prc/section4/prc45.htm),
[NIST Fisher test](https://www.itl.nist.gov/div898/software/dataplot/refman1/auxillar/fishexac.htm),
and [R's probability-order two-sided definition](https://search.r-project.org/R/refmans/stats/html/fisher.test.html).
Tests enumerate all label assignments for every margin through total ten,
check the independent determinant formula for 2×2 Pearson, and compare exact
statistics with the cellwise floating-point formula on generated tables.
