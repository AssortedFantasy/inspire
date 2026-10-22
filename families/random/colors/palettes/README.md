# Palettes

Five equal-weight semantic children: hue relationships (`harmony`), ordered
numeric mappings (`scales`), separated category colors (`categorical`), measured
text/background pairs (`contrast`), and published map schemes (`cartographic`).
All siblings below them have equal weight. `palettes` retains weight 3 against
`representations` weight 1, so three quarters of color samples are palettes.

Generators emit swatches directly, with role labels for complementary and
text/background colors. Published palettes retain a short source credit. HSL
harmonies preserve hue relationships; numeric ramps use ordered OKLCH lightness
and hue-preserving sRGB gamut fitting. Categorical swatches use distances
measured after byte quantization. Only the contrast child claims a measured
text contrast threshold.
