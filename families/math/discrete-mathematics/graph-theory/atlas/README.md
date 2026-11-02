# Small graph atlas

Three datasets contain 1,249 nonisomorphic simple undirected graphs on three
through seven vertices, imported from NetworkX 3.4.2's graph atlas. Stable IDs
retain the atlas graph number. The four graphs on fewer than three vertices are
omitted. Vertices retain source labels; an edge `u-v` is undirected.

- `forests`: 76 acyclic graphs, including trees and disconnected forests.
- `bipartite-cyclic`: 70 bipartite graphs containing cycles.
- `nonbipartite`: 1,103 graphs requiring at least three colors.

Leaves have equal local weight to balance graph classes rather than uniform
selection over the atlas. Every graph includes its vertex and edge sets. Three
compact views rotate by stable atlas number: sorted degree sequence/components/cycle rank,
minimum coloring/maximum independent set, or triangles/spanning trees/independence
number. The importer computes all invariants for every graph and retains its
independent checks. Spanning-tree counts are zero for disconnected graphs.

Source: [NetworkX 3.4.2 atlas.dat.gz](https://raw.githubusercontent.com/networkx/networkx/networkx-3.4.2/networkx/generators/atlas.dat.gz),
SHA256 `73fc416df0164923607751cb759f4ae81deb5f6550bf25be59c86de3b747e41d`.
[NetworkX's atlas documentation](https://networkx.org/documentation/stable/reference/generated/networkx.generators.atlas.graph_atlas_g.html)
describes the stable ordering and known ordering exceptions. NetworkX is
BSD-3-Clause; the full pinned notice is in [LICENSE.txt](LICENSE.txt) and must
accompany binary distributions. Source identifiers and attribution stay in
record IDs and provenance.

The standard-library importer downloads only at development time or reads a
cached input; runtime remains offline. It computes invariants independently,
using exhaustive coloring and independent-set searches, BFS components, and
the matrix-tree theorem with fraction-free determinant elimination. It checks
the source checksum and graph numbering, edge validity, the degree-sum identity,
coloring witnesses, cycle rank, and forest/bipartite/spanning-tree implications.

```sh
python -B families/math/discrete-mathematics/graph-theory/atlas/import_data.py --input PATH/atlas.dat.gz
python -B families/math/discrete-mathematics/graph-theory/atlas/import_data.py --input PATH/atlas.dat.gz --check
python -B families/math/discrete-mathematics/graph-theory/atlas/test_import_data.py
```

Raw downloads belong outside `families/`. Generated records are committed;
`--check` reproduces every record without modifying them.
