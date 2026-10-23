# Physics

Eight equally weighted subjects contain nine generators. The family has weight
2 among top-level siblings; internal nodes retain weight 1. The two mechanics
leaves split their subject's share equally.

- `mechanics/collisions`: equal-mass exchange, heavy/light impacts, coalescence,
  elastic and dissipative oblique impacts, grazing incidence, and zero normal
  restitution with tangential slip. Smooth impacts vary contact orientation;
  coalescence removes relative motion in both dimensions. Outputs alternate
  laboratory and center-of-mass frames. Masses are 0.1–36 kg and incoming
  components remain below 35 m/s. External impulse, friction and rotation are
  excluded. Eight regimes have equal weight.
- `mechanics/projectiles`: level and cliff launches, inclined landing planes,
  grazing arcs, complementary angles, linear drag with still air or headwind,
  and targets with two, one or no launch angles. Twelve regimes have equal
  weight. Vacuum cases use rounded Earth, Moon, Mars, Titan or Ceres gravity;
  speed scales with sqrt(g) to keep level ranges below 576 m. Cliffs are 5–100 m
  high. Drag cases use Earth gravity, 4–30 m/s launch speeds and the explicit
  ideal acceleration −(v − wind)/τ, spanning weak and strong drag and the
  headwind return boundary. Gravity is constant and landing planes are flat or
  infinitely inclined. Rotation, curvature and lift are excluded.
- `optics/lens-systems`: single converging/diverging lenses, virtual magnifiers,
  2f conjugates, near-focus conjugates, an object exactly at focus, a two-lens
  relay, telephoto pairs, and Galilean/Keplerian beam expanders. Eleven equally
  likely cases give telephoto pairs two shares. Focal-length magnitudes span 10–960 mm;
  near-focus offsets are 1–4%. Paraxial thin-lens ray matrices in air exclude
  aberrations, diffraction, apertures and loss.
- `waves/standing`: fixed-end strings, lightly touched string harmonics,
  open–open pipes, and stopped pipes, each with equal weight. Outputs show a
  spectrum or a single mode's displacement/pressure extrema. Strings span
  0.3–1.8 m, 15–150 N and 0.15–4 g/m; pipes span 0.08–1.8 m in dry air at
  −10–40 °C. Strings are uniform and flexible; pipes have ideal boundaries
  without end corrections. A light touch selects full-string modes with a node
  at the contact. Pitch comparisons use equal temperament at A4 = 440 Hz.
- `circuits/rc-transients`: RC charging, reversal, loading and discharge;
  capacitor sharing including exact charge cancellation; finite pulses,
  periodic ripple and high-pass edges; RL steps, reversals and decay; and
  low/high voltage clamp turnoff. Fourteen regimes have equal weight. Base RC
  components span 100 Ω–100 kΩ and 1 nF–47 µF, with load and second-capacitor
  ratios sampled separately. Inductance spans 100 µH–1 H. Components and
  switches are ideal; clamps have a constant forward drop and block reverse
  current. Tables expose continuity, finite-time extinction or periodic limits.
- `thermodynamics/cycles`: Carnot engines, refrigerators and heat pumps, Otto,
  Diesel, Brayton and Stirling cycles. Fourteen regimes mix ordinary cycles
  with small temperature lifts, weak heating, cutoff limits, maximum-work
  Brayton pressure ratios and zero/partial/perfect regeneration. State tables
  include pressure, specific volume, temperature and relative entropy; heat
  tables use external heat, separating Stirling regeneration. The working
  fluid is a constant-capacity ideal gas with γ = 1.4 and R = 0.287 kJ/(kg K),
  including at high temperature. Cold states span 280–340 K; peak temperatures
  depend on the cycle. All processes follow the stated ideal constraints.
- `acoustics/room-modes`: empty rectangular rigid rooms, cubes, square and
  nearly square floors, 3:2:1 coincidences, corridors, low ceilings, and exact
  or near axial–tangential crossings. Nine geometric regimes have equal weight;
  outputs alternate complete low spectra, mode-family subsets, pressure
  geometry and selected coincidences. Edges span roughly 1.4–26 m and sound
  speed is fixed at 343 m/s. The lossless Neumann eigenproblem excludes the
  zero-frequency uniform field. Mode enumeration proves a sufficient index
  bound for every requested cutoff and retains all ties, including across
  axial, tangential and oblique families. No damping or wall absorption is
  inferred from mode spacing.
- `orbital/transfers`: circular-orbit Hohmann ascent/descent and near-equal
  radii, finite-apex bi-elliptic comparisons around crossover ratios, split
  plane changes at transfer apsides, and circular heliocentric rendezvous
  windows. Eight regimes have equal weight. Local orbits use Earth, Moon or
  Mars; bi-elliptic cases orbit the Sun with outer/inner ratios near 12–20.
  Inclined cases state the common-node geometry and minimize the two-apsis
  split with bounded bracketing/refinement. Solar windows use rounded J2000
  semimajor axes for Mercury through Jupiter. All radii are center distances.
  The model uses point-mass Newtonian gravity and instantaneous burns;
  heliocentric speed mismatches exclude parking-orbit escape and capture.
- `dimensional/buckingham-pi`: twenty equally likely physical relations,
  including drag, waves, heat transport, oscillators, beams, circuits,
  propulsion, blast scaling and diffusion/reaction. Exact rational row
  reduction constructs primitive integer nullspace bases rather than storing
  named groups. Matrices have rank 2–4 across mass, length, time, temperature
  and current; alternate repeating sets expose equivalent reductions. Half
  the samples include exact numerical multipliers under a change of base
  units. These change numerical values of constants too, without implying
  a physically realizable scale model. Shapes, boundary/initial profiles,
  gas composition and omitted dimensionless material ratios are fixed. Waves
  are linear, solids isotropic, drop property ratios fixed, and blast scaling
  fixes an ideal-gas equation of state. Powers requiring positive quantities
  use magnitudes or positive model parameters. The unknown function Phi and
  dimensionless constants are left undetermined by dimensional analysis.

