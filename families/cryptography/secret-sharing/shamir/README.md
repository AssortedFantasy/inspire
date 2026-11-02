# Shamir sharing over prime fields

Original arithmetic artifacts with uniformly sampled coefficients in GF(p),
including zero. The polynomial has degree at most k−1. Share coordinates are
distinct random nonzero field elements; repeated or zero share values are valid.
Sampling mixes threshold two, unanimous thresholds, redundant shares,
and every available nonzero coordinate in a small field.

Each artifact includes the polynomial, all shares, a random k-share reconstruction
subset, its Lagrange weights at zero in subset order, and the recovered secret.
Weights print as one vector, followed by the weighted reconstruction. Half also exhibit
a second polynomial through k−1 of those shares with a different secret. That
polynomial is constructed by adding a scaled product of the observed linear
factors, preserving the shares while selecting any other constant term.

Tests check every k-subset across sampled fields and thresholds, the monomial
identities of interpolation weights, zero and lower-degree polynomials, and every
candidate secret below threshold. Exhaustive GF(5) enumeration independently
checks that each pair of shares admits one polynomial per secret.

Reference: Adi Shamir, [“How to Share a Secret”](https://people.cs.pitt.edu/~litman/courses/cs2001/lectures/lecture-02-p612-shamir.pdf),
*Communications of the ACM* 22(11), 612–613 (1979), §2. No source text is embedded.
