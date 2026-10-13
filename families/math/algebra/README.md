# Algebra

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
