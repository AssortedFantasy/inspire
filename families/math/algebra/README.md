# Algebra

Generates one numerical identity: distribution over addition or subtraction,
difference of squares, or subtraction of a difference. Inputs are independent
integers from -9 to 9, excluding -1, 0, and 1; they may coincide. Each identity
reuses its inputs where the algebra requires it.

Output follows ordinary operator precedence, retains necessary grouping, and
combines adjacent signs. For example:

```text
8 * (8 - 3) = 8 * 8 + 8 * (-3)
(8 + 6) * (8 - 6) = 8 * 8 - 6 * 6
```
