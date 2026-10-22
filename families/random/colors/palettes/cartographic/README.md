# Cartographic palettes

264 unchanged palette designs from the official ColorBrewer JSON export,
covering all 35 schemes and all class counts present at revision
`7d135fc4e19eda73f2eb1bf55fcdf4a04fe4881f`:

- `sequential`: 125 palettes, 18 schemes.
- `diverging`: 81 palettes, 9 schemes.
- `qualitative`: 58 palettes, 8 schemes.

Children currently have equal local weight; rows are uniform within each child.
Palette order is preserved; RGB triplets are converted losslessly to hex.
The pinned upstream export omits the nine-class YlOrRd row; the importer
preserves the export as published rather than inventing a replacement.
These are scheme data, not assertions that every class count is accessible
or appropriate for every display. Outputs contain the ordered hex swatches and a short scheme credit.

Run `python import_data.py` to fetch and verify the pinned source, or use
`python import_data.py --input downloaded-colorbrewer.json`. The importer
checks SHA-256, palette sizes, byte ranges, unique colors, and category counts.
Source downloads stay outside the sampling tree.

Source: https://github.com/axismaps/colorbrewer/tree/7d135fc4e19eda73f2eb1bf55fcdf4a04fe4881f

This product includes color specifications and designs developed by Cynthia
Brewer (http://colorbrewer.org/). Copyright (c) 2002 Cynthia Brewer, Mark Harrower,
and The Pennsylvania State University. See [LICENSE.txt](LICENSE.txt) for the
Apache-Style ColorBrewer license, preserved from the pinned upstream
[`export/LICENSE.txt`](https://raw.githubusercontent.com/axismaps/colorbrewer/7d135fc4e19eda73f2eb1bf55fcdf4a04fe4881f/export/LICENSE.txt).
The acknowledgment is bundled in `--data-licenses` and the source documentation.
