# Society

Three procedural leaves emit computed social-choice objects. The family has
local weight 2; its `economics` branch also has weight 2, giving each leaf one
third of this family's samples. Other nodes use weight 1.

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

These are small synthetic models, not forecasts or historical observations.
Matching omits ties, unacceptable partners, quotas and transfers. Elections
omit truncated ballots and strategic behavior. Game payoffs are utility units;
the equilibrium solver is specialized to these nondegenerate constructions.
Tests independently enumerate stable assignments, verify both optimality
extremes, check vote conservation and promised election regimes, and verify
mixed equilibria through exact action-payoff indifference.

Sources for the algorithms and model conventions:

- [Felt and Natzke, *The Mathematics of Voting*, MIT BLOSSOMS](https://web.mit.edu/blossoms/videos/lessons/mathematics_voting/).
- [Gale and Shapley, *College Admissions and the Stability of Marriage* (1962)](https://iqua.ece.toronto.edu/baochun/ece1771f/papers/Gale-Shapley-1962.pdf).
- [Daskalakis and Farina, *Setting and equilibria: the Nash equilibrium*, MIT 6.7980](https://www.mit.edu/~6.7980/nfgs_nash.html).
- [MIT 14.126 game-theory slides (2016), stag hunt and risk dominance](https://ocw.mit.edu/courses/14-126-game-theory-spring-2016/bba04a523a1daeeeffee6c9f2377af17_MIT14_126S16_gametheory.pdf).

No source text or implementation is embedded. All instances and code are
original constructions.
