# UTF-8

Five forms emit named scalars, encoding-boundary tables, scalar byte offsets,
malformed byte sequences, and incremental decoding through arbitrary chunk
boundaries. Samples cover all four UTF-8 widths, combining marks, a zero-width
joiner sequence, mathematical and technical symbols, music symbols, and emoji.
Boundary tables include the surrogate gap and U+10FFFF. Malformed inputs mix
overlong encodings, surrogate encodings, out-of-range values, stray continuation
bytes, wrong continuations, invalid leads, and truncation after valid prefixes.

`names.rs` contains the exact Unicode 15.0.0 names of assigned scalars in ten
selected blocks. The Basic Latin selection starts at U+0020; controls and
unassigned points are omitted. `import_names.py` rebuilds the table from Python 3.12's pinned
`unicodedata` 15.0.0 database and refuses other versions. Block boundaries are
the matching Unicode 15.0.0 `Blocks.txt` boundaries. Rebuild or verify offline:

```sh
python -B families/computing/encoding/utf8/import_names.py --check
```

The formatted table SHA-256 is
`c1e1d774fc86943cc6905eb781abcecd600209f0e66a4fe0cf340dff8cc8cfc9`.
The names are reformatted source data; [LICENSE.txt](LICENSE.txt) carries the
Unicode data notice. Other output modes use scalar numbers without inventing
names for values outside this subset.

UTF-16 output contains code units, without byte serialization. Scalar counts
do not claim to count grapheme clusters. The decoder reports the first rejected
sequence's starting byte and does not perform replacement, normalization,
grapheme segmentation, or BOM stripping. U+FEFF remains a scalar in the stream.

Tests exhaust all Unicode scalar values against Rust's UTF-8 and UTF-16
operations, compare validity and first-error offsets with `str::from_utf8` on
every one- and two-byte input and a boundary grid of three- and four-byte inputs,
and check every two-cut segmentation of a mixed-width boundary stream.

References: Unicode [15.0 UCD](https://www.unicode.org/Public/15.0.0/ucd/),
[Unicode encoding forms](https://www.unicode.org/versions/Unicode16.0.0/core-spec/chapter-3/),
IETF [RFC 3629](https://www.rfc-editor.org/rfc/rfc3629.html), and Python's
[`unicodedata`](https://docs.python.org/3.12/library/unicodedata.html).
No source prose or code is embedded.
