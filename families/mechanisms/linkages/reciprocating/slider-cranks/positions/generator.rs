use rand::{Rng, rngs::StdRng};
use std::fmt::Write;
fn position(r: f64, l: f64, theta: f64) -> f64 {
    r * theta.cos() + (l * l - r * r * theta.sin().powi(2)).sqrt()
}
pub fn generate(rng: &mut StdRng) -> String {
    let r = rng.random_range(10..=60) as f64;
    let l = r * rng.random_range(3..=7) as f64;
    let mut out = format!(
        "In-line slider-crank: crank radius r = {r:.0} mm; connecting rod l = {l:.0} mm. Crank pivot (0,0); slider constrained to the x axis; the rod's slider end lies to the right of its crank end. Rigid links and ideal revolute/prismatic joints.\nx(theta) = r cos(theta) + sqrt(l^2 - r^2 sin(theta)^2). Stroke = {:.0} mm.\nCrank angle (degrees) | slider x (mm)",
        2.0 * r
    );
    for deg in [0, 30, 60, 90, 120, 150, 180, 210, 240, 270, 300, 330] {
        write!(
            out,
            "\n{deg:3} | {:.4}",
            position(r, l, (deg as f64).to_radians())
        )
        .unwrap();
    }
    out
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn connecting_rod_length_and_dead_centers() {
        for r in 10..=60 {
            for ratio in 3..=7 {
                let r = r as f64;
                let l = r * ratio as f64;
                for deg in 0..360 {
                    let t = (deg as f64).to_radians();
                    let x = position(r, l, t);
                    assert!(((x - r * t.cos()).hypot(r * t.sin()) - l).abs() < 1e-9);
                    assert!((l - r - 1e-9..=l + r + 1e-9).contains(&x));
                }
                assert!(
                    (position(r, l, 0.0) - position(r, l, std::f64::consts::PI) - 2.0 * r).abs()
                        < 1e-9
                );
            }
        }
    }
}
