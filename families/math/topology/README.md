# Topology and metric spaces

`metric-spaces` keeps 11 metric concepts and examples together and adds
positive weighted tree metrics on 4–7 vertices. Each metric output includes the
tree, full distance matrix and diameter witness. Trees are sampled by attaching
each new vertex to a randomly selected earlier vertex, with edge lengths 1–9;
this is a bounded recursive-tree distribution, not uniform over all trees.
The support contains 389,885,238 distinct labeled weighted trees:
`sum((n-1)! * 9^(n-1), n=4..7)`.
Tests independently run Floyd–Warshall and check every metric axiom and the
four-point condition across 500 samples. A separate 500-seed test parses the
printed edges, matrix and diameter witness and checks them against independent
shortest-path calculations. Reference context for the four-point condition:
[Gómez and Mémoli, The Four Point Condition: An Elementary Tropicalization of
Ptolemy's Inequality, version 3 (2023-02-05)](https://arxiv.org/abs/2111.10328v3).

`topological-spaces` keeps compactness, density, connectedness and associated
counterexamples together and adds finite topologies from partial orders on 3–5
points. Outputs contain covering relations, every open upper set and minimal
neighborhoods, with the actual T1/Hausdorff/discrete status. A random permutation
orders the labels; each forward pair is included independently with probability
1/3, then the relation is transitively closed. This reaches every partial order
on the selected labels, but does not sample partial orders uniformly. Enumeration
of at most 32 subsets is bounded.
Tests check union/intersection closure and independently recover the order from
the emitted neighborhoods across 500 samples. The upper-set convention is
explicit; its order dual gives the other common convention.
Reference: [J. P. May and Elle Pishevar, Finite Spaces and Larger Contexts,
chapter 1, especially §1.6](https://math.uchicago.edu/~may/PAPERS/FINITE.pdf).
That source uses lower sets; reversing its order gives the upper-set convention
used here.

Procedural code and rendered explanations are original. Existing concept records
retain their IDs and reference provenance after moving from the former mixed
`math/definitions`, `math/constructions` and `math/counterexamples` datasets.

The metric corpus also covers induced topology, total boundedness, product
metrics and the distinction between topology and completeness. Original
source-grounded explanations were checked against [Lebl's metric topology
section](https://www.jirka.org/ra/html/sec_mettop.html), [Eck's completeness and
completion chapter](https://math.hws.edu/eck/metric-spaces/completeness.html), and
[Addington's metric spaces and topology notes](https://pages.uoregon.edu/adding/courses/431/431notes.pdf).
These are reference checks, not imported datasets; no source prose is reproduced.
