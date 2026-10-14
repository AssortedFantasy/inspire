"""Write distinct original SPICE generator leaves (development time only)."""

from pathlib import Path

BASE = Path(__file__).resolve().parent
COMMON = '''use rand::{rngs::StdRng, Rng};
use std::f64::consts::PI;

fn pick(rng: &mut StdRng, values: &[f64]) -> f64 {
    values[rng.random_range(0..values.len())]
}
'''

CASES = {
    "passive/resistive/loaded-divider": '''
    let v = pick(rng, &[1.8, 3.3, 5.0, 9.0, 12.0]);
    let r1 = pick(rng, &[1000.0, 2200.0, 4700.0, 10000.0, 22000.0]);
    let r2 = pick(rng, &[1000.0, 3300.0, 6800.0, 10000.0, 47000.0]);
    let load = r2 * pick(rng, &[2.0, 5.0, 10.0, 20.0]);
    let parallel = r2 * load / (r2 + load);
    let out = v * parallel / (r1 + parallel);
    let resistance = r1 * parallel / (r1 + parallel);
    format!("Loaded resistive voltage divider\\n* Ideal voltage source and resistors; values in V and ohms.\\n* DC output {out:.5e} V; loaded Thevenin resistance {resistance:.5e} ohm.\\nV1 in 0 DC {v:.5e}\\nR1 in out {r1:.5e}\\nR2 out 0 {r2:.5e}\\nRload out 0 {load:.5e}\\n.op\\n.dc V1 0 {v:.5e} {:.5e}\\n.print dc v(in) v(out)\\n.end", v / 20.0)
''',
    "passive/resistive/bridge": '''
    let v = pick(rng, &[1.8, 3.3, 5.0, 9.0, 12.0]);
    let r1 = pick(rng, &[1000.0, 2200.0, 4700.0, 10000.0]);
    let r2 = pick(rng, &[1000.0, 3300.0, 6800.0, 10000.0]);
    let r3 = pick(rng, &[2200.0, 4700.0, 10000.0]);
    let imbalance = pick(rng, &[0.8, 0.9, 1.0, 1.1, 1.2]);
    let r4 = r3 * r2 / r1 * imbalance;
    let out = v * (r2 / (r1 + r2) - r4 / (r3 + r4));
    format!("Resistive bridge with differential output\\n* Ideal source and resistors; output is unloaded.\\n* Right-arm ratio is {imbalance} times left-arm ratio; v(left,right) = {out:.5e} V.\\nV1 in 0 DC {v:.5e}\\nR1 in left {r1:.5e}\\nR2 left 0 {r2:.5e}\\nR3 in right {r3:.5e}\\nR4 right 0 {r4:.5e}\\n.op\\n.dc V1 0 {v:.5e} {:.5e}\\n.print dc v(left) v(right) v(left,right)\\n.end", v / 20.0)
''',
    "passive/resistive/tapped-divider": '''
    let v = pick(rng, &[3.3, 5.0, 9.0, 12.0]);
    let r1 = pick(rng, &[1000.0, 2200.0, 4700.0, 10000.0]);
    let r2 = pick(rng, &[1000.0, 3300.0, 6800.0, 10000.0]);
    let r3 = pick(rng, &[1000.0, 2200.0, 4700.0]);
    let tap = v * (r2 + r3) / (r1 + r2 + r3);
    let out = v * r3 / (r1 + r2 + r3);
    let current = v / (r1 + r2 + r3);
    format!("Three-resistor tapped divider\\n* Ideal source; taps unloaded; resistance in ohms.\\n* DC tap {tap:.5e} V; out {out:.5e} V; source current {current:.5e} A.\\nV1 in 0 DC {v:.5e}\\nR1 in tap {r1:.5e}\\nR2 tap out {r2:.5e}\\nR3 out 0 {r3:.5e}\\n.op\\n.dc V1 0 {v:.5e} {:.5e}\\n.print dc v(tap) v(out)\\n.end", v / 20.0)
''',
    "passive/rc/low-pass": '''
    let r = pick(rng, &[1000.0, 2200.0, 4700.0, 10000.0, 22000.0]);
    let c = pick(rng, &[1e-9, 4.7e-9, 10e-9, 47e-9, 100e-9, 1e-6]);
    let tau = r * c;
    let fc = 1.0 / (2.0 * PI * tau);
    format!("RC low-pass filter\\n* Ideal source; unloaded output; values in SI.\\n* Time constant {tau:.5e} s; -3.0103 dB corner {fc:.5e} Hz.\\nV1 in 0 DC 0 AC 1 PULSE(0 1 0 {:.5e} {:.5e} {:.5e} {:.5e})\\nR1 in out {r:.5e}\\nC1 out 0 {c:.5e}\\n.op\\n.ac dec 40 {:.5e} {:.5e}\\n.tran {:.5e} {:.5e}\\n.print ac vm(out) vp(out)\\n.print tran v(in) v(out)\\n.end", tau / 100.0, tau / 100.0, tau * 5.0, tau * 10.0, fc / 100.0, fc * 100.0, tau / 100.0, tau * 20.0)
''',
    "passive/rc/high-pass": '''
    let r = pick(rng, &[1000.0, 2200.0, 4700.0, 10000.0, 22000.0]);
    let c = pick(rng, &[1e-9, 4.7e-9, 10e-9, 47e-9, 100e-9, 1e-6]);
    let tau = r * c;
    let fc = 1.0 / (2.0 * PI * tau);
    format!("RC high-pass filter\\n* Ideal source; output resistor provides a DC ground path; values in SI.\\n* Time constant {tau:.5e} s; -3.0103 dB corner {fc:.5e} Hz.\\nV1 in 0 DC 0 AC 1 PULSE(0 1 0 {:.5e} {:.5e} {:.5e} {:.5e})\\nC1 in out {c:.5e}\\nR1 out 0 {r:.5e}\\n.op\\n.ac dec 40 {:.5e} {:.5e}\\n.tran {:.5e} {:.5e}\\n.print ac vm(out) vp(out)\\n.print tran v(in) v(out)\\n.end", tau / 100.0, tau / 100.0, tau * 5.0, tau * 10.0, fc / 100.0, fc * 100.0, tau / 100.0, tau * 20.0)
''',
    "passive/rc/loaded-low-pass": '''
    let r1 = pick(rng, &[1000.0, 2200.0, 4700.0, 10000.0]);
    let r2 = r1 * pick(rng, &[1.0, 2.0, 5.0, 10.0]);
    let c = pick(rng, &[10e-9, 47e-9, 100e-9, 1e-6]);
    let tau = r1 * r2 / (r1 + r2) * c;
    let fc = 1.0 / (2.0 * PI * tau);
    let gain = r2 / (r1 + r2);
    format!("Loaded RC low-pass filter\\n* Ideal source with resistive load; values in SI.\\n* DC gain {gain:.5e}; corner {fc:.5e} Hz (-3.0103 dB from DC gain).\\nV1 in 0 DC 1 AC 1\\nR1 in out {r1:.5e}\\nRload out 0 {r2:.5e}\\nC1 out 0 {c:.5e}\\n.op\\n.ac dec 40 {:.5e} {:.5e}\\n.print ac vm(out) vp(out)\\n.end", fc / 100.0, fc * 100.0)
''',
    "passive/rc/dual-low-pass": '''
    let r1 = pick(rng, &[1000.0, 2200.0, 4700.0, 10000.0]);
    let r2 = pick(rng, &[1000.0, 3300.0, 6800.0, 10000.0]);
    let c1 = pick(rng, &[10e-9, 47e-9, 100e-9, 1e-6]);
    let c2 = c1 * pick(rng, &[0.1, 0.5, 1.0, 2.0]);
    let a = r1 * c1 + r1 * c2 + r2 * c2;
    let b = r1 * r2 * c1 * c2;
    let root = (a * a - 4.0 * b).sqrt();
    let slow_tau = (a + root) / 2.0;
    let fast_tau = 2.0 * b / (a + root);
    let low = 1.0 / (2.0 * PI * slow_tau);
    let high = 1.0 / (2.0 * PI * fast_tau);
    format!("Unbuffered two-stage RC low-pass ladder\\n* Ideal voltage source; unloaded output; interstage loading included.\\n* H(s)=1/(1+a*s+b*s^2); a={a:.5e} s, b={b:.5e} s^2.\\n* Real pole frequencies {low:.5e} and {high:.5e} Hz.\\nV1 in 0 DC 1 AC 1\\nR1 in mid {r1:.5e}\\nC1 mid 0 {c1:.5e}\\nR2 mid out {r2:.5e}\\nC2 out 0 {c2:.5e}\\n.op\\n.ac dec 40 {:.5e} {:.5e}\\n.print ac vm(out) vp(out)\\n.end", low / 100.0, high * 100.0)
''',
}

