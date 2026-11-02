# Truth tables

Random binary nodes have distinct syntactic operands. If independently sampled
operands coincide, rotating every atom in the right operand preserves its shape
and depth while making it distinct. Law substitutions are also distinct, and
contraposition avoids complementary substitutions that would collapse its two
sides to the same syntax.
These checks apply to random construction, not the general AST constructors:
excluded middle, Peirce's law, and other deliberate identities retain repeated
subexpressions. Tautology and contradiction regimes remain equally frequent.

Tests check operand distinctness across random depths and all five regimes,
preserve explicit identity semantics, and compare the evaluator and regime
invariants with independent bit-vector semantics.
