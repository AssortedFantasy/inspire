# People

Factual person profiles grouped by discipline and role. Arts separates literature,
visual art, music and performance; science separates formal, physical and life
sciences; thought, society and technology cover scholarship, social contributions
and technical work. Multi-disciplinary people appear once, in one browsing role.

The original 132 compact contribution profiles are retained in `curated.json`,
with their original IDs, text and references. Every original is explicitly mapped
to a reviewed Wikidata QID; full-name variants and homonyms are documented there.
Profiles keep their identities when moved. Their descriptions use Nobel Prize
Outreach, MacTutor, museums, the National Park Service and National Inventors
Hall of Fame references, without reproducing biographies or images.

## Downloaded source

The expanded corpus comes from Issa Annamoradnejad and Rahimberdi Annamoradnejad,
*Age dataset: A structured general-purpose dataset on life, work, and death of
1.22 million distinguished people* (2022), DOI `10.36190/2022.82`.
The five [AgeDataset V1 CSV files](https://github.com/Moradnejad/AgeDataset) were
actually downloaded and scanned: 1,223,009 source rows. `sources.json` pins all
download URLs to Git revision `776b4811e992ed593a59c01ff327dcc056e3ab4d` and
records their SHA-256 checksums. Raw downloads stay in `target/people-sources/`.

The source README declares CC BY 4.0; its repository also contains an MIT notice.
Both complete source notices are retained in `LICENSE`, which the executable
embeds for `--data-licenses`. English names and short descriptions are copied
from the dataset, with whitespace and final punctuation normalized; sampled
profiles contain only the name and description. Source URLs, licenses and
modification notes stay in provenance and the bundled notices. The source's
inferred birth/death years, gender and death circumstances are omitted. Years
already present in a source description remain source metadata and are not
independently asserted or expanded.

## Selection and reproduction

```sh
python -B families/people/import_data.py
python -B families/people/verify_import.py
```

The importer verifies every downloaded input checksum and preserves the originals.
The initial selection added up to 120 unique people per role. Explicit occupation terms in source
descriptions determine role membership; the dataset's broad inferred occupation
column is not used. Fictional/mythological or disputed identity descriptions and
current-status claims are filtered. Multi-role placement follows the documented
order in `ROLES`, with original profiles taking priority. QIDs deduplicate people
throughout the complete family, including homonyms with similar names.
`selection.json` pins the resulting role/QID order. Review exclusions are applied
after this bounded selection, so rebuilding cannot silently introduce an
unreviewed replacement. Additional selected identities require their own review.
`corrections.json` records bounded source-grounded corrections, exclusions and
their primary references. These are applied before role classification; source
errors in a clinical role or surname are corrected without generating bulk bios.

Selection cycles through source `Country` buckets to reduce dominance by large
national datasets. These are historical and modern citizenship-like source
labels; they are never presented as birthplace, ethnicity or modern nationality.
Each bucket retains a bounded prefix in source QID order. This heuristic supports
variety and recognizable early source entities; it is not a notability score.

All child weights equal their descendant record counts, so every person has the
same conditional probability when sampling `people`, at every folder depth.
The incoming `people` weight remains an independent top-level editorial choice.

Coverage inherits Wikipedia/Wikidata and source-collection biases. English short
descriptions vary in detail, historical citizenships can fragment geographic
buckets, and the 2022 deceased-person dataset omits recent deaths and most living
contributors. The preserved originals include living people. Source descriptions
can contain errors; the corpus does not claim that all statements were separately
verified against primary biographies. Leaf reviews check actual emitted material.
