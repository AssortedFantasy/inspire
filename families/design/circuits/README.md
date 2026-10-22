# Circuits

Seventeen original, self-contained ngspice-style netlist generators, each dedicated
to one topology. `design/circuits` remains a branch selector.

- `passive/resistive`: loaded divider, differential bridge, tapped divider.
- `passive/rc`: low-pass, high-pass, loaded low-pass, two-stage ladder, band-pass.
- `passive/rl`: low-pass and high-pass.
- `passive/rlc`: low-pass, band-pass, high-pass, series-resonant notch.
- `active/amplifiers`: inverting, noninverting, and two-input summing VCVS models.

Each output is a runnable deck with a title, components, and one analysis.
Filters choose an AC sweep, pulse response, or sinusoidal transient. AC sweeps
cover two decades below and above relevant corner, pole, or natural frequencies;
transient sources and sampling times follow the circuit's time scale. Resistive
and amplifier decks use DC sweeps. Every circuit has a DC reference.

Component values span E12 decades. Values use at most four significant figures,
with ordinary decimals near unity and compact scientific notation elsewhere.
No computed-property commentary or SPICE convention explanations are emitted.

Loaded RC corners include the actual parallel resistance. The unbuffered ladder
includes interstage loading and uses its true real poles to set the sweep.
The RC band-pass is also unbuffered. RLC damping spans approximately 0.25 to 1.5.
Amplifiers use linear VCVSs with open-loop gain from 1,000 to 1,000,000; finite
closed-loop error is retained. These models have no device parasitics, supply
rails, clipping, or amplifier bandwidth.

`rebuild.py` writes the leaf sources and topology weights. The generators share
the independent test solver in `validate.rs`. Format generated Rust with
`rustfmt --edition 2024`. Tests independently solve 200 emitted decks per topology
at DC and multiple AC frequencies, checking transfer functions, voltages,
currents, loading, resonance, connectivity, and analysis directives. Summing
inputs are excited separately; VCVS polarity and finite input error are checked.
Ancestor weights match topology counts so each circuit has equal probability.

Syntax references checked 2026-10-13: [official ngspice tutorial](https://ngspice.sourceforge.io/ngspice-tutorial.html)
(passive DC, transient, AC, amplifier sections) and [ngspice manual](https://ngspice.sourceforge.io/docs/ngspice-manual.pdf).
These are references for original generators; no external netlist corpus is copied.
