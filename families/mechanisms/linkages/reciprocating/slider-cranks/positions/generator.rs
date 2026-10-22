include!("../../../../number.rs");

use rand::{Rng, rngs::StdRng};
use std::fmt::Write;
fn position(r: f64, l: f64, theta: f64) -> f64 {
    r * theta.cos() + (l * l - r * r * theta.sin().powi(2)).sqrt()
}
pub fn generate(rng: &mut StdRng) -> String {
    let r = rng.random_range(10..=60) as f64;
    let l = r * rng.random_range(3..=7) as f64;
    let mut out = format!(
        "Slider-crank\nPivot (0, 0), slider on +x\nCrank {} mm, rod {} mm\nx(θ) = {} cos θ + √({}² − {}² sin² θ) mm",
        n(r),
        n(l),
        n(r),
        n(l),
        n(r)
    );
    if rng.random_bool(0.5) {
        out.push_str("\n\nθ (°) | slider x (mm)");
        for deg in [0, 30, 60, 90, 120, 150, 180] {
            write!(
                out,
                "\n{deg} | {}",
                n(position(r, l, (deg as f64).to_radians()))
            )
            .unwrap();
        }
    } else {
        let deg = rng.random_range(0..360);
        let theta = (deg as f64).to_radians();
        write!(
            out,
            "\n\nθ = {deg}°\nCrank pin ({}, {}) mm\nSlider ({}, 0) mm\nRod angle {}°",
            n(r * theta.cos()),
            n(r * theta.sin()),
            n(position(r, l, theta)),
            n((-r * theta.sin() / l).asin().to_degrees())
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
