# Random artifacts

Colors, temporal objects and UUIDs remain separate domains. The incoming family
weight remains 1; colors and dates each have local weight 2, while UUID has weight
1. Thus their conditional shares are 40%, 40%, and 20% regardless of descendant
counts. Colors favor structured palettes over single encoded colors by 3:1.

The former three generators expand into 21 coherent leaves: 18 generators and
three corpora containing 264 official ColorBrewer palettes. See
[colors](colors/README.md), [dates](dates/README.md), and [UUID](uuid/README.md) for
subtree distributions and invariants. Bare encodings, palette relationships,
cartographic semantics, calendar dates, instants and intervals have distinct
paths; datasets with incompatible meanings do not share a leaf.

`random/colors` and `random/dates` now select branches. Direct selection remains
supported; precise single-object paths include `random/colors/representations/hex`
and `random/dates/instants/utc`. UUID retains `random/uuid` and its version-4
format. No arbitrary noise-string generator was added.

All generation uses Inspire's supplied seeded RNG and bounded local computation.
Runtime is offline. ColorBrewer imports pin the upstream commit and checksum,
retain redistribution terms and attribution, and separate sequential, diverging
and qualitative palette objects. Other additions use constrained arithmetic
with substantive gamut, contrast, ordering and calendar tests.
