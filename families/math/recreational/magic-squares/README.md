# Normal magic squares

Four equally sampled constructions:

- Siamese odd squares of orders 3, 5, 7, 9 and 11.
- Diagonal-complement doubly even squares of orders 4, 8 and 12.
- Strachey quadrant exchanges for singly even orders 6 and 10.
- Affine pandiagonal squares of prime orders 5, 7 and 11. Two distinct slopes
  from 2 through n−2 give orthogonal digit arrays; every toroidal diagonal has
  the magic sum.

Ordinary squares receive a simultaneous permutation of their rows and columns
that commutes with reversal. This preserves both diagonals and provides more
arrangements than rotations alone. All squares may be rotated, reflected and
complemented. Pandiagonal squares retain their cyclic row/column order.

Outputs contain one normal square on 1 through n² and its exact magic sum.
Generation is bounded by a 12×12 grid and uses only the supplied RNG. These are
construction families, not uniform samples of all normal magic squares. The
singly even construction is Strachey's method, not the backlog's proposed LUX
method.

Tests independently check every row, column and principal diagonal, and the
exact occurrence of 1 through n², for each construction order. All affine
parameters receive exhaustive broken-diagonal checks. Transformations are
checked separately, including preservation of pandiagonality.

Primary reference context: [Manjunath and Ambikasaran, *Spectrum of MATLAB's
magic squares*](https://arxiv.org/html/2205.15563v1), sections 4–6, and
[Ball, *Mathematical Recreations and Essays* (1922), p. 140](https://en.wikisource.org/wiki/Page:Mathematical_recreations_and_essays_(1922,_IA_cu31924001539562).djvu/160),
which records Strachey's construction communicated to the author. The affine
regime is implemented directly from two orthogonal linear forms over F_n.
No source prose or external data is embedded.
