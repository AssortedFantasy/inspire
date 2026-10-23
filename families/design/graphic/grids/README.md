# Page grids

Original integer-millimetre layouts: manuscript recto/verso, modular pages,
editorial pages with marginalia, and asymmetric posters. Column and row spans
describe layout zones, including the gutters inside a span. Manuscript margins
are inner/top/outer/bottom 2:3:4:6 units on a 2:3 page; the other regimes allocate
the remaining space by largest remainders. Every axis closes exactly.

Tests reconstruct both axes and rasterize every zone onto grid cells, checking
that the cells are covered once. Grids specify geometry, without typography,
bleed, content fitting, or a claim to reproduce a historical canon.

Track, gutter, and span terminology follows the primary
[W3C CSS Grid Layout specification](https://www.w3.org/TR/css-grid-2/#grid-concepts),
checked 2026-10-22. These are print-layout artifacts rather than CSS output.
