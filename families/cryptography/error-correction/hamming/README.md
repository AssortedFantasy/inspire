# Hamming traces

Multiword even-parity Hamming(7,4) and extended Hamming(8,4) traces. Check bits
occupy positions 1, 2, 4; a nibble's most significant bit goes in position 3,
then 5, 6, 7. Position 8 is overall parity. Printed codewords run left to right
in increasing position order. Syndromes print as the binary position number.

Sampling mixes clean blocks, single errors in data and check bits, overall
parity errors, and detected double errors. Paired traces expose the (7,4)
decoder's double-error miscorrection beside SECDED rejection. Burst traces
compare row-major transmission with column interleaving at lengths one below,
equal to, and one above the lane count. `WRONG` marks decoded data that differs
from the transmitted payload, including failures beyond SECDED's two-error
regime. Payloads contain 4–8 independent nibbles. Clean blocks retain a 1/12
share of draws and print the codewords and shared zero syndrome in one compact
specimen instead of repeating identical decoder rows.

Tests enumerate both codebooks from independent parity constraints, check
minimum distances 3 and 4, compare every possible received word against its
nearest codewords, exhaust all single and double error patterns, and check
every burst alignment around the interleaving boundary.

Primary source: R. W. Hamming, [*Error Detecting and Error Correcting Codes*,
Bell System Technical Journal 29 (1950), 147–160](https://doi.org/10.1002/j.1538-7305.1950.tb00463.x).
Position layout and extended parity cross-check:
[University of New Brunswick, *Error Correction and the Hamming Code*](https://www.ece.unb.ca/tervo/ece4253/hamming.shtml).
All output is calculated locally; no source text or data is embedded.
