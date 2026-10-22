use rand::{Rng, rngs::StdRng};
use std::{f64::consts::PI, fmt::Write};

include!("../../../../number.rs");

fn state(slots: u32, centers: f64, theta: f64) -> (f64, f64, f64) {
    let half_step = PI / slots as f64;
    let radius = centers * half_step.sin();
    let x = centers - radius * theta.cos();
    let y = radius * theta.sin();
    let angle = -y.atan2(x) - half_step;
    let ratio = -radius * (centers * theta.cos() - radius) / (x * x + y * y);
    (angle, ratio, x.hypot(y))
}

pub fn generate(rng: &mut StdRng) -> String {
    let slots = rng.random_range(4..=8);
    let centers = rng.random_range(4..=12) as f64 * 10.0;
    let rpm = rng.random_range(2..=12) * 10;
    let half_step = PI / slots as f64;
    let engagement = PI / 2.0 - half_step;
    let mut out = format!(
        "External Geneva drive, {slots} slots\nCenters {} mm, crank radius {} mm\nDriver {rpm} rpm counterclockwise\nIndex −{}°, dwell {}° of driver rotation\nθ = 0 when the pin is nearest the driven center",
        n(centers),
        n(centers * half_step.sin()),
        n(360.0 / slots as f64),
        n(180.0 + 360.0 / slots as f64)
    );
    out.push_str("\n\nθ (°) | wheel Δφ (°) | wheel rpm | pin radius (mm)");
    for fraction in [-1.0, -0.75, -0.5, 0.0, 0.5, 0.75, 1.0] {
        let theta = fraction * engagement;
        let (phi, ratio, reach) = state(slots, centers, theta);
        write!(
            out,
            "\n{} | {} | {} | {}",
            n(theta.to_degrees()),
            n(phi.to_degrees()),
            n(ratio * rpm as f64),
            n(reach)
        )
        .unwrap();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engagement_completes_one_index_with_tangent_entry_and_exit() {
        for slots in 4..=8 {
            let alpha = PI / 2.0 - PI / slots as f64;
            for centers in [40.0, 90.0, 120.0] {
                let start = state(slots, centers, -alpha);
                let end = state(slots, centers, alpha);
                assert!(start.0.abs() < 1e-12);
                assert!((end.0 + 2.0 * PI / slots as f64).abs() < 1e-12);
                assert!(start.1.abs() < 1e-12 && end.1.abs() < 1e-12);
                let mut previous = start.0;
                for q in 1..100 {
                    let theta = -alpha + 2.0 * alpha * q as f64 / 100.0;
                    let (angle, ratio, radius) = state(slots, centers, theta);
                    let numerical = (state(slots, centers, theta + 1e-6).0
                        - state(slots, centers, theta - 1e-6).0)
                        / 2e-6;
                    assert!(angle < previous && ratio < 0.0 && radius > 0.0);
                    assert!((numerical - ratio).abs() < 1e-8);
                    previous = angle;
                }
            }
        }
    }
}
