# Integer partitions

Choose n uniformly from 8 through 26, enumerate its partitions in decreasing
part order, and select one of five equally weighted views:

- Ferrers diagram and conjugate. Half of this view's selections are restricted
  to self-conjugate shapes, whose distinct odd diagonal hooks are also shown.
- The odd-parts/distinct-parts bijection, in either direction. Binary grouping
  of repeated odd parts is computed for the selected partition. Identity-only
  examples are excluded.
- Durfee decomposition into a square, right arm and lower leg, with exact area
  totals and the number of partitions at that Durfee side. The smallest and
  largest possible squares receive extra weight.
- Rank and conjugate rank, with exact counts N(n,r) and N(n,−r). Two thirds
  select ranks −3 through 3; one third select the two extreme ranks.
- A hook-length array and the exact standard-tableau count n!/∏h. These shapes
  fit within 10 rows and 10 columns.

Each view samples uniformly from its stated restriction after choosing its
parameters. There are at most p(26) = 2,436 candidate partitions. Computation
and recursion are bounded, all arithmetic is exact, and 26! fits in u128. Empty
arm/leg partitions use (). Output gives one focused view rather than every
invariant. Nothing is read from outside the supplied RNG.

Tests compare enumeration with coin-change dynamic programming, exhaustively
check conjugation and both restricted sets of the Glaisher bijection through
n=26, and verify Durfee counts independently from the coefficient of
x^(n−d²)/∏(1−x^i)² for i=1,…,d. Hook-length counts are checked against the
independent corner-removal recurrence through n=12 and for exact divisibility
through n=26.

Reference context: [Richard P. Stanley, *Enumerative Combinatorics*, vol. 1,
section 1.8](https://math.mit.edu/~rstan/ec/ec1.pdf), for partition diagrams,
conjugation and partition identities; [Freeman Dyson, *Some Guesses in the
Theory of Partitions*](https://qseries.org/fgarvan/references/dyson/dyson-1944.pdf), for rank;
and [Frame, Robinson and Thrall, *The Hook Graphs of the Symmetric
Group*](https://doi.org/10.4153/CJM-1954-030-1), for the hook-length formula.
No source prose or external dataset is embedded.
