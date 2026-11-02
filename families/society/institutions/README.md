# Institutions

46 clauses and court entries from medieval English primary documents, in the
translations and modernized spelling of Bland, Brown and Tawney's *English
Economic History: Select Documents* (1914). Each record contains a complete
selected clause or entry and a short attribution. Original brackets and
ellipses remain; footnote callouts, sidenotes and italic markers are omitted.

- `court-decisions` (13): Bradford debt, inheritance, wardship, tenure,
  sureties, mobility and fencing cases, plus contrasting enclosure judgments
  from Leyburn and Woodhouse. The cases retain their decisions and remedies.
- `guild-rules` (13): London white tawyers' mutual aid, burial, admission,
  discipline and elected inspection, and Bristol dyers' damage arbitration,
  competence tests and household dyeing exemption.
- `commons-and-boroughs` (20): Wimeswould grazing, gleaning and harvest rules,
  Tewkesbury property and municipal clauses, Leicester grievances about the
  lord's officers, and a reciprocal Marlborough–Southampton toll settlement.

These are historical rules and records, including coercive tenure and labour
restrictions. They are not a survey of contemporary law. Wimeswould follows
the document heading in this edition; its editor notes defects in the surviving
text. The selected clauses avoid its broken passages. This modest collection
adds lived institutional constraints; it does not yet cover modern cooperatives
or institutions outside England. Existing Babylonian law remains in `history`.

The branch has sibling weight 2 against economics 7 and voting 3. Its leaf
weights equal their record counts, giving every record the same probability
within the branch. Economics and voting retain five sixths of society draws.

The [source catalogue](https://www.gutenberg.org/ebooks/43211) identifies the
edition as public domain in the USA. `LICENSE.txt` bundles the downloaded
Project Gutenberg license. `sources.json` pins the text URL, SHA-256, document
locators and inclusive line ranges. Rebuild or verify deterministically:

```sh
python -B scripts/import_institutions.py --download
python -B scripts/import_institutions.py --check
```

The download cache lives under ignored `target/imports/institutions/`. The
importer rejects changed source bytes and mismatched selection boundaries.
All 46 rendered records, including attribution, contain 44–385 `o200k_base`
tokens; none is truncated to meet the 512-token cap.
