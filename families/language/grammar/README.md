# Grammar

335 literary usage quotations from W. M. Baskervill and J. W. Sewell,
*An English Grammar* (1895), transcribed in
[Project Gutenberg ebook 14006](https://www.gutenberg.org/ebooks/14006).
One leaf samples every quotation equally.

The output contains the source's words and author attribution, followed by the
intermediary work credit. Subjects include possessives, pronouns, articles,
comparison, conditionals, auxiliaries, relative clauses, coordination, agreement,
participles, adverbs, and prepositions. Historical usage is preserved; the book's
prescriptive judgments are omitted.

The importer selects 335 individually read paragraphs from 1,066 attributed
paragraphs. It excludes chopped-off phrases, ellipses, editorial interpolations,
exercise instructions, and context-dependent lists. HTML emphasis and exercise
numbering are removed and whitespace is normalized. No example or explanation
has been invented or paraphrased. Attribution is retained as printed in the
textbook rather than expanded into unverified original-work citations.

A separate pass read every selected output against the cached source. It removed
fragments, context-dependent apparatus references, near duplicates, and source
errors: paragraph 941 drops a negation from Bacon, and paragraph 2244 misprints
Gayarré's name.

Reproduce and verify from the repository root:

```sh
python -B families/language/import_grammar.py
python -B families/language/import_grammar.py --check
```

The source cache is `target/imports/language/grammar-14006.html`. Its SHA256 is
`700d383181edbad977023c138d41227279192dd368fcd171d783799e582c3577`.
Record IDs preserve the source HTML paragraph number. The importer rejects a
changed source snapshot, missing selections, duplicate outputs, and incomplete
extracts. The bundled `LICENSE.md` records the public-domain source and its ebook
redistribution terms.
