# Computing

Three generators emit cellular-automaton grids, executable two-thread traces,
and exact binary32 arithmetic. The family has weight 2. Its three branches and
their single leaves have equal weights, so each leaf has one third of the
family's share. Existing sorting, graph, and dynamic-programming traces remain
under `procedures/algorithms`.

- `automata/cellular` mixes extinction, shifts and oscillation, additive rules,
  irregular evolution, Rule 54/110, and traffic/majority dynamics. Initial rows
  include impulses, perturbed repeating backgrounds, domain walls, and random
  configurations. Traffic and majority rows start near half occupancy. Some
  runs compare a single-cell perturbation side by side. Rows show t=0 through
  t=15, stopping at a detected repeat for unpaired runs. Boundaries are finite
  rings or a fixed zero exterior, not a claim about an infinite-lattice class.
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
