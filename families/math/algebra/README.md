# Algebra

Algebra is divided by mathematical object: groups, rings, modules, and
polynomials. Generators describe actual objects and include evaluated examples;
they never ask the consumer to perform the calculation. Sibling weights balance
topics, independently of corpus size.

Existing algebra facts retain their stable IDs in the relevant `concepts`
corpora. The old polynomial generator moved to `polynomials/identities`; the
formerly mixed finite-group generator is separated by construction.
Descendant READMEs document the precise CLI paths, bounds, sources, and invariant
checks.

- `groups/`: group facts and seven separate finite-group constructions.
- `polynomials/`: exact symbolic identities and rational factorizations with
  root multiplicities.
- `rings/`: ring facts, quadratic finite fields, and composite residue rings.
- `modules/`: module facts and tensor products of finite sums of cyclic modules.

These generators use exact bounded arithmetic and the supplied RNG. Runtime
requires neither a computer-algebra system nor access to online sources.
Reference notes distinguish original mathematical explanations from imported
source material; a reference does not imply that prose was downloaded.
