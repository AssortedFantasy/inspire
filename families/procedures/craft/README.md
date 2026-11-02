# Craft procedures

The existing needlework collection is joined by 28 bookbinding operations and
15 pottery operations from two public-domain workshop manuals:

- Douglas Cockerell, *Bookbinding, and the Care of Books*, New York, 1910
  (copyright 1901): paper repair, collation, sewing, spine construction, boards,
  and leather finishing.
- Mary White, *How to Make Pottery*, New York, 1904: kneading, crack repair,
  wheel work, trimming, incising, piercing, relief, and packing.

These are complete operations within the crafts, not complete instructions for
binding a book or firing a pot. All selected source paragraphs and emitted
records were read. Procedures that need an unavailable diagram, historical
chemical-cleaning recipes, glaze formulas, and kiln construction were excluded.
The short packing instruction is deliberately retained alongside longer work.

Wording is verbatim, with HTML, page numbers, and navigational figure references
removed and whitespace normalized. Four sentence-boundary selections are
specified explicitly in the manifest; one sentence referring to a different
decorative example is omitted. No source wording is paraphrased. Author, work,
and edition credit appears in every output. The source texts are historical
practice; this collection makes no claim to be a modern conservation manual.

`source_manifest.json` pins download URLs, SHA256 hashes, catalogue links, and
every selection's starting text and paragraph count. `LICENSE.txt` contains the
source notice and Gutenberg license. The existing needlework files and their
importer remain separate.

From the repository root, using `families/procedures/requirements.txt`:

```sh
python -B families/procedures/craft/import_craft.py --download
python -B families/procedures/craft/import_craft.py --check
python -B -m unittest discover -s families/procedures/craft -p 'test_*.py'
```

Downloads live under ignored `target/imports/craft/`. A changed upstream file
fails its hash check rather than silently changing the corpus. The parent
`families/procedures/import_data.py` still regenerates the original needlework;
it does not overwrite these branches.

The craft parent keeps weight 2, needlework keeps weight 36, and bookbinding and
pottery have weights 28 and 15. Within each new branch, weights equal descendant
record counts. Thus each new record and each existing needlework record has
the same conditional probability within craft, 1/79.

All 43 new outputs are below the 512-token cap (maximum 328 `o200k_base` tokens
after removing redundant figure navigation). Selection replay, boundary
validation, duplicate/overlap checks, schema checks, and count weights are
covered by `test_import_craft.py`.
