# Galois fields

Exact polynomial models of F8, F16, F64, F27 and F125. This leaf focuses on
degree-three and higher extensions; `rings/quadratic-fields` covers quadratic
arithmetic. It selects primitive elements, full-degree nonprimitive elements,
elements of proper nonprime subfields, full-degree trace-zero elements, and
full-degree norm-one elements in odd characteristic before choosing a
presentation. Norm-one sampling uses F27 and F125, where the norm constraint
cuts down the nonzero elements.

Outputs give a minimal polynomial and a Frobenius orbit, a trace/norm pair, or
the embedding degree of a proper subfield. Primitive refers to multiplicative
order q−1, independently of merely generating the extension as a field.
Trace and norm always run from the displayed ambient field K to its prime
field, including when the chosen element lies in a proper subfield.

Five fixed irreducible moduli bound all searches by 125 elements and degree 6.
The supplied RNG selects from explicitly enumerated finite candidate sets.
No floating point, external state, arbitrary-size extensions, or random
polynomial rejection is used.

Tests independently divide by all possible monic factors through half-degree,
check minimal-polynomial roots and irreducibility, verify field operations and
subfield sizes, and compare trace/norm with the trace/determinant of the
multiplication matrix. All five sampling regimes must be populated.

Reference: Keith Conrad, [Finite fields](https://kconrad.math.uconn.edu/blurbs/galoistheory/finitefields.pdf),
especially the polynomial construction, cyclic multiplicative group,
Frobenius conjugates, subfields, trace and norm. Original computed examples;
no source prose or dataset is embedded.
