# Food data sources

US Department of Agriculture, Agricultural Research Service. FoodData Central,
SR Legacy, April 2018. https://fdc.nal.usda.gov/

USDA FoodData Central data are in the public domain and are not copyrighted.
USDA requests attribution to FoodData Central. See
https://fdc.nal.usda.gov/api-guide/ .

Inspire joins food descriptions, categories and selected nutrients by source
identifiers. Source units, preparation names and per-100-g edible-portion basis
are retained; numeric trailing zeros are removed. Source archive and checksum
are recorded in scripts/catalogue_sources.json. No website prose is imported.

Corpora: Darius Kazemi and contributors,
https://github.com/dariusk/corpora , revision
2e7adec11a0561696c236ae1fca82e88be173d42.

The source dedication states that, to the extent possible under law, Darius
Kazemi has waived all copyright and related or neighboring rights to Corpora,
published from the United States, under CC0 1.0 Universal:
https://creativecommons.org/publicdomain/zero/1.0/ .

Inspire selects literal vocabulary labels, normalizes whitespace, removes
duplicate labels and adds `apple` to apple-cultivar names. These datasets are
not a comprehensive or current reference. No recipes or interpretive prose
are generated from the lists.
