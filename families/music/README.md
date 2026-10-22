# Music

Music balances six topics: repertoire, instruments, pitch, harmony, rhythm and
melody. Local weights balance concepts, not record counts: the largest catalogue
does not crowd out generated musical structures. Instrument subtrees and their
descendants use record-count weights, so each imported instrument has the same
conditional probability. Repertoire separates historical recording charts from
classical work metadata. The classical catalogue is
predominantly Western art music, not a survey of all world musical traditions.

## Downloaded catalogues

`repertoire/classical/{chamber,keyboard,orchestral,stage,vocal}` contains 24,749
work records by 220 composers from [Open Opus](https://www.openopus.org/).
The full API 1.20.6 dump was downloaded on 2026-10-13 from
`https://api.openopus.org/work/dump.json`; its SHA256 is
`6e610314c613eba428907a790ffb632446322439c0c2b526305e06d113ca164f`.
The publisher explicitly dedicates **data** to the public domain/CC0; the GPLv3
license of the API software does not govern these data. No API code is copied.

`import_data.py` retains composer, work title/catalogue designation, optional
subtitle and composer period. The selected ensemble category is represented by
the leaf path. It normalizes Unicode NFC
and whitespace, rejects unusable numeric title fragments and corrupt text, and
deduplicates by composer plus case-folded title (226 entries removed). Each
record has a stable content-derived identifier because this dump supplies no
work IDs. Catalogue subdivisions and arrangements remain when their titles
distinguish them. Classifications are explicitly credited to Open Opus, which
acknowledges uneven historical standardization; no composition dates are
inferred from the API's placeholder composer birth dates. The five leaves have
4,743; 5,223; 5,088; 1,252; and 8,443 records, respectively.

`instruments/sound-mechanism/` retains 11 additional instrument names from
[Darius Kazemi's Corpora](https://github.com/dariusk/corpora), revision
`2e7adec11a0561696c236ae1fca82e88be173d42`. The imported path is
`data/music/instruments.json`, SHA256
`a2be9c68eb6a536bd37961c78598cbf7c4a303b77ae495f24da27aeff01d2527`.
From the 55 CC0 names, two piano-size variants and 42 names already represented
in MIMO are omitted. Name comparisons ignore case, punctuation and spacing.
Both importers apply this deduplication and rebuild the record-count weights.
The retained names are Air horn, Calliope, Cello, Didgeridoo, French horn,
Kick drum, Pan flute, Piano, Piccolo, Pipe organ and Washtub bass.

Samples print the sourced names. The mechanism grouping remains in the tree:
air-driven (7), strings (3) and struck membranes (1), across seven leaves.
Original descriptive prose has been removed from the records. These are broad
practical categories, not complete Hornbostel–Sachs classifications. Source
spellings `saxaphone` and `mellotran` are normalized before deduplication.

`instruments/catalogue/` adds 2,135 instrument kinds from the multilingual
[MIMO vocabulary exported by DOREMUS](https://github.com/DOREMUS-ANR/knowledge-base/tree/9cfb5c9cc3147084c9ac01161102f8ac368b8f1b/vocabularies).
The seven branches are electronic (24), keyboard (73), mechanical (47), mirlitons
(4), percussion (743), stringed (524), and wind (720). Keyboard, percussion,
stringed and wind expand into 6, 17, 17 and 22 source-supported families,
respectively; the other three are leaves. Thus the catalogue has 65 final leaves.
Each record retains its
MIMO concept ID, English preferred name, useful broader kind and up to four
English alternate names. Useful broader kinds appear in parentheses; redundant
plural classifications and broad electronic/mechanical labels are omitted. Aliases
are comma-separated. Samples omit source URLs and license boilerplate; record
provenance retains source details and the full license is bundled. This adds
globally varied kinds
such as rababa, shakuhachi, kora and regional gongs with actual vocabulary links,
without inferring geographic origins or acoustic mechanisms from their names.
Categories retain the museum vocabulary's historical groupings; they are not
presented as Hornbostel–Sachs classifications. Catalogue leaf weights equal
record counts at every split so every included kind has the same conditional
probability. Multi-parent kinds are placed under the smallest numerical source
family ID, while their emitted text retains every meaningful immediate broader
kind. The `trumps` category includes named jaw harps in the source vocabulary.

The pinned DOREMUS commit is `9cfb5c9cc3147084c9ac01161102f8ac368b8f1b`;
`vocabularies/mop-mimo.ttl` has SHA256
`adebe2bc255e6947971e32bf261018acfe7a96123387ff228247fb25786ca1dd`.
The source contains 43,783 RDF triples, including 2,324 English preferred labels.
The importer excludes instrument components/accessories, unclassified other
instruments, the Gamelan ensemble umbrella, opaque `Others` contexts, root
categories and redundant/administrative umbrellas. Electronic and mechanical
records whose immediate broader labels are the redundant `Electronic` and
`Mechanical` use the explicit family ancestor instead. Sixty-two source umbrella
concepts become branch labels rather than repeated payload records. The audit
also excludes opaque `Medium` (5291), misclassified `Txistu` (3777), conflicting
`Salamuri` (4766), unresolved `Bayan` (6795), and redundant English label/context
concepts 3239 (harp-guitar), 3705 (dulcimer) and 4938 (Jula jula), retaining
3225, 3644 and 5239 (Julajula) instead. Both flute names normalize identically
and share the same immediate source parent; 5239 also retains an explicit
Hornbostel–Sachs link in the source.
Spelling corrections are concept 3613 `Appalchian dulcimer` → `Appalachian
dulcimer` and 4296 `Bass ophocleide` → `Bass ophicleide`, corroborated by source
alternate-language labels and links. Changes remain documented rather than
silently presented as verbatim preferred labels. It uses a real RDF
parser because the file contains predicate continuations and multiline literals.
No definitions, images, museum object descriptions or recordings are imported.
MIMO retains copyright; the DOREMUS vocabulary README and
[BnF's DOREMUS declaration](https://api.bnf.fr/fr/dumps-DOREMUS) explicitly grant
CC BY 4.0 to the MIMO reference vocabulary. Every emitted record carries owner,
license, source URL and modification notice; the full license is embedded in
`instruments/catalogue/LICENSE.txt`.

Replay with `python -B families/music/import_mimo.py`, default cache
`target/imports/mimo/`. Its development dependencies are pinned in
`import_requirements.txt`; install them normally or with
`python -m pip install --target target/imports/mimo/python-deps -r families/music/import_requirements.txt`.
They are never required at runtime. Raw RDF and temporary dependencies remain
outside the sampling tree. Cantometrics was inspected but is not included
because its dataset license metadata is inconsistent.

Rebuild the catalogues with `python -B families/music/import_data.py`.
The default cache is `target/imports/`; `--cache` accepts another location.
Downloads stay outside the sampling tree. Checksums reject upstream changes;
reproduction requires the pinned Open Opus snapshot, since its endpoint is live.
The committed records remain the offline source of truth. `LICENSE` records the
data dedications; record provenance preserves download URLs and revisions.

## Generated structures

Each generator produces a concrete symbolic musical object using only the seeded
RNG. Diatonic pitches use the correct letter spellings, with enharmonic tonics
chosen to avoid needless accidentals. Melody and meter outputs are ABC notation.

- `pitch/scales/`: seven diatonic modes and four symmetric interval cycles.
- `harmony/chords/`: four triad qualities with closed or open inversions; seven
  seventh-chord qualities with inversions and optional drop-2 voicings.
- `harmony/progressions/cadences/`: sixteen major/minor chord sequences including
  secondary dominants, borrowed chords, Neapolitan and Picardy endings.
- `harmony/voice-leading/triads/`: two to five triads in three ordered voices.
  Exhaustive bounded search minimizes total semitone movement per transition
  among distinct-tone closed voicings with bass MIDI 48–72.
- `rhythm/meters/`: four-bar simple, compound and additive phrases with recurring
  motifs, changed responses, rests and sixteenth-note subdivisions.
- `rhythm/patterns/even-pulses/`: cyclic attack grids with exact pulse counts and
  cyclic inter-onset gaps differing by at most one subdivision.
- `rhythm/patterns/drum-grooves/`: kick, snare and hi-hat grids with two-bar
  variations and fills.
- `rhythm/patterns/polymeter/`: independent percussion cycles repeated to their
  least common period.
- `melody/phrases/diatonic/`: two-, four- and eight-bar phrases in five modes and
  five meters. Rhythmic cells, varied contours, transposition, inversion, returning
  motifs and passing notes provide structure; final notes resolve to the tonic.

Tests verify pitch membership and spelling, ordered inversions, mode step cycles,
chord quality, cadential roots, minimum voice movement, exact rhythmic bar
filling, cyclic gaps, percussion cycle lengths and melodic range/resolution.

## Historical chart metadata

`repertoire/recorded/charts/billboard-1959/` preserves the existing 30 entries
from Billboard's 1959 year-end Hot 100. Former path: `music/chart-titles`.
The titles, credited performers and ranks were checked against Wikipedia
revision 1368373981, which cites *The Billboard*, December 14, 1959, page 78,
covering January–November weekly charts. The primary scan returned HTTP 403;
verification used the transcription. Both links and that limitation remain in
each record's provenance. No lyrics, recordings, artwork or reviews are copied.
