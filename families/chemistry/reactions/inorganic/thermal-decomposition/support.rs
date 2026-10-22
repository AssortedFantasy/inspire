// Copied beside each reaction generator by generate_reactions.py.
use rand::{Rng, rngs::StdRng};

pub struct Reaction {
    pub identity: &'static str,
    pub left: &'static [(u32, &'static str)],
    pub right: &'static [(u32, &'static str)],
    pub conditions: &'static str,
    pub equilibrium: bool,
}

fn side(terms: &[(u32, &str)]) -> String {
    terms
        .iter()
        .map(|(n, species)| {
            if *n == 1 {
                (*species).to_owned()
            } else {
                format!("{n} {species}")
            }
        })
        .collect::<Vec<_>>()
        .join(" + ")
}

pub fn generate(rng: &mut StdRng, reactions: &[Reaction]) -> String {
    let r = &reactions[rng.random_range(0..reactions.len())];
    let equation = format!(
        "{} {} {}",
        side(r.left),
        if r.equilibrium { "<=>" } else { "->" },
        side(r.right)
    );
    [r.identity, &equation, r.conditions]
        .into_iter()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::BTreeMap;

    fn number(bytes: &[u8], i: &mut usize) -> u32 {
        let mut n = 0;
        while *i < bytes.len() && bytes[*i].is_ascii_digit() {
            n = n * 10 + u32::from(bytes[*i] - b'0');
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
            for (e, n) in piece {
                *result.entry(e).or_insert(0) += n * multiplier;
            }
        }
        result
    }

    fn species(text: &str) -> (BTreeMap<String, u32>, i64) {
        let end = text.rfind('(').expect("Every species specifies a phase");
        assert!(["(aq)", "(s)", "(l)", "(g)"].contains(&&text[end..]));
        let (formula, charge) = match text[..end].split_once('^') {
            None => (&text[..end], 0),
            Some((f, q)) => {
                let magnitude = q[..q.len() - 1].parse::<i64>().unwrap_or(1);
                let sign = match q.as_bytes()[q.len() - 1] {
                    b'+' => 1,
                    b'-' => -1,
                    _ => panic!("charge"),
                };
                (f, sign * magnitude)
            }
        };
        let mut i = 0;
        let atoms = group(formula.as_bytes(), &mut i);
        assert_eq!(i, formula.len());
        (atoms, charge)
    }

    fn inventory(terms: &[(u32, &str)]) -> (BTreeMap<String, u32>, i64) {
        let mut atoms = BTreeMap::new();
        let mut charge = 0;
        for (coefficient, text) in terms {
            let (piece, q) = species(text);
            charge += q * i64::from(*coefficient);
            for (e, n) in piece {
                *atoms.entry(e).or_insert(0) += n * coefficient;
            }
        }
        (atoms, charge)
    }

    #[test]
    fn atom_and_charge_conservation_and_primitive_coefficients() {
        for r in super::super::REACTIONS {
            assert_eq!(inventory(r.left), inventory(r.right), "{}", side(r.left));
            let mut divisor = 0;
            for (n, _) in r.left.iter().chain(r.right) {
                assert!(*n > 0);
                let mut n = *n;
                while n != 0 {
                    (divisor, n) = (n, divisor % n);
                }
            }
            assert_eq!(divisor, 1, "{}", side(r.left));
        }
        assert_eq!(species("Ca(NO3)2(aq)").0["O"], 6);
        assert_eq!(species("Cr2O7^2-(aq)").1, -2);
    }

    #[test]
    fn deterministic_samples() {
        let mut a = StdRng::seed_from_u64(72);
        let mut b = StdRng::seed_from_u64(72);
        for _ in 0..200 {
            let output = generate(&mut a, super::super::REACTIONS);
            assert_eq!(output, generate(&mut b, super::super::REACTIONS));
            assert!(!output.ends_with('\n'));
        }
    }
}
