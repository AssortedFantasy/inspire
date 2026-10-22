# Weaving drafts

Original repeat drafts with two-color warp and weft plans. Each output contains
either a threading and rising-shed lift repeat or a warp-over drawdown (`X` over,
`.` under). Color and interlacement repeats are independent. Counts after `×`
are ends for warp stripes and picks for weft stripes.

- Plain weave uses two shafts.
- Basket weave has independent groups of 2–4 ends and 2–4 picks.
- Twills use 3–8 shafts and straight, point, or broken threading. Every lift
  advances one shaft, in either direction, with a cyclic starting offset.
- Satins use 5, 7, or 8 shafts, either face, and coprime binding steps excluding
  adjacent diagonals. Every end and pick has one binding.

`rebuild.py` writes and formats all four generators. Tests inspect 500 drafts per
leaf for valid sheds, interlacement on both axes, and cyclic weave structure.
These are symbolic drafts without yarn, sett, or finished-size assumptions.

Basic terminology was checked against the primary
[GeoDict 2026 WeaveGeo documentation](https://geodict-userguide.math2market.de/2026/weavegeo_weave-patterns-weave-types.html)
on 2026-10-13. Drafts, renderers, and constraints are original.
Point and staggered-reversal twill terminology was checked against
[Handwoven's visual guide](https://handwovenmagazine.com/twills-handwoven-archives/)
on 2026-10-22.
