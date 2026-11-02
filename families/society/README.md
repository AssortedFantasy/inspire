# Society

Seven procedural leaves emit computed social-choice and market objects, and
three corpus leaves contain 46 historical institutional clauses and court
entries. The family has local weight 2. Economics, voting and institutions
have sibling weights 7, 3 and 2. Elections, matching and game theory each
receive one sixth of society draws; the four newer generators each receive
one twelfth. Institutions receive the remaining one sixth, with corpus leaf
weights proportional to record counts. The three established generator leaves
have weight 2; other generator nodes use weight 1.

- `voting/elections`: complete strict rankings over three or four candidates.
  Equal shares of decisive consensus, three-way majority cycles, center squeeze,
  majority/Borda reversals and unrestricted profiles. Counts use plurality,
  Borda scores from `n−1` down to zero, IRV and strict Condorcet comparisons.
  Plurality and Borda retain co-winners. IRV explores every tied elimination;
  it never resolves ties by candidate label. All head-to-head vote totals appear.
- `economics/matching`: three to six agents on each side with strict complete
  preferences and unit capacities. Equal shares of shared-priority unique
  matchings, one preference rotation, two independent rotations and unrestricted
  preferences. Deferred acceptance runs from both sides; exhaustive enumeration
  of at most 720 assignments supplies the stable-matching count. The constructed
  rotations have exactly two or four stable matchings.
- `economics/game-theory`: equal shares of Prisoner's Dilemma, stag hunt,
  hawk–dove, opposed-preference coordination, unequal-stakes matching pennies
  and weighted rock–paper–scissors. Integer payoff inequalities preserve each
  regime while changing mixed-strategy thresholds. All pure and fully mixed
  Nash equilibria are exact. These constructions have no other equilibria.
  Pareto comparisons concern displayed pure cells, not lotteries. Stag hunt
  spans both sides of risk dominance and its equality boundary.
- `economics/market-clearing`: linear inverse demand and marginal cost with
  exact clearing, tax incidence and transfers, price ceilings and floors.
  Equal shares of ordinary clearing, taxes with asymmetric/equal incidence,
  ceilings, floors, shutdown thresholds, and hard-capacity tax transitions.
  Controls include binding, nonbinding and equality cases. Capacity samples
  cross the tax level at which scarcity rent disappears. Prices, quantities
  and welfare use reduced fractions. [Model and sources](economics/market-clearing/README.md).
- `economics/fair-division`: additive common-scale values for indivisible
  goods. Exhaustive search compares maximum total value with maximum total
  value subject to EF1, and counts exactly envy-free allocations separately.
  Seven regimes include aligned specialties, a utilitarian monopoly,
  identical goods with a division remainder, a dominant trophy, cyclic
  interests, unrestricted values, and a one-point EF1 boundary.
  [Model and sources](economics/fair-division/README.md).
- `economics/auctions`: one item and three to six bidders with explicit
  private values, submitted bids, a reserve and announced tie priority.
  Each profile is evaluated under first-price, second-price and all-pay
  payment rules. Nine regimes include inefficient awards, reserve failures,
  reserve equality, ties, overbidding and negative all-pay utility.
  [Model and sources](economics/auctions/README.md).
- `voting/apportionment`: Hamilton, Jefferson, Webster, Huntington–Hill and
  Adams seat allocations for three to six states. Five equal regimes cover
  ordinary populations, constructive Alabama paradoxes, divisor disagreements,
  equal populations and integral quotas. Alabama cases show both house sizes.
  Exact integer priorities decide seats, with an explicit label tie rule.
  [Model and sources](voting/apportionment/README.md).
- `institutions`: complete selected clauses and court entries concerning
  commons, borough property, mutual aid, work, inspection and everyday disputes.
  Sources include London tawyers, Bristol dyers, Bradford court rolls,
  Wimeswould common fields, Tewkesbury's charter and Leicester grievances.
  [Collection, sources and reproduction](institutions/README.md).

The procedural outputs are small synthetic models, not forecasts or historical observations.
Matching omits ties, unacceptable partners, quotas and transfers. Elections
omit truncated ballots and strategic behavior. Game payoffs are utility units;
the equilibrium solver is specialized to these nondegenerate constructions.
Fair division has no transfers or divisible goods, and EF1 permits envy.
Auction bids are fixed inputs, not asserted equilibrium strategies. Its
all-pay reserve is an allocation threshold with no refund of losing bids.
Price-control welfare depends on unspecified rationing and is not reported.
Tests independently enumerate stable assignments, verify both optimality
extremes, check vote conservation and promised election regimes, and verify
mixed equilibria through exact action-payoff indifference. New leaves check
market complementarity and surplus accounting, exhaust small valuation
matrices with explicit single-good removal, and check auction outcomes
against sorted order statistics and truthful second-price deviations.
Apportionment tests check quotas, common divisor intervals, house monotonicity,
constructive paradoxes and an external Census fixture.

Sources for the algorithms and model conventions:

- [Felt and Natzke, *The Mathematics of Voting*, MIT BLOSSOMS](https://web.mit.edu/blossoms/videos/lessons/mathematics_voting/).
- [Gale and Shapley, *College Admissions and the Stability of Marriage* (1962)](https://iqua.ece.toronto.edu/baochun/ece1771f/papers/Gale-Shapley-1962.pdf).
- [Daskalakis and Farina, *Setting and equilibria: the Nash equilibrium*, MIT 6.7980](https://www.mit.edu/~6.7980/nfgs_nash.html).
- [MIT 14.126 game-theory slides (2016), stag hunt and risk dominance](https://ocw.mit.edu/courses/14-126-game-theory-spring-2016/bba04a523a1daeeeffee6c9f2377af17_MIT14_126S16_gametheory.pdf).

The algorithm references supply no embedded text or implementation; procedural
instances and code are original constructions. The institutional corpus embeds
public-domain primary text with source and edition attribution.
