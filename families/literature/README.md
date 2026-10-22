# Literature

The collection contains 3,413 records in 50 leaves: 36 retained literary
passages, 12 compact form patterns, and 3,365 source-derived literary units from
25 checksum-pinned editions. Every passage
names its writer, work, edition and location; translated material names its
particular English translator. Fictional speakers and historical attitudes are
the cited work's, rather than factual assertions by Inspire.

- `verse/`: English sonnet excerpts, Wheatley and four Dunbar collections;
  Indic, Chinese and Persian translations. Whole short poems retain line order.
- `narrative/`: whole classical and French fables; Japanese folktales; British,
  French, Russian and Chinese diaspora fiction; Douglass and Du Bois nonfiction;
  complete numbered anecdotes from Sa’di's Gulistan in James Ross's translation.
- `prose-poetry/`: consecutive paragraph groups from Gibran's The Prophet.
- `aphorisms/`: complete numbered units from Tagore's Stray Birds.
- `dialogue/`, `quotations/`: original verified passages, grouped by author.
- `poetic-forms/`: line counts, rhyme schemes and other compact form data.

Source notes and precise SHA-256 values are in `sources.json`. The pinned cache
lives outside the sampling tree. Development requires beautifulsoup4==4.12.3:

```sh
python -B families/literature/import_data.py --download
python -B families/literature/import_data.py --check
python -B families/literature/test_import_data.py
```

An upstream update fails checksum validation instead of silently changing the
corpus. Downloading the original snapshot is necessary for replay; runtime
always uses the embedded records. The 48 retained records' canonical hashes are
kept in `baseline_manifest.json`; the form entries were revised into compact
data on 2026-10-22.

Extraction excludes metadata, introductions by other writers, illustrations,
notes, page numbers, licensing boilerplate and duplicate complete units. Long
poems and fables are omitted rather than cut mid-sentence or mid-line. Narrative
excerpts use complete paragraphs of 230-1,500 characters, spread across chapters
and stories, with a per-section cap. This is a deliberate bounded selection,
not a claim to reproduce complete books. Chapter locators count original HTML
paragraphs before filtering. Gibran groups preserve consecutive paragraph order.
Gulistan anecdotes retain every paragraph of the numbered unit, including its
conclusion. Units outside 90–2,400 characters and sections marked by omission
asterisks are excluded; the anthology's introduction is never sampled.

These particular published versions are public domain in the USA and Canada;
this is not a worldwide rights claim. FitzGerald's Rubaiyat is labeled a free
adaptation, and Kabir's attribution follows the historical anthology. Dunbar's
literary dialect is preserved. English translations reflect their historical
editors and translators. See `LICENSE.txt` for source and rights policy.

Local sibling weights balance literary forms and then traditions, writers and
works; they do not make every individual passage equally likely. Verse and
narrative receive most of the family weight. Small collections of form data,
dialogue and quotations have a reduced share and weights matching record counts
within each collection. Retained fiction passages compete with imported works
in proportion to their record counts, so a single familiar quotation does not
receive half an author's draws.
