import unittest

from creative_render import extract_svg, validate_svg


class SvgValidationTests(unittest.TestCase):
    def test_local_references_and_gradients_are_unchanged(self):
        svg = '<svg xmlns="http://www.w3.org/2000/svg"><defs><linearGradient id="g"/></defs><rect fill="url(#g)"/></svg>'
        self.assertEqual(extract_svg('```svg\n' + svg + '\n```'), svg)
        validate_svg(svg)

    def test_external_content_is_rejected(self):
        for content in ['<script/>', '<rect onclick="go()"/>', '<use href="https://example.org/a.svg"/>',
                        '<style>rect {fill:url(https://example.org/a.svg)}</style>', '<foreignObject/>']:
            with self.subTest(content=content), self.assertRaises(ValueError):
                validate_svg('<svg xmlns="http://www.w3.org/2000/svg">' + content + '</svg>')

    def test_does_not_repair_malformed_output(self):
        with self.assertRaises(Exception):
            validate_svg('<svg xmlns="http://www.w3.org/2000/svg"><rect>')
        self.assertEqual(extract_svg('Here is an SVG: <svg/>'), 'Here is an SVG: <svg/>')


if __name__ == '__main__':
    unittest.main()
