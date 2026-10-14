# Probability

Two leaves separate finite distributions from stochastic processes. Their
outputs state actual probabilities and exact rational summaries, without
asking the consuming agent to solve a problem.

`discrete-distributions/finite-mass-functions` samples 3–7 distinct integer
outcomes from -9 through 12 and positive integer weights from 1 through 12.
It prints the complete mass function, first two moments, variance, positive
outcome probability and all modes. Tests independently express the variance
as a weighted sum of pairwise squared differences across 2,000 distributions.

`markov-chains/absorbing` uses two transient states and two absorbing states.
Each transient row has four strictly positive rational probabilities sharing
a denominator from 6 through 20. Positive exit probabilities ensure eventual
absorption. Exact integer inversion of `I-Q` gives both absorption probabilities
and expected hitting times. Tests check first-step equations in 2,000 chains
and independently propagate probability mass and survival time in a fixed case.

Original implementations and wording, with reference context checked against
[Grinstead and Snell, Introduction to Probability, chapters 6 and 11](https://chance.dartmouth.edu/teaching_aids/books_articles/probability_book/pdf.html).
No source prose or code is copied; these are bounded procedural constructions,
not imported corpora.
