# Population dynamics

Four model structures have equal sampling weight: classical Lotka–Volterra
predator–prey, logistic prey with bilinear predation, normalized two-species
competition, and Rosenzweig–MacArthur logistic prey with a Holling II response.
All parameters are synthetic and all populations are continuous densities.
Predator–prey outputs use x for prey and y for predators. Competition densities
are scaled by each species' own carrying capacity.

The classical model has a center, a conserved first integral, and a continuum of
neutral periodic orbits. Its orbit bounds solve the first-integral equation by
100 bisection steps on two fixed, checked brackets. The reported period is the
small-amplitude linear period, not a period estimate for the sampled orbit.

Logistic bilinear samples include predator exclusion, the invasion threshold,
stable coexistence nodes, and stable coexistence foci. Competition samples
include exclusion in either direction, stable coexistence, bistability with a
saddle, invasion boundaries, and the degenerate equilibrium line. A zero
eigenvalue is marked nonhyperbolic rather than assigned a hyperbolic stability
label.

For the saturating model, `x* = mh/(ea-m)`, the invasion threshold is `K = x*`,
and the Hopf threshold is `K_H = h + 2x*`. Samples straddle both thresholds.
Periodic-orbit claims apply to this specific autonomous planar model with
positive parameters and `ea > m`; no trajectories are numerically integrated.
No stochasticity, age structure, space, seasonality, or empirical calibration
is implied. Displayed numbers use approximately six significant digits.

Tests substitute every reported equilibrium into the vector field, compare
analytic Jacobians with centered differences, check the conserved first
integral and orbit extrema, and distinguish nodes, foci, saddles, equilibrium
lines, and the trace crossing at the Hopf threshold.

Sources:

- R. R. Rosales, MIT, [Hamiltonian for the Lotka–Volterra predator–prey model](https://math.mit.edu/classes/18.353J/PSetAnswers/AnswerPSet_2024_04.pdf),
  section 2, the conserved quantity and neutral closed orbits.
- J. R. Chasnov, HKUST, [Mathematical Biology](https://www.math.ust.hk/~machas/mathematical-biology.pdf),
  sections 1.3–1.4, competition and predator–prey models.
- M. L. Rosenzweig and R. H. MacArthur (1963),
  [Graphical Representation and Stability Conditions of Predator–Prey Interactions](https://doi.org/10.1086/282272).
- N. L. P. Lundström and G. Söderbacka,
  [Estimates of Size of Cycle in a Predator-Prey System](https://doi.org/10.1007/s12591-018-0422-x)
  (2018), logistic Holling II equations, Hopf threshold, and unique attracting
  cycle in the introduction.

Equations and synthetic parameters are generated locally; no source prose or
empirical population records are embedded.
