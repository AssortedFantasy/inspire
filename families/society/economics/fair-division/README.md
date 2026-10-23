# Fair division

Two or three agents assign nonnegative integer values to three through seven
indivisible goods. Values add across a bundle and use a common point scale.
Every good goes to exactly one agent; there are no transfers or disposal.

The generator enumerates all assignments (at most 2,187). It displays a
maximum-total-value allocation and the maximum-total-value allocation subject
to envy-freeness up to one good (EF1). It checks EF1 separately for each ordered
pair: an observer must value their own bundle at least as highly as an envied
bundle after removing its most valuable good in that observer's valuation.
Displayed envy includes this best removal witness. Exact envy-freeness is
checked separately and may be impossible.

Among welfare ties, prefer fewer failing EF1 pairs, then smaller maximum envy,
then smaller total envy, then the lexicographically first owner vector. Thus a
fairness cost never arises solely from an arbitrary utilitarian tie. The two
displayed allocations coincide whenever some welfare optimum satisfies EF1.
Agent and item labels are shuffled before solving.

Seven regimes have equal sampling probability:

- Two strong specialties per agent yield an envy-free welfare optimum.
- A strict leader on every good forces a utilitarian monopoly and a positive
  welfare cost for EF1.
- Identical unit values place exact envy-freeness on the boundary where the
  number of goods is divisible by the number of agents.
- A trophy worth more than all accessories combined makes envy-freeness
  impossible, while EF1 still permits some envy.
- Three overlapping pairs of interests form a cycle, with two goods per edge.
- Unrestricted values include zeroes and ties.
- Two disputed goods and one private good cross the EF1 boundary by one point
  below, exactly on, or one point above it. Both disputed goods have a strict
  utilitarian owner. Below the boundary, the exact EF1 welfare loss equals the
  cheaper of the two disputed goods' ownership margins.

Tests independently recurse over assignments and explicitly try each possible
single-good removal. They exhaust every two-agent, three-good valuation matrix
with entries in `{0, 1, 2}`, compare optimum values and fairness counts, and
verify the constructed regimes, boundary costs, and utility-maximizing tie
handling.

Model references:

- Lipton, Markakis, Mossel and Saberi, [*On Approximately Fair Allocations of
  Indivisible Goods* (2004)](https://www.cse.iitb.ac.in/~swaprava/courses/cs6002/papers/lipton04.pdf).
- Caragiannis, Kurokawa, Moulin, Procaccia, Shah and Wang, [*The Unreasonable
  Fairness of Maximum Nash Welfare* (2016)](https://cs.au.dk/~iannis/publ/C80-ec16b.pdf),
  for the additive-goods model and EF1 convention. This generator optimizes
  total value under EF1 directly; it does not use the Nash-welfare algorithm.

All instances and code are original constructions. No source text is embedded.
