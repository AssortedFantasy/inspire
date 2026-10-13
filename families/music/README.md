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
subtitle and source ensemble/period classifications. It normalizes Unicode NFC
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
Original descriptions group names by sound mechanism: air columns/reeds (25),
strings (21), vibrating membranes (2), struck solid bars (3), and electronic
generation/playback (2). These are broad practical categories, not complete
Hornbostel–Sachs classifications. Names `saxaphone` and `mellotran` are corrected
to `saxophone` and `Mellotron`; `grand piano` and `baby grand piano` are omitted
as size variants of `piano`. Traditional synonyms such as violin/fiddle remain.
The tambourine's membrane and jingles are both identified. The source list is
a modest, uneven selection, not an exhaustive instrument catalogue.

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
