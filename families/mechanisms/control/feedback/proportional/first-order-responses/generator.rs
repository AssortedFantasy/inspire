use rand::{Rng, rngs::StdRng};
use std::fmt::Write;
fn response(k: f64, tau: f64, kp: f64, r: f64, t: f64) -> f64 {
    let dc = k * kp / (1.0 + k * kp);
    r * dc * (1.0 - (-(1.0 + k * kp) * t / tau).exp())
}
pub fn generate(rng: &mut StdRng) -> String {
    let k = rng.random_range(1..=4) as f64;
    let tau = rng.random_range(2..=10) as f64;
    let kp = rng.random_range(1..=8) as f64 / 2.0;
    let r = rng.random_range(2..=10) as f64;
    let closed = tau / (1.0 + k * kp);
    let steady = r * k * kp / (1.0 + k * kp);
    let mut out = format!(
        "Dimensionless first-order process: tau dy/dt + y = K u, with tau={tau:.1} s, K={k:.1}. Negative unity feedback uses u=Kp(r-y), Kp={kp:.1}; target steps from 0 to r={r:.1} at t=0, initial y=0. Ideal measurement, no delay or actuator saturation.\nClosed-loop time constant {closed:.4} s; steady output {steady:.4}, persistent error {:.4}.\nTime s | output y | command u",
        r - steady
    );
    for multiple in [0.0, 0.5, 1.0, 2.0, 4.0] {
        let t = closed * multiple;
        let y = response(k, tau, kp, r, t);
        write!(out, "\n{t:.4} | {y:.5} | {:.5}", kp * (r - y)).unwrap();
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution_satisfies_process_equation_and_final_value() {
        for k in 1..=4 {
            for tau in 2..=10 {
                for gain in 1..=8 {
                    let k = k as f64;
                    let tau = tau as f64;
                    let kp = gain as f64 / 2.0;
                    let r = 3.0;
                    let limit = r * k * kp / (1.0 + k * kp);
                    let tc = tau / (1.0 + k * kp);
                    assert!((response(k, tau, kp, r, 40.0 * tc) - limit).abs() < 1e-12);
                    for q in 1..20 {
                        let t = tc * q as f64 / 10.0;
                        let dt = tc * 1e-5;
                        let y = response(k, tau, kp, r, t);
                        let dy = (response(k, tau, kp, r, t + dt)
                            - response(k, tau, kp, r, t - dt))
                            / (2.0 * dt);
                        assert!((tau * dy + y - k * kp * (r - y)).abs() < 1e-7);
                    }
                }
            }
        }
    }
}