Computation is bounded and uses only the supplied RNG. Tests check momentum,
restitution, impulse and kinetic-energy balances; ray-matrix determinants,
angle-independent imaging, afocal scaling and telephoto geometry; and wave
dispersion, endpoint conditions and every emitted node/antinode location.
New-leaf tests check projectile ODEs, landing/envelope boundaries and drag
limits; Kirchhoff, charge, energy and periodic closure; thermodynamic process
constraints, first-law balances, efficiency identities and cycle limits;
Helmholtz fields, rigid boundaries, orthogonality and complete mode ordering;
orbital energy/angular momentum, time reversal, plane-change minima and
rendezvous phase closure; and exact dimension cancellation, rank/nullity,
basis independence and numerical unit invariance.

## References

These are original implementations of physical equations; no source text or
code is redistributed.

- J. Peraire and S. Widnall, MIT 16.07, [Linear Impulse and Momentum;
  Collisions](https://ocw.mit.edu/courses/16-07-dynamics-fall-2009/a867999737497c79fd8ffe97641c503f_MIT16_07F09_Lec09.pdf),
  especially the restitution and oblique-collision sections.
- C. Warde, MIT 6.161, [Geometric
  Optics](https://web.mit.edu/6.161/www/Geometric-Optics-9-07.pdf),
  ray-transfer matrices, multiple lenses and telescopes.
- OpenStax, University Physics Vol. 1, [string wave
  speed](https://openstax.org/books/university-physics-volume-1/pages/16-3-wave-speed-on-a-stretched-string),
  [standing
  waves](https://openstax.org/books/university-physics-volume-1/pages/16-6-standing-waves-and-resonance),
  [sound speed](https://openstax.org/books/university-physics-volume-1/pages/17-2-speed-of-sound)
  and [pipe
  modes](https://openstax.org/books/university-physics-volume-1/pages/17-4-normal-modes-of-a-standing-sound-wave).
  Dry-air speed uses 331 sqrt((T + 273.15)/273.15) m/s.
- MIT 8.01L, [Projectile
  Motion](https://www.ocw.mit.edu/courses/8-01l-physics-i-classical-mechanics-fall-2005/94254977fd72307b0206470bf33c9532_lec6.pdf),
  and R. Fitzpatrick, UT Austin, [linear-drag
  trajectories](https://farside.ph.utexas.edu/teaching/336k/Newtonhtml/node29.html).
  Rounded gravities follow NASA planetary factsheets and NASA
  [Titan](https://ntrs.nasa.gov/api/citations/20150014581/downloads/20150014581.pdf)
  and [Ceres](https://www.kiss.caltech.edu/workshops/3d/presentations/mueller.pdf)
  engineering notes.
- MIT 6.071, [RC/RL
  transients](https://ocw.mit.edu/courses/6-071j-introduction-to-electronics-signals-and-measurement-spring-2006/4b6a5fe626d65de20ceb97d8d23f38a4_transient1_rl_rc.pdf),
  and Texas Instruments, [inductive
  discharge](https://www.ti.com/document-viewer/lit/html/SNVAA45).
- MIT Unified Engineering, [Carnot](https://web.mit.edu/16.unified/www/FALL/thermodynamics/notes/node24.html),
  [Otto](https://web.mit.edu/16.unified/www/FALL/thermodynamics/notes/node26.html),
  [Diesel](https://web.mit.edu/16.unified/www/FALL/thermodynamics/notes/node27.html)
  and [Brayton](https://web.mit.edu/16.unified/www/FALL/thermodynamics/notes/node28.html)
  cycles; OpenStax, [entropy and Stirling
  regeneration](https://openstax.org/books/university-physics-volume-2/pages/4-6-entropy).
- COMSOL, [Eigenmodes of a
  Room](https://doc.comsol.com/6.0/doc/com.comsol.help.models.mph.eigenmodes_of_room/models.mph.eigenmodes_of_room.pdf),
  rectangular-cavity eigenfrequencies and rigid-wall pressure boundaries.
- C. W. Brunner, [Conceptual Design of a Communications Relay Satellite for a
  Lunar Sample Return Mission](https://ntrs.nasa.gov/api/citations/20050232849/downloads/20050232849.pdf),
  NASA/CR-2005-213034, §2.3.1; NASA OpenMDAO, [Hohmann transfer with split plane
  change](https://openmdao.org/newdocs/versions/latest/examples/hohmann_transfer/hohmann_transfer.html);
  NASA/JPL, [launch
  windows](https://www.jpl.nasa.gov/edu/resources/lesson-plan/lets-go-to-mars-calculating-launch-windows/).
  Constants use JPL [astrodynamic
  parameters](https://ssd.jpl.nasa.gov/astro_par.html); circular solar radii use
  JPL [approximate planetary elements](https://ssd.jpl.nasa.gov/planets/approx_pos.html).
- MIT 2.25, [The Buckingham Pi Theorem in Dimensional
  Analysis](https://ocw.mit.edu/courses/2-25-advanced-fluid-mechanics-fall-2013/c0a4521f55e9191d557c167e99e97469_MIT2_25F13_The_Buckingham.pdf),
  and MIT Unified Engineering, [dimensional analysis and dynamic
  similarity](https://ocw.mit.edu/courses/16-01-unified-engineering-i-ii-iii-iv-fall-2005-spring-2006/b0d2168f3fc45735017ff56c05bd9a0c_f04_fall.pdf).
