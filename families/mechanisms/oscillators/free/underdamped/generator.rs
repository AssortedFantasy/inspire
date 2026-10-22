use rand::{Rng, rngs::StdRng};
use std::{f64::consts::TAU, fmt::Write};

include!("../../../number.rs");

fn motion(wn: f64, zeta: f64, x0: f64, v0: f64, t: f64) -> (f64, f64) {
    let decay = zeta * wn;
    let wd = wn * (1.0 - zeta * zeta).sqrt();
    let b = (v0 + decay * x0) / wd;
    let (sin, cos) = (wd * t).sin_cos();
    let envelope = (-decay * t).exp();
    (
        envelope * (x0 * cos + b * sin),
        envelope * ((b * wd - decay * x0) * cos - (x0 * wd + decay * b) * sin),
    )
}

pub fn generate(rng: &mut StdRng) -> String {
    let mass = rng.random_range(1..=8) as f64;
    let wn = rng.random_range(2..=12) as f64;
    let zeta = rng.random_range(1..=8) as f64 / 10.0;
    let x0 =
        rng.random_range(5..=30) as f64 / 1000.0 * if rng.random_bool(0.5) { 1.0 } else { -1.0 };
    let v0 = wn * rng.random_range(-30..=30) as f64 / 1000.0;
    let spring = mass * wn * wn;
    let decay = zeta * wn;
    let wd = wn * (1.0 - zeta * zeta).sqrt();
    let period = TAU / wd;
    let mut out = format!(
        "m = {} kg\nc = {} N·s/m\nk = {} N/m",
        n(mass),
        n(2.0 * decay * mass),
        n(spring)
    );
    match rng.random_range(0..4) {
        0 => {
            let b = (v0 + decay * x0) / wd;
            if b.abs() < 1e-12 {
                write!(
                    out,
                    "\n\nx(t) ≈ {} exp(-{}t) cos({}t) mm\nt in s",
                    n(1000.0 * x0),
                    n(decay),
                    n(wd)
                )
                .unwrap();
                return out;
            }
            write!(
                out,
                "\n\nx(t) ≈ exp(-{}t) [{} cos({}t) {} {} sin({}t)] mm\nt in s",
                n(decay),
                n(1000.0 * x0),
                n(wd),
                if b < 0.0 { "-" } else { "+" },
                n(1000.0 * b.abs()),
                n(wd)
            )
            .unwrap();
        }
        mode @ (1 | 2) => {
            let energy = mode == 2;
            write!(
                out,
                "\nx(0) = {} mm\nx′(0) = {} mm/s\n\nt (s) | x (mm) | {}",
                n(1000.0 * x0),
                n(1000.0 * v0),
                if energy { "E (mJ)" } else { "x′ (mm/s)" }
            )
            .unwrap();
            let horizon = (2.0 * period).min(5.0 / decay);
            for fraction in [0.0, 0.05, 0.1, 0.2, 0.35, 0.5, 0.75, 1.0] {
                let t = horizon * fraction;
                let (x, v) = motion(wn, zeta, x0, v0, t);
                let extra = if energy {
                    500.0 * (mass * v * v + spring * x * x)
                } else {
                    1000.0 * v
                };
                write!(out, "\n{} | {} | {}", n(t), n(1000.0 * x), n(extra)).unwrap();
            }
        }
        _ => {
            write!(
                out,
                "\n\nλ = -{} ± {}i s⁻¹\nT = {} s\nEnvelope half-life = {} s\nAmplitude ratio per cycle = {}",
                n(decay),
                n(wd),
                n(period),
                n(2.0_f64.ln() / decay),
                n((-decay * period).exp())
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
    fn arbitrary_initial_motion_obeys_ode_and_energy_balance() {
        for wn in [2.0, 7.0, 12.0] {
            for zeta in [0.1, 0.4, 0.8] {
                for v0 in [-0.1, 0.0, 0.2] {
                    let (x, v) = motion(wn, zeta, -0.02, v0, 0.0);
                    assert!((x + 0.02).abs() < 1e-12);
                    assert!((v - v0).abs() < 1e-12);
                    for step in 1..40 {
                        let t = step as f64 / (10.0 * wn);
                        let dt = 1e-5 / wn;
                        let (x, v) = motion(wn, zeta, -0.02, v0, t);
                        let (lo_x, lo_v) = motion(wn, zeta, -0.02, v0, t - dt);
                        let (hi_x, hi_v) = motion(wn, zeta, -0.02, v0, t + dt);
                        assert!(((hi_x - lo_x) / (2.0 * dt) - v).abs() < 1e-8);
                        let acceleration = (hi_v - lo_v) / (2.0 * dt);
                        assert!((acceleration + 2.0 * zeta * wn * v + wn * wn * x).abs() < 1e-8);
                        let energy = |x: f64, v: f64| (v * v + wn * wn * x * x) / 2.0;
                        let rate = (energy(hi_x, hi_v) - energy(lo_x, lo_v)) / (2.0 * dt);
                        assert!((rate + 2.0 * zeta * wn * v * v).abs() < 1e-8);
                    }
                }
            }
        }
    }
}
