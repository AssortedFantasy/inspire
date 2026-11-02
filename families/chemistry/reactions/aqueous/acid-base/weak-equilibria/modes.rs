use rand::{Rng, rngs::StdRng};

const KW: f64 = 1e-14;
const K: f64 = 1.8e-5;

// HA/A- plus a strong-base counterion. The base titration is its charge-dual.
// Solve electroneutrality, mass balance and water equilibrium together; neither
// the buffer approximation nor complete neutralization is used near equivalence.
fn weak_h(total: f64, counterion: f64) -> f64 {
    let (mut low, mut high) = (-14.0_f64, 0.0_f64);
    for _ in 0..90 {
        let log_h = (low + high) / 2.0;
        let h = 10.0_f64.powf(log_h);
        let residual = h + counterion - KW / h - total * K / (K + h);
        if residual > 0.0 {
            high = log_h;
        } else {
            low = log_h;
        }
    }
    10.0_f64.powf((low + high) / 2.0)
}

fn strong_h(excess: f64) -> f64 {
    let dominant = (excess.abs() + (excess * excess + 4.0 * KW).sqrt()) / 2.0;
    if excess >= 0.0 {
        dominant
    } else {
        KW / dominant
    }
}

fn ph(initial_ml: f64, concentration: f64, added_ml: f64, weak: bool, base: bool) -> f64 {
    let total = concentration * initial_ml / (initial_ml + added_ml);
    let counterion = concentration * added_ml / (initial_ml + added_ml);
    let h = if weak {
        weak_h(total, counterion)
    } else {
        strong_h(total - counterion)
    };
    let acid_ph = -h.log10();
    if base { 14.0 - acid_ph } else { acid_ph }
}

pub fn generate(rng: &mut StdRng) -> String {
    let initial_ml = [10.0, 20.0, 25.0, 40.0][rng.random_range(0..4)];
    let concentration = [0.005, 0.01, 0.02][rng.random_range(0..3)];
    let base = rng.random_bool(0.5);
    let (weak, strong, titrant, constant) = if base {
        ("NH3", "NaOH", "HCl", "Kb")
    } else {
        ("CH3COOH", "HCl", "NaOH", "Ka")
    };
    let fractions: &[f64] = match rng.random_range(0..3) {
        0 => &[0.0, 0.5, 1.0, 1.5],
        1 => &[0.99, 1.0, 1.01],
        _ => &[0.1, 0.5, 0.9],
    };
    let mut out = format!(
        "Separate {initial_ml} mL samples of {concentration} M {weak} and {strong}\n\
         {concentration} M {titrant} titrant\n\
         Ideal aqueous equilibrium, 25 °C, {constant}({weak}) = 1.8e-5\n\n\
         Added (mL)    pH {weak}    pH {strong}"
    );
    for fraction in fractions {
        let added_ml = initial_ml * fraction;
        let weak_ph = ph(initial_ml, concentration, added_ml, true, base);
        let strong_ph = ph(initial_ml, concentration, added_ml, false, base);
        out.push_str(&format!(
            "\n{added_ml:>10.2}    {weak_ph:>8.2}    {strong_ph:>7.2}"
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn titration_mass_charge_and_water_equilibria() {
        for initial_ml in [10.0, 20.0, 25.0, 40.0] {
            for concentration in [0.005, 0.01, 0.02] {
                let mut previous = 0.0;
                for fraction in [0.0, 0.1, 0.5, 0.9, 0.99, 1.0, 1.01, 1.5] {
                    let total = concentration / (1.0 + fraction);
                    let counterion = total * fraction;
                    let h = weak_h(total, counterion);
                    // Recover both species from charge balance independently
                    // of the solver's acid-distribution expression.
                    let anion = h + counterion - KW / h;
                    let acid = total - anion;
                    assert!(anion > 0.0 && acid > 0.0);
                    assert!((h * anion / acid / K - 1.0).abs() < 1e-7);
                    let current = ph(
                        initial_ml,
                        concentration,
                        initial_ml * fraction,
                        true,
                        false,
                    );
                    assert!(current > previous);
                    previous = current;
                    let strong = strong_h(total - counterion);
                    assert!((strong - KW / strong - (total - counterion)).abs() < 1e-12);
                    assert!(
                        (ph(initial_ml, concentration, initial_ml * fraction, true, true)
                            + current
                            - 14.0)
                            .abs()
                            < 1e-12
                    );
                }
            }
        }
    }

    #[test]
    fn buffer_and_equivalence_are_different_regimes() {
        let half = ph(25.0, 0.01, 12.5, true, false);
        assert!((half + K.log10()).abs() < 0.02);
        let equivalence = ph(25.0, 0.01, 25.0, true, false);
        assert!((8.1..8.4).contains(&equivalence));
        assert_eq!(ph(25.0, 0.01, 25.0, false, false), 7.0);
        let weak_after = ph(25.0, 0.01, 37.5, true, false);
        let strong_after = ph(25.0, 0.01, 37.5, false, false);
        assert!((weak_after - strong_after).abs() < 0.001);
    }
}
