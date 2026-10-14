# Rational factorizations

CLI path: `math/algebra/polynomials/rational-factorizations`.

Original procedural integer polynomials that split completely over the rationals.
Each output gives an expanded polynomial, its complete primitive linear
factorization, and every rational root with its exact multiplicity.

Degrees range from 2 through 6. Reduced roots have numerators from -5 through 5
and positive denominators from 1 through 3. At least two roots are distinct;
multiplicities range from 1 through 3. Signed integer content ranges from ±1
through ±4. Integer convolution computes coefficients, and up to 64 candidate
draws enforce an absolute coefficient bound of 1500 before a fixed safe fallback.

Tests independently expand every subset of the linear factors, check primitive
content, and evaluate exact derivatives to certify each root multiplicity.
The seed coverage checks degrees, denominators, multiplicities and signed content.

Reference: [SageMath polynomial constructions](https://doc.sagemath.org/html/en/constructions/polynomials.html)
documents factorization over rational polynomial rings and roots with multiplicity.
Reference checked October 13, 2026; the public documentation page is unversioned.
No external dataset or reference prose is imported; all examples are generated
by original arithmetic.
