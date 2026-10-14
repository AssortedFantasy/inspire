use rand::{Rng, rngs::StdRng};

fn speeds(teeth: [u32; 4], input: f64) -> [f64; 3] {
    [
        input,
        -input * teeth[0] as f64 / teeth[1] as f64,
        input * teeth[0] as f64 * teeth[2] as f64 / (teeth[1] as f64 * teeth[3] as f64),
    ]
}

pub fn generate(rng: &mut StdRng) -> String {
    let teeth = std::array::from_fn(|_| rng.random_range(3..=12) * 6);
    let input = rng.random_range(2..=12) as f64 * 60.0;
    let torque = rng.random_range(2..=20) as f64;
    let rpm = speeds(teeth, input);
    let output_torque = torque * input / rpm[2];
    format!(
        "Ideal two-stage compound spur-gear train. A ({} teeth) meshes externally with B ({} teeth); B and C ({} teeth) share a shaft; C meshes externally with D ({} teeth). All gears have the same module, rigid shafts, negligible loss, and no slip.\nInput A: {:.0} rpm, {:.0} N m. Intermediate shaft: {:.3} rpm. Output D: {:.3} rpm, {:.3} N m available load torque. Signed rpm is measured from the same viewing direction; positive is A's direction.\nSpeed ratio D/A = {:.6}; ideal input/output power = {:.3} W.",
        teeth[0],
        teeth[1],
        teeth[2],
        teeth[3],
        input,
        torque,
        rpm[1],
        rpm[2],
        output_torque,
        rpm[2] / input,
        torque * input * std::f64::consts::TAU / 60.0
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn both_meshes_and_power_balance() {
        for a in 3..=12 {
            for b in 3..=12 {
                for c in 3..=12 {
                    for d in 3..=12 {
                        let t = [a * 6, b * 6, c * 6, d * 6];
                        let r = speeds(t, 360.0);
                        assert!((r[0] * t[0] as f64 + r[1] * t[1] as f64).abs() < 1e-8);
                        assert!((r[1] * t[2] as f64 + r[2] * t[3] as f64).abs() < 1e-8);
                        let out = 7.0 * r[0] / r[2];
                        assert!((out * r[2] - 7.0 * r[0]).abs() < 1e-8);
                    }
                }
            }
        }
    }
}
