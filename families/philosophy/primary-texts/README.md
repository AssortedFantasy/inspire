# Primary texts

44 complete paragraph units from three contrasting philosophical voices:

- David Hume, *An Enquiry Concerning Human Understanding*: 15 units on
  sensation, counterexamples, induction, voluntary action and testimony. The
  missing shade of blue, a child's candle burn, marble adhesion, an old
  greyhound and a purse left at Charing Cross give the arguments concrete stakes.
- Mary Wollstonecraft, *A Vindication of the Rights of Woman*: 15 units on
  rational education, dependence, public and parental authority, affection,
  servants and animals. Her argument that women and soldiers receive similar
  education contests an inference from observed behavior to innate difference.
- William James, *Essays in Radical Empiricism*: 14 units on experienced
  relations, embodied attention, shared objects, value and unfinished knowing.
  The Memorial Hall example retains all three consecutive paragraphs, including
  both failed reference and successful recognition. The appreciation example
  retains its two consecutive paragraphs.

These are selected passages, not complete treatises or an overview of all
philosophical traditions. Hume's account of impressions and James's defense of
experienced relations take different approaches; Wollstonecraft brings social
and ethical argument into the same family. No source author or work duplicates
the existing `belief` or `literature` corpora.

`sources.json` pins the original UTF-8 bytes by URL and SHA-256. Each selection
specifies its location, opening text and zero-based paragraph indices after the
ebook start marker, before filtering. Records give one-based indices in
provenance. Extraction collapses print line wrapping, removes italic markup and
note calls, and removes Hume's leading paragraph numbers. All selected words,
spelling and punctuation survive. No paraphrasing, sentence clipping, modernization
or token-dependent truncation is performed. Introductions by other writers and
footnotes are not sampled.

The source editions are public domain in the USA. Original authors, works and
edition dates appear in every output; source and rights details and the source
ebook license are bundled in `LICENSE.txt`.

Replay uses only the Python standard library:

```sh
python -B families/philosophy/primary-texts/import_data.py --download
python -B families/philosophy/primary-texts/import_data.py --check
```

The download cache stays in ignored `target/imports/philosophy/`. Changed upstream
bytes fail checksum validation. With the cache present, regeneration works
offline; runtime needs only the embedded records. All 44 outputs, including
credits, pass the 512-token `o200k_base` cap. Leaf weights 15:15:14 make the
individual records equally likely within this branch. Its local weight 1 versus
`logic` weight 3 preserves a three-quarter share for formal generators without
changing the root family weight.
