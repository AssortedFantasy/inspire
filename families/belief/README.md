# Belief

688 verbatim units from four checksum-pinned public-domain English editions:

- 81 complete chapters of the Tao Te Ching, James Legge's 1891 translation.
- All 423 Dhammapada verses in 414 printed numbered units, F. Max Müller's
  1881 translation. The edition's nine paired verses remain joined.
- 52 passages covering 18 chapters of Snorri's Younger Edda in Rasmus Björn Anderson's
  1901 edition, including Gefjun's plowing, creation, divine households,
  Skirner's journey, Sleipner, Idun, Skade and the origin of poetry.
- 141 passages covering 53 named sections from the Greek and Roman portion of Thomas
  Bulfinch's Age of Fable, in Edward Everett Hale's revision.

The Edda selects printed chapters of 500–8,000 characters; longer chapters
are omitted whole. Its preface, introduction, afterwords, notes, vocabulary
and index are excluded. Bulfinch selects named sections between Python
and Aurora and Tithonus, 400–9,000 characters, excluding the isolated poem
Upon a Lady's Embroidery, Olympic and Other Games, and the umbrella heading
The Water Deities. Literary quotations within an accepted section remain
attached to their introductions. The final bridge from Aristaeus to the next
section is excluded.

Every output, including its source credit, fits within 512 `o200k_base` tokens.
Fourteen Edda chapters and 35 Bulfinch sections are divided into consecutive
excerpts at paragraph or complete sentence boundaries. All selected words
remain in source order; verse lines are preserved. `excerpts.json` freezes the
reviewed character spans, so import replay needs no tokenizer. Each excerpt's
provenance gives its span in the cleaned source body, and its ID adds an excerpt
ordinal to the original section ID. The 22 shorter mythology units and both
scripture corpora remain intact.

Source numbering, paragraph breaks and verse lines remain intact, apart from
removing chapter numbers repeated in the credit, HTML layout whitespace,
page numbers and footnote links. Translators and locations appear in output;
exact source locators and checksums stay in provenance. These historical
translations and retellings represent their particular editions.

The pinned development cache is outside the tree. Reproduce with Python and
beautifulsoup4==4.12.3:

```sh
python -B families/belief/import_data.py --download
python -B families/belief/import_data.py --check
```

An upstream change fails checksum validation. `sources.json` records the exact
editions, URLs and hashes; `LICENSE.txt` carries the source notice and ebook
license. The published texts are public domain in the USA and Canada.
The family has weight 2; mythology and scripture have equal local weight,
with equal weight for their two traditions. Records are uniform within a leaf.
