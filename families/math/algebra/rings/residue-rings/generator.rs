use rand::{Rng, rngs::StdRng};

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn radical(mut n: u32) -> u32 {
    let mut result = 1;
    let factor_bound = n;
    for p in 2..=factor_bound {
        if n.is_multiple_of(p) {
            result *= p;
            while n.is_multiple_of(p) {
                n /= p;
            }
        }
        if n == 1 {
            break;
        }
    }
    result
}

fn divisors(n: u32) -> Vec<u32> {
    (2..n).filter(|d| n.is_multiple_of(*d)).collect()
}

fn pick(rng: &mut StdRng, values: &[u32]) -> u32 {
    values[rng.random_range(0..values.len())]
}

fn inverse(n: u32, a: u32) -> Option<u32> {
    (1..n).find(|b| a * b % n == 1)
}

fn nilpotence_index(n: u32, x: u32) -> Option<u32> {
    let mut power = 1;
    for k in 1..=n {
        power = power * x % n;
        if power == 0 {
            return Some(k);
        }
    }
    None
}

pub fn generate(rng: &mut StdRng) -> String {
    let composites: Vec<_> = (4..=120).filter(|&n| !divisors(n).is_empty()).collect();
    let n = pick(rng, &composites);
    describe(rng, n)
}

fn prime_powers(mut n: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let upper = n;
    for p in 2..=upper {
        if !n.is_multiple_of(p) {
            continue;
        }
        let mut power = 1;
        while n.is_multiple_of(p) {
            n /= p;
            power *= p;
        }
        factors.push(power);
        if n == 1 {
            break;
        }
    }
    factors
}

fn idempotents(n: u32) -> Vec<u32> {
    (0..n).filter(|&x| x * x % n == x).collect()
}

