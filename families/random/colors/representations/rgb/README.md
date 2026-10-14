# RGB colors

One opaque CSS `rgb(r g b)` value in modern space-separated notation. Each
8-bit sRGB channel is sampled independently and uniformly across 0–255. The
domain contains all 16,777,216 RGB combinations; it shares the color domain of
the hex sibling but keeps the decimal representation separate.

Original bounded generator; syntax follows [CSS Color 4](https://www.w3.org/TR/2026/CRD-css-color-4-20261009/#rgb-functions).
Tests preserve all byte values in decimal form and compare generated colors
with the supplied RNG's RGB draws.
