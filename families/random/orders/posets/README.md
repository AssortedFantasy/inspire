# Finite orders

Eight equally likely constructions: a chain, an antichain, the three-dimensional
Boolean lattice, a six- or eight-element crown, the four-element N order,
recursive series/parallel composition, an alternating fence, and sparse/dense
acyclic relations followed by transitive closure. Other orders have four to
eight elements. Labels are shuffled after construction. This is not uniform
sampling of isomorphism classes; chains, antichains and the fixed Boolean/N
orders deliberately recur as boundary cases.

Outputs give the strict covering relation, an exact count of linear extensions,
the antichain count at every size starting at zero, and every maximum-cardinality
antichain. The empty antichain is included in the count. Maximum does not mean
merely inclusion-maximal. Counts use subset dynamic programming and exhaustive
subset enumeration; there is no approximate or Monte Carlo counting.

This leaf exposes order combinatorics, separate from the neighborhood and open-set
views in `math/topology/topological-spaces/finite-poset-topologies`.

Reference: Kangas, Hankala, Niinimäki and Koivisto,
[Counting Linear Extensions of Sparse Posets](https://www.cs.helsinki.fi/u/mkhkoivi/publications/ijcai-2016.pdf).
Tests reconstruct each order from its covers, compare extension counts with
complete permutation enumeration, and compare antichains with the maximal-element
images of all order ideals. Boolean-lattice and antichain counts are checked
against exact standard values.
