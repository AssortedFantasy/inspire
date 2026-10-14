use rand::{Rng, rngs::StdRng};
fn carrier(sun: u32, ring: u32, ws: f64, wr: f64) -> f64 {
    (sun as f64 * ws + ring as f64 * wr) / (sun + ring) as f64
}
pub fn generate(rng: &mut StdRng) -> String {
    let sun = rng.random_range(4..=10) * 6;
    let planet = rng.random_range(3..=8) * 6;
    let ring = sun + 2 * planet;
    let ws = rng.random_range(-12..=12) as f64 * 60.0;
    let wr = rng.random_range(-6..=6) as f64 * 60.0;
    let wc = carrier(sun, ring, ws, wr);
    let wp = wc - (ws - wc) * sun as f64 / planet as f64;
    format!(
        "Ideal simple planetary train: sun {} teeth, each of three planets {} teeth, internal ring {} teeth. Equal tooth module; three planets are equally spaced. Ring teeth = sun + twice planet teeth.\nSun: {ws:.0} rpm; ring: {wr:.0} rpm; carrier: {wc:.3} rpm; each planet spins about its own axis at {wp:.3} rpm. All signs use one common viewing direction.\nWillis relation: (sun speed - carrier speed)/(ring speed - carrier speed) = -{ring}/{sun}, interpreted by the cross-multiplied equality even when both differences vanish.",
        sun, planet, ring
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
