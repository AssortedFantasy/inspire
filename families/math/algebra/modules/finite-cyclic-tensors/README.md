# Tensor products of finite cyclic modules

Exact tensor products over Z of direct sums with one to three cyclic summands,
each modulus 2–48. Use Z/mZ ⊗_Z Z/nZ ≅ Z/gcd(m,n)Z and distribution over
finite direct sums. Each sample selects one view: the decomposition,
a nonzero simple tensor and its order, or a scalar bilinearity identity.
Coordinates follow the lexicographic order of summand pairs. Trivial Z/1Z factors
and their forced zero coordinates disappear from the displayed decomposition.

Bounded rejection sampling excludes zero tensor-product modules and zero sample
tensors, with explicit nonzero fallbacks after 64 attempts. Module orders fit
u64. Tests check representative independence throughout the modulus bounds,
cyclic generator surjectivity, additivity and scalar multiplication in both
tensor slots, exact element orders by repeated sums, and maximal size 48^9.

Reference context: [Stacks Project, tensor products](https://stacks.math.columbia.edu/tag/00CV).
The cyclic formula follows by tensoring Z --m--> Z -> Z/mZ -> 0 with Z/nZ.
No source prose or external dataset is embedded.
