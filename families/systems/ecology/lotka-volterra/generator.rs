use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

include!("../../number.rs");

#[derive(Clone, Copy, Debug)]
enum Model {
    Classic {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
    },
    Logistic {
        r: f64,
        k: f64,
        a: f64,
        e: f64,
        m: f64,
    },
    Competition {
        r: f64,
        s: f64,
        alpha: f64,
        beta: f64,
    },
    Saturating {
        r: f64,
        k: f64,
        a: f64,
        h: f64,
        e: f64,
        m: f64,
    },
}

impl Model {
    #[cfg(test)]
    fn field(self, x: f64, y: f64) -> [f64; 2] {
        match self {
            Self::Classic { a, b, c, d } => [x * (a - b * y), y * (d * x - c)],
            Self::Logistic { r, k, a, e, m } => {
                [r * x * (1.0 - x / k) - a * x * y, y * (e * a * x - m)]
            }
            Self::Competition { r, s, alpha, beta } => {
                [r * x * (1.0 - x - alpha * y), s * y * (1.0 - y - beta * x)]
            }
            Self::Saturating { r, k, a, h, e, m } => [
                r * x * (1.0 - x / k) - a * x * y / (h + x),
                y * (e * a * x / (h + x) - m),
            ],
        }
    }

    fn jacobian(self, x: f64, y: f64) -> [f64; 4] {
        match self {
            Self::Classic { a, b, c, d } => [a - b * y, -b * x, d * y, d * x - c],
            Self::Logistic { r, k, a, e, m } => [
                r * (1.0 - 2.0 * x / k) - a * y,
                -a * x,
                e * a * y,
                e * a * x - m,
            ],
            Self::Competition { r, s, alpha, beta } => [
                r * (1.0 - 2.0 * x - alpha * y),
                -r * alpha * x,
                -s * beta * y,
                s * (1.0 - 2.0 * y - beta * x),
            ],
            Self::Saturating { r, k, a, h, e, m } => [
                r * (1.0 - 2.0 * x / k) - a * y * h / (h + x).powi(2),
                -a * x / (h + x),
                e * a * y * h / (h + x).powi(2),
                e * a * x / (h + x) - m,
            ],
        }
    }

    fn equilibria(self) -> Vec<(f64, f64)> {
        let mut result = vec![(0.0, 0.0)];
        match self {
            Self::Classic { a, b, c, d } => result.push((c / d, a / b)),
            Self::Logistic { r, k, a, e, m } => {
                result.push((k, 0.0));
                let x = m / (e * a);
                if k > x + 1e-9 {
                    result.push((x, r / a * (1.0 - x / k)));
                }
            }
            Self::Competition { alpha, beta, .. } => {
                result.extend([(1.0, 0.0), (0.0, 1.0)]);
                let denominator = 1.0 - alpha * beta;
                if denominator.abs() > 1e-9 {
                    let (x, y) = ((1.0 - alpha) / denominator, (1.0 - beta) / denominator);
                    if x > 1e-9 && y > 1e-9 {
                        result.push((x, y));
                    }
                }
            }
            Self::Saturating { r, k, a, h, e, m } => {
                result.push((k, 0.0));
                let x = m * h / (e * a - m);
                if k > x + 1e-9 {
                    result.push((x, r * (1.0 - x / k) * (h + x) / a));
                }
            }
        }
        result
    }
}

fn spectrum(j: [f64; 4]) -> (String, &'static str) {
    let trace = j[0] + j[3];
    let det = j[0] * j[3] - j[1] * j[2];
    let disc = trace * trace - 4.0 * det;
    let text = if disc < -1e-9 {
        format!("{} ± {}i", n(trace / 2.0), n((-disc).sqrt() / 2.0))
    } else {
        let root = disc.max(0.0).sqrt();
        format!("{}, {}", n((trace + root) / 2.0), n((trace - root) / 2.0))
    };
    let regime = if det < -1e-9 {
        "saddle"
    } else if det.abs() < 1e-9 {
        "nonhyperbolic"
    } else if trace.abs() < 1e-9 {
        "imaginary eigenvalues"
    } else if trace < 0.0 {
        if disc < -1e-9 {
            "stable focus"
        } else {
            "stable node"
        }
    } else if disc < -1e-9 {
        "unstable focus"
    } else {
        "unstable node"
    };
    (text, regime)
}

