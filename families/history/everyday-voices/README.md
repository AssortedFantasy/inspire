# Everyday voices

42 short primary-source passages in three leaves: 14 each from Ignatius
Sancho's letters, Jane Austen's letters, and Dorothy Wordsworth's journals.
They cover family invitations, postage and household accounts, spare clothing,
shopping, refused favors, unwanted publicity, baking, sewing, errands, and
close observations of weather, plants, animals, and neighbors.

These are historical writers describing ordinary stakes, not a representative
sample of social classes or a modern conversational corpus. Sancho writes as
a friend, father, husband, and grocer; Austen's correspondence is with her
sister; Wordsworth combines domestic work with outdoor observation. The
published editions retain their editors' historical choices and anonymized
names. Dates and letter numbers follow those editions, including their
inconsistencies. No dialogue or connecting prose is synthesized.

Each record is a complete paragraph, short letter including its signature,
or a contiguous group of complete sentences. Selections omit introductions,
editorial notes, medical instructions, and passages requiring omitted context.
Original spelling and punctuation remain; HTML markup, inline page labels,
footnote callouts, and surplus whitespace are removed. Author, work, and a
letter number or journal date follow the text.

`sources.json` pins the three downloaded editions by URL and SHA-256.
`selections.json` pins parsed paragraph ranges, exact character ranges, and
excerpt hashes. The importer rejects changed sources and changed passages.
`LICENSE.txt` includes source rights notices and the full Gutenberg license.

```sh
python -B families/history/everyday-voices/import_data.py --download
python -B families/history/everyday-voices/import_data.py --check
python -B families/history/everyday-voices/test_import.py
```

Downloads stay in ignored `target/imports/everyday-voices/`. The binary embeds
only the generated records and notices. The branch has sibling weight 1;
source leaves have weights matching their record counts, so every passage has
the same probability within this branch. All 42 attributed records passed
the 512-token cap; the longest is 296 `o200k_base` tokens.
