# Discrete signals

Eight or sixteen integer samples: an off-grid step, impulse, Nyquist alternation,
ramp, rectangular pulse, sparse Haar expansion, or ordinary bounded sequence,
with equal regime probability. Haar and Hadamard views are equally likely.

`H_n` is the unnormalized Sylvester matrix, with entry
`H[k,i] = (-1)^popcount(k & i)`, indexed from zero. It is Hadamard order,
not sequency order; applying it twice gives `n*x`.

Haar uses adjacent averages `(a+b)/2` and details `(a-b)/2`, recursively on
the averages. Coefficients are the final average followed by details from
coarsest to finest, left to right within a scale. They are exact reduced dyadic
fractions; this is the averaging normalization, not an orthonormal transform.
No sampling rate, physical frequency, padding, or periodic extension is implied.

References: [MathWorks transform conventions](https://www.mathworks.com/help/signal/ug/discrete-walsh-hadamard-transform.html)
and [Haar transform](https://www.mathworks.com/help/wavelet/ref/haart.html).
Tests compare the butterfly with every matrix entry, check Parseval and both
inverse identities, and independently check Haar coefficient supports.
