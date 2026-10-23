# IPv4 subnets

Five forms emit equal splits, largest-first variable-length allocations,
point-to-point /31 links, exact range covers, and prefix subtraction. Parent
blocks use private 10/8 addresses with /18–/26 prefixes, including boundaries
inside octets. LAN requests mix full capacity with one host beyond the capacity
of the next smaller block. Free space is emitted as an exact CIDR cover.

IPv4 intervals and counts use 64-bit arithmetic, including the full /0 test.
The greedy range cover chooses the largest aligned block that fits. Tests
compare its block count with independent dynamic programming over short ranges,
check exact coverage and nonmergeability, verify LAN request minima, and check
generated allocations remain within their parents.

LAN tables reserve the network and directed-broadcast addresses. /31 output
models point-to-point links with both endpoint addresses usable under RFC 3021;
/32 entries in a range cover denote single addresses. Address-count columns
in range covers include all addresses. There is no IPv6, routing policy, DHCP,
or claim that an emitted private block describes an existing network.

References: IETF [RFC 4632](https://www.rfc-editor.org/rfc/rfc4632.html),
especially §3.1, for prefix notation and address-block sizes, and
[RFC 3021](https://www.rfc-editor.org/rfc/rfc3021.html) for /31 endpoint
semantics. These are original computed artifacts; no source prose is embedded.
