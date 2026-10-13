use rand::{Rng, rngs::StdRng};
use std::collections::BTreeMap;

// Named, real molecules. Formulas are molecular composition, not structure.
const COMPOUNDS: &[(&str, &str)] = &[
    ("water", "H2O"),
    ("ammonia", "NH3"),
    ("carbon monoxide", "CO"),
    ("nitric oxide", "NO"),
    ("hydrogen sulfide", "H2S"),
    ("carbon dioxide", "CO2"),
    ("sulfur dioxide", "SO2"),
    ("carbon disulfide", "CS2"),
    ("methane", "CH4"),
    ("ethane", "C2H6"),
    ("propane", "C3H8"),
    ("butane", "C4H10"),
    ("2-methylpropane", "C4H10"),
    ("pentane", "C5H12"),
    ("2-methylbutane", "C5H12"),
    ("2,2-dimethylpropane", "C5H12"),
    ("hexane", "C6H14"),
    ("heptane", "C7H16"),
    ("octane", "C8H18"),
    ("ethene", "C2H4"),
    ("propene", "C3H6"),
    ("1-butene", "C4H8"),
    ("(Z)-2-butene", "C4H8"),
    ("(E)-2-butene", "C4H8"),
    ("ethyne", "C2H2"),
    ("1-propyne", "C3H4"),
    ("1-butyne", "C4H6"),
    ("cyclopropane", "C3H6"),
    ("cyclobutane", "C4H8"),
    ("cyclopentane", "C5H10"),
    ("cyclohexane", "C6H12"),
    ("benzene", "C6H6"),
    ("toluene", "C7H8"),
    ("ethylbenzene", "C8H10"),
    ("naphthalene", "C10H8"),
    ("methanol", "CH4O"),
    ("ethanol", "C2H6O"),
    ("1-propanol", "C3H8O"),
    ("2-propanol", "C3H8O"),
    ("1,2-ethanediol", "C2H6O2"),
    ("1-butanol", "C4H10O"),
    ("phenol", "C6H6O"),
    ("propanone", "C3H6O"),
    ("butanone", "C4H8O"),
    ("dimethyl ether", "C2H6O"),
    ("diethyl ether", "C4H10O"),
    ("methyl methanoate", "C2H4O2"),
    ("ethyl ethanoate", "C4H8O2"),
    ("methanal", "CH2O"),
    ("ethanal", "C2H4O"),
    ("methanoic acid", "CH2O2"),
    ("ethanoic acid", "C2H4O2"),
    ("benzoic acid", "C7H6O2"),
    ("methanamine", "CH5N"),
    ("ethylamine", "C2H7N"),
    ("urea", "CH4N2O"),
];

pub fn generate(rng: &mut StdRng) -> String {
    let (name, formula) = COMPOUNDS[rng.random_range(0..COMPOUNDS.len())];
    let atoms = composition(formula);
    let divisor = atoms.values().copied().reduce(gcd).unwrap();
    let empirical = atoms
        .iter()
        .map(|(element, count)| {
            let count = count / divisor;
            format!(
                "{element}{}",
                if count == 1 {
                    String::new()
                } else {
                    count.to_string()
                }
            )
        })
        .collect::<String>();
    let inventory = atoms
        .iter()
        .map(|(element, count)| format!("{element}: {count}"))
        .collect::<Vec<_>>()
        .join(", ");
    let isomers: Vec<_> = COMPOUNDS
        .iter()
        .filter(|(other, f)| *f == formula && *other != name)
        .map(|(other, _)| *other)
        .collect();
    if !isomers.is_empty() && rng.random_ratio(1, 3) {
        let other = isomers[rng.random_range(0..isomers.len())];
        format!(
            "{name} and {other}\nShared molecular formula: {formula}. Empirical formula: {empirical}.\nThey are different compounds; molecular composition alone does not specify structure."
        )
    } else {
        let molecules = rng.random_range(2..=9);
        let total: u32 = atoms.values().sum();
        format!(
            "{name}\nMolecular formula: {formula}. Empirical formula: {empirical}.\nAtoms per molecule: {inventory} ({total} total).\n{molecules} molecules contain {} atoms in total.",
            molecules * total
        )
    }
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn composition(formula: &str) -> BTreeMap<String, u32> {
    let bytes = formula.as_bytes();
    let mut atoms = BTreeMap::new();
    let mut i = 0;
    while i < bytes.len() {
        let start = i;
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_lowercase() {
            i += 1;
        }
        let element = &formula[start..i];
        let start = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        let count = if start == i {
            1
        } else {
            formula[start..i].parse().unwrap()
        };
        *atoms.entry(element.to_owned()).or_insert(0) += count;
    }
    atoms
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::BTreeSet;

    #[test]
    fn catalog_has_unique_names_and_valid_small_molecular_compositions() {
        let mut names = BTreeSet::new();
        assert!(COMPOUNDS.len() >= 40);
        for (name, formula) in COMPOUNDS {
            assert!(names.insert(name));
            let atoms = composition(formula);
            assert!(
                atoms
                    .keys()
                    .all(|e| ["C", "H", "N", "O", "S"].contains(&e.as_str()))
            );
            assert!(atoms.values().all(|n| *n > 0 && *n <= 18));
            let divisor = atoms.values().copied().reduce(gcd).unwrap();
            assert_eq!(atoms.values().map(|n| n / divisor).reduce(gcd), Some(1));
            if atoms.contains_key("C") && !atoms.contains_key("S") {
                // For these closed-shell C/H/N/O molecules, double-bond
                // equivalents are a nonnegative integer.
                let c = i64::from(atoms["C"]);
                let h = i64::from(*atoms.get("H").unwrap_or(&0));
                let n = i64::from(*atoms.get("N").unwrap_or(&0));
                let twice_dbe = 2 * c + 2 + n - h;
                assert!(twice_dbe >= 0 && twice_dbe % 2 == 0, "{name}");
            }
        }
    }

    #[test]
    fn independent_composition_examples_and_seeded_outputs() {
        assert_eq!(composition("C2H6O2").values().sum::<u32>(), 10);
        assert_eq!(composition("NH3")["H"], 3);
        assert_eq!(composition("CH4N2O")["N"], 2);
        let mut a = StdRng::seed_from_u64(918);
        let mut b = StdRng::seed_from_u64(918);
        let mut isomers = false;
        for _ in 0..200 {
            let output = generate(&mut a);
            assert_eq!(output, generate(&mut b));
            assert!(!output.ends_with('\n'));
            isomers |= output.contains("different compounds");
        }
        assert!(isomers);
    }
}
