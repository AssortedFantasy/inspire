use rand::{Rng, rngs::StdRng};
use std::{f64::consts::PI, fmt::Write};

include!("../../../../number.rs");

fn conductance(radius: f64, length: f64, viscosity: f64) -> f64 {
    PI * radius.powi(4) / (8.0 * viscosity * length)
}

pub fn generate(rng: &mut StdRng) -> String {
    let count = rng.random_range(2..=5);
    let viscosity = rng.random_range(3..=9) as f64 / 100.0;
    let pressure = rng.random_range(1..=8) as f64 * 100.0;
    let tubes: Vec<_> = (0..count)
        .map(|_| {
            (
                rng.random_range(5..=20) as f64 / 10000.0,
                rng.random_range(5..=30) as f64 / 10.0,
            )
        })
        .collect();
    let total: f64 = tubes
        .iter()
        .map(|&(r, l)| pressure * conductance(r, l, viscosity))
        .sum();
    let mut out = format!(
        "Laminar parallel capillary manifold\nNewtonian liquid, μ = {} Pa·s\nCommon pressure drop {} Pa\n\nBranch | radius (mm) | length (m) | Q (mL/min)",
        n(viscosity),
        n(pressure)
    );
    for (index, &(radius, length)) in tubes.iter().enumerate() {
        write!(
            out,
            "\n{} | {} | {} | {}",
            index + 1,
            n(radius * 1000.0),
            n(length),
            n(pressure * conductance(radius, length, viscosity) * 6e7)
        )
        .unwrap();
    }
    write!(out, "\n\nTotal {} mL/min", n(total * 6e7)).unwrap();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_flows_add_and_stay_laminar() {
        for radius in 5..=20 {
            for length in 5..=30 {
                let r = radius as f64 / 10000.0;
                let l = length as f64 / 10.0;
                let c = conductance(r, l, 0.03);
                assert!((conductance(2.0 * r, l, 0.03) / c - 16.0).abs() < 1e-10);
                assert!((conductance(r, 2.0 * l, 0.03) / c - 0.5).abs() < 1e-10);
                let q = c * 800.0;
                assert!(1000.0 * q / (PI * r * r) * 2.0 * r / 0.03 < 2000.0);
            }
        }
        let c1 = conductance(0.001, 1.0, 0.05);
        let c2 = conductance(0.0015, 2.0, 0.05);
        let flow = 1e-6;
        let pressure = flow / (c1 + c2);
        assert!((pressure * c1 + pressure * c2 - flow).abs() < 1e-15);
    }
}
