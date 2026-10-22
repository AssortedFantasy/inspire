# OKLCH colors

One opaque CSS `oklch(L C h)` value. Lightness covers 0.250 through 0.900
in 0.001 steps and hue covers 0 through 359.9 degrees in 0.1-degree steps.
Chroma uses 15–95% of the available sRGB gamut at the sampled lightness and hue,
found with 24 bounded bisection steps. This creates muted through vivid colors
without silently relying on browser clipping. Decimal chroma is rounded downward to six places; trailing zeros are omitted.

Original procedural sampling design. Conversion matrices are from
[Björn Ottosson's Oklab reference implementation](https://bottosson.github.io/posts/oklab/),
updated 25 January 2021 and explicitly offered as public-domain code or under MIT.
CSS notation follows [CSS Color 4](https://www.w3.org/TR/2026/CRD-css-color-4-20261009/#specifying-oklab-oklch).

Tests verify a known sRGB red, neutral coordinates, gamut boundaries across a
lightness/hue grid, and the gamut of actual emitted decimal values.
