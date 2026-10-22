include!("../../../../number.rs");

use rand::{Rng, rngs::StdRng};
use std::f64::consts::PI;
fn flow(r: f64, length: f64, mu: f64, dp: f64) -> f64 {
    PI * r.powi(4) * dp / (8.0 * mu * length)
}
pub fn generate(rng: &mut StdRng) -> String {
    let radius = rng.random_range(1..=4) as f64 / 1000.0;
    let length = rng.random_range(1..=4) as f64;
    let mu = rng.random_range(2..=8) as f64 / 100.0;
    let dp = rng.random_range(1..=6) as f64 * 100.0;
    let q = flow(radius, length, mu, dp);
    let velocity = q / (PI * radius * radius);
    let mut out = format!(
        "Laminar Newtonian tube flow\nRadius {} mm, length {} m\nViscosity {} Pa·s\nPressure drop {} Pa\nQ = {} mL/s",
        n(radius * 1000.0),
        n(length),
        n(mu),
        n(dp),
        n(q * 1e6)
    );
    if rng.random_bool(0.5) {
        out.push_str("\n\nr/radius | velocity (mm/s)");
        for fraction in [0.0, 0.25, 0.5, 0.75, 1.0] {
            out.push_str(&format!(
                "\n{} | {}",
                n(fraction),
                n(2.0 * velocity * (1.0 - fraction * fraction) * 1000.0)
            ));
        }
    } else {
        out.push_str(&format!(
            "\nMean speed {} mm/s\nWall shear {} Pa\nDensity 900 kg/m³\nRe = {}",
            n(velocity * 1000.0),
            n(dp * radius / (2.0 * length)),
            n(900.0 * velocity * 2.0 * radius / mu)
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn integrate_parabolic_profile_and_check_reynolds_bound() {
        for millimeters in 1..=4 {
            for viscosity in 2..=8 {
                for pressure in 1..=6 {
                    let r = millimeters as f64 / 1000.0;
                    let mu = viscosity as f64 / 100.0;
                    let dp = pressure as f64 * 100.0;
                    let q = flow(r, 1.0, mu, dp);
                    let mut integrated = 0.0;
                    let n = 10000;
                    let dr = r / n as f64;
                    for i in 0..n {
                        let s = (i as f64 + 0.5) * dr;
                        let v = dp * (r * r - s * s) / (4.0 * mu);
                        integrated += v * TAU * s * dr;
                    }
                    assert!(((integrated - q) / q).abs() < 1e-7);
                    assert!(900.0 * q / (PI * r * r) * 2.0 * r / mu < 2000.0);
                }
            }
        }
    }
    const TAU: f64 = std::f64::consts::TAU;
}
