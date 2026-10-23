# Kakuro

Original small cross-sum puzzles with 6, 8, or 12 white cells. Four layouts mix
rectangles, a missing corner, a central cross, and offset overlapping bands.
Reflections and transpositions vary their orientation. The printed grid uses
`#` for blocks, `.` for empty white cells, and `down\across` clue cells.

Fills use low-digit, high-digit, and full-digit regimes, with no repetitions
within a run. A separate MRV backtracking counter checks uniqueness from sums
alone. It enforces distinct digits and remaining-sum bounds and stops after two
answers. Generation permits 64 attempts, each with a 6,000-node budget; exhaustion
never passes as uniqueness. Verified per-layout fallbacks keep work bounded.
Only symmetries that preserve uniqueness are applied afterwards. No filled cells
are exposed. Tests check uniqueness, sums, run incidence, and distinct digits.

The rule reference is [Nikoli's Kakuro page](https://www.nikoli.co.jp/en/puzzles/kakuro/).
No published puzzles are embedded. There is no human difficulty claim.
