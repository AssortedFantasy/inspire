# Arts

1,867 records across four equally weighted topics: painting, drawing, printmaking
and theatre. The three museum topics each contain 600 CC0 catalogue works from
the Cleveland Museum of Art. Selection cycles through source culture groups,
with deterministic ID-hash ordering within each group. Titles, dates, creators,
techniques and cultures remain literal catalogue metadata; images, essays and
ownership histories are excluded. Only source records marked CC0 are selected.
Duplicate metadata signatures, incomplete records, labelled forgeries and
fragments are excluded. The source's regional, period and attribution uncertainty
is retained; culture is a catalogue field, not a precise ethnic classification.

Museum metadata reuses the pinned source documented under
[objects](../objects/README.md), revision
`d8dd744bc29e7986df428341133c14e18eb56b3f`, SHA-256
`137ddfab3264555d52c0211f804544cab1c62170ce3d753d924c9d464d74e449`.
No new claims about authorship or physical location are inferred.

```sh
python -B families/arts/import_paintings.py --download
python -B families/arts/import_paintings.py --check
```

The standard-library importer shares `target/imports/cleveland/data.csv` with
objects. The three museum topics have equal shares; every record within each
topic is equally likely. `painting/LICENSE.md` records their common reuse notice.

67 verbatim stage-direction passages from four checksum-pinned public-domain
plays: five from Ibsen's A Doll's House, twelve from An Enemy of the People,
twenty-one from Shaw's Arms and the Man, and twenty-nine from Pygmalion. Both
Ibsen plays use R. Farquharson Sharp's English translation.

Each passage retains consecutive standalone direction paragraphs within an act.
Two long Shaw openings are split between complete paragraphs, separating the
setting from the characters while preserving all source text.
Complete blocks of 120–8,000 characters are included, preserving room layouts,
costume descriptions, entrances, movement and pantomime. Short cues and large
blocks are omitted whole. Inline cues within spoken dialogue are excluded.
Two misleading HTML cases are rejected: Pygmalion's interleaved Higgins and
Pickering speech, and a Stockmann speech mislabeled as a stage paragraph.
Prefaces, introductions, cast lists and Pygmalion's prose sequel are excluded.

Output credits the writer, play, act and translator. Provenance gives the exact
act-relative source paragraph span and pinned source hash. Only HTML layout
whitespace is normalized. The directions are the playwrights' descriptions,
including their historical language and character judgments.

Reproduce with Python and beautifulsoup4==4.12.3:

```sh
python -B families/arts/import_data.py --download
python -B families/arts/import_data.py --check
```

The ignored cache lives under `target/imports/cultural`. Upstream changes fail
checksum validation. `sources.json` identifies editions and rights; `LICENSE.txt`
includes the source notice and ebook license. The underlying English texts are
public domain in the USA and Canada. The family has weight 2. Writer and play
weights match descendant record counts, so all 67 passages have equal probability
within the stage-direction collection, including the five-block leaf.
