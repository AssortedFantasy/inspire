# Weaving drafts

Original rising-shed lift plans, split by interlacement: two-shaft plain weave,
grouped basket weave, straight twill, and warp-faced satin. Every output contains
the threading repeat, warp-color repeat, weft color, every lifted shaft on every
pick, and the complete repeat drawdown. No loom-specific treadling or hidden
tie-up is needed; a direct lift plan defines the sheds. Repeats close cyclically.

Basket groups range from 2x2 through 4x4. Twills use 3-8 shafts, all nontrivial
over/under ratios, either diagonal direction, and a cyclic starting offset. Satins
use 5, 7, or 8 shafts with coprime binding steps excluding adjacent diagonals;
each end and pick has exactly one binding and the stated maximum warp float.
These are symbolic drafts, without unsupported yarn diameter, sett, shrinkage,
strength, or finished-dimension claims.

The basic weave terminology was checked against the primary
[GeoDict 2026 WeaveGeo documentation](https://geodict-userguide.math2market.de/2026/weavegeo_weave-patterns-weave-types.html)
on 2026-10-13. Drafts, renderers, and mathematical constraints are original; no
external patterns or explanatory text are copied. `rebuild.py` writes the four
generators. Tests inspect 500 drafts per leaf: both axes interlace, shed numbers
are valid and unique, grouped/twill/satin structure holds through the repeat
boundary, and drawdown floats match the claimed structure.
