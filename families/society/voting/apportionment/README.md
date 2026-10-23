# Apportionment

Synthetic populations for three to six states, allocated by Hamilton,
Jefferson, Webster, Huntington–Hill and Adams. Each of five regimes has equal
probability:

- Unrestricted positive populations and houses of two to eight seats per state.
- Alabama paradoxes with one or two states losing a Hamilton seat when the
  house gains one. Perturbed three-state proportions near 5:3:1 use houses
  9k+4 and 9k+5. Six-state proportions near 15:15:9:5:5:2 use 51k+25 and
  51k+26. Perturbation bounds preserve the strict remainder-order changes.
- Divisor disagreements. One construction produces four distinct Jefferson,
  Webster, Huntington–Hill and Adams allocations. Another gives Jefferson
  every seat in the largest state while Webster represents every state.
- Equal populations with an incomplete final round of seats, exposing exact
  priority and remainder ties.
- Integral quotas, where all five methods agree.

Hamilton awards floors of exact quotas and then largest remainders. The
divisor methods award successive seats using priorities p/(s+1), p/(2s+1),
p/sqrt(s(s+1)) and p/s for Jefferson, Webster, Huntington–Hill and Adams.
Only Huntington–Hill and Adams reserve one seat per state; the other methods
may assign zero. Exact ties go to the earlier displayed state label. Labels
are shuffled before allocation. This is an explicit synthetic tie rule,
not a claim about any legislature's legal procedure.

All ordering uses integer cross-products, with squared priorities for
Huntington–Hill. Displayed quotas round to three decimal places; they never
determine allocations. Alabama samples show both house sizes. Other samples
show one allocation. Runtime is bounded and uses only the supplied RNG.

Tests check conservation, Hamilton's quota and remainder ordering, a common
valid divisor interval for every state's rounded allocation, divisor-method
house monotonicity, guaranteed Alabama losses and divisor disagreements,
integral quotas, deterministic equality, and an exact irrational Hill tie.
The Census Bureau's four-state example supplies an external fixture.

Primary references:

- [U.S. Census Bureau, *Methods of Apportionment*](https://www.census.gov/about/history/historical-censuses-and-surveys/census-programs-surveys/decennial-census/methods.html), for the methods and four-state example.
- [U.S. Census Bureau, *Computing Apportionment*](https://www.census.gov/topics/public-sector/congressional-apportionment/about/computing.html), for the equal-proportions priority sequence and initial seat.
- [H. F. Niemeyer and A. C. Niemeyer, *Apportionment methods*](https://arxiv.org/abs/1510.07528), *Mathematical Social Sciences* 56 (2008), 240–253, for divisor methods and apportionment paradoxes.

No source prose or implementation is embedded. Code and generated instances
are original constructions.
