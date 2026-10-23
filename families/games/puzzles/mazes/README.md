# Mazes

Original ASCII mazes from randomized depth-first backtracking, frontier-edge
Prim, and Kruskal with disjoint sets. Each is a connected spanning tree, so the
marked entrance and exit have exactly one route. Long corridors, bushy branching,
wide rooms, tall rooms, and narrow strips mix different textures. Three quarters
of entrances maximize distance between perimeter cells; the rest cross randomly
chosen opposite sides. The summary counts cell-to-cell steps and interior dead
ends, excluding the two cells opened to the exterior.

All work is bounded by grids of at most 140 cells and finite graph traversals.
Tests check adjacency, reciprocal passages, connectivity, edge counts, rendered
walls, and the two exterior openings. No uniform spanning-tree distribution or
human difficulty rating is claimed.

Algorithm references: Jamis Buck's [recursive backtracking](https://weblog.jamisbuck.org/2010/12/27/maze-generation-recursive-backtracking),
[randomized Kruskal](https://weblog.jamisbuck.org/2011/1/3/maze-generation-kruskal-s-algorithm),
and [randomized Prim](https://weblog.jamisbuck.org/2011/1/10/maze-generation-prim-s-algorithm).
Code and output are original; no source prose or implementation is embedded.
