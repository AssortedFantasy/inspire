# Hex colors

One opaque, lowercase CSS `#rrggbb` value. Each 8-bit sRGB channel is sampled
independently and uniformly, covering all 16,777,216 combinations, including
black and white. Alpha belongs in a separate domain, not this representation.

Original bounded generator; syntax follows [CSS Color 4](https://www.w3.org/TR/2026/CRD-css-color-4-20261009/#hex-notation).
Tests roundtrip every channel value and check generated values against the
underlying RGB draws.
