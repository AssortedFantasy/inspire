// Independent complex modified-nodal checks of emitted resistor, capacitor,
// inductor, voltage-source and VCVS decks. This is a test solver, not SPICE.
use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Default)]
struct C {
    re: f64,
    im: f64,
}
impl C {
    fn real(re: f64) -> Self {
        Self { re, im: 0.0 }
    }
    fn abs(self) -> f64 {
        self.re.hypot(self.im)
    }
}
impl Add for C {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}
impl Sub for C {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}
impl Neg for C {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}
impl Mul for C {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}
impl Div for C {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let d = rhs.re * rhs.re + rhs.im * rhs.im;
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / d,
            im: (self.im * rhs.re - self.re * rhs.im) / d,
        }
    }
}

fn elements(deck: &str) -> Vec<Vec<&str>> {
    deck.lines()
        .skip(1)
        .filter(|l| !l.starts_with(['*', '.']))
        .map(|l| l.split_whitespace().collect())
        .collect()
}

fn value(parts: &[Vec<&str>], name: &str) -> f64 {
    parts.iter().find(|p| p[0] == name).unwrap()[3]
        .parse()
        .unwrap()
}

fn close(got: C, expected: C) {
    assert!(
        (got - expected).abs() < 2e-5 * expected.abs().max(1.0),
        "{got:?} != {expected:?}"
    );
}

// Frequency 0 solves DC with inductors as exact shorts and capacitors open.
// Excitation is always one volt to compare transfer functions independently of
// the chosen deck's DC operating source level. The emitted circuits are linear.
fn solve(parts: &[Vec<&str>], frequency: f64) -> HashMap<String, C> {
    solve_input(parts, frequency, "V1")
}

fn solve_input(parts: &[Vec<&str>], frequency: f64, input: &str) -> HashMap<String, C> {
    let mut nodes = Vec::new();
    for p in parts {
        for name in p
            .iter()
            .take(if p[0].starts_with('E') { 5 } else { 3 })
            .skip(1)
        {
            if *name != "0" && !nodes.contains(name) {
                nodes.push(*name);
            }
        }
    }
    let idx: HashMap<_, _> = nodes
        .iter()
        .enumerate()
        .map(|(i, name)| (*name, i))
        .collect();
    let sources: Vec<_> = parts
        .iter()
        .filter(|p| p[0].starts_with(['V', 'E']) || (frequency == 0.0 && p[0].starts_with('L')))
        .collect();
    let n = nodes.len() + sources.len();
    let mut a = vec![vec![C::default(); n + 1]; n];
    let stamp = |a: &mut Vec<Vec<C>>, row: &str, col: &str, y: C| {
        if let (Some(&r), Some(&c)) = (idx.get(row), idx.get(col)) {
            a[r][c] = a[r][c] + y;
        }
    };
    for p in parts {
        let element = p[0].as_bytes()[0];
        let y = match element {
            b'R' => C::real(1.0 / p[3].parse::<f64>().unwrap()),
            b'C' => C {
                re: 0.0,
                im: 2.0 * PI * frequency * p[3].parse::<f64>().unwrap(),
            },
            b'L' if frequency > 0.0 => C {
                re: 0.0,
                im: -1.0 / (2.0 * PI * frequency * p[3].parse::<f64>().unwrap()),
            },
            _ => continue,
        };
        stamp(&mut a, p[1], p[1], y);
        stamp(&mut a, p[2], p[2], y);
        stamp(&mut a, p[1], p[2], -y);
        stamp(&mut a, p[2], p[1], -y);
    }
    for (i, p) in sources.iter().enumerate() {
        let row = nodes.len() + i;
        for (name, sign) in [(p[1], 1.0), (p[2], -1.0)] {
            if let Some(&j) = idx.get(name) {
                a[j][row] = a[j][row] + C::real(sign);
                a[row][j] = a[row][j] + C::real(sign);
            }
        }
        if p[0].starts_with('V') {
            a[row][n] = C::real(if p[0] == input { 1.0 } else { 0.0 });
        }
        if p[0].starts_with('E') {
            let gain: f64 = p[5].parse().unwrap();
            for (name, sign) in [(p[3], -gain), (p[4], gain)] {
                if let Some(&j) = idx.get(name) {
                    a[row][j] = a[row][j] + C::real(sign);
                }
            }
        }
    }
    for col in 0..n {
        let pivot = (col..n)
            .max_by(|&i, &j| a[i][col].abs().total_cmp(&a[j][col].abs()))
            .unwrap();
        a.swap(col, pivot);
        assert!(a[col][col].abs() > 1e-18, "Singular nodal matrix");
        let scale = a[col][col];
        for value in &mut a[col][col..] {
            *value = *value / scale;
        }
        let pivot_row = a[col].clone();
        for (row_index, row) in a.iter_mut().enumerate() {
            if row_index == col {
                continue;
            }
            let factor = row[col];
            for (value, pivot_value) in row[col..].iter_mut().zip(&pivot_row[col..]) {
                *value = *value - factor * *pivot_value;
            }
        }
    }
    let mut result: HashMap<_, _> = nodes
        .iter()
        .enumerate()
        .map(|(i, name)| ((*name).to_owned(), a[i][n]))
        .collect();
    for (i, p) in sources.iter().enumerate() {
        result.insert(format!("@{}", p[0]), a[nodes.len() + i][n]);
    }
    result.insert("0".to_owned(), C::default());
    result
}

