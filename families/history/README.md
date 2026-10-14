# History

The original 72 factual paraphrases remain, with their IDs, in deeper branches
for Egyptian museum models, Vitruvius's engineering, textiles, printing and
households. `original-records.json` records that preservation baseline.

Seven downloaded public-domain primary texts add 995 excerpts in 32 leaves.
They cover Babylonian legal clauses, Faxian's pilgrimage and manuscript
collection, Polo's court and economic accounts, Morse's domestic architecture,
Equiano's Atlantic memoir, Montagu's embassy correspondence and Díaz's
descriptions of craft and markets in Mexico. The complete family has 1,067
records in 38 leaves. Sources, genres and topics occupy separate branches;
legal clauses, technical observations, memoir and correspondence do not share
a single dataset. Default sibling weights balance topics rather than corpus size.

Excerpts identify the author, period, edition or translation and chapter or
clause in their emitted text. They are evidence of the source's account and
perspective, not independently confirmed facts or universal cultural practice.
For example, a claim by Díaz remains his retrospective conquistador account;
a recommendation by Vitruvius remains a recommendation. Old translations,
historical place names and archaic spelling remain visible.

`sources.json` pins download URLs, encodings and SHA-256 hashes. The explicit
`selections.json` manifest pins each prose entry, contiguous character range,
topic and excerpt hash. It excludes modern introductions, translator notes,
contents, chapter synopses, image-dependent passages, uninformative transitions,
medical passages and selected racial generalizations or graphic abuse.
`exclusions.json` records later preparation exclusions; the selection manifest
is the authoritative reviewed whitelist. No text is synthesized between
excerpts. Only whitespace, print page labels and footnote callouts are removed.

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
