use rand::{Rng, rngs::StdRng, seq::SliceRandom};

const PRIMES: [u64; 6] = [7, 11, 17, 31, 101, 257];

struct Sharing {
    p: u64,
    coefficients: Vec<u64>,
    shares: Vec<(u64, u64)>,
    chosen: Vec<(u64, u64)>,
}

pub fn generate(rng: &mut StdRng) -> String {
    let sharing = sample(rng);
    let Sharing {
        p,
        coefficients,
        shares,
        chosen,
    } = &sharing;
    let k = coefficients.len();
    let weights = lagrange_at_zero(chosen, *p);
    let terms = weights
        .iter()
        .zip(chosen)
        .map(|(weight, (_, y))| format!("{weight}·{y}"))
        .collect::<Vec<_>>()
        .join(" + ");
    let mut lines = vec![
        format!("{k}-of-{} over GF({p})", shares.len()),
        format!("f(x) = {}", polynomial(coefficients)),
        format!("Shares (x, f(x))\n{}", points(shares)),
        format!("\nReconstruct with x = {}", coordinates(chosen)),
    ];
    lines.push(format!(
        "λ(0) = [{}]",
        weights
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    ));
    lines.push(format!("s = ({terms}) mod {p} = {}", recover(chosen, *p)));

    if rng.random_bool(0.5) {
        let omitted = rng.random_range(0..k);
        let observed = chosen
            .iter()
            .enumerate()
            .filter_map(|(index, &point)| (index != omitted).then_some(point))
            .collect::<Vec<_>>();
        let other_secret = (coefficients[0] + rng.random_range(1..*p)) % p;
        let alternative = compatible_polynomial(coefficients, &observed, other_secret, *p);
        lines.push(format!(
            "\nTwo polynomials through {}\n{}",
            if observed.len() == 1 {
                "this share"
            } else {
                "these shares"
            },
            points(&observed)
        ));
        lines.push(format!("g(x) = {}", polynomial(&alternative)));
        lines.push(format!("f(0) = {}, g(0) = {other_secret}", coefficients[0]));
    }
    lines.join("\n")
}

fn sample(rng: &mut StdRng) -> Sharing {
    let regime = rng.random_range(0..4);
    let p = if regime == 3 {
        PRIMES[rng.random_range(0..2)]
    } else {
        PRIMES[rng.random_range(0..PRIMES.len())]
    };
    let max_n = 9.min(p as usize - 1);
    let (k, n) = match regime {
        0 => (2, rng.random_range(3..=max_n)),
        1 => {
            let n = rng.random_range(2..=max_n);
            (n, n)
        }
        2 => {
            let k = rng.random_range(3..=5.min(max_n - 1));
            (k, rng.random_range(k + 1..=max_n))
        }
        _ => (rng.random_range(2..=5), p as usize - 1),
    };
    // All coefficients include zero, including the highest coefficient.
    let coefficients = (0..k).map(|_| rng.random_range(0..p)).collect::<Vec<_>>();
    let mut xs = (1..p).collect::<Vec<_>>();
    xs.shuffle(rng);
    xs.truncate(n);
    xs.sort_unstable();
    let shares = xs
        .into_iter()
        .map(|x| (x, evaluate(&coefficients, x, p)))
        .collect::<Vec<_>>();
    let mut chosen = shares.clone();
    chosen.shuffle(rng);
    chosen.truncate(k);
    chosen.sort_unstable();
    Sharing {
        p,
        coefficients,
        shares,
        chosen,
    }
}

fn evaluate(coefficients: &[u64], x: u64, p: u64) -> u64 {
    coefficients
        .iter()
        .rev()
        .fold(0, |value, coefficient| (value * x + coefficient) % p)
}

fn inverse(value: u64, p: u64) -> u64 {
    debug_assert!(value > 0 && value < p);
    let (mut base, mut exponent, mut result) = (value, p - 2, 1);
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = result * base % p;
        }
        base = base * base % p;
        exponent >>= 1;
    }
    result
}

