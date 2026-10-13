use rand::{Rng, rngs::StdRng};

// Only named C/H/O fuels; (name, formula, C, H, O).
const FUELS: &[(&str, &str, u32, u32, u32)] = &[
    ("methane", "CH4", 1, 4, 0),
    ("ethane", "C2H6", 2, 6, 0),
    ("propane", "C3H8", 3, 8, 0),
    ("butane", "C4H10", 4, 10, 0),
    ("pentane", "C5H12", 5, 12, 0),
    ("hexane", "C6H14", 6, 14, 0),
    ("heptane", "C7H16", 7, 16, 0),
    ("octane", "C8H18", 8, 18, 0),
    ("ethene", "C2H4", 2, 4, 0),
    ("propene", "C3H6", 3, 6, 0),
    ("ethyne", "C2H2", 2, 2, 0),
    ("1-propyne", "C3H4", 3, 4, 0),
    ("cyclopropane", "C3H6", 3, 6, 0),
    ("cyclohexane", "C6H12", 6, 12, 0),
    ("benzene", "C6H6", 6, 6, 0),
    ("toluene", "C7H8", 7, 8, 0),
    ("methanol", "CH4O", 1, 4, 1),
    ("ethanol", "C2H6O", 2, 6, 1),
    ("1-propanol", "C3H8O", 3, 8, 1),
    ("1-butanol", "C4H10O", 4, 10, 1),
    ("1,2-ethanediol", "C2H6O2", 2, 6, 2),
    ("propanone", "C3H6O", 3, 6, 1),
    ("dimethyl ether", "C2H6O", 2, 6, 1),
    ("diethyl ether", "C4H10O", 4, 10, 1),
    ("methanal", "CH2O", 1, 2, 1),
    ("ethanal", "C2H4O", 2, 4, 1),
    ("methanoic acid", "CH2O2", 1, 2, 2),
    ("ethanoic acid", "C2H4O2", 2, 4, 2),
];

