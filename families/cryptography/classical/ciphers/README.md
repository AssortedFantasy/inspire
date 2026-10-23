# Classical ciphers

Four architectures share 24 exact, contiguous passages from Lewis Carroll,
*Alice's Adventures in Wonderland* (1865), chapters I–II. Text is taken from
[Project Gutenberg ebook 928](https://www.gutenberg.org/files/928/928-h/928-h.htm).
Whole passages alternate with exact contiguous spans of 10–32 words. The latter
are sampled uniformly across all eligible spans, rather than giving each short
source equal weight. The generator removes punctuation and spaces and uppercases letters. It prints
the original passage, ciphertext, necessary key material, and the
author/work/chapter credit.
It does not synthesize sayings or paraphrase the source.

- Vigenère alternates repeating keys and plaintext autokey. Keys are ordinary
  words or random letters, mixing short periods with longer keys.
- Playfair uses a keyword square or a shuffled square. I/J merge. Repeated
  letters and an odd final letter take X, except X takes Q. Decryption retains
  fillers; their removal is inherently ambiguous.
- Rail fence mixes two to seven rails, every cycle offset, and both directions
  between zigzag and row order. The bottom and top turning points get extra
  weight. Offset zero starts on the top rail moving downward.
- Columnar transposition mixes unpadded and complete rectangles. Keyword ties
  resolve left to right. The complete form reports its trailing X count.
  Repeated-letter keywords exercise tied column ranks; incomplete rectangles
  retain unequal column lengths.

Algorithm specifications and worked vectors come from the American Cryptogram
Association: [Vigenère](https://www.cryptogram.org/downloads/aca.info/ciphers/Vigenere.pdf),
[autokey](https://www.cryptogram.org/downloads/aca.info/ciphers/Autokey.pdf),
[Playfair](https://www.cryptogram.org/downloads/aca.info/ciphers/Playfair.pdf),
[rail fence](https://www.cryptogram.org/downloads/aca.info/ciphers/Railfence.pdf),
[complete columnar](https://www.cryptogram.org/downloads/aca.info/ciphers/CompleteColTransposition.pdf),
and [incomplete columnar](https://www.cryptogram.org/downloads/aca.info/ciphers/IncompleteColTransposition.pdf).
No ACA explanatory prose is embedded in output.

Tests check the published vectors, all 600 distinct Playfair digraphs, fence
cycle boundaries and offsets, tied and very short columnar rows, and seeded
specimens against separately implemented decryption. Runtime uses only the
provided RNG and bounded local work on these short passages.