fn structure(deck: &str, parts: &[Vec<&str>]) {
    assert!(deck.ends_with(".end") && deck.contains("\n.op\n") && deck.contains("\n.print "));
    let analyses: Vec<_> = deck
        .lines()
        .filter(|l| l.starts_with(".ac ") || l.starts_with(".tran ") || l.starts_with(".dc "))
        .collect();
    assert_eq!(analyses.len(), 1);
    let source = parts.iter().find(|p| p[0] == "V1").unwrap().join(" ");
    if analyses[0].starts_with(".ac ") {
        assert!(source.contains(" AC 1"));
    } else if analyses[0].starts_with(".tran ") {
        if let Some((_, pulse)) = source.split_once("PULSE(") {
            let pulse: Vec<f64> = pulse
                .trim_end_matches(')')
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            assert_eq!(pulse.len(), 7);
            assert!(pulse[2..].iter().all(|v| v.is_finite() && *v > 0.0));
            assert!(pulse[5] > pulse[3] + pulse[4] && pulse[6] > pulse[5]);
        } else {
            let (_, sinusoid) = source.split_once("SIN(").unwrap();
            let sinusoid: Vec<f64> = sinusoid
                .trim_end_matches(')')
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            assert_eq!(sinusoid.len(), 3);
            assert!(sinusoid[2].is_finite() && sinusoid[2] > 0.0);
        }
    }
    let mut names = HashSet::new();
    let mut nodes = HashSet::from(["0"]);
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for p in parts {
        assert!(names.insert(p[0]), "Repeated element name");
        assert_ne!(p[1], p[2]);
        nodes.extend([p[1], p[2]]);
        let kind = p[0].as_bytes()[0];
        assert!(matches!(kind, b'R' | b'C' | b'L' | b'V' | b'E'));
        if kind != b'V' {
            let v: f64 = p[if kind == b'E' { 5 } else { 3 }].parse().unwrap();
            assert!(v.is_finite() && v > 0.0);
            assert_eq!(p.len(), if kind == b'E' { 6 } else { 4 });
        } else {
            assert_eq!(p[3], "DC");
            let v: f64 = p[4].parse().unwrap();
            assert!(v.is_finite());
            if let Some(i) = p.iter().position(|t| *t == "AC") {
                assert_eq!(p[i + 1], "1");
            }
        }
        if kind == b'E' {
            nodes.extend([p[3], p[4]]);
        }
        if kind != b'C' {
            edges.entry(p[1]).or_default().push(p[2]);
            edges.entry(p[2]).or_default().push(p[1]);
        }
    }
    let mut grounded = HashSet::from(["0"]);
    let mut stack = vec!["0"];
    while let Some(node) = stack.pop() {
        for &next in edges.get(node).into_iter().flatten() {
            if grounded.insert(next) {
                stack.push(next);
            }
        }
    }
    assert_eq!(nodes, grounded, "{deck}");
    for line in deck
        .lines()
        .filter(|l| l.starts_with(".ac ") || l.starts_with(".tran "))
    {
        let p: Vec<_> = line.split_whitespace().collect();
        let offset = if p[0] == ".ac" {
            assert_eq!(p[1], "dec");
            3
        } else {
            1
        };
        let start: f64 = p[offset].parse().unwrap();
        let stop: f64 = p[offset + 1].parse().unwrap();
        assert!(start.is_finite() && stop.is_finite() && start > 0.0 && stop > start);
    }
}

