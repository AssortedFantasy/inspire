use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

include!("../../../number.rs");

fn crossing(from: f64, to: f64, equilibrium: f64, tau: f64) -> f64 {
    -tau * ((to - equilibrium) / (from - equilibrium)).ln()
}

pub fn generate(rng: &mut StdRng) -> String {
    let ambient = rng.random_range(10..=20) as f64;
    let low = ambient + rng.random_range(4..=10) as f64;
    let high = low + rng.random_range(1..=4) as f64;
    let gain = high - ambient + rng.random_range(10..=30) as f64;
    let tau = rng.random_range(3..=18) as f64 * 60.0;
    let power = rng.random_range(3..=15) * 100;
    let hot = ambient + gain;
    let on = crossing(low, high, hot, tau);
    let off = crossing(high, low, ambient, tau);
    let mut out = format!(
        "Thermostat cycle\n{}T′ = {} − T + {}u, T in °C, t in s\nu ∈ {{0, 1}}, heater {power} W\nSwitch ON at {}°C, OFF at {}°C\n\nHeating {} s\nCooling {} s\nDuty cycle {}%",
        n(tau),
        n(ambient),
        n(gain),
        n(low),
        n(high),
        n(on),
        n(off),
        n(100.0 * on / (on + off))
    );
    if rng.random_bool(0.5) {
        write!(
            out,
            "\nMean heater power {} W\nCycles per hour {}",
            n(power as f64 * on / (on + off)),
            n(3600.0 / (on + off))
        )
        .unwrap();
    } else {
        out.push_str("\n\nt (s) | T (°C) | u");
        for (elapsed, from, equilibrium, command, offset) in
            [(on, low, hot, 1, 0.0), (off, high, ambient, 0, on)]
        {
            for fraction in [0.0, 0.5, 1.0] {
                let t = elapsed * fraction;
                let temp = equilibrium + (from - equilibrium) * (-t / tau).exp();
                write!(out, "\n{} | {} | {command}", n(offset + t), n(temp)).unwrap();
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threshold_times_satisfy_exact_thermal_solution() {
        for ambient in 10..=20 {
            for width in 1..=4 {
                for tau in [180.0, 600.0, 1080.0] {
                    let ambient = ambient as f64;
                    let low = ambient + 8.0;
                    let high = low + width as f64;
                    let hot = high + 15.0;
                    for (from, to, equilibrium) in [(low, high, hot), (high, low, ambient)] {
                        let time = crossing(from, to, equilibrium, tau);
                        assert!(time > 0.0 && time.is_finite());
                        let reached = equilibrium + (from - equilibrium) * (-time / tau).exp();
                        assert!((reached - to).abs() < 1e-12);
                    }
                }
            }
        }
    }
}
