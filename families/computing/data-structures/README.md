# Data structures

`heaps` emits binary, ternary, and 4-ary integer-priority queues in min and max
order. Six regimes cover paired Floyd/incremental builds, insertion and root
removal cascades, duplicate priorities, priority changes, and mixed workloads.
Stable ties retain each item's original insertion ticket, including after a
priority change. Initial/final arrays bound traces without repeating the full
queue after each operation; sift paths use array indexes.
Build costs count swaps rather than comparisons. Tests compare pops and key
changes with independently sorted queues and verify every parent/child edge.

`hash-tables` emits four equally likely fixed-capacity maps: linear probing,
quadratic probing, Robin Hood hashing, and separate chaining. Integer keys
include zero. Affine hashes permute the residues modulo a small prime, while
constructed colliders expose clusters, wraparound, and lookup termination.
A compact initial layout precedes the consequential operations, with their
actual probe or chain paths, followed by the final physical layout. Probing
tables have five or seven slots; chained examples have at most seven initial
entries. A mixed tail adds at most one operation. There is no resizing or concurrency.

Linear and quadratic tables preserve tombstones until reuse. Quadratic probing
uses distinct offsets i² for i=0 through floor(p/2), so its orbit can exhaust
while other slots remain empty. Robin Hood insertion swaps when the incoming
displacement exceeds the resident's, retaining residents on ties; deletion
backshifts until an empty slot or a zero-displacement resident. Chained updates
retain order; optional move-to-front applies to successful lookups. Tests compare
long mixed workloads with independent maps and exercise full tables, tombstone
reuse, quadratic orbit exhaustion, displacement swaps, and wraparound deletion.

The implementations and examples are original. References:

- MIT 6.006 [Heaps](https://ocw.mit.edu/courses/6-006-introduction-to-algorithms-fall-2011/resources/mit6_006f11_lec04/),
  for the invariant and bottom-up build, generalized here to additional arities.
- MIT 6.006 [Open Addressing](https://ocw.mit.edu/courses/6-006-introduction-to-algorithms-spring-2008/resources/lec7/),
  for linear probing, tombstones, and clustering.
- University of Washington CSE 123 [Hashing](https://courses.cs.washington.edu/courses/cse123/24su/lectures/15/15.pdf),
  for quadratic probing and chaining.
- Pedro Celis, [Robin Hood Hashing](https://cs.uwaterloo.ca/research/tr/1986/CS-86-14.pdf),
  for displacement-based insertion, and Emmanuel Goossaert's
  [backward-shift deletion implementation](https://codecapsule.com/2013/11/17/robin-hood-hashing-backward-shift-deletion/).
