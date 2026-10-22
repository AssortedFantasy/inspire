use rand::{Rng, rngs::StdRng};

pub fn generate(rng: &mut StdRng) -> String {
    let (terms, convergents) = sample(rng);
    let tail = terms[1..]
        .iter()
        .map(i64::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    let rows = convergents
        .iter()
        .map(|(p, q)| {
            if *q == 1 {
                p.to_string()
            } else {
                format!("{p}/{q}")
            }
        })
        .collect::<Vec<_>>()
        .join(" -> ");
    let (p, q) = convergents[convergents.len() - 1];
    format!("[{0}; {tail}] = {p}/{q}\nConvergents: {rows}", terms[0])
}

fn sample(rng: &mut StdRng) -> (Vec<i64>, Vec<(i64, i64)>) {
    let count = rng.random_range(3..=8);
    let mut terms: Vec<i64> = (0..count).map(|_| rng.random_range(1..=9)).collect();
    terms[0] = rng.random_range(0..=9);
    terms[count - 1] = rng.random_range(2..=9);
    let convergents = convergents(&terms);
    (terms, convergents)
}

fn convergents(terms: &[i64]) -> Vec<(i64, i64)> {
    let (mut pp, mut p, mut qp, mut q) = (0, 1, 1, 0);
    terms
        .iter()
        .map(|a| {
            (pp, p) = (p, a * p + pp);
            (qp, q) = (q, a * q + qp);
            (p, q)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a.abs()
    }

    #[test]
    fn known_expansions_and_independent_backward_evaluation() {
        assert_eq!(
            convergents(&[1, 2, 2, 2]),
            [(1, 1), (3, 2), (7, 5), (17, 12)]
        );
        assert_eq!(convergents(&[0, 1, 5]), [(0, 1), (1, 1), (5, 6)]);
        let mut rng = StdRng::seed_from_u64(941);
        for _ in 0..2_000 {
            let (terms, ratios) = sample(&mut rng);
            let (mut p, mut q) = (*terms.last().unwrap(), 1);
            for a in terms[..terms.len() - 1].iter().rev() {
                (p, q) = (a * p + q, p);
            }
            assert_eq!(ratios.last(), Some(&(p, q)));
            for (i, &(p, q)) in ratios.iter().enumerate() {
                assert_eq!(gcd(p, q), 1);
                assert!(q > 0);
                if i > 0 {
                    let (pp, qp) = ratios[i - 1];
                    assert_eq!(p * qp - pp * q, if i % 2 == 1 { 1 } else { -1 });
                }
            }
        }
    }
}
