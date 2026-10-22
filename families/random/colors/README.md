# Colors

Separate CSS color representations and structured palettes. The incoming weight
remains 2. `palettes` has weight 3 and `representations` weight 1, so three
quarters of color samples are useful multi-color objects.

- `representations/{hex,rgb,hsl,oklch}`: one opaque color per precise notation.
- `palettes/categorical/distinct`: farthest-point OKLab palette selection.
- `palettes/harmony/{analogous,complementary}`: constrained HSL hue relationships.
- `palettes/scales/{sequential,diverging}`: ordered OKLCH scales with
  hue-preserving gamut fitting into sRGB.
- `palettes/contrast/wcag-pairs`: measured opaque text/background pairs ≥4.5:1.
- `palettes/cartographic/{sequential,diverging,qualitative}`: 264 pinned official
  ColorBrewer palettes, separated by data semantics, with source/license notes
  and a reproducible importer.

Runtime remains offline; notation, palettes, source data and validation details
are documented within each branch. Palettes do not imply accessibility unless
an explicit contrast calculation supports it.
