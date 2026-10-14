# CSS color representations

Each leaf emits one opaque CSS color, with a fixed notation. The leaves have
equal local weights; choose a leaf explicitly when its notation matters.

- `hex`: uniformly sampled 8-bit sRGB channels, written as `#rrggbb`.
- `rgb`: uniformly sampled 8-bit sRGB channels, written as `rgb(r g b)`.
- `hsl`: hue across the full circle, with useful saturation and lightness ranges.
- `oklch`: perceptual lightness, chroma, and hue, constrained to the sRGB gamut.

These are bounded procedural domains rather than named-color datasets. Hex and
RGB deliberately cover the same colors in separate representations; HSL and
OKLCH use their own sampling spaces. No alpha or palette structure is mixed into
these single-color leaves.

Syntax and color-space reference: [CSS Color Module Level 4, 9 October 2026](https://www.w3.org/TR/2026/CRD-css-color-4-20261009/).
