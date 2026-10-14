use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

fn displacement(wn: f64, zeta: f64, x0: f64, t: f64) -> f64 {
    if zeta < 1.0 {
        let wd = wn * (1.0 - zeta * zeta).sqrt();
        x0 * (-zeta * wn * t).exp() * ((wd * t).cos() + zeta * wn / wd * (wd * t).sin())
    } else if zeta == 1.0 {
        x0 * (1.0 + wn * t) * (-wn * t).exp()
    } else {
        let q = (zeta * zeta - 1.0).sqrt();
        let r1 = -wn * (zeta - q);
        let r2 = -wn * (zeta + q);
        x0 * (-r2 * (r1 * t).exp() + r1 * (r2 * t).exp()) / (r1 - r2)
    }
}
pub fn generate(rng: &mut StdRng) -> String {
    let mass = rng.random_range(1..=8) as f64;
    let wn = rng.random_range(2..=8) as f64;
    let zeta = rng.random_range(1..=8) as f64 / 10.0;
    let x0 = rng.random_range(5..=30) as f64 / 1000.0;
    let spring = mass * wn * wn;
    let damping = 2.0 * zeta * mass * wn;
    let mut out = format!(
        "Underdamped free spring-mass-damper: mass {mass:.1} kg, stiffness {spring:.1} N/m, viscous damping {damping:.3} N s/m. Linear ideal model m x'' + c x' + k x = 0; displacement from equilibrium x(0)={:.1} mm, initial velocity 0.\nNatural angular frequency {wn:.3} rad/s; damping ratio {zeta:.3}. No external forcing.\nTime s | displacement mm",
        x0 * 1000.0
    );
    for multiple in [0.0, 0.5, 1.0, 2.0, 4.0, 8.0] {
        let t = multiple / wn;
        write!(
            out,
            "\n{t:.4} | {:.5}",
            displacement(wn, zeta, x0, t) * 1000.0
        )
        .unwrap();
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn analytic_motion_satisfies_ode_and_initial_rest() {
        for wn in 2..=8 {
            let w = wn as f64;
            for z in [0.1, 0.3, 0.5, 0.8] {
                let x0 = 0.02;
                let dt = 1e-5 / w;
                assert!((displacement(w, z, x0, 0.0) - x0).abs() < 1e-12);
                let v0 = (displacement(w, z, x0, dt) - displacement(w, z, x0, -dt)) / (2.0 * dt);
                assert!(v0.abs() < 1e-8);
                for q in 1..100 {
                    let t = q as f64 / (20.0 * w);
                    let lo = displacement(w, z, x0, t - dt);
                    let hi = displacement(w, z, x0, t + dt);
                    let x = displacement(w, z, x0, t);
                    let v = (hi - lo) / (2.0 * dt);
                    let a = (hi - 2.0 * x + lo) / (dt * dt);
                    assert!((a + 2.0 * z * w * v + w * w * x).abs() < 2e-5);
                }
                assert!(displacement(w, z, x0, 100.0 / w).abs() < 1e-6);
            }
        }
    }
}
