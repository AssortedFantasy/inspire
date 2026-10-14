# Music

Music balances six topics: repertoire, instruments, pitch, harmony, rhythm and
melody. Local weights balance concepts, not record counts: the largest catalogue
does not crowd out generated musical structures. Repertoire separates historical
recording charts from classical work metadata. The classical catalogue is
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

`instruments/sound-mechanism/` transforms 55 CC0 names from
[Darius Kazemi's Corpora](https://github.com/dariusk/corpora), revision
`2e7adec11a0561696c236ae1fca82e88be173d42`, into 53 records. The imported path is
`data/music/instruments.json`, SHA256
`a2be9c68eb6a536bd37961c78598cbf7c4a303b77ae495f24da27aeff01d2527`.
Names are grouped by sound mechanism: air-driven (24), strings (21), vibrating
membranes (3), struck solid bars (3), and electronic generation/playback (2).
Samples print the name and, where it adds useful detail, an instrument-specific
mechanism note. These are broad practical categories, not complete
Hornbostel–Sachs classifications. Names `saxaphone` and `mellotran` are corrected
to `saxophone` and `Mellotron`; `grand piano` and `baby grand piano` are omitted
as size variants of `piano`. Traditional synonyms such as violin/fiddle remain.
Air-driven kinds divide into flutes (4), reeds (9), lip-excited instruments (8),
and pressure-fed instruments (3). Strings divide into plucked (12), friction (5),
keyboard-struck (2), and varied-excitation structural families (2). Membranes
separate struck drumheads (2) from voice-excited mirlitons (1). Kazoo is correctly
classified as a voice-excited membrane; an air horn uses a diaphragm driven by
compressed air. Piano hammers and clavichord tangents are distinguished;
dulcimer and zither remain qualified broad types rather than forced into one
excitation mechanism. The tambourine's membrane and jingles are both identified,
with its headless variant qualified. Original Mellotron tape mechanisms and
later digital models are distinguished. The source list is
a modest, uneven selection, not an exhaustive instrument catalogue.

`instruments/catalogue/` adds 2,136 instrument kinds from the multilingual
[MIMO vocabulary exported by DOREMUS](https://github.com/DOREMUS-ANR/knowledge-base/tree/9cfb5c9cc3147084c9ac01161102f8ac368b8f1b/vocabularies).
The seven branches are electronic (24), keyboard (73), mechanical (47), mirlitons
(4), percussion (743), stringed (524), and wind (721). Keyboard, percussion,
stringed and wind expand into 6, 17, 17 and 22 source-supported families,
respectively; the other three are leaves. Thus the catalogue has 65 final leaves.
Each record retains its
MIMO concept ID, English preferred name, useful broader kind and up to four
English alternate names. Samples omit source URLs, license boilerplate and
repeated classifications that match the instrument name; record provenance
retains source details and the full license is bundled. This adds globally varied kinds
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
concepts 3239 (harp-guitar) and 3705 (dulcimer), retaining 3225 and 3644 instead.
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

Each generator produces one kind of concrete symbolic musical object using only
the seeded RNG. Pitch names intentionally denote equal-tempered pitch classes
with sharp enharmonic labels; diatonic staff spellings are not implied.

- `pitch/scales/`: seven diatonic modes and three symmetric interval cycles.
- `harmony/chords/`: four triad qualities with actual closed-position inversions;
  six distinct seventh-chord qualities.
- `harmony/progressions/cadences/`: major-key chord sequences with accurately
  labelled authentic, plagal, deceptive or half endings; voicing is unspecified.
- `harmony/voice-leading/triads/`: three ordered voices connecting related
  diatonic triads. Exhaustive bounded search minimizes total semitone movement
  among distinct-tone closed voicings with bass MIDI 48–72. This is a geometric
  voice-leading model, not a full eighteenth-century counterpoint model.
- `rhythm/meters/`: four-bar simple, compound and additive rhythms with explicit
  duration units, beat grouping, rests, attacks, meter and tempo units.
- `rhythm/patterns/even-pulses/`: cyclic attack grids with exactly the stated
  pulse count and cyclic inter-onset gaps differing by at most one subdivision.
- `melody/phrases/diatonic/`: original two-bar arches with diatonic pitches,
  quarter-note durations, bounded thirds, one apex and tonic endings.

Tests verify pitch membership, inversion bass/order, mode step cycles, chord
quality, cadential endpoints, global minimum voice movement, exact rhythmic
bar filling, cyclic gaps, and melodic contour/step bounds. Tempo and duration
units are printed explicitly so symbolic output is interpretable without audio.

## Historical chart metadata

`repertoire/recorded/charts/billboard-1959/` preserves the existing 30 entries
from Billboard's 1959 year-end Hot 100. Former path: `music/chart-titles`.
The titles, credited performers and ranks were checked against Wikipedia
revision 1368373981, which cites *The Billboard*, December 14, 1959, page 78,
covering January–November weekly charts. The primary scan returned HTTP 403;
verification used the transcription. Both links and that limitation remain in
each record's provenance. No lyrics, recordings, artwork or reviews are copied.