for topology, components, numerator in [
    ("low-pass", "R1 in mid {r:.5e}\\nL1 mid out {l:.5e}\\nC1 out 0 {c:.5e}", "1"),
    ("band-pass", "C1 in mid {c:.5e}\\nL1 mid out {l:.5e}\\nR1 out 0 {r:.5e}", "s*R*C"),
    ("high-pass", "R1 in mid {r:.5e}\\nC1 mid out {c:.5e}\\nL1 out 0 {l:.5e}", "s^2*L*C"),
]:
    CASES[f"passive/rlc/{topology}"] = '''
    let l = pick(rng, &[1e-3, 4.7e-3, 10e-3, 47e-3]);
    let c = pick(rng, &[1e-9, 4.7e-9, 10e-9, 47e-9, 100e-9, 1e-6]);
    let zeta = pick(rng, &[0.25, 0.5, 0.707, 1.0, 1.5]);
    let r = 2.0 * zeta * (l / c).sqrt();
    let f0 = 1.0 / (2.0 * PI * (l * c).sqrt());
    let q = 1.0 / (2.0 * zeta);
    let period = 1.0 / f0;
''' + f'''    format!("Series RLC {topology} filter\\n* Ideal source and parts; unloaded output; values in SI.\\n* Natural frequency {{f0:.5e}} Hz; damping ratio {{zeta}}; Q={{q:.5e}}.\\n* H(s)={numerator}/(1+s*R*C+s^2*L*C).\\nV1 in 0 DC 0 AC 1 PULSE(0 1 0 {{:.5e}} {{:.5e}} {{:.5e}} {{:.5e}})\\n{components}\\n.op\\n.ac dec 40 {{:.5e}} {{:.5e}}\\n.tran {{:.5e}} {{:.5e}}\\n.print ac vm(out) vp(out)\\n.print tran v(in) v(out)\\n.end", period / 1000.0, period / 1000.0, period * 10.0, period * 20.0, f0 / 100.0, f0 * 100.0, period / 200.0, period * 40.0)
'''

