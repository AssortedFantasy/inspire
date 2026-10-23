# Physics

Three equally weighted subjects contain one generator each. The family has
weight 2 among top-level siblings; internal nodes retain weight 1.

- `mechanics/collisions`: equal-mass exchange, heavy/light impacts, coalescence,
  elastic and dissipative oblique impacts, grazing incidence, and zero normal
  restitution with tangential slip. Smooth impacts vary contact orientation;
  coalescence removes relative motion in both dimensions. Outputs alternate
  laboratory and center-of-mass frames. Masses are 0.1–36 kg and incoming
  components remain below 35 m/s. External impulse, friction and rotation are
  excluded. Eight regimes have equal weight.
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

Computation is bounded and uses only the supplied RNG. Tests check momentum,
restitution, impulse and kinetic-energy balances; ray-matrix determinants,
angle-independent imaging, afocal scaling and telephoto geometry; and wave
dispersion, endpoint conditions and every emitted node/antinode location.

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
