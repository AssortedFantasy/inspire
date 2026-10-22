# Mastermind

Four-position codes use six symbols, with repetition. Each clue gives an exact
position count and a misplaced-symbol count; duplicate symbols are matched only
once. The answer is omitted.

A secret is sampled uniformly from all 1,296 codes. Random probe pools narrow the
consistent candidates, with an additional non-answer candidate probe guaranteeing
progress. Clues are then pruned until every remaining clue is necessary for the
unique solution. Output varies in both the number and structure of its clues.
All work is local and bounded by the finite code space. Tests independently
exhaust the code space for each sampled puzzle and each single-clue deletion.