fn lagrange_at_zero(shares: &[(u64, u64)], p: u64) -> Vec<u64> {
    shares
        .iter()
        .enumerate()
        .map(|(i, &(xi, _))| {
            let (numerator, denominator) = shares.iter().enumerate().filter(|&(j, _)| j != i).fold(
                (1, 1),
                |(numerator, denominator), (_, &(xj, _))| {
                    (
                        numerator * (p - xj) % p,
                        denominator * ((xi + p - xj) % p) % p,
                    )
                },
            );
            numerator * inverse(denominator, p) % p
        })
        .collect()
}

fn recover(shares: &[(u64, u64)], p: u64) -> u64 {
    lagrange_at_zero(shares, p)
        .iter()
        .zip(shares)
        .fold(0, |sum, (weight, (_, y))| (sum + weight * y) % p)
}

fn compatible_polynomial(
    coefficients: &[u64],
    observed: &[(u64, u64)],
    secret: u64,
    p: u64,
) -> Vec<u64> {
    assert_eq!(observed.len() + 1, coefficients.len());
    // H vanishes on the observed shares, while H(0) is invertible.
    let mut vanishing = vec![1];
    for &(x, _) in observed {
        let mut next = vec![0; vanishing.len() + 1];
        for (degree, &coefficient) in vanishing.iter().enumerate() {
            next[degree] = (next[degree] + coefficient * (p - x)) % p;
            next[degree + 1] = (next[degree + 1] + coefficient) % p;
        }
        vanishing = next;
    }
    let scale = (secret + p - coefficients[0]) * inverse(vanishing[0], p) % p;
    coefficients
        .iter()
        .zip(vanishing)
        .map(|(coefficient, change)| (coefficient + scale * change) % p)
        .collect()
}

fn polynomial(coefficients: &[u64]) -> String {
    let terms = coefficients
        .iter()
        .enumerate()
        .filter(|&(_, &coefficient)| coefficient != 0)
        .map(|(degree, &coefficient)| match degree {
            0 => coefficient.to_string(),
            1 if coefficient == 1 => "x".into(),
            _ if coefficient == 1 => format!("x^{degree}"),
            1 => format!("{coefficient}x"),
            _ => format!("{coefficient}x^{degree}"),
        })
        .collect::<Vec<_>>();
    if terms.is_empty() {
        "0".into()
    } else {
        terms.join(" + ")
    }
}

