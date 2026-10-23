# Fault trees

AND/OR failure logic with two to eight distinct basic events. Samples mix ordinary
series and redundant systems, banks of trains, shared common failures, cyclic
overlap, nested mixed gates, and voting logic expanded into AND/OR gates.

Probabilities are synthetic independent Bernoulli probabilities for one demand.
A repeated letter is the same event everywhere, so intermediate gates need not
be independent. There are no repair times, failure rates, or event ordering.

Minimal cut sets use Boolean union/product followed by subset absorption. The top
probability sums all satisfying basic-event assignments with integer weights
`product(p_i or 100-p_i) / 100^n`. At most 256 assignments are needed. Output gives
the reduced exact fraction and a rounded decimal. Overlapping cuts are not added
as though they were disjoint.

Tests compare cut sets with exhaustive minimal satisfying assignments and compare
assignment probabilities with inclusion-exclusion. They also cover shared causes,
absorption, duplicate occurrences, and deterministic probability endpoints.

Sources for the logic and probability model:

- NRC, [Fault Tree Handbook, NUREG-0492](https://www.nrc.gov/regulations-legislation/nureg-series-publications/publications-prepared-by-nrc-staff/sr0492),
  Boolean reduction and fault-tree evaluation.
- Idaho National Laboratory for NRC,
  [SAPHIRE Version 8 Technical Reference, NUREG/CR-7039 Volume 2](https://www.nrc.gov/regulations-legislation/nureg-series-publications/publications-prepared-by-nrc-contractors/systems-analysis-programs-for-hands-on-integrated-reliability-evaluations-saphire-version-8-nureg/v2),
  minimal cut sets and top-event probabilities.

Trees and probabilities are generated locally. No source text or failure data is
embedded.
