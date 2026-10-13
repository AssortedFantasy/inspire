# Chess openings

Generates a curated, varied set of short chess opening lines. Each result names
the opening, gives its ECO code, and prints the complete line in numbered SAN.
The line is never cut short, so its label describes the position reached. This
is an opening-line sampler, not a chess engine or a claim that every move is
best.

Each entry reproduces an exact name, ECO code, and PGN endpoint from the
[Lichess chess-openings dataset](https://github.com/lichess-org/chess-openings),
which describes PGN as a known or commonly played move sequence and releases
the collection under [CC0](https://creativecommons.org/publicdomain/zero/1.0/).
This is a small hand-curated subset. Move notation follows the algebraic system
described in the [2023 FIDE Laws of Chess](https://handbook.fide.com/chapter/E012023).

The generator uses only the supplied seeded RNG and has no runtime data or
network dependency. Its tests replay every stored move from the standard start
position and verify that generated labels match the full sampled line.
