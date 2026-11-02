# Precipitation and selective onset

35% of draws retain the nine ordinary net ionic equations. The rest compare
two of AgCl, AgBr and AgI in a shared solution. Constructed concentration ratios
give either precipitation order, simultaneous onset, nearly coincident onsets,
or a useful separation. At the second onset the first halide's remaining
dissolved fraction exposes how much separation actually occurred.

This is an ideal free-ion equilibrium model at 25 °C with negligible dilution,
independent pure solid phases, and no complexation, solid solutions, activity
corrections or nucleation kinetics. For each halide X, onset occurs when
[Ag+] = Ksp/[X-]. After onset, the dissolved halide is Ksp/[Ag+]. Initial chloride
or bromide is 0.0001–0.001 M; the other concentration is constructed around the
onset-order boundary. Displayed values use four significant figures.

Ksp values AgCl = 1.6e-10, AgBr = 5.0e-13 and AgI = 1.5e-16 are from
[OpenStax Chemistry 2e Appendix J](https://openstax.org/books/chemistry-2e/pages/j-solubility-products).
The concentration-dependent order reversal is treated in
[§15.1](https://openstax.org/books/chemistry-2e/pages/15-1-precipitation-and-dissolution).
Only factual constants are used; no textbook prose or illustrations are bundled.

`modes.rs` is hand-maintained; `chemistry/generate_reactions.py` regenerates the
ordinary equation catalog and dispatch. Tests exhaust the constructed regimes
and check saturation products, onset order and dissolved/solid inventories.