fn points(shares: &[(u64, u64)]) -> String {
    shares
        .iter()
        .map(|(x, y)| format!("({x}, {y})"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn coordinates(shares: &[(u64, u64)]) -> String {
    format!(
        "{{{}}}",
        shares
            .iter()
            .map(|(x, _)| x.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn subsets(shares: &[(u64, u64)], size: usize) -> Vec<Vec<(u64, u64)>> {
        (0u32..1 << shares.len())
            .filter(|mask| mask.count_ones() as usize == size)
            .map(|mask| {
                shares
                    .iter()
                    .enumerate()
                    .filter_map(|(index, &point)| (mask & (1 << index) != 0).then_some(point))
                    .collect()
            })
            .collect()
    }

    fn verify_all_subsets(coefficients: &[u64], shares: &[(u64, u64)], p: u64) {
        let k = coefficients.len();
        for chosen in subsets(shares, k) {
            assert_eq!(recover(&chosen, p), coefficients[0]);
            let weights = lagrange_at_zero(&chosen, p);
            assert!(weights.iter().all(|&weight| weight > 0 && weight < p));
            for degree in 0..k {
                // Lagrange weights must evaluate every monomial of degree < k at zero.
                let moment = chosen
                    .iter()
                    .zip(&weights)
                    .fold(0, |sum, ((x, _), weight)| {
                        let power = (0..degree).fold(1, |value, _| value * x % p);
                        (sum + weight * power) % p
                    });
                assert_eq!(moment, u64::from(degree == 0));
            }
        }
    }

    #[test]
    fn known_interpolation_and_degenerate_polynomials() {
        let shares = [(1, 7), (2, 13), (3, 6)];
        assert_eq!(lagrange_at_zero(&shares, 17), [3, 14, 1]);
        assert_eq!(recover(&shares, 17), 5);
        for coefficients in [vec![5, 0, 2], vec![0, 1, 0], vec![0, 0, 0]] {
            let shares = (1..=6)
                .map(|x| (x, evaluate(&coefficients, x, 17)))
                .collect::<Vec<_>>();
            verify_all_subsets(&coefficients, &shares, 17);
        }
        for p in PRIMES {
            for value in 1..p {
                assert_eq!(value * inverse(value, p) % p, 1);
            }
        }
    }

    #[test]
    fn every_threshold_subset_recovers_across_fields_and_regimes() {
        let mut rng = StdRng::seed_from_u64(0x5a_a1_12);
        let mut saw_zero_secret = false;
        let mut saw_zero_leading_coefficient = false;
        for _ in 0..192 {
            let sharing = sample(&mut rng);
            let p = sharing.p;
            let k = sharing.coefficients.len();
            assert!(2 <= k && k <= sharing.shares.len());
            assert!(sharing.shares.len() < p as usize);
            assert!(sharing.shares.iter().all(|&(x, y)| x > 0 && x < p && y < p));
            assert!(sharing.shares.windows(2).all(|pair| pair[0].0 < pair[1].0));
            assert_eq!(sharing.chosen.len(), k);
            assert!(sharing.chosen.windows(2).all(|pair| pair[0].0 < pair[1].0));
            assert_eq!(recover(&sharing.chosen, p), sharing.coefficients[0]);
            assert!(
                sharing
                    .chosen
                    .iter()
                    .all(|point| sharing.shares.contains(point))
            );
            verify_all_subsets(&sharing.coefficients, &sharing.shares, p);
            let observed = &sharing.chosen[..k - 1];
            for secret in 0..p {
                let alternative = compatible_polynomial(&sharing.coefficients, observed, secret, p);
                assert_eq!(alternative[0], secret);
                for &(x, y) in observed {
                    assert_eq!(evaluate(&alternative, x, p), y);
                }
                if secret != sharing.coefficients[0] {
                    let (x, y) = sharing.chosen[k - 1];
                    assert_ne!(evaluate(&alternative, x, p), y);
                }
            }
            saw_zero_secret |= sharing.coefficients[0] == 0;
            saw_zero_leading_coefficient |= sharing.coefficients[k - 1] == 0;
        }
        assert!(saw_zero_secret && saw_zero_leading_coefficient);
    }

    #[test]
    fn every_secret_fits_each_subthreshold_subset() {
        for p in [5, 7, 11] {
            for coefficients in [vec![0, 0, 0], vec![p - 1, 2, 0], vec![2, 0, p - 1]] {
                let shares = (1..p.min(7))
                    .map(|x| (x, evaluate(&coefficients, x, p)))
                    .collect::<Vec<_>>();
                for observed in subsets(&shares, coefficients.len() - 1) {
                    for secret in 0..p {
                        let alternative =
                            compatible_polynomial(&coefficients, &observed, secret, p);
                        assert_eq!(alternative.len(), coefficients.len());
                        assert_eq!(alternative[0], secret);
                        for &(x, y) in &observed {
                            assert_eq!(evaluate(&alternative, x, p), y);
                        }
                        if secret != coefficients[0] {
                            for &(x, y) in &shares {
                                if !observed.iter().any(|point| point.0 == x) {
                                    assert_ne!(evaluate(&alternative, x, p), y);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn exhaustive_small_field_has_one_polynomial_per_secret_for_two_shares() {
        let p = 5;
        for xs in subsets(&[(1, 0), (2, 0), (3, 0), (4, 0)], 2) {
            let mut counts = [[[0u16; 5]; 5]; 5];
            for a in 0..p {
                for b in 0..p {
                    for c in 0..p {
                        let coefficients = [a, b, c];
                        let y0 = evaluate(&coefficients, xs[0].0, p) as usize;
                        let y1 = evaluate(&coefficients, xs[1].0, p) as usize;
                        counts[y0][y1][a as usize] += 1;
                        let shares = (1..p)
                            .map(|x| (x, evaluate(&coefficients, x, p)))
                            .collect::<Vec<_>>();
                        verify_all_subsets(&coefficients, &shares, p);
                    }
                }
            }
            assert!(counts.iter().flatten().all(|secrets| *secrets == [1; 5]));
        }
    }
}
