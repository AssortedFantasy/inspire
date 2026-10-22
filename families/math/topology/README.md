# Topology and metric spaces

`metric-spaces/concepts` contains 11 explicit metrics, spaces and formulas.
`metric-spaces/weighted-tree-metrics` samples a labeled tree on 4–8 vertices
from a uniformly random Prüfer code and independently assigns lengths 1–9.
Conditional on vertex count, every labeled tree is equally likely. Outputs
contain the weighted edges and one of the distance matrix, diameter/radius/
centers, or an open and closed ball. Radius and centers refer to the vertex
metric, rather than the geometric realization with edge interiors.

Tests enumerate all 16 labeled four-vertex Prüfer codes, verify degrees and
connectivity, and compare 500 sampled distance matrices with independent
Floyd–Warshall calculations. They also check the metric axioms and tree
four-point condition.

`topological-spaces/concepts` contains six concrete spaces, covers and set
identities. All corpus records retain their IDs and source provenance.
`topological-spaces/finite-poset-topologies` samples partial orders on 3–5
points and takes their upper-set topologies. Outputs display either all open
sets or all minimal open neighborhoods. Half also include a subset with its
computed interior and closure. Generic separation and compactness statements
are omitted.

A random permutation orders the labels, each forward pair is included with
probability 1/3, and transitive closure completes the order. This reaches every
partial order, with nonuniform probabilities. At most 32 subsets are enumerated.
Tests check union/intersection closure, recover the order from neighborhoods,
and compare every subset's interior and closure with independent neighborhood
and closed-set calculations.

References: [Gómez and Mémoli, The Four Point Condition](https://arxiv.org/abs/2111.10328v3),
[J. P. May and Elle Pishevar, Finite Spaces and Larger Contexts, chapter 1](https://math.uchicago.edu/~may/PAPERS/FINITE.pdf).
The latter uses lower sets; reversing its order gives the upper-set convention
used by this generator. Existing corpus records retain their source provenance.
