# Diverging palettes

Five, seven, or nine ordered sRGB colors form two fixed-hue OKLCH wings around
one pale gray midpoint. The first hue spans the full circle; the second is
120–240 degrees away. Both wings share evenly spaced lightness (endpoints
0.42–0.55, midpoint 0.94–0.97) and linearly taper chroma to zero at the center.
Requested endpoint chroma is 0.09–0.15. A bounded binary search reduces that
shared chroma until every step on both wings fits sRGB, preserving balance
instead of clipping channels or desaturating one wing independently.

Tests check actual byte-quantized palettes for neutral centers, distinct colors,
matched paired lightness and chroma, hue retention, meaningful step separation,
strict luminance rise/fall, and reproducibility. Boundary grids and random
samples cover all hues and lengths. These are balanced procedural numerical
scales, without a color-vision accessibility claim.

Conversions follow [CSS Color 4](https://www.w3.org/TR/css-color-4/#color-conversion-code).
