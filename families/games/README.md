# Games

The tree balances five domains: board games, cards, chance constructions,
impartial games, and puzzles. Branches balance topics rather than corpus size.

- `board/chess/openings`: 3,865 exact Lichess CC0 lines in 22 opening
  leaves, grouped below flank, king-pawn, and queen-pawn systems. Full SAN lines
  preserve their named endpoints; every move and check suffix is legally replayed.
- `board/alignment`: reachable tic-tac-toe and Connect Four positions with
  complete histories; play stops at the first win.
- `board/capture/reversi`: reachable center-start positions, required passes,
  legal next moves, disc counts, and terminal status.
- `cards`: five-card poker hands, 2-6-player Texas hold'em showdowns, complete
  bridge deals with conserved high-card points, blackjack initial deals, and
  cribbage hands and cribs with exact show-point breakdowns.
- `chance/dice`: seven mechanisms using compact roll notation.
- `abstract/impartial`: normal and misère Nim, normal Kayles, and finite
  subtraction positions. Nim and Kayles report exact outcomes and every winning
  move; only normal play reports Sprague–Grundy values.
- `puzzles`: unique Sudoku, nonograms, Kakuro, and irredundant Mastermind and
  logic-grid clue sets, plus solvable eight-puzzles and ASCII tree mazes from
  three spanning-tree algorithms.

Procedural leaves are original code and text, using only bounded local work and
one supplied RNG. They sample legal objects; Nim and Kayles have exact analyses.
Board-state sampling is biased by random trajectories; puzzle generation has no asserted
human difficulty level. Board positions retain move histories; puzzle outputs
show the puzzle directly. Card outputs do not model betting or bidding.

## Opening import

Run `python -B families/games/import_openings.py`. Development downloads go into
`target/games-import`; source TSVs and the full license are pinned to revision
`a6189a30dc273ccb21fc2536a9a2fefd5592a67a` (2026-10-06). Exact URLs are derived from
that revision in the importer; SHA-256 values and leaf counts are recorded in
`openings-source.json` and checked on repeat imports. All 21 previous opening
texts remain in the expanded dataset. Source row IDs are content-derived hashes
because upstream provides no stable row IDs. No source names or moves are edited.

The upstream [Lichess opening book](https://github.com/lichess-org/chess-openings)
is CC0; its complete notice is embedded from `board/chess/openings/LICENSE`.
There is no runtime network access. `tests/chess_openings.rs` uses test-only
shakmaty to replay captures, disambiguation, castling, promotions, en passant,
check/checkmate suffixes, and legality from the standard start position.

Grouping uses exact name prefixes plus ECO ranges. The broad `other-systems` and
`asymmetric-defenses` leaves collect closely related lines outside the named
major defenses, not unrelated games. Corpus leaves sample records uniformly;
branch probabilities are topic-balanced, so opening records do not all have
identical global probability.