fn pick(rng: &mut StdRng, values: &[f64]) -> f64 {
    values[rng.random_range(0..values.len())]
}

fn make(rng: &mut StdRng, mode: usize) -> Model {
    match mode {
        0 => {
            let a = pick(rng, &[0.25, 0.5, 1.0, 2.0]);
            let c = pick(rng, &[0.25, 0.5, 1.0, 2.0]);
            Model::Classic {
                a,
                b: a / rng.random_range(2..=12) as f64,
                c,
                d: c / rng.random_range(2..=12) as f64,
            }
        }
        1 => {
            let r = pick(rng, &[0.25, 0.5, 1.0, 2.0]);
            let m = r * pick(rng, &[0.02, 0.1, 0.5, 1.0, 2.0]);
            let x = rng.random_range(2..=12) as f64;
            Model::Logistic {
                r,
                k: x * pick(rng, &[0.8, 1.0, 1.02, 1.1, 1.5, 2.0, 4.0]),
                a: m / x,
                e: 1.0,
                m,
            }
        }
        2 => {
            let low = &[0.2, 0.4, 0.65, 0.9, 0.98];
            let high = &[1.02, 1.1, 1.4, 1.8, 2.2];
            let (alpha, beta) = match rng.random_range(0..7) {
                0 => (pick(rng, low), pick(rng, low)),
                1 => (pick(rng, low), pick(rng, high)),
                2 => (pick(rng, high), pick(rng, low)),
                3 | 4 => (pick(rng, high), pick(rng, high)),
                5 => (1.0, 1.0),
                _ => (1.0, pick(rng, low)),
            };
            Model::Competition {
                r: pick(rng, &[0.25, 0.5, 1.0, 2.0]),
                s: pick(rng, &[0.25, 0.5, 1.0, 2.0]),
                alpha,
                beta,
            }
        }
        _ => {
            let r = pick(rng, &[0.5, 1.0, 2.0]);
            let a = pick(rng, &[1.0, 2.0, 4.0]);
            let h = rng.random_range(2..=10) as f64;
            let m = a * pick(rng, &[0.2, 0.4, 0.6]);
            let x = m * h / (a - m);
            let hopf = h + 2.0 * x;
            let k = match rng.random_range(0..7) {
                0 => 0.85 * x,
                1 => x,
                2 => 1.05 * x,
                3 => 0.9 * hopf,
                4 => hopf,
                5 => 1.02 * hopf,
                _ => 1.3 * hopf,
            };
            Model::Saturating {
                r,
                k,
                a,
                h,
                e: 1.0,
                m,
            }
        }
    }
}

fn classical_h(a: f64, b: f64, c: f64, d: f64, x: f64, y: f64) -> f64 {
    d * x - c * x.ln() + b * y - a * y.ln()
}

