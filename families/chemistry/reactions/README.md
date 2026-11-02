# Reaction classes

`generate_reactions.py` writes ten bounded generators from 87 reviewed templates.
Each leaf has its own `support.rs` so moving the node preserves its code. Edit the
Python catalog and regenerate, then format both Rust files.

Every child weight equals its descendant equation count, so each of the 87
reactions has the same probability within this branch. Regeneration updates
these weights without changing the incoming `reactions` topic weight.
`chemistry/verify_data.py` independently checks weights against emitted catalogs.

All species specify phases; charged species use `^+`, `^-`, `^2+`, etc. Net ionic
equations omit spectator ions. Equilibrium arrows differ from idealized complete
combustion, precipitation and displacement arrows.

Neutralization enumerates soluble combinations of dilute strong monoprotic acids
and alkali hydroxides plus one barium-hydroxide/nitric-acid example. It does not
randomly combine salts or infer that every exchange precipitates. Combustion
coefficients follow C/H/O accounting for 28 actual fuels, with water vapor among
hot products. Fuel names distinguish isomers that share a molecular formula.
Outputs use primitive integer coefficients, with no randomly scaled extents or
repeated consumption/production calculations.

Two leaves also expose equilibrium contrasts, retaining their ordinary equations
in 35% of draws. Weak equilibria compare strong and weak titration curves across
buffering, equivalence and overshoot. Ionic salts compare selective silver-halide
precipitation, including reversed and coincident onsets. Their hand-maintained
`modes.rs` files and leaf READMEs contain bounded models, assumptions and sources;
regeneration preserves them. Existing equation-count weights remain unchanged.

Rust tests independently parse displayed formulas, parentheses and charges, and
check every complete equation rather than sampled outputs alone. Balance is
necessary but does not establish feasibility; the source catalog contains
established reactions. Displayed conditions identify useful heat, pressure and
catalyst requirements without restating phase labels or chemical conventions.
