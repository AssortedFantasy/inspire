# Modules

Two separate leaves cover module definitions and concrete finite tensor products:

- `math/algebra/modules/concepts`: source-checked equations and worked
  examples about modules, submodules, quotients, and tensor products. Rings and
  scalar actions are unital; tensor products of two left modules assume a
  commutative base ring. The legacy IDs `module`, `tensor-product`, and
  `tensor-quotient` are preserved.
- `math/algebra/modules/finite-cyclic-tensors`: exact tensor products over Z of
  one to three cyclic summands per input, with moduli 2–48. Outputs select the gcd
  decomposition, a nonzero simple tensor with its additive order, or evaluated
  bilinearity. Zero examples are rejected with bounded nonzero fallbacks.

Each leaf has the same default sibling weight. Facts are sampled uniformly
within `concepts`; generator examples use the supplied RNG and require no
runtime downloads. Sources are primary Stacks Project references recorded in
the corpus provenance and generator README. The corpus contains factual notation
and calculations; the generator uses bounded exact arithmetic.

Sample either exact CLI path with `inspire --family <path> --seed 42`.