fn turning_points(weight: f64, equilibrium: f64, excess: f64) -> (f64, f64) {
    // z - ln(z) - 1 is strictly monotone on either side of z = 1.
    let energy = |z: f64| weight * (z - z.ln() - 1.0);
    let root = |mut lo: f64, mut hi: f64, increasing: bool| {
        for _ in 0..100 {
            let mid = (lo + hi) / 2.0;
            if (energy(mid) < excess) == increasing {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        equilibrium * (lo + hi) / 2.0
    };
    // Generated initial states bound excess/weight, so these fixed brackets suffice.
    (root(1e-12, 1.0, false), root(1.0, 1000.0, true))
}

pub fn generate(rng: &mut StdRng) -> String {
    let mode = rng.random_range(0..4);
    let model = make(rng, mode);
    let mut out = match model {
        Model::Classic { a, b, c, d } => format!(
            "Classical predator–prey\nx′ = x({} − {}y)\ny′ = y({}x − {})",
            n(a),
            n(b),
            n(d),
            n(c)
        ),
        Model::Logistic { r, k, a, e, m } => format!(
            "Logistic prey, bilinear predation\nx′ = {}x(1 − x/{}) − {}xy\ny′ = y({}x − {})",
            n(r),
            n(k),
            n(a),
            n(e * a),
            n(m)
        ),
        Model::Competition { r, s, alpha, beta } => format!(
            "Two-species competition, scaled carrying capacities 1\nx′ = {}x(1 − x − {}y)\ny′ = {}y(1 − y − {}x)",
            n(r),
            n(alpha),
            n(s),
            n(beta)
        ),
        Model::Saturating { r, k, a, h, e, m } => format!(
            "Rosenzweig–MacArthur, logistic prey and Holling II response\nx′ = {}x(1 − x/{}) − {}xy/({} + x)\ny′ = y({}x/({} + x) − {})",
            n(r),
            n(k),
            n(a),
            n(h),
            n(e * a),
            n(h),
            n(m)
        ),
    };
    if let Model::Competition {
        r,
        s,
        alpha: 1.0,
        beta: 1.0,
    } = model
    {
        write!(out,"\n\nEquilibria (0, 0) and x + y = 1, x,y ≥ 0\nAt origin λ = {}, {}\nOn equilibrium line λ = 0, −({}x + {}y)\nAttracting equilibrium line",n(r),n(s),n(r),n(s)).unwrap();
        return out;
    }
    out.push_str("\n\nEquilibria and Jacobian eigenvalues");
    for (x, y) in model.equilibria() {
        let (eigen, regime) = spectrum(model.jacobian(x, y));
        let regime = if mode == 0 && x > 0.0 {
            "center, neutral closed orbits"
        } else {
            regime
        };
        write!(out, "\n({}, {})  λ = {eigen}  {regime}", n(x), n(y)).unwrap();
    }
    match model {
        Model::Classic { a, b, c, d } => {
            let (x, y) = (c / d, a / b);
            let x0 = x * pick(rng, &[0.5, 0.75, 1.25, 1.5, 2.0]);
            let y0 = y * pick(rng, &[0.5, 0.75, 1.0, 1.5]);
            let h0 = classical_h(a, b, c, d, x0, y0);
            let excess = h0 - classical_h(a, b, c, d, x, y);
            let (xmin, xmax) = turning_points(c, x, excess);
            let (ymin, ymax) = turning_points(a, y, excess);
            write!(out,"\n\n(x0, y0) = ({}, {})\nH = {}x − {}ln(x) + {}y − {}ln(y) = {}\nOrbit x-range [{}, {}], y-range [{}, {}]\nSmall-amplitude period = {}",n(x0),n(y0),n(d),n(c),n(b),n(a),n(h0),n(xmin),n(xmax),n(ymin),n(ymax),n(2.0*std::f64::consts::PI/(a*c).sqrt())).unwrap();
        }
        Model::Logistic { k, a, e, m, .. } => {
            write!(
                out,
                "\n\nPredator invasion threshold K = {}",
                n(m / (e * a))
            )
            .unwrap();
            if k < m / (e * a) - 1e-9 {
                out.push_str("\nPredator exclusion");
            } else if (k - m / (e * a)).abs() < 1e-9 {
                out.push_str("\nTranscritical threshold");
            }
        }
        Model::Competition { alpha, beta, .. } => {
            out.push_str(match (alpha < 1.0, beta < 1.0, alpha > 1.0, beta > 1.0) {
                (true, true, _, _) => "\n\nStable coexistence",
                (true, _, _, true) => "\n\nx excludes y",
                (_, true, true, _) => "\n\ny excludes x",
                (_, _, true, true) => "\n\nBistability, basin boundary through coexistence saddle",
                _ => "\n\nCompetition invasion boundary",
            });
        }
        Model::Saturating { k, a, h, e, m, .. } => {
            let x = m * h / (e * a - m);
            let hopf = h + 2.0 * x;
            write!(
                out,
                "\n\nPredator invasion threshold K = {}\nHopf threshold K_H = {}",
                n(x),
                n(hopf)
            )
            .unwrap();
            if k > hopf + 1e-9 {
                out.push_str("\nAttracting periodic orbit around unstable coexistence");
            } else if (k - hopf).abs() < 1e-9 {
                out.push_str("\nAt the Hopf threshold");
            } else if k < x - 1e-9 {
                out.push_str("\nPredator exclusion");
            } else if (k - x).abs() < 1e-9 {
                out.push_str("\nTranscritical threshold");
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn equilibria_and_analytic_jacobians_match_vector_field() {
        for seed in 0..800 {
            let mut rng = StdRng::seed_from_u64(seed);
            for mode in 0..4 {
                let model = make(&mut rng, mode);
                for (x, y) in model.equilibria() {
                    assert!(model.field(x, y).iter().all(|v| v.abs() < 1e-9));
                    assert!(x >= 0.0 && y >= 0.0);
                }
                let (x, y) = (rng.random_range(0.1..3.0), rng.random_range(0.1..3.0));
                let j = model.jacobian(x, y);
                let eps = 1e-5;
                let plus = model.field(x + eps, y);
                let minus = model.field(x - eps, y);
                assert!((j[0] - (plus[0] - minus[0]) / (2.0 * eps)).abs() < 1e-6);
                assert!((j[2] - (plus[1] - minus[1]) / (2.0 * eps)).abs() < 1e-6);
                let plus = model.field(x, y + eps);
                let minus = model.field(x, y - eps);
                assert!((j[1] - (plus[0] - minus[0]) / (2.0 * eps)).abs() < 1e-6);
                assert!((j[3] - (plus[1] - minus[1]) / (2.0 * eps)).abs() < 1e-6);
            }
        }
    }

    #[test]
    fn classical_invariant_and_orbit_bounds() {
        for seed in 0..300 {
            let mut rng = StdRng::seed_from_u64(seed);
            let model = make(&mut rng, 0);
            let Model::Classic { a, b, c, d } = model else {
                unreachable!()
            };
            let (xe, ye) = (c / d, a / b);
            let (x, y) = (xe * 0.6, ye * 1.7);
            let f = model.field(x, y);
            assert!(((d - c / x) * f[0] + (b - a / y) * f[1]).abs() < 1e-10);
            let energy = classical_h(a, b, c, d, x, y);
            let excess = energy - classical_h(a, b, c, d, xe, ye);
            let (lo, hi) = turning_points(c, xe, excess);
            assert!(lo < x && hi > x);
            assert!((classical_h(a, b, c, d, lo, ye) - energy).abs() < 1e-9);
            assert!((classical_h(a, b, c, d, hi, ye) - energy).abs() < 1e-9);
            assert_eq!(spectrum(model.jacobian(xe, ye)).1, "imaginary eigenvalues");
        }
    }

    #[test]
    fn logistic_nodes_foci_and_exclusion_are_distinct() {
        let node = Model::Logistic {
            r: 1.0,
            k: 1.02,
            a: 0.1,
            e: 1.0,
            m: 0.1,
        };
        let focus = Model::Logistic {
            r: 1.0,
            k: 4.0,
            a: 1.0,
            e: 1.0,
            m: 1.0,
        };
        for (model, wanted) in [(node, "stable node"), (focus, "stable focus")] {
            let (x, y) = model.equilibria()[2];
            assert_eq!(spectrum(model.jacobian(x, y)).1, wanted);
        }
        let extinct = Model::Logistic {
            r: 1.0,
            k: 0.8,
            a: 1.0,
            e: 1.0,
            m: 1.0,
        };
        assert_eq!(extinct.equilibria().len(), 2);
        assert_eq!(spectrum(extinct.jacobian(0.8, 0.0)).1, "stable node");
    }

    #[test]
    fn competition_invasion_signs_and_bistability() {
        for (alpha, beta, coexistence) in [(0.4, 0.7, "stable node"), (1.3, 1.8, "saddle")] {
            let m = Model::Competition {
                r: 1.0,
                s: 2.0,
                alpha,
                beta,
            };
            let (x, y) = m.equilibria()[3];
            assert_eq!(spectrum(m.jacobian(x, y)).1, coexistence);
        }
        let m = Model::Competition {
            r: 1.0,
            s: 2.0,
            alpha: 1.0,
            beta: 1.0,
        };
        for x in [0.0, 0.1, 0.4, 0.9, 1.0] {
            assert!(m.field(x, 1.0 - x).iter().all(|v| v.abs() < 1e-12));
            let j = m.jacobian(x, 1.0 - x);
            assert!((j[0] * j[3] - j[1] * j[2]).abs() < 1e-12);
            assert!(j[0] + j[3] < 0.0);
        }
    }

    #[test]
    fn saturating_trace_changes_sign_at_hopf_not_at_invasion() {
        for k in [1.1, 2.9, 3.0, 3.1] {
            let m = Model::Saturating {
                r: 1.0,
                k,
                a: 2.0,
                h: 1.0,
                e: 1.0,
                m: 1.0,
            };
            let (x, y) = m.equilibria()[2];
            let j = m.jacobian(x, y);
            let trace = j[0] + j[3];
            assert!((trace - x / (1.0 + x) * (1.0 - 3.0 / k)).abs() < 1e-12);
            assert!((trace > 1e-9) == (k > 3.0));
            assert!(j[0] * j[3] - j[1] * j[2] > 0.0);
        }
    }
}
