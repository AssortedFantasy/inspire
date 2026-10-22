use rand::{Rng, rngs::StdRng};
use std::{f64::consts::TAU, fmt::Write};

include!("../../../number.rs");

fn response(m: f64, k: f64, c: f64, force: f64, omega: f64) -> (f64, f64) {
    let real = k - m * omega * omega;
    let imaginary = c * omega;
    (force / real.hypot(imaginary), imaginary.atan2(real))
}

pub fn generate(rng: &mut StdRng) -> String {
    let mass = rng.random_range(1..=8) as f64;
    let wn = rng.random_range(2..=12) as f64;
    let zeta = rng.random_range(5..=65) as f64 / 100.0;
    let spring = mass * wn * wn;
    let damping = 2.0 * zeta * mass * wn;
    let force = rng.random_range(1..=12) as f64;
    let mut out = format!(
        "m = {} kg\nc = {} N·s/m\nk = {} N/m",
        n(mass),
        n(damping),
        n(spring)
    );
    match rng.random_range(0..4) {
        0 => {
            write!(
                out,
                "\nF(t) = {} cos(ωt) N\n\nω (rad/s) | X (mm) | lag (°)",
                n(force)
            )
            .unwrap();
            let ratios = if zeta < 0.2 {
                [0.25, 0.75, 0.9, 1.0, 1.1, 1.5, 3.0]
            } else {
                [0.25, 0.5, 0.8, 1.0, 1.2, 2.0, 3.0]
            };
            for ratio in ratios {
                let omega = ratio * wn;
                let (amplitude, phase) = response(mass, spring, damping, force, omega);
                write!(
                    out,
                    "\n{} | {} | {}",
                    n(omega),
                    n(amplitude * 1000.0),
                    n(phase.to_degrees())
                )
                .unwrap();
            }
        }
        1 => {
            let ratio = [0.25, 0.5, 0.8, 1.0, 1.2, 2.0, 3.0][rng.random_range(0..7)];
            let omega = ratio * wn;
            let (amplitude, phase) = response(mass, spring, damping, force, omega);
            write!(
                out,
                "\n\nF(t) = {} cos({}t) N\nx_ss(t) ≈ {} cos({}t - {}) mm\nt in s",
                n(force),
                n(omega),
                n(amplitude * 1000.0),
                n(omega),
                n(phase)
            )
            .unwrap();
        }
        2 => {
            out.push_str("\n\nG(iω) = k / (k - mω² + icω)\n\nω/ωₙ | gain (dB) | phase (°)");
            for ratio in [0.1, 0.3, 0.7, 1.0, 1.4, 3.0, 10.0] {
                let (gain, lag) = response(mass, spring, damping, spring, ratio * wn);
                write!(
                    out,
                    "\n{} | {} | {}",
                    n(ratio),
                    n(20.0 * gain.log10()),
                    n(-lag.to_degrees())
                )
                .unwrap();
            }
        }
        _ => {
            let ratio = [0.5, 0.8, 1.0, 1.2, 2.0][rng.random_range(0..5)];
            let omega = ratio * wn;
            let (amplitude, _) = response(mass, spring, damping, force, omega);
            let power = damping * omega * omega * amplitude * amplitude / 2.0;
            write!(
                out,
                "\nF(t) = {} cos({}t) N\n\nMean absorbed power = {} W\nWork per cycle = {} J",
                n(force),
                n(omega),
                n(power),
                n(power * TAU / omega)
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
    fn amplitude_phase_satisfy_sinusoidal_force_balance() {
        for mass in [1.0, 3.0, 8.0] {
            for wn in [2.0, 7.0, 12.0] {
                for zeta in [0.05, 0.2, 0.65] {
                    let k = mass * wn * wn;
                    let c = 2.0 * zeta * mass * wn;
                    for ratio in [0.1, 0.5, 0.9, 1.0, 1.1, 2.0, 10.0] {
                        let omega = ratio * wn;
                        let (amplitude, phase) = response(mass, k, c, 7.0, omega);
                        assert!(amplitude > 0.0 && (0.0..=std::f64::consts::PI).contains(&phase));
                        for step in 0..24 {
                            let angle = TAU * step as f64 / 24.0;
                            let x = amplitude * (angle - phase).cos();
                            let v = -amplitude * omega * (angle - phase).sin();
                            let acceleration = -omega * omega * x;
                            assert!(
                                (mass * acceleration + c * v + k * x - 7.0 * angle.cos()).abs()
                                    < 1e-9
                            );
                        }
                        let force_power = 7.0 * amplitude * omega * phase.sin() / 2.0;
                        let damper_power = c * omega * omega * amplitude * amplitude / 2.0;
                        assert!((force_power - damper_power).abs() < 1e-9);
                    }
                }
            }
        }
    }

    #[test]
    fn normalized_response_has_correct_static_and_resonant_limits() {
        let (static_gain, static_lag) = response(2.0, 32.0, 3.2, 32.0, 0.0);
        assert_eq!(static_gain, 1.0);
        assert_eq!(static_lag, 0.0);
        let (gain, lag) = response(2.0, 32.0, 3.2, 32.0, 4.0);
        assert!((gain - 2.5).abs() < 1e-12);
        assert!((lag - std::f64::consts::FRAC_PI_2).abs() < 1e-12);
    }
}
