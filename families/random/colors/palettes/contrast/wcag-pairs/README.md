# Text/background contrast pairs

Uniform opaque sRGB backgrounds; foreground candidates use independently varied
dark (0–95) or light (160–255) byte channels. Dark/light preference maximizes
black/white contrast. Up to 48 candidates are tried; a bounded fallback uses
black or white, one of which always achieves at least 4.5:1.

The actual quantized RGB colors are measured with the WCAG relative-luminance
formula. Every pair meets the normal-text AA threshold of 4.5:1. Printed ratios
round down. No alpha, overlays, AAA guarantee, or broader accessibility claim.
Tests cover primary-color luminances, the sRGB transfer boundary, reference gray
contrast thresholds, 10,000 accepted pairs, candidate diversity, and the ratios
and channels parsed from emitted text.

Formula and threshold: https://www.w3.org/TR/WCAG22/#contrast-minimum and
https://www.w3.org/TR/WCAG22/#dfn-relative-luminance
