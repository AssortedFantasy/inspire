# Execution generators

The generated arrays, strings, graphs and numerical tables are original
implementations. They use only the supplied RNG and bounded local computation.
Existing graph, alignment, dynamic programming, insertion, selection and merge
leaves retain their own tests. The following additions cover contrasting
execution shapes rather than changing only the input magnitudes.

## Sorting

`sequences/sorting/quick-sort/trace` chooses between last-pivot Lomuto partition
(`<=` on the left) and a three-way partition around the same last-element pivot.
The inputs mix ordinary keys, ascending and descending runs, all-equal keys,
three-key duplicates and organ-pipe shapes. A trace lists each subrange, pivot value and placed pivot-block bounds. Lomuto's middle block contains the
single placed pivot; equal keys may remain on its left. A key comparison counts
one `<=` or one three-way comparison, including the pivot's own inspection in
the three-way variant. No shuffle or insertion-sort cutoff hides the deliberately
quadratic ordered and equal-key Lomuto cases. Inputs have at most eleven items.

`sequences/sorting/radix-sort/trace` uses stable LSD buckets in base 2, 10 or 16,
with two or three unsigned digits. Identity letters expose stability across
passes and equal complete keys. Inputs include ordinary mixed digits, leading
zero columns, a shared low digit, duplicate keys and values at digit boundaries.
Only occupied buckets are printed, using identity letters to avoid repeating
complete keys. The final sorted keys are shown once; every pass is performed.

Quicksort tests enumerate all ternary arrays through length seven and compare
the result with an independent sort. Every partition preserves its subrange's
multiset and the exterior, satisfies its left/middle/right relations, and shrinks
both recursive intervals. The equal-key fixture checks the 45-versus-10 comparison
boundary for ten items. Radix tests compare every pass with a stable sort by all
processed digits and check each bucket against the previous pass's source order.

