# Synthetic identifiers

Six equally likely formats: ISBN-10, ISBN-13, ISSN, ISMN, a generic Luhn string,
and a German IBAN. Every output is labeled synthetic. These are format/checksum
specimens, not assigned publications, registered publishers, active banks,
or usable accounts. No external registry is embedded or queried at runtime.

ISBN-10 and ISSN equally mix ordinary random bodies, constructed X checks, and
constructed zero checks. The latter two use a bounded enumeration of the last
two body digits. ISBN-10 and ISBN-13 choose one-digit registration groups
0, 1, 2, 3, 4, 5 or 7. ISBN-13 uses 978; its publisher/item split is intentionally
absent because registry allocations are not checked. ISMN uses 979-0 and the
correct three-to-seven-digit registrant split determined by its leading digit.
ISSN retains its four-plus-four grouping. Mod-11 checks use X for ten; EAN checks
use alternating 1/3 weights. No old M-prefixed ISMN is produced.

Luhn strings have 8–19 digits and both parities; one quarter force two leading
zeros. They do not claim a payment-card issuer or numbering plan. German IBANs
have country DE, two international check digits, eight numeric bank-code digits,
and ten numeric account digits. Half force three leading account zeros. The
MOD-97-10 check is correct; bank assignment, bank-specific domestic account
checks, and national conversion exceptions are outside this model. Other
countries are excluded rather than filling an arbitrary alphanumeric BBAN.

Primary references: [International ISBN Agency](https://www.isbn-international.org/content/what-isbn/10),
[ISBN manual](https://www.isbn-international.org/sites/default/files/ISBN%20Manual%202012%20-corr.pdf),
[ISSN manual](https://www.issn.org/wp-content/uploads/2021/09/ISSNManual_ENG2015_23-01-2015.pdf),
[ISMN ranges](https://ismn-international.org/ismn/ranges/),
[Luhn's patent US2950048A](https://patents.google.com/patent/US2950048),
[SWIFT IBAN registry](https://www.swift.com/standards/standards-resources?category%5B0%5D=169031),
and [HypoVereinsbank format specification](https://www.hypovereinsbank.de/content/dam/hypovereinsbank/unternehmen/pdf/Downloadcenter/ZV-Formate-EN.pdf).
Tests validate whole strings using separate algorithms, mutate each numeric
position, check published examples, and exercise zero/X/leading-zero boundaries.
