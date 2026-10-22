use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

include!("../../../number.rs");

fn motion(wn: f64, x0: f64, v0: f64, t: f64) -> (f64, f64) {
    let b = v0 + wn * x0;
    let envelope = (-wn * t).exp();
    (envelope * (x0 + b * t), envelope * (v0 - wn * b * t))
}

pub fn generate(rng: &mut StdRng) -> String {
    let mass = rng.random_range(1..=8) as f64;
    let wn = rng.random_range(2..=12) as f64;
    let x0 =
        rng.random_range(5..=30) as f64 / 1000.0 * if rng.random_bool(0.5) { 1.0 } else { -1.0 };
    let v0 = wn * rng.random_range(-30..=30) as f64 / 1000.0;
    let b = v0 + wn * x0;
    let mut out = format!(
        "m = {} kg\nc = {} N·s/m\nk = {} N/m",
        n(mass),
        n(2.0 * mass * wn),
        n(mass * wn * wn)
    );
    match rng.random_range(0..3) {
        0 => {
            if b.abs() < 1e-12 {
                write!(
                    out,
                    "\n\nx(t) = {} exp(-{}t) mm\nt in s",
                    n(1000.0 * x0),
                    n(wn)
                )
                .unwrap();
                return out;
            }
            write!(
                out,
                "\n\nx(t) = ({} {} {}t) exp(-{}t) mm\nt in s",
                n(1000.0 * x0),
                if b < 0.0 { "-" } else { "+" },
                n(1000.0 * b.abs()),
                n(wn)
            )
            .unwrap();
        }
        1 => {
            write!(
                out,
                "\nx(0) = {} mm\nx′(0) = {} mm/s\n\nt (s) | x (mm) | x′ (mm/s)",
                n(1000.0 * x0),
                n(1000.0 * v0)
            )
            .unwrap();
            for multiple in [0.0, 0.25, 0.5, 1.0, 2.0, 4.0, 6.0, 8.0] {
                let t = multiple / wn;
                let (x, v) = motion(wn, x0, v0, t);
                write!(out, "\n{} | {} | {}", n(t), n(1000.0 * x), n(1000.0 * v)).unwrap();
            }
        }
        _ => {
            write!(
                out,
                "\n\nH(s) = X(s)/F(s) = 1 / [{}(s + {})²]\nλ₁ = λ₂ = -{} s⁻¹\nh(t) ≈ {}t exp(-{}t) m/(N·s), t ≥ 0",
                n(mass),
                n(wn),
                n(wn),
                n(1.0 / mass),
                n(wn)
            )
            .unwrap();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_root_solution_accepts_arbitrary_initial_velocity() {
        for wn in [2.0, 7.0, 12.0] {
            for v0 in [-0.1, 0.0, 0.2] {
                let (x, v) = motion(wn, 0.02, v0, 0.0);
                assert!((x - 0.02).abs() < 1e-12);
                assert!((v - v0).abs() < 1e-12);
                for step in 1..50 {
                    let t = step as f64 / (10.0 * wn);
                    let dt = 1e-5 / wn;
                    let (x, v) = motion(wn, 0.02, v0, t);
                    let (lo_x, lo_v) = motion(wn, 0.02, v0, t - dt);
                    let (hi_x, hi_v) = motion(wn, 0.02, v0, t + dt);
                    assert!(((hi_x - lo_x) / (2.0 * dt) - v).abs() < 1e-8);
                    let acceleration = (hi_v - lo_v) / (2.0 * dt);
                    assert!((acceleration + 2.0 * wn * v + wn * wn * x).abs() < 1e-8);
                }
                let (x, v) = motion(wn, 0.02, v0, 40.0 / wn);
                assert!(x.abs() < 1e-12 && v.abs() < 1e-12);
            }
        }
    }

    #[test]
    fn impulse_response_has_unit_momentum_jump() {
        for mass in [1.0, 3.0, 8.0] {
            let (x, v) = motion(4.0, 0.0, 1.0 / mass, 0.0);
            assert_eq!(x, 0.0);
            assert!((mass * v - 1.0).abs() < 1e-12);
        }
    }
}
