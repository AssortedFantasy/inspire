# Probability

Two leaves separate finite distributions from stochastic processes. Their
outputs state actual probabilities and exact rational summaries, without
asking the consuming agent to solve a problem.

`discrete-distributions/finite-mass-functions` samples 3–7 distinct integer
outcomes from -9 through 12 and positive integer weights from 1 through 12.
It prints the complete mass function followed by one focused view: mean and
variance, positive-outcome probability and modes, or a cumulative probability.
Tests independently express the variance
as a weighted sum of pairwise squared differences across 2,000 distributions.

`markov-chains/absorbing` uses two to four transient states and two absorbing
states. Random sparse transition supports vary the graph itself, including
loops, one-way connections, disconnected transient classes and competing exits.
Rows share a denominator from 6 through 12, with a positive exit from every
transient state. Exact rational elimination gives absorption probabilities and
expected hitting times; each output shows the transition matrix and one of
those summaries. Tests check exact first-step equations in 2,000 chains and
independently propagate mass and survival time from every initial state in
100 chains. Positive exits bound expected absorption time by the denominator.

Original implementations and wording, with reference context checked against
[Grinstead and Snell, Introduction to Probability, chapters 6 and 11](https://chance.dartmouth.edu/teaching_aids/books_articles/probability_book/pdf.html).
No source prose or code is copied; these are bounded procedural constructions,
not imported corpora.
