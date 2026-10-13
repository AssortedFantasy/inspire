# Circuits

Self-contained ngspice-style netlists for a loaded voltage divider, RC low-pass
or high-pass filter, damped series RLC low-pass filter, and resistive bridge.
All parts are ideal and positive, every node has a DC path to ground, and each
deck includes an operating point plus a DC sweep or AC and transient analyses.
AC ranges span two decades around the nominal corner/natural frequency; pulse
and transient timing scale with the circuit. The RLC damping ratio is bounded
from 0.25 to 1.0; bridge ratios are balanced or offset by 10%.

Values are in SI units using scientific notation, avoiding SPICE suffix ambiguity.
These are small educational/design stimuli, not validated hardware designs:
no component tolerances, source impedance, parasitics, ratings, or device models.

Syntax references: [ngspice manual](https://ngspice.sourceforge.io/docs.html)
and [official passive-circuit tutorial](https://ngspice.sourceforge.io/ngspice-tutorial.html).
