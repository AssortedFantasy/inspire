# Capacity bottlenecks

Small directed acyclic networks with integer capacities. Samples mix a shared
bridge, parallel routes, layered cross-routing, split/merge networks, and irregular
networks. A cross-link regime samples just below, at, and above the capacity that
moves the minimum cut.

One source and sink, conserved steady flow, no edge costs or losses. Capacities and
flows are in units per hour. Graphs have at most eight vertices and no antiparallel
edges. Edmonds–Karp gives an integer maximum flow and a residual-reachability
minimum cut. Enumerating all source/sink partitions counts tied minimum cuts.

Each sample also upgrades one existing edge. Every candidate is re-solved;
selection favors effective upgrades with probability 3/4 when available. The
remaining probes mix nonbinding edges and cut edges blocked by a competing
minimum cut. Blocked cut-edge probes show the unchanged competing cut. An
effective upgrade can still run into another bottleneck before using all of its
added capacity.

Tests exhaustively certify max-flow/min-cut equality, capacity bounds, conservation,
and every one-unit upgrade. Selected interventions are checked independently
against all cuts, including tied and partially useful upgrades. A separate
network forces reversal of an earlier augmentation through a reverse residual arc.

Source: MIT 6.854,
[Lecture 6: Network Flows](https://courses.csail.mit.edu/6.854/19/Scribe/s6-maxflow/s6-maxflow.html),
flow/cut definitions, residual networks, max-flow/min-cut theorem, and shortest
augmenting paths. Networks and capacities are generated locally.