fn set(values: &[u32]) -> String {
    if values.len() > 12 {
        return format!(
            "{{0, {}, {}, ..., {}}}",
            values[1],
            values[2],
            values.last().unwrap()
        );
    }
    format!(
        "{{{}}}",
        values
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn describe(rng: &mut StdRng, n: u32) -> String {
    let context = format!("R = Z/{n}Z");
    match rng.random_range(0..5) {
        0 => {
            let units: Vec<_> = (2..n).filter(|&x| gcd(n, x) == 1).collect();
            let unit = pick(rng, &units);
            let inv = inverse(n, unit).unwrap();
            format!("{context}\n{unit} · {inv} = 1\n|R^×| = {}", units.len() + 1)
        }
        1 => {
            let zero_divisors: Vec<_> = (1..n).filter(|&x| gcd(n, x) != 1).collect();
            let z = pick(rng, &zero_divisors);
            let witness = n / gcd(n, z);
            let annihilator: Vec<_> = (0..n).filter(|&x| (x * z).is_multiple_of(n)).collect();
            format!(
                "{context}\n{z} · {witness} = 0\nAnn({z}) = {}",
                set(&annihilator)
            )
        }
        2 => {
            let rad = radical(n);
            let nilpotents: Vec<_> = (0..n).step_by(rad as usize).collect();
            if rad == n {
                format!(
                    "{context}\nNil(R) = {{0}}\n{{e ∈ R | e² = e}} = {}",
                    set(&idempotents(n))
                )
            } else {
                let x = rad * rng.random_range(1..n / rad);
                let k = nilpotence_index(n, x).unwrap();
                let powers = (2..=k)
                    .scan(x, |power, exponent| {
                        *power = *power * x % n;
                        Some(format!("{x}^{exponent} = {power}"))
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("{context}\nNil(R) = {}\n{powers}", set(&nilpotents))
            }
        }
        3 => {
            let d = pick(rng, &divisors(n));
            let ideal: Vec<_> = (0..n).step_by(d as usize).collect();
            format!(
                "{context}\nI = ({d}) = {}\nAnn(I) = ({})\nR/I ≅ Z/{d}Z",
                set(&ideal),
                n / d
            )
        }
        _ => {
            let factors = prime_powers(n);
            if factors.len() == 1 {
                let p = radical(n);
                let mut power = n / p;
                let mut ideals = vec!["(0)".to_owned()];
                while power > 1 {
                    ideals.push(format!("({power})"));
                    power /= p;
                }
                ideals.push("R".to_owned());
                return format!("{context}\n{}\nR/({p}) ≅ F_{p}", ideals.join(" ⊂ "));
            }
            let product = factors
                .iter()
                .map(|q| format!("Z/{q}Z"))
                .collect::<Vec<_>>()
                .join(" × ");
            let a = rng.random_range(0..n);
            let image = factors
                .iter()
                .map(|q| (a % q).to_string())
                .collect::<Vec<_>>()
                .join(", ");
            let lifts = factors
                .iter()
                .map(|&q| {
                    let other = n / q;
                    other * inverse(q, other % q).unwrap() % n
                })
                .collect::<Vec<_>>();
            format!(
                "Z/{n}Z ≅ {product}\n{a} ↦ ({image})\nOrthogonal idempotents: {}",
                lifts
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    #[test]
    fn exhaustive_ring_invariants() {
        for n in 4..=120 {
            for x in 0..n {
                assert_eq!(inverse(n, x).is_some(), gcd(n, x) == 1);
                let annihilates_nonzero = (1..n).any(|y| x * y % n == 0);
                if x != 0 {
                    assert_eq!(annihilates_nonzero, gcd(n, x) > 1);
                    if gcd(n, x) > 1 {
                        let witness = n / gcd(n, x);
                        assert!((1..n).contains(&witness));
                        assert_eq!(x * witness % n, 0);
                    }
                }
                // Repeated multiplication gives a factorization-independent oracle.
                assert_eq!(nilpotence_index(n, x).is_some(), x % radical(n) == 0);
            }
            for d in divisors(n) {
                let ideal: HashSet<_> = (0..n).map(|x| d * x % n).collect();
                assert_eq!(ideal.len(), (n / d) as usize);
                let annihilator: HashSet<_> = (0..n)
                    .filter(|&x| ideal.iter().all(|&i| x * i % n == 0))
                    .collect();
                assert_eq!(annihilator.len(), d as usize);
                assert_eq!(annihilator, (0..n).map(|x| (n / d) * x % n).collect());
                for x in 0..n {
                    assert_eq!(ideal.contains(&x), x % d == 0);
                    for y in 0..n {
                        assert_eq!((x * y % n) % d, (x % d) * (y % d) % d);
                        assert_eq!(((x + y) % n) % d, ((x % d) + (y % d)) % d);
                        assert_eq!(ideal.contains(&((x + n - y) % n)), x % d == y % d);
                    }
                }
            }
        }
    }

    #[test]
    fn crt_factors_and_orthogonal_idempotents_reconstruct_every_residue() {
        for n in 4..=120 {
            let factors = prime_powers(n);
            assert_eq!(factors.iter().product::<u32>(), n);
            let lifts: Vec<_> = factors
                .iter()
                .map(|&q| {
                    let other = n / q;
                    other * inverse(q, other % q).unwrap() % n
                })
                .collect();
            for (i, &e) in lifts.iter().enumerate() {
                assert_eq!(e * e % n, e);
                for (j, &f) in lifts.iter().enumerate() {
                    if i != j {
                        assert_eq!(e * f % n, 0);
                    }
                    assert_eq!(e % factors[j], u32::from(i == j));
                }
            }
            for x in 0..n {
                assert_eq!(
                    factors
                        .iter()
                        .zip(&lifts)
                        .map(|(&q, &e)| (x % q) * e)
                        .sum::<u32>()
                        % n,
                    x
                );
            }
            assert_eq!(idempotents(n).len(), 1 << factors.len());
        }
    }

    #[test]
    fn seeded_variety_and_samples() {
        let mut seen = HashSet::new();
        let mut moduli = HashSet::new();
        for seed in 0..512 {
            let text = generate(&mut StdRng::seed_from_u64(seed));
            assert!(!text.ends_with('\n'));
            moduli.insert(text.lines().next().unwrap().to_string());
            seen.insert(text.clone());
            if seed < 2 {
                println!("{text}\n");
            }
            assert_eq!(text, generate(&mut StdRng::seed_from_u64(seed)));
        }
        assert!(seen.len() > 400);
        assert!(moduli.len() > 70);
    }
}
