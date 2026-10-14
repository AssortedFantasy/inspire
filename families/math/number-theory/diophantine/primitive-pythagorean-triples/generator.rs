use rand::{Rng, rngs::StdRng};

pub fn generate(rng: &mut StdRng) -> String {
    let (m, n, a, b, c) = sample(rng);
    format!(
        "Primitive Pythagorean triple: ({a}, {b}, {c}), with {a}^2 + {b}^2 = {c}^2 = {}.\nEuclid parameters: m={m}, n={n}; m>n>0, gcd(m,n)=1, and m-n is odd. The legs are m^2-n^2 and 2*m*n, listed in increasing order.\nRight triangle: area {}; perimeter {}; inradius {}; circumradius {c}/2. Its three side lengths have gcd 1.",
        c * c,
        a * b / 2,
        a + b + c,
        (a + b - c) / 2,
    )
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn sample(rng: &mut StdRng) -> (i64, i64, i64, i64, i64) {
    // Sample uniformly from valid parameter pairs, without an unbounded retry.
    let pairs: Vec<_> = (2..=48)
        .flat_map(|m| (1..m).map(move |n| (m, n)))
        .filter(|&(m, n)| (m - n) % 2 == 1 && gcd(m, n) == 1)
        .collect();
    let (m, n) = pairs[rng.random_range(0..pairs.len())];
    let (x, y) = (m * m - n * n, 2 * m * n);
    (m, n, x.min(y), x.max(y), m * m + n * n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn all_allowed_pairs_are_primitive_and_have_consistent_geometry() {
        for m in 2..=48 {
            for n in 1..m {
                if gcd(m, n) != 1 || (m - n) % 2 != 1 {
                    continue;
                }
                let (a, b, c) = (m * m - n * n, 2 * m * n, m * m + n * n);
                assert_eq!(a * a + b * b, c * c);
                assert_eq!(gcd(gcd(a, b), c), 1);
                assert_eq!(a * b / 2, (a + b + c) * (a + b - c) / 4);
                assert!(a > 0 && b > 0 && c > a && c > b);
            }
        }
        let mut rng = StdRng::seed_from_u64(441);
        for _ in 0..200 {
            let (_, _, a, b, _) = sample(&mut rng);
            assert!(a < b);
        }
    }
}
