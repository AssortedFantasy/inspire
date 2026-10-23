# Random artifacts

The incoming family weight remains 1. Local weights preserve space for colors
and temporal objects alongside exact combinatorial and numerical artifacts:

| Child | Weight | Conditional share |
| --- | ---: | ---: |
| `colors` | 2 | 1/7 |
| `dates` | 2 | 1/7 |
| `distributions` | 2 | 1/7 |
| `geometry` | 2 | 1/7 |
| `orders` | 2 | 1/7 |
| `signals` | 2 | 1/7 |
| `identifiers` | 1 | 1/14 |
| `uuid` | 1 | 1/14 |

There are 26 leaves: 23 generators and three corpora containing 264 official
ColorBrewer palettes. Bare codes receive less weight than structured objects.
Colors favor structured palettes over single encoded colors by 3:1. See
[colors](colors/README.md), [dates](dates/README.md), and [UUID](uuid/README.md)
for their existing distributions and invariants.

- [identifiers](identifiers/README.md): six synthetic checksum formats, including
  ISBN-10 X/zero boundaries, modern ISMN and structurally correct German IBAN.
- [distributions/contingency](distributions/contingency/README.md): 2×2 and 3×3
  fixed-margin tables, exact rational Pearson statistics and two-sided Fisher p.
- [geometry/shapes](geometry/shapes/README.md): simple convex, radial, orthogonal,
  deep-bay and thin polygon paths with exact area and numerical perimeter.
- [orders/posets](orders/posets/README.md): eight constructions, reduced covers,
  exact linear-extension counts and complete maximum-antichain sets.
- [signals/waveforms](signals/waveforms/README.md): structured 8/16-sample signals,
  exact averaging Haar coefficients or Sylvester-ordered Hadamard transforms.

`random/colors` and `random/dates` now select branches. Direct selection remains
supported; precise single-object paths include `random/colors/representations/hex`
and `random/dates/instants/utc`. UUID retains `random/uuid` and its version-4
format. No arbitrary noise-string generator was added. The graph-atlas and
permutation proposals remain with their existing mathematical implementations.

All generation uses Inspire's supplied seeded RNG and bounded local computation.
Runtime is offline. ColorBrewer imports pin the upstream commit and checksum,
retain redistribution terms and attribution, and separate sequential, diverging
and qualitative palette objects. Other additions use constrained arithmetic
with substantive gamut, contrast, ordering and calendar tests. The five new
leaves add independent checksum validation, exact label-assignment enumeration,
polygon edge-intersection and slicing checks, exhaustive order permutations,
and transform matrix/inverse checks. Their READMEs give primary references,
normalizations, sampling biases, and model limits.
