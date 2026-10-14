"""Import pinned NetworkX graph atlas; standard library only, no runtime dependency."""

import argparse
import gzip
import hashlib
import itertools
import json
from pathlib import Path
import urllib.request

BASE = "https://raw.githubusercontent.com/networkx/networkx/networkx-3.4.2/"
URL = BASE + "networkx/generators/atlas.dat.gz"
SHA256 = "73fc416df0164923607751cb759f4ae81deb5f6550bf25be59c86de3b747e41d"
ROOT = Path(__file__).resolve().parent


def components(adj):
    remaining = set(range(len(adj)))
    result = []
    while remaining:
        reached = {min(remaining)}
        frontier = list(reached)
        while frontier:
            u = frontier.pop()
            for v in adj[u] - reached:
                reached.add(v)
                frontier.append(v)
        remaining -= reached
        result.append(sorted(reached))
    return result


def coloring(adj, k):
    order = sorted(range(len(adj)), key=lambda v: -len(adj[v]))
    colors = [-1] * len(adj)

    def visit(i):
        if i == len(order):
            return True
        v = order[i]
        forbidden = {colors[u] for u in adj[v]}
        used = max(colors, default=-1)
        for c in range(min(k, used + 2)):
            if c not in forbidden:
                colors[v] = c
                if visit(i + 1):
                    return True
                colors[v] = -1
        return False

    return colors if visit(0) else None


def determinant(matrix):
    # Fraction-free Bareiss elimination, with row pivoting.
    a = [row[:] for row in matrix]
    sign, previous = 1, 1
    if not a:
        return 1
    for i in range(len(a) - 1):
        if not a[i][i]:
            pivot = next((j for j in range(i + 1, len(a)) if a[j][i]), None)
            if pivot is None:
                return 0
            a[i], a[pivot] = a[pivot], a[i]
            sign *= -1
        p = a[i][i]
        for j in range(i + 1, len(a)):
            for k in range(i + 1, len(a)):
                numerator = p * a[j][k] - a[j][i] * a[i][k]
                assert numerator % previous == 0
                a[j][k] = numerator // previous
        previous = p
        for j in range(i + 1, len(a)):
            a[j][i] = 0
    return sign * a[-1][-1]


def invariants(n, edges):
    adj = [set() for _ in range(n)]
    assert len(set(edges)) == len(edges)
    for u, v in edges:
        assert 0 <= u < v < n
        adj[u].add(v)
        adj[v].add(u)
    parts = components(adj)
    rank = len(edges) - n + len(parts)
    chi = next(k for k in range(1, n + 1) if coloring(adj, k) is not None)
    colors = coloring(adj, chi)
    assert all(colors[u] != colors[v] for u, v in edges)
    degree = sorted(map(len, adj), reverse=True)
    assert sum(degree) == 2 * len(edges)
    triangles = sum(
        b in adj[a] and c in adj[a] and c in adj[b]
        for a, b, c in itertools.combinations(range(n), 3)
    )
    independent_set = max(
        (subset
        for size in range(n + 1)
        for subset in itertools.combinations(range(n), size)
        if all(v not in adj[u] for u, v in itertools.combinations(subset, 2))),
        key=len,
    )
    independent = len(independent_set)
    laplacian = [
        [len(adj[i]) if i == j else -int(j in adj[i]) for j in range(n - 1)]
        for i in range(n - 1)
    ]
    spanning = determinant(laplacian)
    assert (spanning > 0) == (len(parts) == 1)
    if rank == 0:
        assert chi <= 2 and triangles == 0
        assert spanning == int(len(parts) == 1)
    elif chi <= 2:
        assert triangles == 0
    category = "forests" if rank == 0 else "bipartite-cyclic" if chi <= 2 else "nonbipartite"
    return category, degree, parts, rank, chi, colors, triangles, independent, independent_set, spanning


def parse(raw):
    graphs = []
    current = None
    for line in gzip.decompress(raw).decode("ascii").splitlines():
        words = line.split()
        if words[0] == "GRAPH":
            current = [int(words[1]), None, []]
            graphs.append(current)
        elif words[0] == "NODES":
            current[1] = int(words[1])
        else:
            u, v = map(int, words)
            current[2].append((min(u, v), max(u, v)))
    assert len(graphs) == 1253
    assert [g[0] for g in graphs] == list(range(1253))
    return graphs


def import_records(raw):
    assert hashlib.sha256(raw).hexdigest() == SHA256, "source checksum changed"
    output = {key: [] for key in ("forests", "bipartite-cyclic", "nonbipartite")}
    for index, n, edges in parse(raw):
        # Omit four trivial objects of orders zero, one, and two.
        if n < 3:
            continue
        category, degree, parts, rank, chi, colors, triangles, independent, independent_set, spanning = invariants(n, edges)
        edge_text = ", ".join(f"{u}-{v}" for u, v in edges) or "none"
        color_classes = [[v for v, c in enumerate(colors) if c == color] for color in range(chi)]
        text = (
            f"Simple undirected graph with vertices 0..{n - 1}.\n"
            f"Edges: {edge_text}.\n"
            f"Vertices: {n}; edges: {len(edges)}; degrees (descending): {degree}.\n"
            f"Connected components: {parts}; cycle-space dimension over F_2: {rank}.\n"
            f"Chromatic number: {chi}; a minimum coloring has color classes {color_classes}.\n"
            f"Triangles: {triangles}; independence number: {independent}, with maximum independent set {list(independent_set)}; spanning trees: {spanning}."
        )
        output[category].append({
            "id": f"networkx-3.4.2-G{index}",
            "text": text,
            "provenance": (
                f"NetworkX 3.4.2 atlas.dat.gz, graph G{index}; {URL}; SHA256 {SHA256}. "
                "BSD-3-Clause, Copyright (C) 2004-2024 NetworkX Developers; full license in "
                "../LICENSE.txt. Original invariant calculations and rendering by Inspire importer."
            ),
        })
    assert sum(map(len, output.values())) == 1249
    return output


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, help="cached atlas.dat.gz outside sampling tree")
    parser.add_argument("--check", action="store_true", help="compare current records without writing")
    args = parser.parse_args()
    raw = args.input.read_bytes() if args.input else urllib.request.urlopen(URL).read()
    for category, records in import_records(raw).items():
        path = ROOT / category / "records.json"
        if args.check:
            assert json.loads(path.read_text(encoding="utf-8")) == records, path
        else:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(json.dumps(records, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        print(f"{category}: {len(records)} graphs")


if __name__ == "__main__":
    main()
