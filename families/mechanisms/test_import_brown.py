"""Focused checks for exact, sentence-bounded Brown source extraction."""

import unittest

from import_brown import Paragraphs, excerpt


class ExcerptTests(unittest.TestCase):
    def selection(self, **overrides):
        return {"number": 7, "start": None, "end": None, **overrides}

    def test_preserves_human_wording_across_inline_markup(self):
        parser = Paragraphs()
        parser.feed("<p>7. The <em>pawl</em> rises.\n The wheel turns.</p>")
        self.assertEqual(
            excerpt(parser.paragraphs, self.selection(start="The wheel")),
            "The wheel turns.",
        )

    def test_paired_movement_and_complete_operative_excerpt(self):
        self.assertEqual(
            excerpt(
                ["7 and 8. Two views of a wheel. The pawl rises. A tooth is shown."],
                self.selection(
                    number_label="7 and 8", start="The pawl", end=" A tooth"
                ),
            ),
            "The pawl rises.",
        )

    def test_accepts_independent_clause_after_brown_introduction(self):
        self.assertEqual(
            excerpt(
                ["7. The action is as follows:—The wheel turns."],
                self.selection(start="The wheel"),
            ),
            "The wheel turns.",
        )

    def test_rejects_partial_and_ambiguous_boundaries(self):
        cases = [
            ("7. The pawl raises the wheel.", {"start": "the wheel"}),
            ("7. The pawl raises the wheel.", {"end": " the wheel"}),
            ("7. The wheel turns. The wheel turns.", {"start": "The wheel"}),
            ("7. The wheel turns.", {"start": "The pawl"}),
        ]
        for paragraph, boundaries in cases:
            with self.subTest(boundaries=boundaries), self.assertRaises(ValueError):
                excerpt([paragraph], self.selection(**boundaries))


if __name__ == "__main__":
    unittest.main()
