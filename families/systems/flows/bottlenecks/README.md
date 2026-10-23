# Capacity bottlenecks

Small directed acyclic networks with integer capacities. Samples mix a shared
bridge, parallel routes, layered cross-routing, split/merge networks, and irregular
networks. A cross-link regime samples just below, at, and above the capacity that
moves the minimum cut.

One source and sink, conserved steady flow, no edge costs or losses. Capacities and
flows are in units per hour. Graphs have at most eight vertices and no antiparallel
edges. Edmonds–Karp gives an integer maximum flow and a residual-reachability
minimum cut. Enumerating all source/sink partitions counts tied minimum cuts.

Each sample also upgrades one existing edge. Every candidate is re-solved before
choosing either an effective or ineffective upgrade when those outcomes exist.

Tests exhaustively certify max-flow/min-cut equality, capacity bounds, conservation,
and every one-unit upgrade. A separate network forces reversal of an earlier
augmentation through a reverse residual arc.

Source: MIT 6.854,
[Lecture 6: Network Flows](https://courses.csail.mit.edu/6.854/19/Scribe/s6-maxflow/s6-maxflow.html),
flow/cut definitions, residual networks, max-flow/min-cut theorem, and shortest
augmenting paths. Networks and capacities are generated locally.
