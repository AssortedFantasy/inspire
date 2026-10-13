# Sudoku

Original 9x9 puzzles following [Nikoli's Sudoku rules](https://www.nikoli.co.jp/en/puzzles/sudoku/).
No published puzzle grids are copied. A randomized minimum-candidate search builds
a completion, with a valid digit/band/stack-permuted pattern as a bounded fallback.
Rotationally symmetric clue pairs are removed only after a solver proves exactly
one solution. Blank cells use `.`; answers are intentionally omitted.

The target is 30-42 clues; uniqueness checks can retain more clues. Clue count is
not a difficulty rating. Generation uses at most 10,000 fill-search nodes and 41
uniqueness searches of at most 4,000 nodes each. Exhaustion rejects a removal;
it never certifies uniqueness. Counting stops after a second solution. This follows
the uniqueness-checking construction approach described by Andrew Stuart in
[Sudoku Creation and Grading](https://www.sudokuwiki.org/Sudoku_Creation_and_Grading.pdf),
with original implementation and stricter explicit computation budgets.
