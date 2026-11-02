# Weak equilibria and titration contrasts

35% of draws retain the nine ordinary acid/base equilibrium equations. The rest
compare equal concentrations of acetic acid and HCl titrated with NaOH, or NH3
and NaOH titrated with HCl. Draws cover buffering, the equivalence crossing, and
overshoot. Concentrations are 0.005–0.02 M and volumes 10–40 mL.

The calculation is an ideal aqueous monoprotic model at 25 °C with additive
volumes, complete strong-electrolyte dissociation and Kw = 1e-14. A bounded
90-step bisection solves weak-species mass balance, electroneutrality and water
equilibrium together, including at equivalence. Activity corrections, dissolved
CO2 and additional equilibria are excluded. The ammonia model is the charge-dual
of the weak-acid calculation. Values are model pH, rounded to two decimals.

Acetic acid Ka = 1.8e-5 and the strong/weak titration contrast are documented in
[OpenStax Chemistry 2e §14.7](https://openstax.org/books/chemistry-2e/pages/14-7-acid-base-titrations).
NH3 Kb = 1.8e-5 is given in the same book's
[chapter 14 exercises](https://openstax.org/books/chemistry-2e/pages/14-exercises).
Only factual constants are used; no textbook prose or illustrations are bundled.

`modes.rs` is hand-maintained. `chemistry/generate_reactions.py` regenerates the
equation catalog and dispatch without overwriting it. Tests independently
recover species from charge balance and check mass action, monotonicity,
buffering, equivalence and the strong/weak convergence after overshoot.
