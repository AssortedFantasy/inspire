"""Write original SPICE generator leaves and their independent circuit tests."""

from pathlib import Path

BASE = Path(__file__).resolve().parent
COMMON = '''use rand::{rngs::StdRng, Rng};

fn pick(rng: &mut StdRng, values: &[f64]) -> f64 {
    values[rng.random_range(0..values.len())]
}

fn preferred(rng: &mut StdRng, low: i32, high: i32) -> f64 {
    pick(rng, &[1.0, 1.2, 1.5, 1.8, 2.2, 2.7, 3.3, 3.9, 4.7, 5.6, 6.8, 8.2])
        * 10.0_f64.powi(rng.random_range(low..=high))
}

fn number(value: f64) -> String {
    let scientific = format!("{value:.3e}");
    let (mantissa, exponent) = scientific.split_once('e').unwrap();
    let mantissa = mantissa.trim_end_matches('0').trim_end_matches('.');
    let exponent: i32 = exponent.parse().unwrap();
    if (-2..=2).contains(&exponent) {
        let decimal = format!("{value:.precision$}", precision = (3 - exponent) as usize);
        decimal.trim_end_matches('0').trim_end_matches('.').to_owned()
    } else { format!("{mantissa}e{exponent}") }
}

fn part(name: &str, from: &str, to: &str, value: f64) -> String {
    format!("{name} {from} {to} {}", number(value))
}
'''

FILTER = '''use std::f64::consts::PI;

fn filter_deck(
    rng: &mut StdRng, title: &str, parts: Vec<String>, time: f64,
    low: f64, high: f64, observations: &str,
) -> String {
    let mode = rng.random_range(0..3);
    let source = match mode {
        0 => "V1 in 0 DC 0 AC 1".to_owned(),
        1 => format!(
            "V1 in 0 DC 0 PULSE(0 1 {} {} {} {} {})",
            number(time), number(time / 100.0), number(time / 100.0),
            number(time * 8.0), number(time * 16.0),
        ),
        _ => format!("V1 in 0 DC 0 SIN(0 1 {})", number(1.0 / (2.0 * PI * time))),
    };
    let mut lines = vec![title.to_owned(), source];
    lines.extend(parts);
    lines.push(".op".to_owned());
    if mode == 0 {
        lines.push(format!(".ac dec 40 {} {}", number(low), number(high)));
        lines.push(format!(".print ac {observations} vp(out)"));
    } else {
        lines.push(format!(".tran {} {}", number(time / 100.0), number(time * 32.0)));
        lines.push(".print tran v(in) v(out)".to_owned());
    }
    lines.push(".end".to_owned());
    lines.join("\\n")
}
'''

CASES = {
    "passive/resistive/loaded-divider": '''
    let v = pick(rng, &[1.8, 3.3, 5.0, 9.0, 12.0]);
    let r1 = preferred(rng, 3, 4);
    let r2 = preferred(rng, 3, 4);
    let load = r2 * pick(rng, &[2.0, 5.0, 10.0, 20.0]);
    ["Loaded voltage divider".to_owned(),
        format!("V1 in 0 DC {}", number(v)),
        part("R1", "in", "out", r1), part("R2", "out", "0", r2),
        part("Rload", "out", "0", load), ".op".to_owned(),
        format!(".dc V1 0 {} {}", number(v), number(v / 20.0)),
        ".print dc v(in) v(out)".to_owned(), ".end".to_owned()].join("\\n")
''',
    "passive/resistive/bridge": '''
    let v = pick(rng, &[1.8, 3.3, 5.0, 9.0, 12.0]);
    let r1 = preferred(rng, 3, 4);
    let r2 = preferred(rng, 3, 4);
    let r3 = preferred(rng, 3, 4);
    let r4 = r3 * r2 / r1 * pick(rng, &[0.8, 0.9, 1.0, 1.1, 1.2]);
    ["Resistive bridge".to_owned(), format!("V1 in 0 DC {}", number(v)),
        part("R1", "in", "left", r1), part("R2", "left", "0", r2),
        part("R3", "in", "right", r3), part("R4", "right", "0", r4),
        ".op".to_owned(), format!(".dc V1 0 {} {}", number(v), number(v / 20.0)),
        ".print dc v(left) v(right) v(left,right)".to_owned(), ".end".to_owned()].join("\\n")
''',
    "passive/resistive/tapped-divider": '''
    let v = pick(rng, &[3.3, 5.0, 9.0, 12.0]);
    let r1 = preferred(rng, 3, 4);
    let r2 = preferred(rng, 3, 4);
    let r3 = preferred(rng, 3, 4);
    ["Tapped voltage divider".to_owned(), format!("V1 in 0 DC {}", number(v)),
        part("R1", "in", "tap", r1), part("R2", "tap", "out", r2),
        part("R3", "out", "0", r3), ".op".to_owned(),
        format!(".dc V1 0 {} {}", number(v), number(v / 20.0)),
        ".print dc v(tap) v(out)".to_owned(), ".end".to_owned()].join("\\n")
''',
}

for topology in ["low-pass", "high-pass", "loaded-low-pass"]:
    loaded = topology == "loaded-low-pass"
    parts = ('part("C1", "in", "out", c), part("R1", "out", "0", r)' if topology == "high-pass"
             else 'part("R1", "in", "out", r), part("C1", "out", "0", c)')
    setup = '''let load = r * pick(rng, &[1.0, 2.0, 5.0, 10.0]);
    let tau = r * load / (r + load) * c;''' if loaded else 'let tau = r * c;'
    if loaded:
        parts += ', part("Rload", "out", "0", load)'
    CASES[f"passive/rc/{topology}"] = f'''
    let r = preferred(rng, 3, 4);
    let c = preferred(rng, -9, -6);
    {setup}
    let fc = 1.0 / (2.0 * PI * tau);
    filter_deck(rng, "RC {topology}", vec![{parts}], tau, fc / 100.0, fc * 100.0, "vm(out)")
'''

