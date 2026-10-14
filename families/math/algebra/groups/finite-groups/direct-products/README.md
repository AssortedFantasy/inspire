# Direct products of cyclic groups

Original procedural examples of C_n1 x ... x C_nr, with two to four
independently sampled factor orders from 2 through 16. Each output defines the
coordinatewise modular addition, identity and cardinality, then supplies a
computed sum, inverse, coordinate and element orders, exponent and cyclicity.
Repeated factor orders are intentional: these yield noncyclic abelian groups.

Mathematical reference checked: J. S. Milne, *Group Theory*, version 4.01,
November 2025, [Direct products, pp. 23–24](https://www.jmilne.org/math/CourseNotes/GT.pdf).
The reference establishes coordinatewise products and the finite-product
cardinality. Orders and cyclicity here are computed using modular arithmetic:
ord(a_i) = n_i/gcd(n_i,a_i), the tuple order and exponent are least common
multiples, and the product is cyclic exactly when factor orders are pairwise
coprime. No downloaded dataset or copied source prose is redistributed.

Tests check associativity, identity, inverses, closure and commutativity on small
products, verify every element order by repeated addition for all factor lists
of length two to four with factors 2–4, and check the exponent/cardinality
cyclicity criterion across every factor list in the full generation range.