CASES["active/amplifiers/inverting"] = '''
    let r1 = pick(rng, &[1000.0, 2200.0, 4700.0, 10000.0]);
    let rf = r1 * pick(rng, &[0.5, 1.0, 2.0, 5.0, 10.0]);
    let a = 100000.0;
    let gain = -a * rf / (rf + r1 * (a + 1.0));
    format!("Inverting finite-gain idealized amplifier\\n* Linear VCVS; gain 1e5, infinite input impedance, zero output impedance; values in SI.\\n* Closed-loop DC and AC gain {gain:.5e}; output unloaded.\\nV1 in 0 DC 0.1 AC 1\\nR1 in inv {r1:.5e}\\nRf inv out {rf:.5e}\\nE1 out 0 0 inv {a:.5e}\\n.op\\n.dc V1 -0.1 0.1 0.005\\n.ac dec 20 1 1e6\\n.print dc v(out)\\n.print ac vm(out) vp(out)\\n.end")
'''
CASES["active/amplifiers/noninverting"] = '''
    let rg = pick(rng, &[1000.0, 2200.0, 4700.0, 10000.0]);
    let rf = rg * pick(rng, &[0.5, 1.0, 2.0, 5.0, 10.0]);
    let a = 100000.0;
    let gain = a / (1.0 + a * rg / (rg + rf));
    format!("Noninverting finite-gain idealized amplifier\\n* Linear VCVS; gain 1e5, infinite input impedance, zero output impedance; values in SI.\\n* Closed-loop DC and AC gain {gain:.5e}; output unloaded.\\nV1 in 0 DC 0.1 AC 1\\nRg inv 0 {rg:.5e}\\nRf out inv {rf:.5e}\\nE1 out 0 in inv {a:.5e}\\n.op\\n.dc V1 -0.1 0.1 0.005\\n.ac dec 20 1 1e6\\n.print dc v(out)\\n.print ac vm(out) vp(out)\\n.end")
'''

def main():
    for path, body in CASES.items():
        leaf = BASE / path
        leaf.mkdir(parents=True, exist_ok=True)
        kind = path.split("/")[-1]
        if "/rc/" in path:
            kind = "rc-" + kind
        elif "/rlc/" in path:
            kind = "rlc-" + kind
        header = COMMON if "/rc/" in path or "/rlc/" in path else COMMON.replace("use std::f64::consts::PI;\n", "")
        text = "// Generated by ../../../rebuild.py; one topology per leaf.\n" + header
        text += "\npub fn generate(rng: &mut StdRng) -> String {\n" + body + "}\n"
        text += '''
#[cfg(test)]
mod validate;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn independent_nodal_analysis_matches_topology() {
        let mut rng = StdRng::seed_from_u64(47);
        for _ in 0..200 {
            validate::verify(&generate(&mut rng), "KIND");
        }
    }
}
'''.replace("KIND", kind)
        (leaf / "generator.rs").write_text(text, encoding="utf-8")
        (leaf / "validate.rs").write_text((BASE / "validate.rs").read_text(encoding="utf-8"), encoding="utf-8")
    print(f"Generated {len(CASES)} individual circuit topologies.")


if __name__ == "__main__":
    main()
