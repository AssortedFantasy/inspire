# Nim

Normal and misère positions with exact outcome classes and all winning heap
reductions. Only normal play reports a Sprague–Grundy nim-value. Misère samples
include all-singleton endgames and the transition through exactly one larger
heap. Ordinary samples deliberately include zero-XOR positions. Heap identities
remain visible so identical-sized heaps still have distinct listed moves.

The normal XOR result and misère endgame exception follow
[C. L. Bouton's original paper](https://www.paradise.caltech.edu/ist4/lectures/Bouton1901.pdf),
especially its final section. Tests compare both outcome formulas and the full
winning-move lists against independent recursive game-tree evaluation for every
four-heap position with sizes 0–6. Terminal positions are tested but not sampled.