for topology in ["dual-low-pass", "band-pass"]:
    high = topology == "band-pass"
    parts = ('part("C1", "in", "mid", c1), part("R1", "mid", "0", r1)' if high
             else 'part("R1", "in", "mid", r1), part("C1", "mid", "0", c1)')
    CASES[f"passive/rc/{topology}"] = f'''
    let r1 = preferred(rng, 3, 4);
    let r2 = r1 * pick(rng, &[0.5, 1.0, 2.0, 5.0]);
    let c1 = preferred(rng, -8, -6);
    let c2 = c1 * pick(rng, &[0.1, 0.22, 0.47, 1.0]);
    let a = r1 * c1 + r1 * c2 + r2 * c2;
    let b = r1 * r2 * c1 * c2;
    let root = (a * a - 4.0 * b).sqrt();
    let slow_tau = (a + root) / 2.0;
    let fast_tau = 2.0 * b / (a + root);
    filter_deck(rng, "{'RC band-pass' if high else 'Two-stage RC low-pass'}", vec![
        {parts}, part("R2", "mid", "out", r2), part("C2", "out", "0", c2),
    ], slow_tau, 1.0 / (2.0 * PI * slow_tau * 100.0), 100.0 / (2.0 * PI * fast_tau), "vm(mid) vm(out)")
'''

for topology in ["low-pass", "high-pass"]:
    parts = ('part("L1", "in", "out", l), part("R1", "out", "0", r)' if topology == "low-pass"
             else 'part("R1", "in", "out", r), part("L1", "out", "0", l)')
    CASES[f"passive/rl/{topology}"] = f'''
    let r = preferred(rng, 2, 3);
    let l = preferred(rng, -3, -1);
    let tau = l / r;
    let fc = 1.0 / (2.0 * PI * tau);
    filter_deck(rng, "RL {topology}", vec![{parts}], tau, fc / 100.0, fc * 100.0, "vm(out)")
'''

for topology, parts in [
    ("low-pass", 'part("R1", "in", "mid", r), part("L1", "mid", "out", l), part("C1", "out", "0", c)'),
    ("band-pass", 'part("C1", "in", "mid", c), part("L1", "mid", "out", l), part("R1", "out", "0", r)'),
    ("high-pass", 'part("R1", "in", "mid", r), part("C1", "mid", "out", c), part("L1", "out", "0", l)'),
    ("notch", 'part("R1", "in", "out", r), part("L1", "out", "notch", l), part("C1", "notch", "0", c)'),
]:
    CASES[f"passive/rlc/{topology}"] = f'''
    let l = preferred(rng, -3, -2);
    let c = preferred(rng, -9, -6);
    let zeta = pick(rng, &[0.25, 0.5, 0.707, 1.0, 1.5]);
    let r = 2.0 * zeta * (l / c).sqrt();
    let f0 = 1.0 / (2.0 * PI * (l * c).sqrt());
    filter_deck(rng, "RLC {topology}", vec![{parts}], 1.0 / (2.0 * PI * f0), f0 / 100.0, f0 * 100.0, "vm(out)")
'''

for topology in ["inverting", "noninverting", "summing"]:
    summing = topology == "summing"
    noninverting = topology == "noninverting"
    CASES[f"active/amplifiers/{topology}"] = f'''
    let r = preferred(rng, 3, 4);
    let rf = preferred(rng, 3, 4);
    let a = pick(rng, &[1e3, 1e4, 1e5, 1e6]);
    {('let r2 = r * pick(rng, &[0.5, 1.0, 2.0, 5.0]);' if summing else '')}
    let mut lines = vec!["{topology.capitalize()} VCVS amplifier".to_owned(),
        "V1 in 0 DC 0.1".to_owned(),
        {('part("Rg", "inv", "0", r)' if noninverting else 'part("R1", "in", "inv", r)')},
        part("Rf", "inv", "out", rf),
        format!("E1 out 0 {'in' if noninverting else '0'} inv {{}}", number(a)),
    ];
    {('lines.extend(["V2 in2 0 DC -0.05".to_owned(), part("R2", "in2", "inv", r2)]);' if summing else '')}
    lines.extend([".op".to_owned(), ".dc V1 -0.1 0.1 0.005".to_owned(),
        ".print dc v(in) v(out)".to_owned(), ".end".to_owned()]);
    lines.join("\\n")
'''


def main():
    for path, body in CASES.items():
        leaf = BASE / path
        leaf.mkdir(parents=True, exist_ok=True)
        kind = path.split("/")[-1]
        circuit = path.split("/")[1]
        if circuit in {"rc", "rl", "rlc"}:
            kind = circuit + "-" + kind
        header = COMMON + (FILTER if circuit in {"rc", "rl", "rlc"} else "")
        text = "// Generated by circuits/rebuild.py; one topology per leaf.\n" + header
        text += "\npub fn generate(rng: &mut StdRng) -> String {\n" + body + "}\n"
        text += '''
#[cfg(test)]
mod validate {
    include!("../../../validate.rs");
}

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
    for path, count in [("passive", 14), ("active", 3), ("passive/rc", 5),
                        ("passive/rl", 2), ("passive/rlc", 4), ("passive/resistive", 3)]:
        (BASE / path / "node.toml").write_text(f"weight = {count}\n", encoding="utf-8")
    print(f"Generated {len(CASES)} individual circuit topologies.")


if __name__ == "__main__":
    main()
