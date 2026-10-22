include!("../../../number.rs");

use rand::{Rng, rngs::StdRng};
use std::f64::consts::PI;
fn press(d1: f64, d2: f64, force: f64, travel: f64) -> (f64, f64, f64) {
    let a1 = PI * d1 * d1 / 4.0;
    let a2 = PI * d2 * d2 / 4.0;
    (force / a1, force * a2 / a1, travel * a1 / a2)
}
pub fn generate(rng: &mut StdRng) -> String {
    let d1 = rng.random_range(10..=30) as f64;
    let d2 = d1 * rng.random_range(2..=6) as f64;
    let f = rng.random_range(1..=12) as f64 * 100.0;
    let x = rng.random_range(2..=10) as f64 * 10.0;
    let (p, out, travel) = press(d1 / 1000.0, d2 / 1000.0, f, x / 1000.0);
    if rng.random_bool(0.5) {
        format!(
            "Lossless hydraulic press\nQuantity | input | output\nPiston Ø (mm) | {} | {}\nForce (N) | {} | {}\nTravel (mm) | {} | {}\n\np = {} MPa",
            n(d1),
            n(d2),
            n(f),
            n(out),
            n(x),
            n(travel * 1000.0),
            n(p / 1e6)
        )
    } else {
        format!(
            "Lossless hydraulic press\nØ{} mm → Ø{} mm\n{} N × {} mm → {} N × {} mm\n\nDisplacement {} cm³\nWork {} J",
            n(d1),
            n(d2),
            n(f),
            n(x),
            n(out),
            n(travel * 1000.0),
            n(PI * d1 * d1 / 4.0 * x / 1000.0),
            n(f * x / 1000.0)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pressure_volume_and_work_agree_independently() {
        for d1 in 10..=30 {
            for ratio in 2..=6 {
                let d1 = d1 as f64 / 1000.0;
                let d2 = d1 * ratio as f64;
                let (p, out, x2) = press(d1, d2, 500.0, 0.06);
                assert!((p * PI * d2 * d2 / 4.0 - out).abs() < 1e-8);
                assert!((PI * d1 * d1 / 4.0 * 0.06 - PI * d2 * d2 / 4.0 * x2).abs() < 1e-10);
                assert!((500.0 * 0.06 - out * x2).abs() < 1e-8);
            }
        }
    }
}
