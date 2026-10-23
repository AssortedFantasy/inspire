# Cribbage hands

Original four-card hands or cribs with a separate starter and exact show scoring.
The breakdown counts distinct fifteen subsets, rank pairs, longest runs with
their multiplicities, flushes, and nobs. Four suited cards score four only in a
hand; the crib needs all five. A starter jack's heels are outside the hand score.

Ordinary shuffled deals mix with double and triple runs, flush boundaries,
fifteen-rich rank shapes, and constructed zero hands. Four-fives hands appear
rarely to cover the 28/29 boundary. This deliberately differs from the natural
distribution of dealt hands. All cards are distinct, aces stay low, and every
scoring subset is enumerated in bounded local work.

Tests cover 29, repeated runs, contained shorter runs, ace boundaries, crib versus
hand flushes, starter-dependent nobs, card uniqueness, and impossible totals.
An independent frequency-based scorer cross-checks all 6,175 legal rank
multisets with four suit/starter placements each.
Reference: [American Cribbage Congress tournament rules](https://www.cribbage.org/NewSite/rules/rulebook_2020.pdf),
section 1.7. Code and output are original; no rulebook text is embedded.
