# Palettes

Four equal-weight semantic children: hue relationships (`harmony`), ordered
numeric mappings (`scales`), measured text/background pairs (`contrast`), and
published map color schemes (`cartographic`). All siblings below them currently
have equal weight. Thus sibling topics are balanced independently of corpus
size, and bare representations do not dilute the palette share.

Generators emit a concrete color object and its defining relationship. They
use supplied seeded randomness, bounded loops and no runtime I/O. HSL harmonies
are not perceptually uniform; numeric ramps use ordered OKLCH lightness and
hue-preserving chroma fitting into sRGB;
only the WCAG pair child claims a measured contrast threshold. Published palettes
preserve official palette ordering and class counts with required attribution.
