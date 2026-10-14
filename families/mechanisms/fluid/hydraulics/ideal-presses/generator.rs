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
    format!(
        "Ideal hydraulic press: incompressible liquid, pistons at equal elevation, no friction or leakage, quasistatic motion. Input piston diameter {d1:.0} mm; output diameter {d2:.0} mm.\nInput force {f:.0} N and input travel {x:.0} mm produce pressure {:.3} MPa, output force {out:.3} N and output travel {:.3} mm.\nDisplaced volume {:.3} cm^3; equal ideal input/output work {:.3} J. The volume and work balances trade travel for force.",
        p / 1e6,
        travel * 1000.0,
        PI * d1 * d1 / 4.0 * x / 1000.0,
        f * x / 1000.0
    )
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
