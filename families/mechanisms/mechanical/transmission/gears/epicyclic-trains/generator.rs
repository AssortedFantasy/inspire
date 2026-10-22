include!("../../../../number.rs");

use rand::{Rng, rngs::StdRng};
fn carrier(sun: u32, ring: u32, ws: f64, wr: f64) -> f64 {
    (sun as f64 * ws + ring as f64 * wr) / (sun + ring) as f64
}
pub fn generate(rng: &mut StdRng) -> String {
    let sun = rng.random_range(4..=10) * 6;
    let planet = rng.random_range(3..=8) * 6;
    let ring = sun + 2 * planet;
    let input = rng.random_range(2..=12) as f64 * 60.0;
    let (ws, wr, mode) = match rng.random_range(0..4) {
        0 => (input, 0.0, "ring fixed"),
        1 => (0.0, input, "sun fixed"),
        2 => (input, -input * sun as f64 / ring as f64, "carrier fixed"),
        _ => (
            input,
            rng.random_range(-6..=6) as f64 * 60.0,
            "differential drive",
        ),
    };
    let wc = carrier(sun, ring, ws, wr);
    let wp = wc - (ws - wc) * sun as f64 / planet as f64;
    format!(
        "Planetary gear — {mode}\nSun {sun}T, three planets {planet}T, ring {ring}T\n\nMember | rpm\nSun | {}\nRing | {}\nCarrier | {}\nPlanet spin | {}",
        n(ws),
        n(wr),
        n(wc),
        n(wp)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn both_relative_meshes_close() {
        for sun in (24..=60).step_by(6) {
            for planet in (18..=48).step_by(6) {
                let ring = sun + 2 * planet;
                assert_eq!((sun + ring) % 3, 0);
                for ws in [-720.0, 0.0, 720.0] {
                    for wr in [-360.0, 0.0, 360.0] {
                        let wc = carrier(sun, ring, ws, wr);
                        let wp = wc - (ws - wc) * sun as f64 / planet as f64;
                        assert!(((ws - wc) * sun as f64 + (wp - wc) * planet as f64).abs() < 1e-8);
                        assert!(((wr - wc) * ring as f64 - (wp - wc) * planet as f64).abs() < 1e-8);
                    }
                }
            }
        }
    }
}
