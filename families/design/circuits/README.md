# Circuits

Twelve original, self-contained ngspice-style netlist generators, each dedicated
to one topology. `design/circuits` remains a branch selector.

- `passive/resistive`: loaded divider, differential bridge, tapped divider.
- `passive/rc`: low-pass, high-pass, loaded low-pass, unbuffered two-stage ladder.
- `passive/rlc`: series low-pass, band-pass, high-pass.
- `active/amplifiers`: inverting and noninverting linear finite-gain models.

The original five topologies remain. Added circuits increase structural range,
rather than just multiplying component values. Every deck states its scope and
calculated properties, has positive passive values and a DC reference, and
includes an operating point and DC or AC analysis. Existing first-order and RLC
filters also retain pulse and transient analyses. SI scientific notation avoids
SPICE suffix ambiguity. AC sweeps cover two decades below and above relevant
corner, pole, or natural frequencies.

Loaded RC corners include the actual parallel resistance. The unbuffered ladder
includes interstage loading, with its quadratic transfer function and true real
poles. RLC damping spans 0.25 to 1.5; natural frequency is explicitly distinguished
from a -3 dB corner. Amplifiers use a VCVS with gain 100,000, including finite
closed-loop error; they have no supply rails, clipping, bandwidth, or device model.
No circuit claims hardware suitability, component ratings, tolerances, source
impedance, parasitic behavior, or safety margins.

`rebuild.py` writes the twelve leaf sources and copies the canonical `validate.rs`
beside each generator, so leaves remain independently movable. Deck values use
six significant digits; internal calculations retain full precision. The tests
solve 200 instances per topology at DC and multiple AC frequencies, comparing
transfer functions, voltages, currents, loading, damping, and displayed claims
with tolerances that account for rounded deck values. Inductors are exact shorts
at DC, and VCVS signs and finite input error are checked alongside connectivity
and element directives.

Syntax references checked 2026-10-13: [official ngspice tutorial](https://ngspice.sourceforge.io/ngspice-tutorial.html)
(passive DC, transient, AC, amplifier sections) and [ngspice manual](https://ngspice.sourceforge.io/docs/ngspice-manual.pdf).
These are references for original generators; no external netlist corpus is copied.
