# Tensor products of finite cyclic modules

Exact tensor products over Z of direct sums with one to three cyclic summands,
each modulus 2–48. Use
Z/mZ ⊗_Z Z/nZ ≅ Z/gcd(m,n)Z and distribution over finite direct sums. Report
the complete decomposition, coefficient matrix of a simple tensor, its additive
order, and an evaluated scalar bilinearity identity.

Bounded rejection sampling excludes zero tensor-product modules and zero sample
tensors, with explicit nonzero fallbacks after 64 attempts. Trivial factors
Z/1Z remain visible and are labeled as zero modules. Module orders fit u64.
Tests check representative independence throughout the modulus bounds, cyclic
generator surjectivity, additivity and scalar multiplication in both tensor
slots, exact element orders by repeated sums, and the maximal tensor size 48^9.

Reference context checked online: [Stacks Project, tensor products](https://stacks.math.columbia.edu/tag/00CV),
particularly distribution over direct sums and right exactness. The cyclic
formula follows by tensoring the presentation Z --m--> Z -> Z/mZ -> 0
with Z/nZ. Original implementation; no source prose or dataset is copied.
