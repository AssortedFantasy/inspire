# HSL colors

One opaque CSS `hsl(h s% l%)` value. Hue is uniform across integer degrees
0–359, saturation across 20–100%, and lightness across 15–85%. This avoids
black, white, and achromatic outputs where hue has no visual effect, while
covering dark, medium, and light colors with subdued through vivid saturation.
HSL is an sRGB representation, not a perceptually uniform color space.

Original bounded generator; syntax and semantics follow [CSS Color 4](https://www.w3.org/TR/2026/CRD-css-color-4-20261009/#the-hsl-notation).
Tests check positive chroma and broad hue, saturation, and lightness coverage.
