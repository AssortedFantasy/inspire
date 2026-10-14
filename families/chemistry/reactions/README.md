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
hot products. Extents multiply coefficients without predicting actual conversion.

Rust tests independently parse displayed formulas, parentheses and charges, and
check every complete equation rather than sampled outputs alone. Balance is
necessary but does not establish feasibility: explicit conditions and the reviewed
source catalog provide that constraint.
