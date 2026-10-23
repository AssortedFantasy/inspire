# Cryptography

Four procedural leaves emit exact cipher specimens, modular arithmetic, share
reconstructions, and error-correction traces. The family has weight 2 among
top-level siblings. Its four branches retain weight 1, so each leaf receives
one quarter of the family's share.

- `classical/ciphers` covers Vigenère, Playfair, rail-fence, and columnar
  transposition. Plaintexts come from credited public-domain text. Local
  conventions, corpus attribution, and algorithm references are documented in
  the leaf.
- `modern/rsa-toy` mixes ordinary and CRT decryption, nearby-prime Fermat
  factorization, an unwrapped e=3 cube, multiplicative ciphertexts, and messages
  sharing a factor with n. Distinct primes span 43–499. Operations act directly
  on integers, without message padding or a protocol.
- `secret-sharing/shamir` samples uniform coefficients, including zero, over
  GF(7), GF(11), GF(17), GF(31), GF(101), and GF(257). Thresholds range from two
  through nine. Reconstruction uses random threshold-sized subsets; some
  outputs exhibit two secrets compatible with the same subthreshold shares.
- `error-correction/hamming` emits blocks of 4–8 independent nibbles under
  Hamming(7,4) or extended Hamming(8,4). Cases include clean transmission,
  single-error correction, SECDED double-error rejection, double-error
  miscorrection without overall parity, and burst/interleaving boundaries.

Random keys, coefficients, representatives, payload blocks, and error locations
provide variation beyond a small fixed catalogue. All computation is bounded
and uses only the supplied RNG. Tests check independent cipher inverses, RSA
residues and CRT, every threshold subset and below-threshold ambiguity, and
exhaustive Hamming codebooks and error patterns. Numerical model limits and
primary references are recorded beside each generator.
