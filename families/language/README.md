# Language

English constructions, lexical senses, semantic relations, traditional expressions,
and word histories. Branches balance material types and topics, rather than treating
record counts as sampling weights.

- `grammar/tense-aspect/`: six original examples.
- `grammar/syntax/`: 44 original examples in ten construction families: voice and
  agreement, clause types, relatives, conditionals, adverbial clauses, comparisons,
  information structure, complementation, coordination, and noun phrases.
- `phraseology/idioms/`: 15 attested phrases with original definitions and examples.
- `phraseology/proverbs/`: 25 sayings checked against Preston's numbered entries;
  their readings are interpretations, not universal advice.
- `word-histories/`: 12 concise explanations based on Weekley's 1922 edition.
- `lexicon/senses/`: bounded WordNet collections divided by part of speech and
  native lexical domain. Nouns cover attributes, cognition, communication, feelings,
  phenomena, processes, relations, shapes, states, and time; verbs cover change,
  cognition, communication, creation, emotion, motion, perception, and social action.
- `lexicon/relations/antonyms/`: direct lexical antonyms, separately for nouns,
  verbs, and adjectives, with both definitions to distinguish the relevant senses.

Invented examples are identified in provenance. Idiom wording may be inflected or
modernized; attestation does not establish origin. Historical sources are linked in
each record. Grammar examples illustrate conventional English constructions without
claiming to describe every dialect. Slash-separated WordNet words share the listed
sense; this does not make them interchangeable in every sentence.

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
in WordNet's historical semantic concordances, and emits only tagged lemmas (at most
six). These counts select attested material; they are not present-day frequencies.
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
