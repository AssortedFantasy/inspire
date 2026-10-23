# RSA arithmetic

Six equally likely constructions show direct decryption, CRT recombination,
Fermat factorization of nearby factors, an unwrapped e=3 cube, ciphertext
multiplication, and decryption of a message divisible by a factor. Direct and
CRT cases include zero and n−1 alongside ordinary representatives.

Distinct odd primes come from 43–499. Public exponents are selected from
3, 5, 7, 11, 17, 257, and 65537 when below n and coprime to λ(n). The e=3
construction selects primes congruent to 2 modulo 3. Nearby-factor cases use
at most four steps in the sorted prime list. Arithmetic uses u64 intermediates
and bounded integer searches. These are unpadded integer RSA primitives, with
no encoding, key generation protocol, or signature scheme.

Tests compare square-and-multiply with repeated multiplication, verify Bezout
identities, exhaust all message residues for pairs of small primes, check a
fixed numerical RSA vector, and test CRT, factorization, cube boundaries,
and multiplicativity independently of output formatting.

[RFC 8017, sections 3 and 5.1](https://www.rfc-editor.org/rfc/rfc8017.html)
specifies λ(n), the inverse relation, RSAEP/RSADP, and CRT recombination. The
Fermat construction is the integer identity n=a²−b²=(a−b)(a+b). No reference
prose or code is embedded.
