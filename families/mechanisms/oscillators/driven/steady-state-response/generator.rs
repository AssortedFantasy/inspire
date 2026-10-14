use rand::{Rng, rngs::StdRng};
use std::fmt::Write;
fn response(m: f64, k: f64, c: f64, f: f64, omega: f64) -> (f64, f64) {
    let real = k - m * omega * omega;
    let imaginary = c * omega;
    (f / real.hypot(imaginary), imaginary.atan2(real))
}
pub fn generate(rng: &mut StdRng) -> String {
    let mass = rng.random_range(1..=6) as f64;
    let wn = rng.random_range(2..=8) as f64;
    let zeta = rng.random_range(1..=5) as f64 / 10.0;
    let spring = mass * wn * wn;
    let damping = 2.0 * zeta * mass * wn;
    let force = rng.random_range(1..=12) as f64;
    let mut out = format!(
        "Forced linear spring-mass-damper in sinusoidal steady state: mass {mass:.1} kg, stiffness {spring:.1} N/m, viscous damping {damping:.3} N s/m. Force F(t)={force:.1} cos(omega t) N. The initial transient has decayed.\nResponse x(t)=X cos(omega t-phi); positive phi is displacement lag relative to force. Natural frequency {wn:.3} rad/s, damping ratio {zeta:.3}.\nFrequency ratio omega/wn | amplitude X mm | phase lag degrees"
    );
    for ratio in [0.25, 0.5, 0.8, 1.0, 1.2, 2.0, 3.0] {
        let (a, p) = response(mass, spring, damping, force, ratio * wn);
        write!(
            out,
            "\n{ratio:.2} | {:.5} | {:.4}",
            a * 1000.0,
            p.to_degrees()
        )
        .unwrap();
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn amplitude_phase_satisfy_sinusoidal_force_balance() {
        for m in 1..=6 {
            for wn in 2..=8 {
                for z in 1..=5 {
                    let m = m as f64;
                    let wn = wn as f64;
                    let z = z as f64 / 10.0;
                    let k = m * wn * wn;
                    let c = 2.0 * z * m * wn;
                    for ratio in [0.25, 0.5, 0.8, 1.0, 1.2, 2.0, 3.0] {
                        let w = ratio * wn;
                        let (a, p) = response(m, k, c, 7.0, w);
                        for phase in 0..12 {
                            let t = phase as f64 / (12.0 * w);
                            let x = a * (w * t - p).cos();
                            let v = -a * w * (w * t - p).sin();
                            let accel = -w * w * x;
                            assert!((m * accel + c * v + k * x - 7.0 * (w * t).cos()).abs() < 1e-9);
                        }
                    }
                }
            }
        }
    }
}
