use rand::{Rng, rngs::StdRng};
use std::f64::consts::PI;
fn flow(r: f64, length: f64, mu: f64, dp: f64) -> f64 {
    PI * r.powi(4) * dp / (8.0 * mu * length)
}
pub fn generate(rng: &mut StdRng) -> String {
    let radius = rng.random_range(1..=4) as f64 / 1000.0;
    let length = rng.random_range(1..=4) as f64;
    let mu = rng.random_range(2..=8) as f64 / 100.0;
    let density = 900.0;
    let dp = rng.random_range(1..=6) as f64 * 100.0;
    let q = flow(radius, length, mu, dp);
    let velocity = q / (PI * radius * radius);
    let re = density * velocity * 2.0 * radius / mu;
    format!(
        "Steady, fully developed laminar flow of a Newtonian liquid through a straight circular tube. No-slip wall, rigid tube; entrance effects neglected. Radius {:.1} mm, length {length:.1} m, viscosity {mu:.3} Pa s, density {density:.0} kg/m^3; pressure drop {dp:.0} Pa.\nPoiseuille flow Q = pi r^4 DeltaP/(8 mu L) = {:.6} mL/s. Mean speed {:.6} m/s; Reynolds number {re:.3} (within the assumed laminar range). Centerline speed {:.6} m/s.\nVelocity profile v(s) = DeltaP(r^2-s^2)/(4 mu L), where s is radial distance from the axis. Pressure power dissipated = {:.8} W.",
        radius * 1000.0,
        q * 1e6,
        velocity,
        2.0 * velocity,
        dp * q
    )
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
