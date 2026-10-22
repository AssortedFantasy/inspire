# Reaction classes

`generate_reactions.py` writes ten bounded generators from 87 reviewed templates.
Each leaf has its own `support.rs` so moving the node preserves its code. Edit the
Python catalog and regenerate, then format both Rust files.

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

Rust tests independently parse displayed formulas, parentheses and charges, and
check every complete equation rather than sampled outputs alone. Balance is
necessary but does not establish feasibility; the source catalog contains
established reactions. Displayed conditions identify useful heat, pressure and
catalyst requirements without restating phase labels or chemical conventions.
