# Subtraction games

Original normal-play one-heap games with 2-4 distinct allowed removals from 1-7
and 8-40 tokens. Dynamic programming marks a position winning exactly when a
legal removal reaches a losing position. No legal move loses, including nonempty
heaps smaller than every allowed removal. Winning and losing positions receive
equal probability. Output includes the N/P outcome, every winning removal, and
the eventual N/P cycle with its starting heap and period.

Cycles are proved by repeating a complete window of the last m outcomes,
where m is the largest removal. Once every removal is legal, this window
determines the next outcome. There are at most 2^m states (128 here), so cycle
detection terminates without guessing a period from a finite prefix. The printed
cycle starts at the first repeated state's original window.

Tests exhaust all 91 allowed removal sets, compare repeated full states and
4,097 computed outcomes, check every winning removal through heap 40, and
verify both sampled outcome classes are available.
