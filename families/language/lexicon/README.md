# WordNet review choices

The pinned WordNet 3.0 archive and its license remain the sources for all words,
definitions, and lexical antonym links. Run `python -B families/language/import_wordnet.py
--check` from the repository root to verify the generated collections.

Leaf-local exclusions remove literal ordinal positions and numeric multipliers
after the bounded selection. They do not remove the headwords in other senses,
ordinary temporal vocabulary, compass directions, or short definitions generally.
Removed rows are not filled with previously unreviewed replacements.

The adjective synset `00922439` contains *tickling*, *tingling*, and *titillating*.
Only *tingling* has tagged attestations in this sense, which made the isolated
headword misleading. A reviewed display selection now includes its source synonym
*tickling* alongside *tingling*, with the original definition unchanged.

Negation alone is not grounds for removal. The fair/unfair pair has independent
descriptive content on both sides: the unfair gloss continues past its opening
negation to name injustice, partiality, and deception. It remains, along with other
informative negative definitions. No glosses have been rewritten.
