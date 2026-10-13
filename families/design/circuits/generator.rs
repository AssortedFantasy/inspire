use rand::{Rng, rngs::StdRng};

fn pick(rng: &mut StdRng, values: &[f64]) -> f64 {
    values[rng.random_range(0..values.len())]
}

fn circuit(rng: &mut StdRng, kind: u32) -> String {
    let voltage = pick(rng, &[1.8, 3.3, 5.0, 9.0, 12.0]);
    let r1 = pick(rng, &[1000.0, 2200.0, 4700.0, 10000.0, 22000.0]);
    let r2 = pick(rng, &[1000.0, 3300.0, 6800.0, 10000.0, 47000.0]);
    let c = pick(rng, &[1e-9, 4.7e-9, 10e-9, 47e-9, 100e-9, 1e-6]);
    match kind {
        0 => format!(
            "Loaded resistive voltage divider\n* Ideal source, ohms; output is measured relative to node 0.\nV1 in 0 DC {voltage}\nR1 in out {r1:.6e}\nR2 out 0 {r2:.6e}\nRload out 0 {:.6e}\n.op\n.dc V1 0 {voltage} {:.6e}\n.print dc v(in) v(out)\n.end",
            10.0 * r2,
            voltage / 20.0
        ),
        1 | 2 => {
            let tau = r1 * c;
            let fc = 1.0 / (2.0 * std::f64::consts::PI * tau);
            let title = if kind == 1 {
                "RC low-pass filter"
            } else {
                "RC high-pass filter"
            };
            let parts = if kind == 1 {
                format!("R1 in out {r1:.6e}\nC1 out 0 {c:.6e}")
            } else {
                format!("C1 in out {c:.6e}\nR1 out 0 {r1:.6e}")
            };
            format!(
                "{title}\n* Ideal source; nominal corner {fc:.3e} Hz.\nV1 in 0 DC 0 AC 1 PULSE(0 {voltage} 0 {:.6e} {:.6e} {:.6e} {:.6e})\n{parts}\n.op\n.ac dec 40 {:.6e} {:.6e}\n.tran {:.6e} {:.6e}\n.print ac vm(out) vp(out)\n.print tran v(in) v(out)\n.end",
                tau / 100.0,
                tau / 100.0,
                tau * 5.0,
                tau * 10.0,
                fc / 100.0,
                fc * 100.0,
                tau / 100.0,
                tau * 20.0
            )
        }
        3 => {
            let l = pick(rng, &[1e-3, 4.7e-3, 10e-3, 47e-3]);
            let damping = pick(rng, &[0.25, 0.5, 0.707, 1.0]);
            let r = 2.0 * damping * (l / c).sqrt();
            let omega = 1.0 / (l * c).sqrt();
            let f0 = omega / (2.0 * std::f64::consts::PI);
            let period = 1.0 / f0;
            format!(
                "Damped series RLC low-pass filter\n* Ideal parts; natural frequency {f0:.3e} Hz, damping ratio {damping}.\nV1 in 0 DC 0 AC 1 PULSE(0 {voltage} 0 {:.6e} {:.6e} {:.6e} {:.6e})\nR1 in mid {r:.6e}\nL1 mid out {l:.6e}\nC1 out 0 {c:.6e}\n.op\n.ac dec 40 {:.6e} {:.6e}\n.tran {:.6e} {:.6e}\n.print ac vm(out) vp(out)\n.print tran v(in) v(out)\n.end",
                period / 1000.0,
                period / 1000.0,
                period * 10.0,
                period * 20.0,
                f0 / 100.0,
                f0 * 100.0,
                period / 200.0,
                period * 40.0
            )
        }
        _ => {
            let r3 = pick(rng, &[2200.0, 4700.0, 10000.0]);
            let imbalance = pick(rng, &[0.9, 1.0, 1.1]);
            let r4 = r3 * r2 / r1 * imbalance;
            format!(
                "Resistive bridge with differential output\n* Right-arm ratio is {imbalance} times the left-arm ratio.\nV1 supply 0 DC {voltage}\nR1 supply left {r1:.6e}\nR2 left 0 {r2:.6e}\nR3 supply right {r3:.6e}\nR4 right 0 {r4:.6e}\n.op\n.dc V1 0 {voltage} {:.6e}\n.print dc v(left) v(right) v(left,right)\n.end",
                voltage / 20.0
            )
        }
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let kind = rng.random_range(0..5);
    circuit(rng, kind)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn decks_have_positive_components_and_every_node_has_a_dc_ground_path() {
        let mut rng = StdRng::seed_from_u64(47);
        for kind in 0..5 {
            for _ in 0..200 {
                let deck = circuit(&mut rng, kind);
                assert!(deck.ends_with(".end") && deck.contains("\n.op\n"));
                assert!(deck.contains("\n.print "));
                let mut nodes = HashSet::new();
                let mut names = HashSet::new();
                let mut dc: HashMap<&str, Vec<&str>> = HashMap::new();
                for line in deck.lines().skip(1).filter(|s| !s.starts_with(['*', '.'])) {
                    let p: Vec<_> = line.split_whitespace().collect();
                    assert!(names.insert(p[0]));
                    assert_ne!(p[1], p[2]);
                    nodes.extend([p[1], p[2]]);
                    let element = p[0].as_bytes()[0];
                    if element != b'V' {
                        let value: f64 = p[3].parse().unwrap();
                        assert!(value.is_finite() && value > 0.0);
                    }
                    if matches!(element, b'R' | b'L' | b'V') {
                        dc.entry(p[1]).or_default().push(p[2]);
                        dc.entry(p[2]).or_default().push(p[1]);
                    }
                }
                let mut grounded = HashSet::from(["0"]);
                let mut queue = vec!["0"];
                while let Some(node) = queue.pop() {
                    for next in dc.get(node).into_iter().flatten() {
                        if grounded.insert(next) {
                            queue.push(next);
                        }
                    }
                }
                assert_eq!(nodes, grounded, "{deck}");
                for line in deck
                    .lines()
                    .filter(|l| l.starts_with(".ac ") || l.starts_with(".tran "))
                {
                    let p: Vec<_> = line.split_whitespace().collect();
                    let offset = if p[0] == ".ac" { 3 } else { 1 };
                    let start: f64 = p[offset].parse().unwrap();
                    let stop: f64 = p[offset + 1].parse().unwrap();
                    assert!(start > 0.0 && stop > start);
                }
            }
        }
    }

    #[test]
    fn bridge_ratios_and_rlc_damping_match_their_comments() {
        let mut rng = StdRng::seed_from_u64(19);
        for _ in 0..100 {
            for kind in [3, 4] {
                let deck = circuit(&mut rng, kind);
                let parts: HashMap<_, _> = deck
                    .lines()
                    .skip(1)
                    .filter(|l| l.starts_with(['R', 'L', 'C']))
                    .map(|line| {
                        let p: Vec<_> = line.split_whitespace().collect();
                        (p[0], p[3].parse::<f64>().unwrap())
                    })
                    .collect();
                if kind == 3 {
                    let zeta = parts["R1"] / 2.0 * (parts["C1"] / parts["L1"]).sqrt();
                    assert!(
                        [0.25, 0.5, 0.707, 1.0]
                            .iter()
                            .any(|z| (zeta - z).abs() < 1e-6)
                    );
                } else {
                    let ratio = parts["R4"] / parts["R3"] / (parts["R2"] / parts["R1"]);
                    assert!([0.9, 1.0, 1.1].iter().any(|r| (ratio - r).abs() < 1e-6));
                }
            }
        }
    }
}
