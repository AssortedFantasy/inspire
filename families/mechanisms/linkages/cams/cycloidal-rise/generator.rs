include!("../../../number.rs");

use rand::{Rng, rngs::StdRng};
use std::{f64::consts::TAU, fmt::Write};
fn state(h: f64, beta: f64, omega: f64, theta: f64) -> (f64, f64, f64) {
    let u = theta / beta;
    let s = h * (u - (TAU * u).sin() / TAU);
    let v = h / beta * (1.0 - (TAU * u).cos()) * omega;
    let a = h / (beta * beta) * TAU * (TAU * u).sin() * omega * omega;
    (s, v, a)
}
pub fn generate(rng: &mut StdRng) -> String {
    let h = rng.random_range(10..=80) as f64;
    let deg = rng.random_range(2..=4) * 30;
    let rpm = rng.random_range(1..=6) * 60;
    let beta = (deg as f64).to_radians();
    let omega = rpm as f64 * TAU / 60.0;
    let mut out = format!(
        "Cycloidal cam rise\nLift {} mm over {deg}°, {rpm} rpm\ns = {}[u − sin(2πu)/(2π)] mm, u = θ/{deg}°",
        n(h),
        n(h)
    );
    if rng.random_bool(0.5) {
        out.push_str("\n\nθ (°) | s (mm) | v (mm/s)");
        for q in 0..=8 {
            let u = q as f64 / 8.0;
            let (s, v, _) = state(h, beta, omega, u * beta);
            write!(out, "\n{} | {} | {}", n(u * deg as f64), n(s), n(v)).unwrap();
        }
    } else {
        let (_, v, _) = state(h, beta, omega, beta / 2.0);
        let (_, _, a) = state(h, beta, omega, beta / 4.0);
        write!(out,"\n\nRise duration {} s\nPeak speed {} mm/s at {}°\nAcceleration extrema ±{} mm/s² at {}° and {}°",n(beta/omega),n(v),n(deg as f64/2.0),n(a),n(deg as f64/4.0),n(3.0*deg as f64/4.0)).unwrap();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn endpoints_monotonicity_and_numerical_derivatives() {
        for h in [10.0, 30.0, 80.0] {
            let beta = std::f64::consts::FRAC_PI_2;
            let w = TAU;
            let start = state(h, beta, w, 0.0);
            let end = state(h, beta, w, beta);
            assert!(start.0.abs() < 1e-9 && start.1.abs() < 1e-9 && start.2.abs() < 1e-9);
            assert!((end.0 - h).abs() < 1e-9 && end.1.abs() < 1e-9 && end.2.abs() < 1e-8);
            for q in 1..100 {
                let t = beta * q as f64 / 100.0;
                let dt = 1e-5;
                let (s, v, a) = state(h, beta, w, t);
                let lo = state(h, beta, w, t - w * dt);
                let hi = state(h, beta, w, t + w * dt);
                assert!(s > 0.0 && s < h && v >= 0.0);
                assert!(((hi.0 - lo.0) / (2.0 * dt) - v).abs() < 1e-4);
                assert!(((hi.1 - lo.1) / (2.0 * dt) - a).abs() < 1e-3);
            }
        }
    }
}
