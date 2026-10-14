"""Independent checks for graph invariants; run with Python's unittest."""

import itertools
import sys
import unittest

sys.dont_write_bytecode = True

from import_data import invariants


def brute_spanning_trees(n, edges):
    count = 0
    for candidate in itertools.combinations(edges, n - 1):
        parents = list(range(n))

        def root(v):
            while parents[v] != v:
                v = parents[v]
            return v

        acyclic = True
        for u, v in candidate:
            ru, rv = root(u), root(v)
            if ru == rv:
                acyclic = False
                break
            parents[ru] = rv
        count += acyclic
    return count


def brute_chromatic(n, edges):
    for k in range(1, n + 1):
        if any(
            all(colors[u] != colors[v] for u, v in edges)
            for colors in itertools.product(range(k), repeat=n)
        ):
            return k


class GraphInvariantTests(unittest.TestCase):
    def test_every_labeled_graph_on_four_vertices(self):
        possible = list(itertools.combinations(range(4), 2))
        for mask in range(1 << len(possible)):
            edges = [edge for i, edge in enumerate(possible) if mask & (1 << i)]
            result = invariants(4, edges)
            self.assertEqual(result[4], brute_chromatic(4, edges), edges)
            self.assertEqual(result[-1], brute_spanning_trees(4, edges), edges)

    def test_complete_graphs_and_cycles_have_known_values(self):
        for n in range(3, 8):
            result = invariants(n, list(itertools.combinations(range(n), 2)))
            self.assertEqual(result[4], n)
            self.assertEqual(result[7], 1)
            self.assertEqual(result[-1], n ** (n - 2))
            self.assertEqual(result[6], n * (n - 1) * (n - 2) // 6)
            cycle = [(i, i + 1) for i in range(n - 1)] + [(0, n - 1)]
            result = invariants(n, cycle)
            self.assertEqual(result[3], 1)
            self.assertEqual(result[4], 2 if n % 2 == 0 else 3)
            self.assertEqual(result[-1], n)
            self.assertEqual(result[7], n // 2)

    def test_disconnected_graph_and_empty_edge_set(self):
        result = invariants(7, [(0, 1), (1, 2), (0, 2), (3, 4)])
        self.assertEqual(result[2], [[0, 1, 2], [3, 4], [5], [6]])
        self.assertEqual(result[3], 1)
        self.assertEqual(result[4], 3)
        self.assertEqual(result[6], 1)
        self.assertEqual(result[7], 4)
        self.assertEqual(result[-1], 0)
        result = invariants(7, [])
        self.assertEqual(result[0], "forests")
        self.assertEqual(result[4], 1)
        self.assertEqual(result[7], 7)
        self.assertEqual(result[-1], 0)


if __name__ == "__main__":
    unittest.main()
