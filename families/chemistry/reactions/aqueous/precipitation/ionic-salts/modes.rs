use rand::{Rng, rngs::StdRng};

// OpenStax Chemistry 2e, Appendix J, 25 °C. Only the independent pure
// AgX phases are modeled; no complexes, solid solutions or activity corrections.
const HALIDES: &[(&str, &str, f64)] = &[
    ("Cl^-", "AgCl", 1.6e-10),
    ("Br^-", "AgBr", 5.0e-13),
    ("I^-", "AgI", 1.5e-16),
];
const RATIOS: &[f64] = &[0.01, 0.25, 0.99, 1.0, 1.01, 4.0, 100.0];

fn mixture(a: usize, b: usize, concentration_a: f64, onset_ratio: f64) -> ([f64; 2], [f64; 2]) {
    let onset_a = HALIDES[a].2 / concentration_a;
    let onset_b = onset_a * onset_ratio;
    (
        [concentration_a, HALIDES[b].2 / onset_b],
        [onset_a, onset_b],
    )
}

fn scientific(value: f64) -> String {
    let text = format!("{value:.3e}");
    let (mantissa, exponent) = text.split_once('e').unwrap();
    format!(
        "{}e{exponent}",
        mantissa.trim_end_matches('0').trim_end_matches('.')
    )
}

pub fn generate(rng: &mut StdRng) -> String {
    let (a, b) = [(0, 1), (0, 2), (1, 2)][rng.random_range(0..3)];
    let concentration = [1e-4, 5e-4, 1e-3][rng.random_range(0..3)];
    let ratio = RATIOS[rng.random_range(0..RATIOS.len())];
    let (concentrations, onsets) = mixture(a, b, concentration, ratio);
    let mut out = String::from(
        "Silver-halide precipitation, 25 °C\nIdeal equilibria, negligible dilution\n\n",
    );
    for (index, concentration, onset) in [
        (a, concentrations[0], onsets[0]),
        (b, concentrations[1], onsets[1]),
    ] {
        let (ion, solid, ksp) = HALIDES[index];
        out.push_str(&format!(
            "[{ion}] initially {} M\nKsp({solid}) = {}\n{solid} onset at free [Ag^+] = {} M\n\n",
            scientific(concentration),
            scientific(ksp),
            scientific(onset)
        ));
    }
    if ratio == 1.0 {
        out.push_str(&format!(
            "{} and {} reach saturation together.",
            HALIDES[a].1, HALIDES[b].1
        ));
    } else {
        let (first, second, remaining) = if ratio > 1.0 {
            (a, b, 100.0 / ratio)
        } else {
            (b, a, 100.0 * ratio)
        };
        out.push_str(&format!(
            "{} precipitates first.\nAt {} onset, {remaining:.1}% of initial {} remains dissolved.",
            HALIDES[first].1, HALIDES[second].1, HALIDES[first].0
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saturation_order_and_mass_conservation() {
        for (a, b) in [(0, 1), (0, 2), (1, 2)] {
            for concentration in [1e-4, 5e-4, 1e-3] {
                for ratio in RATIOS {
                    let (initial, onsets) = mixture(a, b, concentration, *ratio);
                    for j in 0..2 {
                        let ksp = HALIDES[[a, b][j]].2;
                        assert!((onsets[j] * initial[j] / ksp - 1.0).abs() < 1e-12);
                    }
                    if *ratio == 1.0 {
                        assert_eq!(onsets[0], onsets[1]);
                        continue;
                    }
                    let (first, second) = if *ratio > 1.0 { (0, 1) } else { (1, 0) };
                    let free_ag = onsets[second];
                    let dissolved = HALIDES[[a, b][first]].2 / free_ag;
                    let solid = initial[first] - dissolved;
                    assert!(solid > 0.0 && dissolved > 0.0);
                    assert!((solid + dissolved - initial[first]).abs() < 1e-15);
                    // Charge-neutral AgX removes exactly one of each ion.
                    let total_ag = solid + free_ag;
                    assert!((total_ag - free_ag - solid).abs() < 1e-15);
                    let fraction = dissolved / initial[first];
                    let expected = if *ratio > 1.0 { 1.0 / ratio } else { *ratio };
                    assert!((fraction - expected).abs() < 1e-12);
                    assert!(onsets[first] < onsets[second]);
                    // Independently recover free silver from its total mass
                    // balance with both possible precipitates present.
                    let (mut low, mut high) = (0.0, total_ag);
                    for _ in 0..100 {
                        let silver = (low + high) / 2.0;
                        let precipitated: f64 = [a, b]
                            .iter()
                            .enumerate()
                            .map(|(j, index)| (initial[j] - HALIDES[*index].2 / silver).max(0.0))
                            .sum();
                        if silver + precipitated > total_ag {
                            high = silver;
                        } else {
                            low = silver;
                        }
                    }
                    assert!(((low + high) / 2.0 / free_ag - 1.0).abs() < 1e-8);
                }
            }
        }
    }
}
