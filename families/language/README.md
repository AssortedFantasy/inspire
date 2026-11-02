# Language

Attested English usage, lexical senses, semantic relations, traditional expressions,
and historical word scholarship. Sampled text contains the source material and its
credit, without invented examples, interpretations, or category introductions.

- `grammar/`: 335 literary usage quotations collected by Baskervill and Sewell in
  *An English Grammar* (1895), retaining the credited authors.
- `phraseology/sayings/`: 614 exact entries from Thomas Preston's *A Dictionary of
  English Proverbs and Proverbial Phrases*. Idioms and proverbs share one collection
  instead of repeating an expression in two tiny leaves.
- `word-histories/`: 51 passages of complete paragraphs from Ernest Weekley's *The Romance of
  Words*, fourth edition (1922), replacing twelve editorial paraphrases.
- `lexicon/senses/`: bounded WordNet collections divided by part of speech and
  native lexical domain. Nouns cover attributes, cognition, communication, feelings,
  phenomena, processes, relations, shapes, states, and time; verbs cover change,
  cognition, communication, creation, emotion, motion, perception, and social action.
- `lexicon/relations/antonyms/`: direct lexical antonyms, separately for nouns,
  verbs, and adjectives, with both definitions to distinguish the relevant senses.

The four branches deliberately receive 50% lexicon, 25% phraseology, 20% grammar,
and 5% word histories within language. This preserves material variety while
reducing the former 25% share assigned to twelve word histories. WordNet's existing
part-of-speech and domain balance remains; record counts do not let it displace
the other sources.

Historical wording is not modernized. Word histories carry Weekley's name, book,
and year; selection avoids obvious discredited derivations, missing quotation
context, and page references. Slash-separated WordNet words share the listed sense.
Compact part-of-speech notation identifies the lexical entry without repeating the
sampling path; antonym pairs display both exact definitions on separate lines.

## Reproducible historical selections

`python -B families/language/import_historical.py` downloads two source snapshots
into `target/imports/language`, verifies their SHA256 checksums, and rebuilds the
sayings and word-history records. `historical-selections.json` stores the reviewed
entry numbers and normalized-text checksums. `weekley_context` joins the cipher/zero paragraph
to the immediately following furnish/veneer comparison, retaining both source
paragraphs verbatim. The importer strips page furniture,
footnote markers, and Preston's index keywords; it preserves the selected wording
and adds a source credit. Source changes fail verification instead of silently
replacing reviewed material. The parser uses only Python's standard library.
Add `--check` to verify the generated files without rewriting them.

The grammar importer and its source selection are documented in [grammar/README.md](grammar/README.md).
Redistribution notices are in [LICENSE.md](LICENSE.md) and [grammar/LICENSE.md](grammar/LICENSE.md).

## Reproducible WordNet selection

`python -B families/language/import_wordnet.py` verifies and transforms the official
[WordNet 3.0 archive](https://wordnetcode.princeton.edu/3.0/WordNet-3.0.tar.gz), SHA256
`640db279c949a88f61f851dd54ebbb22d003f8b90b85267042ef85a3781d3a52`.
The cache lives under `target/imports/wordnet`; no source files or network access are
needed at runtime. Full redistribution terms are in `lexicon/LICENSE`, embedded in
`inspire --data-licenses`.

The importer parses the archive without extracting paths. Domain names come from
the archive's `lexnames`, not inferred topic labels. Stable IDs retain part of speech
and the synset byte offset. It requires at least two aggregate tagged attestations
in WordNet's historical semantic concordances, and normally emits only tagged
lemmas (at most six). One reviewed native-synonym exception pairs `tickling` with
`tingling` to identify their shared adjective sense. These counts select attested
material; they are not present-day frequencies.
Definitions are verbatim, excluding usage quotations; adjective positional markers
are removed from displayed lemmas. Definitions must be 18–240 characters, with no
leftover quote fragments. A conservative lexical filter excludes marked obsolete,
offensive, sexual, psychiatric, and violent material; it is an editorial screen,
not a claim that every historical formulation has been modernized.

The cap is 160 senses per noun/verb domain and 240 per adjective/adverb collection.
Candidates are ordered by a stable SHA256 of the synset identity, so lower-count
attested senses remain represented instead of always selecting the most frequent
ones. Antonyms require an explicit `!` lexical pointer and its exact hexadecimal
source/target word indices; both words must be tagged. Reverse pairs are deduplicated
and each part of speech is capped at 160 pairs. Neither antonyms nor synonyms are
fabricated from spelling or from a broad conceptual contrast. WordNet's 2006 lexical
classifications and glosses remain historical source data, not scientific definitions
or exhaustive descriptions of English.

Dedicated leaf reviews record rejected source senses or antonym-pair IDs with
reasons in leaf-local `exclusions.json`. Import replay applies those exclusions
after bounded selection, without filling removed rows with unreviewed replacements.
A rejected sense is also removed from any antonym pair using that sense. Historical
source definitions are removed rather than silently edited or modernized.
Use `python -B families/language/import_wordnet.py --check` for exact replay.
[Lexicon review notes](lexicon/README.md) document the narrow numeric-sense
exclusions and the retained substantive antonym definitions.
