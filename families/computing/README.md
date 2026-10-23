# Computing

Nine generators emit state machines, data-structure operations, executable
queries, encodings, networks, two-thread traces, and exact binary32 arithmetic.
The family has weight 2. Its seven branches have equal local weights;
`automata` and `data-structures` each divide their share equally between two
leaves. Existing sorting, graph, and dynamic-programming traces remain under
`procedures/algorithms`.

- `automata/cellular` mixes extinction, shifts and oscillation, additive rules,
  irregular evolution, Rule 54/110, and traffic/majority dynamics. Initial rows
  include impulses, perturbed repeating backgrounds, domain walls, and random
  configurations. Traffic and majority rows start near half occupancy. Some
  runs compare a single-cell perturbation side by side. Rows show t=0 through
  t=15, stopping at a detected repeat for unpaired runs. Boundaries are finite
  rings or a fixed zero exterior, not a claim about an infinite-lattice class.
- [`automata/dfa`](automata/dfa/README.md) computes recognizers, minimization,
  Boolean products, shortest witnesses, finite codebooks, and NFA subset
  construction. Binary-language topologies include overlapping patterns,
  residue cycles, bounded balances, and suffix memory.
- [`data-structures/heaps`](data-structures/README.md) mixes binary, ternary, and 4-ary min/max heaps,
  stable duplicate priorities, sift cascades, key changes, and paired Floyd
  versus incremental builds. Arrays and actual sift paths follow operations.
- `data-structures/hash-tables` mixes linear and quadratic probing with
  tombstones, Robin Hood displacement and backshift deletion, and separate
  chaining with optional move-to-front lookup. Probe paths include wraparound,
  collision clusters, and bounded unsuccessful insertion.
- [`sql/queries`](sql/README.md) emits complete SQLite inputs, queries, and
  computed result sets. Seven forms cover outer joins, nullable anti-joins,
  aggregates, peer windows, set/bag operations, recursive graphs, and relational
  division, including empty-set boundaries.
- [`networking/subnets`](networking/subnets/README.md) computes equal splits,
  VLSM LAN allocations, /31 links, minimal range covers, and prefix subtraction.
  Prefix lengths include non-byte boundaries and one-host capacity crossings.
- [`encoding/utf8`](encoding/utf8/README.md) mixes pinned Unicode names with
  exact scalar encodings, width boundaries, offsets, malformed inputs, and
  incremental decoding across partial byte sequences.
- `concurrency/schedules` enumerates legal executions before choosing a result
  and witness. Split increments use sequentially consistent atomic loads and
  stores with local arithmetic. Mutex traces use blocking acquisition and
  reverse-order release, including both successful schedules and circular
  waits. Store-buffer traces compare SC with the TSO store-buffer litmus test.
  Each thread performs one store followed by a load of the other location,
  with explicit flush events. These abstract machines contain no language
  data races. The small TSO model covers this litmus test only, without fences,
  multiple pending writes, general instruction reordering, or reclamation.
- `floating-point/ieee754` alternates field decompositions, neighbors around
  powers of two, midpoint ties, gradual underflow, association loss, signed
  zeros, infinities, and raw NaN encodings. Exact finite values use reduced
  integer-times-power-of-two notation. Arithmetic assumes roundTiesToEven.
  Coverage is binary32 only. NaN classification uses the quiet-bit-equals-one
  convention; raw payloads are never claimed to survive arithmetic.

Tests exhaust all elementary rule neighborhoods, check additive identities and
traffic conservation, enumerate split-increment outcomes and lock deadlocks,
and distinguish SC from buffered store-buffer outcomes. Binary32 tests compare
exact decoding, adjacent gaps, midpoint rounding, and arithmetic constructions
with Rust's floating-point operations.

New leaf tests compare heap operations with sorted reference queues, hash
operations with independent maps, DFA minimization with pair-graph searches,
SQL reachability with exhaustive graph closure, CIDR cover sizes with dynamic
programming, and UTF-8 validity with Rust's standard decoder. Model limits,
sources, and development-time verification are recorded beside the leaves.

## References

The implementations generate their own artifacts; no source prose or code is
embedded.

- Stephen Wolfram, [Rule 110](https://www.wolframscience.com/nks/p676--the-rule-110-cellular-automaton/),
  for elementary neighborhood encoding, and the
  [CellularAutomaton documentation](https://reference.wolfram.com/language/ref/CellularAutomaton.html)
  for finite periodic boundaries.
- Owens, Sarkar, and Sewell,
  [A better x86 memory model: x86-TSO](https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-745.html),
  for the operational write-buffer model and store-buffer litmus behavior.
- Oracle's [IEEE arithmetic formats](https://docs.oracle.com/cd/E77782_01/html/E77791/z4000ac019878.html)
  and Rust's [f32 reference](https://doc.rust-lang.org/std/primitive.f32.html)
  for binary32 encoding, rounding, and NaN portability.
