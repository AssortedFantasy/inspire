# Equal-summary datasets

Pairs of 5–10 exact rational bivariate observations share all five sums
Σx, Σy, Σx², Σxy and Σy². Each pair therefore shares its fitted line, R²,
residual sum of squares and sample covariance matrix. The output shows the
observations, fitted line and either sample variances/covariance or the maximum
leverage in each dataset.

Three equally likely constructions preserve the sums by rational Householder
reflections. A quadratic residual pattern is reflected within the space
perpendicular to constants and x; a design with one unit-leverage observation
is spread into three x clusters; and a local second-difference residual is
reflected jointly with its x coordinates. These are generated constructions,
not Anscombe's original rounded datasets. Intercepts are −5…8, nonzero integer
slopes are −3…3 and residual scales are 1…3. No stochastic inference or rounded
floating-point statistics are emitted.

Tests check all five preserved sums, calculate centered products independently
from pairwise differences, verify both regression normal equations and SSE,
and check that leverage sums to two. They also verify the unit-leverage boundary
and its reduction in the reflected clustered design.

Reference context: F. J. Anscombe,
[*Graphs in Statistical Analysis* (1973)](https://doi.org/10.1080/00031305.1973.10478966).
No source observations or prose are embedded. The original construction here
makes the equality exact rather than relying on decimal-rounded summaries.
