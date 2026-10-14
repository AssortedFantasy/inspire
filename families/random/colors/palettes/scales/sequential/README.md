# Sequential palettes

Five through nine same-hue OKLCH samples, in light-to-dark order. Hue covers the
full circle; lightness endpoints vary over 0.91–0.96 and 0.28–0.42. Lightness is
evenly spaced before byte quantization. Chroma follows a smooth sine envelope
with strength 0.075–0.16, giving pale tints, richer middle tones, and dark shades.

Out-of-gamut colors undergo 18 bounded binary-search steps that reduce chroma
while preserving lightness and hue. The generator then encodes rounded sRGB
hex colors. Tests cover every integer hue at parameter extremes: gamut fit,
strictly descending relative luminance, at least 0.055 OKLab lightness between
neighbors, hue retention, conversion reference colors, and seeded diversity.

Conversions follow [CSS Color 4](https://www.w3.org/TR/css-color-4/#color-conversion-code).
The chroma envelope and strict gamut reduction are this generator's design;
they do not implement the CSS gamut-mapping algorithm or guarantee accessibility.
