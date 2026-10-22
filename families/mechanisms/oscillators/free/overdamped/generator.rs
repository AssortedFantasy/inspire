use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

include!("../../../number.rs");

fn roots(wn: f64, zeta: f64) -> (f64, f64) {
    let q = (zeta * zeta - 1.0).sqrt();
    (-wn / (zeta + q), -wn * (zeta + q))
}

fn coefficients(r1: f64, r2: f64, x0: f64, v0: f64) -> (f64, f64) {
    let a = (v0 - r2 * x0) / (r1 - r2);
    (a, x0 - a)
}

fn motion(wn: f64, zeta: f64, x0: f64, v0: f64, t: f64) -> (f64, f64) {
    let (r1, r2) = roots(wn, zeta);
    let (a, b) = coefficients(r1, r2, x0, v0);
    let slow = a * (r1 * t).exp();
    let fast = b * (r2 * t).exp();
    (slow + fast, r1 * slow + r2 * fast)
}

pub fn generate(rng: &mut StdRng) -> String {
    let mass = rng.random_range(1..=8) as f64;
    let wn = rng.random_range(2..=12) as f64;
    let zeta = rng.random_range(12..=40) as f64 / 10.0;
    let x0 =
        rng.random_range(5..=30) as f64 / 1000.0 * if rng.random_bool(0.5) { 1.0 } else { -1.0 };
    let v0 = wn * rng.random_range(-30..=30) as f64 / 1000.0;
    let (r1, r2) = roots(wn, zeta);
    let mut out = format!(
        "m = {} kg\nc = {} N·s/m\nk = {} N/m",
        n(mass),
        n(2.0 * zeta * mass * wn),
        n(mass * wn * wn)
    );
    match rng.random_range(0..3) {
        0 => {
            let (a, b) = coefficients(r1, r2, x0, v0);
            write!(
                out,
                "\n\nx(t) ≈ {} exp(-{}t) {} {} exp(-{}t) mm\nt in s",
                n(1000.0 * a),
                n(-r1),
                if b < 0.0 { "-" } else { "+" },
                n(1000.0 * b.abs()),
                n(-r2)
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
            let slow_time = -1.0 / r1;
            for fraction in [0.0, 0.025, 0.1, 0.25, 0.5, 1.0, 2.0, 4.0] {
                let t = fraction * slow_time;
                let (x, v) = motion(wn, zeta, x0, v0, t);
                write!(out, "\n{} | {} | {}", n(t), n(1000.0 * x), n(1000.0 * v)).unwrap();
            }
        }
        _ => {
            write!(
                out,
                "\n\nλ₁ = {} s⁻¹\nλ₂ = {} s⁻¹\nτ₁ = {} s\nτ₂ = {} s",
                n(r1),
                n(r2),
                n(-1.0 / r1),
                n(-1.0 / r2)
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
    fn real_modes_satisfy_characteristic_polynomial_and_initial_motion() {
        for wn in [2.0, 7.0, 12.0] {
            for zeta in [1.2, 2.0, 4.0] {
                let (r1, r2) = roots(wn, zeta);
                assert!(r2 < r1 && r1 < 0.0);
                assert!((r1 * r2 - wn * wn).abs() < 1e-10);
                assert!((r1 + r2 + 2.0 * zeta * wn).abs() < 1e-10);
                for v0 in [-0.1, 0.0, 0.2] {
                    let (x, v) = motion(wn, zeta, 0.02, v0, 0.0);
                    assert!((x - 0.02).abs() < 1e-12);
                    assert!((v - v0).abs() < 1e-12);
                    for step in 1..40 {
                        let t = step as f64 / (10.0 * wn);
                        let dt = 1e-5 / wn;
                        let (x, v) = motion(wn, zeta, 0.02, v0, t);
                        let (lo_x, lo_v) = motion(wn, zeta, 0.02, v0, t - dt);
                        let (hi_x, hi_v) = motion(wn, zeta, 0.02, v0, t + dt);
                        assert!(((hi_x - lo_x) / (2.0 * dt) - v).abs() < 1e-8);
                        let acceleration = (hi_v - lo_v) / (2.0 * dt);
                        assert!((acceleration + 2.0 * zeta * wn * v + wn * wn * x).abs() < 1e-7);
                    }
                }
            }
        }
    }
}