#[derive(Clone, Debug)]
struct Reaction {
    title: String,
    left: Vec<(u32, &'static str)>,
    right: Vec<(u32, &'static str)>,
    condition: &'static str,
    equilibrium: bool,
}

pub fn generate(rng: &mut StdRng) -> String {
    let reaction = if rng.random_ratio(2, 3) {
        combustion(rng.random_range(0..FUELS.len()))
    } else {
        named(rng.random_range(0..8))
    };
    let extent = rng.random_range(1..=8);
    let equation = format!(
        "{} {} {}",
        side(&reaction.left, 1),
        if reaction.equilibrium { "<=>" } else { "->" },
        side(&reaction.right, 1)
    );
    format!(
        "{}\n{equation}\nConditions: {}\nFor a forward reaction extent of {extent} mol, consume {}; produce {}. Amounts are stoichiometric changes, not equilibrium yields.",
        reaction.title,
        reaction.condition,
        amounts(&reaction.left, extent),
        amounts(&reaction.right, extent)
    )
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn combustion(index: usize) -> Reaction {
    let (name, formula, c, h, o) = FUELS[index];
    let coefficients = [4, 4 * c + h - 2 * o, 4 * c, 2 * h];
    let divisor = coefficients.into_iter().reduce(gcd).unwrap();
    let [fuel, oxygen, co2, water] = coefficients.map(|n| n / divisor);
    Reaction {
        title: format!("Complete combustion of {name}"),
        left: vec![(fuel, formula), (oxygen, "O2")],
        right: vec![(co2, "CO2"), (water, "H2O")],
        condition: "ideal complete combustion with sufficient oxygen and ignition; water phase depends on temperature; states omitted",
        equilibrium: false,
    }
}

fn named(index: usize) -> Reaction {
    let (title, left, right, condition, equilibrium) = match index {
        0 => (
            "Strong acid/base neutralization",
            vec![(1, "HCl"), (1, "NaOH")],
            vec![(1, "NaCl"), (1, "H2O")],
            "aqueous hydrochloric acid and sodium hydroxide",
            false,
        ),
        1 => (
            "Carbonate and acid",
            vec![(1, "CaCO3"), (2, "HCl")],
            vec![(1, "CaCl2"), (1, "H2O"), (1, "CO2")],
            "calcium carbonate with aqueous hydrochloric acid; carbon dioxide escapes",
            false,
        ),
        2 => (
            "Thermal decomposition of calcium carbonate",
            vec![(1, "CaCO3")],
            vec![(1, "CaO"), (1, "CO2")],
            "strong heating with carbon dioxide removal",
            false,
        ),
        3 => (
            "Silver chloride precipitation",
            vec![(1, "CaCl2"), (2, "AgNO3")],
            vec![(1, "Ca(NO3)2"), (2, "AgCl")],
            "aqueous reactants; AgCl precipitates as a solid",
            false,
        ),
        4 => (
            "Formation of ammonia",
            vec![(1, "N2"), (3, "H2")],
            vec![(2, "NH3")],
            "catalytic Haber equilibrium at elevated temperature and pressure; not complete conversion",
            true,
        ),
        5 => (
            "Sodium and water",
            vec![(2, "Na"), (2, "H2O")],
            vec![(2, "NaOH"), (1, "H2")],
            "sodium metal and liquid water; aqueous NaOH and hydrogen gas",
            false,
        ),
        6 => (
            "Formation of sodium chloride",
            vec![(2, "Na"), (1, "Cl2")],
            vec![(2, "NaCl")],
            "sodium metal and chlorine form solid sodium chloride",
            false,
        ),
        _ => (
            "Barium hydroxide neutralization",
            vec![(1, "Ba(OH)2"), (2, "HNO3")],
            vec![(1, "Ba(NO3)2"), (2, "H2O")],
            "aqueous barium hydroxide and nitric acid",
            false,
        ),
    };
    Reaction {
        title: title.into(),
        left,
        right,
        condition,
        equilibrium,
    }
}

fn side(terms: &[(u32, &str)], multiplier: u32) -> String {
    terms
        .iter()
        .map(|(n, f)| {
            let n = n * multiplier;
            if n == 1 {
                (*f).to_owned()
            } else {
                format!("{n} {f}")
            }
        })
        .collect::<Vec<_>>()
        .join(" + ")
}

fn amounts(terms: &[(u32, &str)], extent: u32) -> String {
    terms
        .iter()
        .map(|(n, f)| format!("{} mol {f}", n * extent))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::BTreeMap;

    // Independent parser including parenthesized ions in neutral formulas.
    fn atoms(formula: &str) -> BTreeMap<String, u32> {
        fn number(bytes: &[u8], i: &mut usize) -> u32 {
            let mut n = 0;
            while *i < bytes.len() && bytes[*i].is_ascii_digit() {
                n = 10 * n + u32::from(bytes[*i] - b'0');
                *i += 1;
            }
            if n == 0 { 1 } else { n }
        }
        fn group(bytes: &[u8], i: &mut usize) -> BTreeMap<String, u32> {
            let mut result = BTreeMap::new();
            while *i < bytes.len() && bytes[*i] != b')' {
                let piece = if bytes[*i] == b'(' {
                    *i += 1;
                    let inner = group(bytes, i);
                    assert_eq!(bytes[*i], b')');
                    *i += 1;
                    inner
                } else {
                    assert!(bytes[*i].is_ascii_uppercase());
                    let start = *i;
                    *i += 1;
                    while *i < bytes.len() && bytes[*i].is_ascii_lowercase() {
                        *i += 1;
                    }
                    BTreeMap::from([(String::from_utf8(bytes[start..*i].to_vec()).unwrap(), 1)])
                };
                let multiplier = number(bytes, i);
                for (element, count) in piece {
                    *result.entry(element).or_insert(0) += count * multiplier;
                }
            }
            result
        }
        let mut i = 0;
        let result = group(formula.as_bytes(), &mut i);
        assert_eq!(i, formula.len());
        result
    }

    fn inventory(terms: &[(u32, &str)]) -> BTreeMap<String, u32> {
        let mut result = BTreeMap::new();
        for (coefficient, formula) in terms {
            for (element, count) in atoms(formula) {
                *result.entry(element).or_insert(0) += count * coefficient;
            }
        }
        result
    }

    #[test]
    fn every_reaction_conserves_every_element_with_primitive_coefficients() {
        let reactions = (0..FUELS.len()).map(combustion).chain((0..8).map(named));
        for reaction in reactions {
            assert_eq!(
                inventory(&reaction.left),
                inventory(&reaction.right),
                "{}",
                reaction.title
            );
            assert!(
                reaction
                    .left
                    .iter()
                    .chain(&reaction.right)
                    .all(|(n, _)| *n > 0)
            );
            assert_eq!(
                reaction
                    .left
                    .iter()
                    .chain(&reaction.right)
                    .map(|(n, _)| *n)
                    .reduce(gcd),
                Some(1)
            );
        }
    }

    #[test]
    fn fuel_metadata_matches_independent_formula_parser() {
        for (_, formula, c, h, o) in FUELS {
            let parsed = atoms(formula);
            assert_eq!(parsed["C"], *c);
            assert_eq!(parsed["H"], *h);
            assert_eq!(*parsed.get("O").unwrap_or(&0), *o);
        }
        assert_eq!(atoms("Ca(NO3)2")["O"], 6);
        assert_eq!(atoms("Ba(OH)2")["H"], 2);
        assert_eq!(side(&combustion(1).left, 1), "2 C2H6 + 7 O2");
    }

    #[test]
    fn seeded_samples_keep_conditions_and_extent_distinct() {
        let mut a = StdRng::seed_from_u64(72);
        let mut b = StdRng::seed_from_u64(72);
        let mut equilibrium = false;
        for _ in 0..300 {
            let output = generate(&mut a);
            assert_eq!(output, generate(&mut b));
            assert!(output.contains("Conditions:"));
            assert!(output.contains("not equilibrium yields"));
            assert!(!output.ends_with('\n'));
            equilibrium |= output.contains("<=>");
        }
        assert!(equilibrium);
    }
}
