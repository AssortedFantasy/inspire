# Distinct categorical palettes

Four through eight opaque sRGB colors, in selection order. Each added swatch
maximizes its distance from its nearest existing neighbor among 128 random
candidates, measured in OKLab after RGB-byte quantization. Candidates vary
OKLCH lightness over 0.48–0.84 and chroma over 0.08–0.24, with uniform hue;
18 bounded bisection steps reduce chroma into the sRGB gamut.

This gives separated hues and brightnesses without repeating fixed hue wheels.
There is no numeric order or prescribed assignment to categories. Tests check
reference conversions, reproducibility, coverage, and separation across 2,000
seeded palettes. Color distance alone does not test color-vision deficiency.

Conversion matrices follow the existing scale generators and
[Björn Ottosson's public-domain Oklab implementation](https://bottosson.github.io/posts/oklab/).
