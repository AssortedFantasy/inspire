include!("../../../../number.rs");

use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

#[derive(Clone, Copy)]
struct Sample {
    y: f64,
    integral: f64,
    command: f64,
    blocked: bool,
}
fn simulate(kp: f64, ki: f64, tau: f64, anti: bool) -> Vec<Sample> {
    let dt = 0.2;
    let decay = (-dt / tau).exp();
    let mut y = 0.0;
    let mut integral = 0.0;
    let mut rows = Vec::new();
    for step in 0..160 {
        let target = if step < 50 { 1.5 } else { 0.35 };
        let error = target - y;
        let raw = kp * error + integral;
        let command = raw.clamp(0.0, 1.0);
        let blocked = anti && ((raw > 1.0 && error > 0.0) || (raw < 0.0 && error < 0.0));
        rows.push(Sample {
            y,
            integral,
            command,
            blocked,
        });
        if !blocked {
            integral += ki * dt * error;
        }
        y = decay * y + (1.0 - decay) * command;
    }
    rows
}
pub fn generate(rng: &mut StdRng) -> String {
    let kp = rng.random_range(2..=6) as f64 / 2.0;
    let ki = rng.random_range(2..=6) as f64 / 4.0;
    let tau = rng.random_range(1..=3) as f64;
    let protected = simulate(kp, ki, tau, true);
    let plain = simulate(kp, ki, tau, false);
    let mut out = format!(
        "PI saturation recovery\n{}y′ + y = u, 0 ≤ u ≤ 1\nKp = {}, Ki = {} s⁻¹, Δt = 0.2 s\ny(0) = I(0) = 0\nr = 1.5 until 10 s, then 0.35\nAnti-windup: freeze I while error drives u past a limit",
        n(tau),
        n(kp),
        n(ki)
    );
    if rng.random_bool(0.5) {
        out.push_str("\n\nt (s) | y (anti-windup) | y (free I)");
        for index in [0, 25, 49, 50, 55, 75, 100, 159] {
            write!(
                out,
                "\n{} | {} | {}",
                n(index as f64 * 0.2),
                n(protected[index].y),
                n(plain[index].y)
            )
            .unwrap();
        }
    } else {
        out.push_str("\n\nt (s) | u | I | freeze");
        for index in [45, 49, 50, 51, 55, 60, 75, 100] {
            let row = protected[index];
            write!(
                out,
                "\n{} | {} | {} | {}",
                n(index as f64 * 0.2),
                n(row.command),
                n(row.integral),
                if row.blocked { "yes" } else { "no" }
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
    fn actuator_bounds_conditional_integration_and_exact_held_input() {
        for kp in [1.0, 2.0, 3.0] {
            for ki in [0.5, 1.0, 1.5] {
                for tau in [1.0, 2.0, 3.0] {
                    let protected = simulate(kp, ki, tau, true);
                    let plain = simulate(kp, ki, tau, false);
                    let decay = (-0.2_f64 / tau).exp();
                    assert!(plain[49].integral > protected[49].integral + 1.0);
                    for pair in protected.windows(2) {
                        let a = pair[0];
                        let b = pair[1];
                        assert!((0.0..=1.0).contains(&a.command));
                        assert!((0.0..=1.0).contains(&a.y));
                        assert!((b.y - (a.y * decay + a.command * (1.0 - decay))).abs() < 1e-12);
                        if a.blocked {
                            assert_eq!(a.integral, b.integral);
                        }
                    }
                    assert!((protected[159].y - 0.35).abs() < 0.025);
                }
            }
        }
    }
}
