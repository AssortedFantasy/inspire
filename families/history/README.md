# History

Eleven public-domain primary texts supply 1,088 excerpts. Eleven additional
records contain museum catalog and material facts. The family has 1,099 records
in 45 leaves. Two independently imported collections add 42 everyday passages
from Sancho, Austen and Dorothy Wordsworth, and 30 Apollo 12 work exchanges.
The family now contains 1,171 records in 49 leaves.

The five established branches retain their relative shares with weight 4 each.
`everyday-voices` and `work-observations` each have weight 1, receiving 1/22 of
history draws. Topic leaves within one text
are weighted by record count so tiny chapters do not repeat disproportionately.

The sources cover Babylonian law, Faxian's pilgrimage, Polo's travels, Morse's
domestic architecture, Equiano's memoir, Montagu's correspondence, Díaz's Mexico,
Vitruvius's architecture, Stewart's metal typesetting, Todd's hand-loom weaving
and Beeton's household management. Menus and specimen correspondence retain
their line structure. Text comes first, followed by author, work and locator.
Historical wording, spelling and perspective remain intact.

The former 72 paraphrases have been replaced with 48 verbatim passages and 11
compact factual records. Overlapping observations of the same museum objects
have been combined. The typesetting source is A. A. Stewart's 1919 manual;
its former `sherman-1918` directory had the wrong author and year.

`sources.json` pins download URLs, encodings and SHA-256 hashes. The explicit
`selections.json` manifest pins each prose entry, contiguous character range,
topic and excerpt hash. A selection can span adjacent source paragraphs and
headings to preserve a complete menu or narrative. It excludes modern introductions, translator notes,
contents, chapter synopses, image-dependent passages, uninformative transitions,
medical passages and selected racial generalizations or graphic abuse.
`exclusions.json` records preparation exclusions, including captions without
their images, dangling introductions to omitted verse and merged narrative
fragments. The selection manifest is the reviewed whitelist. No connecting
prose is synthesized. Only whitespace, print page labels and footnote callouts
are removed. `museum-data.json` keeps the selected factual fields separate from
the prose quotations and is replayed by the same importer.

Replay from the ignored development cache:

```sh
python -B families/history/import_data.py --source-dir target/imports/history --download
python -B families/history/import_data.py --source-dir target/imports/history --check
python -B families/history/test_import.py
```

A changed upstream download fails its checksum rather than silently changing
the corpus. `LICENSE.txt` preserves the full Project Gutenberg redistribution
notice. Required historical credit is also present in every sampled excerpt;
runtime needs only the executable.

[Everyday voices](everyday-voices/README.md) and
[work observations](work-observations/README.md) have separate pinned importers,
manifests and licenses. Their outputs participate in the family-wide replay and
duplicate checks. The Apollo importer needs `pymupdf==1.28.2` for reproducible PDF
text extraction. Source text, speaker turns and scan-checked OCR corrections
remain distinguishable in provenance.
