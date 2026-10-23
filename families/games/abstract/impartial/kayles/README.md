# Kayles

Normal-play rows of 1–24 pins, including single rows, paired losing sums, and
asymmetric winning sums. Every row's Sprague–Grundy value is computed by mex over
all one-pin and adjacent-two-pin removals, including splits. Row values combine
by XOR. Every winning removal is listed, preserving distinct mirrored moves and
identical rows. Bracketed pin indices run from 1 at each row's left end.

Tests independently solve all 4,096 subsets of twelve fixed pin sites directly,
without splitting or XOR, and compare both values and complete winning-move sets.
Misère Kayles is outside this leaf; normal values are never used for that game.

References: [Dudeney, *The Canterbury Puzzles*, problem 73](https://www.gutenberg.org/files/27635/27635-h/27635-h.htm),
and [Conway, *All Games Bright and Beautiful*](https://maths.dur.ac.uk/users/herbert.gangl/LIT/all_games.pdf),
the Kayles section with its split recurrence. Code and sampled positions are
original; no source prose or puzzles are embedded.
