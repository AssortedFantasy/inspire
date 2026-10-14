# Polynomial identities

CLI path: `math/algebra/polynomials/identities`.

Samples small expression trees with addition, subtraction, multiplication, and
`x` (sometimes also `y`), then collects terms using exact integer polynomial
arithmetic. One-third of samples reverse the equality to show a decomposition
instead of a simplification. Shapes and operators vary rather than selecting
from fixed identity templates.

Numeric inputs are sampled without replacement from 2 through 12. Derived
coefficients can coincide, but the original expression never repeats a numeric
input. Variables repeat as needed for products and collection of like terms.

Candidates have three to five operations and at least two numeric inputs. Reject
cancellation to zero, constant subexpressions equal to one or minus one, degree
above two, coefficients above 200 in magnitude, and uninteresting results with
only one term or only unit linear coefficients. Both sides together are limited
to 100 characters. After 64 attempts, use a small distributive identity as a
fallback. All sampling uses the supplied RNG.

Output follows ordinary operator precedence, retains necessary grouping, and
combines adjacent signs. For example:

```text
(9 * x - 6) * (x - 4) = 9 * x^2 - 42 * x + 24
x * y - 11 * x + 12 * y = y * 12 - x * (11 - y)
```

These are original procedural constructions in the integer polynomial ring
`Z[x,y]`, with monomials of total degree at most two. No external dataset or
Sage code is embedded. The mathematical domain and coefficient representation
were checked against the [SageMath multivariate polynomial ring reference](https://doc.sagemath.org/html/en/reference/polynomial_rings/sage/rings/polynomial/multi_polynomial_ring_base.html)
(Sage 10.8, accessed 2026-10-13). Tests check evaluation on an integer grid,
coefficient round trips, grouping, size limits, and variable diversity.
