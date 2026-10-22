include!("../../../number.rs");

use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

const SHAPES: [[f64; 3]; 10] = [
    [3.5, 3.0, 4.0],
    [4.0, 2.5, 3.8],
    [3.0, 2.8, 4.2],
    [4.3, 3.3, 4.9],
    [2.8, 3.8, 4.1],
    [3.6, 2.7, 3.7],
    [4.0, 4.0, 5.0],
    [3.0, 3.0, 4.0],
    [4.2, 2.9, 4.5],
    [3.2, 3.7, 4.6],
];
fn joint(a: f64, b: f64, c: f64, d: f64, t: f64) -> (f64, f64) {
    let ax = a * t.cos();
    let ay = a * t.sin();
    let dx = d - ax;
    let dy = -ay;
    let dist = dx.hypot(dy);
    let along = (b * b - c * c + dist * dist) / (2.0 * dist);
    let high = (b * b - along * along).sqrt();
    (
        ax + along * dx / dist - high * dy / dist,
        ay + along * dy / dist + high * dx / dist,
    )
}
pub fn generate(rng: &mut StdRng) -> String {
    let a = rng.random_range(10..=30) as f64;
    let [rb, rc, rd] = SHAPES[rng.random_range(0..SHAPES.len())];
    let (b, c, d) = (a * rb, a * rc, a * rd);
    let mut out = format!(
        "Four-bar O—A—B—D\nO = (0, 0), D = ({}, 0) mm\nOA = {} mm, AB = {} mm, BD = {} mm\nB lies left of directed A→D\nθ = arg(A)",
        n(d),
        n(a),
        n(b),
        n(c)
    );
    if rng.random_bool(0.5) {
        out.push_str("\n\nθ (°) | Bx (mm) | By (mm)");
        for deg in [0, 45, 90, 135, 180, 225, 270, 315] {
            let (x, y) = joint(a, b, c, d, (deg as f64).to_radians());
            write!(out, "\n{deg} | {} | {}", n(x), n(y)).unwrap();
        }
    } else {
        let deg = rng.random_range(0..360);
        let theta = (deg as f64).to_radians();
        let (x, y) = joint(a, b, c, d, theta);
        write!(
            out,
            "\n\nθ = {deg}°\nA = ({}, {}) mm\nB = ({}, {}) mm\n∠(D→B, +x) = {}°",
            n(a * theta.cos()),
            n(a * theta.sin()),
            n(x),
            n(y),
            n(y.atan2(x - d).to_degrees())
        )
        .unwrap();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn circle_intersections_enforce_both_links_at_every_angle() {
        for a in 10..=30 {
            let a = a as f64;
            for [rb, rc, rd] in SHAPES {
                let (b, c, d) = (a * rb, a * rc, a * rd);
                let mut sorted = [a, b, c, d];
                sorted.sort_by(f64::total_cmp);
                assert!(sorted[0] + sorted[3] < sorted[1] + sorted[2]);
                assert!(d - a > (b - c).abs() && d + a < b + c);
                for deg in 0..360 {
                    let t = (deg as f64).to_radians();
                    let (x, y) = joint(a, b, c, d, t);
                    assert!(((x - a * t.cos()).hypot(y - a * t.sin()) - b).abs() < 1e-9);
                    assert!(((x - d).hypot(y) - c).abs() < 1e-9);
                }
            }
        }
    }
}