References: Sedgewick and Wayne's
[Quicksort](https://algs4.cs.princeton.edu/23quicksort/) and
[String Sorts](https://algs4.cs.princeton.edu/51radix/). These describe the
partition and LSD principles; this generator's Lomuto convention, unshuffled
inputs and small integer radices are explicit local choices. No implementation
code is copied.

The other sequence leaves show one optimal edit alignment and the knapsack
capacity/value frontier at thresholds where the optimum changes. Dijkstra
prints distances and a predecessor tree rather than repeating path prefixes.

## String search and coding

`sequences/search/knuth-morris-pratt` emits a nonempty pattern, text, longest
proper-prefix/suffix table, and the search state after every text character.
Fallback lengths and overlapping match starts remain visible. The five regimes
are ordinary random text with optional planted matches, single-symbol runs,
periodic patterns, nested borders and a repeated-prefix near miss with no match.
All symbols are ASCII DNA letters, so positions count both bytes and characters.
Exhaustive binary patterns through length five and texts through length seven
are checked against naive window matching and brute-force proper borders. Each
resume state is independently checked as the longest proper pattern prefix
that is a suffix of the consumed text.

`sequences/coding/huffman` constructs a binary symbol code from the printed
message's exact frequencies. Ordinary, equal, dominant-symbol, Fibonacci and
near-uniform counts give different tree depths and ties. Queue ties use the
lexicographically sorted symbol sets. The first removed node gets bit zero.
The merge trace, complete codebook and encoded payload are emitted. The
fixed-width comparison uses `ceil(log2(alphabet size))` bits per symbol; both
counts exclude framing, transmitted codebooks and byte padding. Alphabet sizes
are four through seven. Tests decode the payload, verify exact counts, prefix
freedom and Kraft equality, and compare weighted length with an independent
dynamic program over every binary partition of every symbol subset.

References: Victor Adamchik's Carnegie Mellon
[String Matching lecture](https://www.cs.cmu.edu/~15451-s15/LectureNotes/lecture24.pdf)
for KMP failure links, and Aarti Singh's
[Source Coding and Huffman lecture](https://www.cs.cmu.edu/~aarti/Class/10704/lec8-srccodingHuffman.pdf)
for prefix codes and optimal weighted length. These are conceptual references;
the generated data and code are original.

## Numerical methods

`numerical/newton` samples six distinct regimes: positive square roots and
`cos(x)-x` with fast local convergence, roots of multiplicity two through five
with linear convergence, the exact `0 ↔ 1` cycle of `x³-2x+2`, a nonroot with
zero derivative for `x³-a`, and a divergent cube-root orbit. Newton updates are
undamped. The latter obeys `x[n+1] = -2x[n]` away from zero. Each row shows the
iterate and residual, with a separate reference root above the table.
Square roots and cube roots use library references; the cosine and cycle
polynomial reference roots are fixed double-precision constants. Successful
stopping requires residual at most `1e-10` and a step at most
`1e-10(1+|x|)` (or exact zero residual). Other exits identify a zero derivative,
an exact two-cycle or the eight/twelve-update limit. Multiple-root samples reach
the limit while still converging linearly; they are not called divergent.

Tests compare derivatives with symmetric finite differences and compare the
square-root, multiple-root and divergent trajectories with independent closed
recurrences. They check every constructed regime's termination and root error,
including the two-cycle's exact orbit and the nonzero stationary residual.

`numerical/quadrature` computes composite Simpson and trapezoid results on
`[0,1]` for 2, 4, 8, 16, 32 and 64 equal panels. The seven regimes are a cubic,
`x⁴`, an exponential, a narrow Lorentzian peak, an interior absolute-value kink,
`sqrt(x)` and an integer-frequency cosine plus one. Reference integrals are
analytic expressions evaluated in `f64`. Cubics expose Simpson exactness;
quartics expose its factor-of-sixteen error decrease. Kinks and endpoint
singularities violate the smooth fourth-derivative error assumptions. Narrow
peaks can be missed or overestimated; oscillations can produce a false plateau
before refinement resolves them. Every row is computed even when two successive
answers agree. Ratios use absolute reference errors and are suppressed once
either error is below `1e-12`, where rounding can dominate. Displayed references
are rounded; errors use their full floating-point values.

Tests integrate polynomial moments on several intervals, verify the quartic's
analytic Simpson error, compare reference integrals with an independent fine
midpoint sum, and check fourth-order, endpoint-singular and kink error regimes.
An aliasing fixture checks that three agreeing coarse meshes can miss a unit of
integral. No adaptive error guarantee is claimed for these fixed meshes.

References: UBC's [Newton's Method](https://personal.math.ubc.ca/~CLP/CLP1/clp_1_dc/sec_C_1.html),
[Newton-Raphson solutions](https://www.math.ubc.ca/~anstee/math104/104newton-solution.pdf)
and [Numerical Integration](https://personal.math.ubc.ca/~CLP/CLP2/clp_2_ic/sec_numeric_int.html).
The nonconvergent and nonsmooth examples are deliberately constructed from
their displayed formulas.

## Replay

The algorithm sibling weights are graphs 3, sequences 3, optimization 2 and
numerical 2. New leaves retain default weights within their branches.

During development the pre-expansion executable was sampled with seeds
`260422` (stable merge), `271828` (lower-bound search) and `314159` (edit
alignment). Disjoint merge runs encouraged explicit partition shapes and radix
identities; an absent search key encouraged KMP near misses; an insertion
alignment encouraged preserving per-character state rather than emitting only
the final answer. These samples affected the implementation choices, not the
generator RNG state.

```powershell
inspire --seed 260422 --family procedures/algorithms/sequences/sorting/quick-sort/trace
inspire --seed 271828 --family procedures/algorithms/sequences/search/knuth-morris-pratt
inspire --seed 314159 --family procedures/algorithms/sequences/coding/huffman
inspire --seed 0 --family procedures/algorithms/numerical/newton
inspire --seed 1 --family procedures/algorithms/numerical/quadrature
```
