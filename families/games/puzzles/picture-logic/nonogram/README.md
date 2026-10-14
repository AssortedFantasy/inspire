# Unique 5x5 nonograms

Original random binary grids are accepted only when exhaustive row-pattern
search finds exactly one grid for both clue sets. Generation tries at most 32
grids, then uses the independently verified unique Ferrers shape
31/15/7/3/1 as a bounded fallback. Between 7 and 18 cells are filled in random
accepted grids. Clues use 0 for an empty line. No difficulty level is claimed.
