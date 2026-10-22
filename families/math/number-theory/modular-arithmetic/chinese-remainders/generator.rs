use rand::{Rng, rngs::StdRng, seq::SliceRandom};

pub fn generate(rng: &mut StdRng) -> String {
    let (moduli, residues) = sample(rng);
    let mut lines = moduli
        .iter()
        .zip(&residues)
        .map(|(m, r)| format!("x = {r} (mod {m})"))
        .collect::<Vec<_>>();
    match solve(&moduli, &residues) {
        Ok((x, period)) => lines.push(format!("x = {x} + {period}k, k in Z")),
        Err((i, j, divisor)) => lines.push(format!(
            "No solution: {} and {} differ modulo gcd({}, {}) = {divisor}",
            residues[i], residues[j], moduli[i], moduli[j],
        )),
    }
    lines.join("\n")
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

type Solution = Result<(i64, i64), (usize, usize, i64)>;

fn solve(moduli: &[i64], residues: &[i64]) -> Solution {
    for i in 0..moduli.len() {
        for j in i + 1..moduli.len() {
            let divisor = gcd(moduli[i], moduli[j]);
            if (residues[i] - residues[j]) % divisor != 0 {
                return Err((i, j, divisor));
            }
        }
    }
    let (mut x, mut period) = (0, 1);
    for (&m, &r) in moduli.iter().zip(residues) {
        let divisor = gcd(period, m);
        let reduced = m / divisor;
        if reduced > 1 {
            let step = ((r - x) / divisor * inverse(period / divisor, reduced)).rem_euclid(reduced);
            x += period * step;
        }
        period *= reduced;
        x = x.rem_euclid(period);
    }
    Ok((x, period))
}

fn sample(rng: &mut StdRng) -> (Vec<i64>, Vec<i64>) {
    let count = rng.random_range(2..=4);
    let mut moduli: Vec<i64> = (2..=32).collect();
    moduli.shuffle(rng);
    moduli.truncate(count);
    let hidden = rng.random_range(0..=10_000);
    let mut residues: Vec<_> = moduli.iter().map(|m| hidden % m).collect();
    if rng.random_ratio(1, 3) {
        let slot = rng.random_range(0..count);
        residues[slot] = rng.random_range(0..moduli[slot]);
    }
    (moduli, residues)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn all_small_systems_agree_with_independent_enumeration() {
        for a in 2..=12 {
            for b in 2..=12 {
                let period = a * b / gcd(a, b);
                for r in 0..a {
                    for s in 0..b {
                        let actual = (0..period).find(|x| x % a == r && x % b == s);
                        assert_eq!(solve(&[a, b], &[r, s]).ok(), actual.map(|x| (x, period)));
                    }
                }
            }
        }
        assert_eq!(solve(&[6, 8, 9], &[4, 4, 7]), Ok((52, 72)));
    }

    #[test]
    fn samples_have_exact_periods_and_incompatibility_witnesses() {
        let mut rng = StdRng::seed_from_u64(615);
        let (mut shared, mut inconsistent) = (false, false);
        for _ in 0..2_000 {
            let (moduli, residues) = sample(&mut rng);
            let period = moduli.iter().fold(1, |p, &m| p / gcd(p, m) * m);
            shared |= period < moduli.iter().product::<i64>();
            match solve(&moduli, &residues) {
                Ok((x, m)) => {
                    assert_eq!(m, period);
                    assert!((0..m).contains(&x));
                    assert!(moduli.iter().zip(&residues).all(|(&m, &r)| x % m == r));
                }
                Err((i, j, d)) => {
                    inconsistent = true;
                    assert_eq!(d, gcd(moduli[i], moduli[j]));
                    assert_ne!((residues[i] - residues[j]) % d, 0);
                }
            }
        }
        assert!(shared && inconsistent);
    }
}