pub fn verify(deck: &str, kind: &str) {
    let p = elements(deck);
    structure(deck, &p);
    let dc = solve(&p, 0.0);
    let one = C::real(1.0);
    match kind {
        "loaded-divider" => {
            let r1 = value(&p, "R1");
            let r2 = value(&p, "R2");
            let load = value(&p, "Rload");
            let bottom = 1.0 / (1.0 / r2 + 1.0 / load);
            close(dc["out"], C::real(bottom / (r1 + bottom)));
            close(dc["@V1"], C::real(-1.0 / (r1 + bottom)));
        }
        "bridge" => {
            let r1 = value(&p, "R1");
            let r2 = value(&p, "R2");
            let r3 = value(&p, "R3");
            let r4 = value(&p, "R4");
            close(dc["left"], C::real(r2 / (r1 + r2)));
            close(dc["right"], C::real(r4 / (r3 + r4)));
            close(dc["@V1"], C::real(-1.0 / (r1 + r2) - 1.0 / (r3 + r4)));
        }
        "tapped-divider" => {
            let r1 = value(&p, "R1");
            let r2 = value(&p, "R2");
            let r3 = value(&p, "R3");
            let sum = r1 + r2 + r3;
            close(dc["tap"], C::real((r2 + r3) / sum));
            close(dc["out"], C::real(r3 / sum));
            close(dc["@V1"], C::real(-1.0 / sum));
        }
        "rc-low-pass" | "rc-high-pass" | "rc-loaded-low-pass" | "rl-low-pass" | "rl-high-pass" => {
            let r = value(&p, "R1");
            let loaded = kind == "rc-loaded-low-pass";
            let (gain, resistance) = if loaded {
                let load = value(&p, "Rload");
                (load / (r + load), r * load / (r + load))
            } else {
                (1.0, r)
            };
            let tau = if kind.starts_with("rl-") {
                value(&p, "L1") / r
            } else {
                resistance * value(&p, "C1")
            };
            let highpass = kind.ends_with("high-pass");
            close(dc["out"], C::real(if highpass { 0.0 } else { gain }));
            let fc = 1.0 / (2.0 * PI * tau);
            for multiple in [0.01, 1.0, 100.0] {
                let numerator = if highpass {
                    C {
                        re: 0.0,
                        im: multiple,
                    }
                } else {
                    C::real(gain)
                };
                close(
                    solve(&p, fc * multiple)["out"],
                    numerator
                        / C {
                            re: 1.0,
                            im: multiple,
                        },
                );
            }
        }
        "rc-dual-low-pass" | "rc-band-pass" => {
            let r1 = value(&p, "R1");
            let r2 = value(&p, "R2");
            let c1 = value(&p, "C1");
            let c2 = value(&p, "C2");
            let a = r1 * c1 + (r1 + r2) * c2;
            let b = r1 * r2 * c1 * c2;
            let bandpass = kind == "rc-band-pass";
            close(dc["out"], C::real(if bandpass { 0.0 } else { 1.0 }));
            for multiple in [0.01, 1.0, 100.0] {
                let omega = multiple / b.sqrt();
                let numerator = if bandpass {
                    C {
                        re: 0.0,
                        im: omega * r1 * c1,
                    }
                } else {
                    one
                };
                close(
                    solve(&p, omega / (2.0 * PI))["out"],
                    numerator
                        / C {
                            re: 1.0 - b * omega * omega,
                            im: a * omega,
                        },
                );
            }
        }
        "rlc-low-pass" | "rlc-band-pass" | "rlc-high-pass" | "rlc-notch" => {
            let r = value(&p, "R1");
            let l = value(&p, "L1");
            let c = value(&p, "C1");
            let f0 = 1.0 / (2.0 * PI * (l * c).sqrt());
            let zeta = r / 2.0 * (c / l).sqrt();
            close(
                dc["out"],
                C::real(if matches!(kind, "rlc-low-pass" | "rlc-notch") {
                    1.0
                } else {
                    0.0
                }),
            );
            for multiple in [0.01, 1.0, 100.0] {
                let numerator = match kind {
                    "rlc-low-pass" => one,
                    "rlc-band-pass" => C {
                        re: 0.0,
                        im: 2.0 * zeta * multiple,
                    },
                    "rlc-notch" => C::real(1.0 - multiple * multiple),
                    _ => C::real(-multiple * multiple),
                };
                close(
                    solve(&p, f0 * multiple)["out"],
                    numerator
                        / C {
                            re: 1.0 - multiple * multiple,
                            im: 2.0 * zeta * multiple,
                        },
                );
            }
        }
        "inverting" | "noninverting" | "summing" => {
            let rf = value(&p, "Rf");
            let a = p.iter().find(|p| p[0] == "E1").unwrap()[5]
                .parse::<f64>()
                .unwrap();
            let noninverting = kind == "noninverting";
            let r = value(&p, if noninverting { "Rg" } else { "R1" });
            let conductance = 1.0 / rf
                + 1.0 / r
                + if kind == "summing" {
                    1.0 / value(&p, "R2")
                } else {
                    0.0
                };
            let gain = if noninverting {
                a * (rf + r) / (rf + r + a * r)
            } else {
                -a / r / (conductance + a / rf)
            };
            close(dc["out"], C::real(gain));
            for f in [1.0, 1000.0, 1e6] {
                close(solve(&p, f)["out"], C::real(gain));
                if kind == "summing" {
                    let gain2 = -a / value(&p, "R2") / (conductance + a / rf);
                    close(solve_input(&p, f, "V2")["out"], C::real(gain2));
                }
            }
            let differential = if noninverting {
                dc["in"] - dc["inv"]
            } else {
                -dc["inv"]
            };
            close(dc["out"], differential * C::real(a));
        }
        _ => panic!("Unknown topology {kind}"),
    }
}
