use rand::{Rng, rngs::StdRng};

pub fn generate(rng: &mut StdRng) -> String {
    let (moduli, residues, result, modulus, terms) = sample(rng);
    let system = moduli
        .iter()
        .zip(&residues)
        .map(|(m, r)| format!("x = {r} (mod {m})"))
        .collect::<Vec<_>>()
        .join("; ");
    let reconstruction = terms
        .iter()
        .map(|(r, m, inverse)| format!("{r}*{m}*{inverse}"))
        .collect::<Vec<_>>()
        .join(" + ");
    format!(
        "Chinese remainder system: {system}.\nAll integer solutions: x = {result} + {modulus}*k, k in Z; {result} is the least nonnegative solution.\nThe moduli are pairwise coprime. Reconstruction: x = {reconstruction} (mod {modulus}); each factor after the residue is M/m_i and its inverse modulo m_i."
    )
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn inverse(a: i64, modulus: i64) -> i64 {
    let (mut r, mut s, mut t, mut u) = (modulus, a, 0, 1);
    while s != 0 {
        let q = r / s;
        (r, s) = (s, r - q * s);
        (t, u) = (u, t - q * u);
    }
    assert_eq!(r, 1);
    t.rem_euclid(modulus)
}

type Example = (Vec<i64>, Vec<i64>, i64, i64, Vec<(i64, i64, i64)>);

fn sample(rng: &mut StdRng) -> Example {
    let count = rng.random_range(2..=4);
    let mut moduli = Vec::new();
    for _ in 0..count {
        let candidates: Vec<i64> = (2..=40)
            .filter(|&m| moduli.iter().all(|&n| gcd(m, n) == 1))
            .collect();
        moduli.push(candidates[rng.random_range(0..candidates.len())]);
    }
    let residues: Vec<_> = moduli.iter().map(|&m| rng.random_range(0..m)).collect();
    let modulus: i64 = moduli.iter().product();
    let terms: Vec<_> = moduli
        .iter()
        .zip(&residues)
        .map(|(&m, &r)| (r, modulus / m, inverse(modulus / m, m)))
        .collect();
    let result = terms.iter().map(|&(r, m, inv)| r * m * inv).sum::<i64>() % modulus;
    (moduli, residues, result, modulus, terms)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn inverses_and_every_generated_congruence_are_exact() {
        for m in 2..=40 {
            for a in 1..m {
                if gcd(a, m) == 1 {
                    assert_eq!((a * inverse(a, m)) % m, 1);
                }
            }
        }
        let mut rng = StdRng::seed_from_u64(615);
        for _ in 0..2_000 {
            let (moduli, residues, x, total, terms) = sample(&mut rng);
            assert!(x >= 0 && x < total);
            assert_eq!(moduli.iter().product::<i64>(), total);
            for ((m, r), (_, partial, inv)) in moduli.iter().zip(&residues).zip(&terms) {
                assert_eq!(x % m, *r);
                assert_eq!((partial * inv) % m, 1);
            }
            // For small systems, independently enumerate all solutions in one period.
            if total <= 500 {
                let solutions: Vec<_> = (0..total)
                    .filter(|n| moduli.iter().zip(&residues).all(|(m, r)| n % m == *r))
                    .collect();
                assert_eq!(solutions, [x]);
            }
        }
    }
}
