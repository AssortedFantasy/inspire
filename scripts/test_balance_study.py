"""Small enumerable populations check the balance study's estimators and locks."""

from collections import Counter
from fractions import Fraction
from itertools import combinations, product
import unittest

import numpy as np
from scipy import sparse

from balance_experiments import groups, optimize
from balance_geometry import corrected_diagonal
from balance_sampling import collision_statistics, merge_corpus_record, mixture_collision


class BalanceStudyTests(unittest.TestCase):
    def test_exact_identity_merges_cross_leaf_duplicates(self):
        exact = {}
        record = {"id": "record", "text": "same text"}
        merge_corpus_record(exact, "first", 0, record, Fraction(1, 5))
        merge_corpus_record(exact, "second", 0, record, Fraction(3, 10))
        self.assertEqual(len(exact), 1)
        self.assertEqual(len(next(iter(exact.values()))["members"]), 2)
        self.assertAlmostEqual(mixture_collision(
            {key: row["probability"] for key, row in exact.items()}, {}, {}), .25)

    def test_generator_pair_estimator_does_not_count_observation_self_matches(self):
        stats = collision_statistics(Counter({"a": 1, "b": 1, "c": 1}))
        self.assertEqual(stats["self_collision_estimate"], 0)
        self.assertAlmostEqual(stats["empirical_self_collision"], 1 / 3)
        self.assertAlmostEqual(collision_statistics(Counter({"a": 3, "b": 1}))[
            "self_collision_estimate"], .5)

    def test_cross_generator_and_corpus_matches_survive_diagonal_correction(self):
        # Each constant generator has collision 1; all three sources print x.
        self.assertAlmostEqual(mixture_collision(
            {"x": .2}, {"a": Counter(x=4), "b": Counter(x=4)},
            {"a": .3, "b": .5}), 1)

    def test_finite_corpus_correction_averages_to_known_population_kernel(self):
        vectors = np.array([[1., 0], [0, 1], [-1, 0], [2**-.5, 2**-.5]])
        kernel = np.exp(-(2 - 2 * vectors @ vectors.T) / 2)
        estimates = []
        for subset in combinations(range(4), 2):
            local = kernel[np.ix_(subset, subset)].mean()
            rows = [{"conditional_weight": .5, "kind": "corpus"}] * 2
            estimates.append(corrected_diagonal(
                np.array([[local]]), rows, {"leaf": [0, 1]}, ["leaf"],
                {"leaf": [0, 1]}, {"leaf": 4})[0, 0])
        self.assertAlmostEqual(np.mean(estimates), kernel.mean(), places=12)
        rows = [{"conditional_weight": .25, "kind": "corpus"}] * 4
        value = corrected_diagonal(np.array([[kernel.mean()]]), rows,
                                   {"leaf": list(range(4))}, ["leaf"],
                                   {"leaf": list(range(4))}, {"leaf": 4})
        self.assertAlmostEqual(value[0, 0], kernel.mean(), places=12)

    def test_generator_correction_averages_over_independent_draws(self):
        kernel = np.array([[1., .3], [.3, 1.]])
        estimates = []
        for draws in product(range(2), repeat=2):
            local = kernel[np.ix_(draws, draws)].mean()
            rows = [{"conditional_weight": .5, "kind": "generator"}] * 2
            estimates.append(corrected_diagonal(
                np.array([[local]]), rows, {"leaf": [0, 1]}, ["leaf"],
                {"leaf": [0, 1]}, {})[0, 0])
        self.assertAlmostEqual(np.mean(estimates), kernel.mean(), places=12)

    def test_homogeneous_lock_preserves_equal_record_exposure(self):
        leaves = [{"path": "people/a", "probability": .1},
                  {"path": "people/b", "probability": .2},
                  {"path": "other", "probability": .7}]
        baseline, prior, transform, names = groups(leaves)
        np.testing.assert_allclose(transform @ prior, baseline)
        np.testing.assert_allclose(transform.sum(axis=0), 1)
        changed = np.array([.6 if name == "people" else .4 for name in names])
        probabilities = transform @ changed
        self.assertAlmostEqual(probabilities.sum(), 1)
        self.assertAlmostEqual(probabilities[0] / 1, probabilities[1] / 2)

    def test_optimizer_matches_known_solution_and_enforces_atom_cap(self):
        matrix = np.diag([1., 4.])
        prior = np.array([.5, .5])
        weights, trace = optimize(matrix, prior)
        self.assertTrue(all(step["success"] for step in trace))
        np.testing.assert_allclose(weights, [.8, .2], atol=1e-6)
        capped, trace = optimize(matrix, prior, exposure_matrix=sparse.eye(2).tocsr(),
                                 exposure_cap=.55)
        self.assertTrue(all(step["success"] for step in trace))
        np.testing.assert_allclose(capped, [.55, .45], atol=1e-6)

    def test_minimum_kl_cap_preserves_ratios_outside_constrained_atom(self):
        weights, trace = optimize(np.zeros((3, 3)), np.array([.2, .3, .5]), kl=1.,
                                  exposure_matrix=sparse.csr_matrix([[1., 0, 0]]),
                                  exposure_cap=.1)
        self.assertTrue(all(step["success"] for step in trace))
        np.testing.assert_allclose(weights, [.1, .3375, .5625], atol=1e-10)
        with self.assertRaises(ValueError):
            optimize(np.zeros((3, 3)), np.array([.2, .3, .5]), kl=1.,
                     exposure_matrix=sparse.eye(3).tocsr(), exposure_cap=.2)

    def test_duplicate_leaf_features_do_not_override_prior_under_kl(self):
        # All leaves represent the same constant output: the kernel objective
        # is constant on the simplex, regardless of how that output is split.
        prior = np.array([.2, .3, .5])
        weights, trace = optimize(np.ones((3, 3)), prior, kl=.1)
        self.assertTrue(all(step["success"] for step in trace))
        # Conic objective tolerances allow small mass differences on flat faces.
        np.testing.assert_allclose(weights, prior, atol=1e-4)


if __name__ == "__main__":
    unittest.main()
