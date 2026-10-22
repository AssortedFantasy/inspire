use rand::{Rng, rngs::StdRng, seq::SliceRandom};

pub fn generate(rng: &mut StdRng) -> String {
    let distribution = sample(rng);
    let (total, mean_numerator, second_numerator) = moments(&distribution);
    let masses = distribution
        .iter()
        .map(|(_, w)| fraction(*w, total))
        .collect::<Vec<_>>()
        .join(", ");
    let support = distribution
        .iter()
        .map(|(x, _)| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let variance = total * second_numerator - mean_numerator * mean_numerator;
    let positive: i64 = distribution
        .iter()
        .filter(|(x, _)| *x > 0)
        .map(|(_, w)| w)
        .sum();
    let max_weight = distribution.iter().map(|(_, w)| w).max().unwrap();
    let modes = distribution
        .iter()
        .filter(|(_, w)| w == max_weight)
        .map(|(x, _)| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let summary = match rng.random_range(0..3) {
        0 => format!(
            "E[X] = {}\nVar(X) = {}",
            fraction(mean_numerator, total),
            fraction(variance, total * total)
        ),
        1 => format!("P(X > 0) = {}\nModes: {modes}", fraction(positive, total)),
        _ => {
            let threshold = distribution[rng.random_range(1..distribution.len() - 1)].0;
            let weight: i64 = distribution
                .iter()
                .filter(|(x, _)| *x <= threshold)
                .map(|(_, w)| w)
                .sum();
            format!("P(X <= {threshold}) = {}", fraction(weight, total))
        }
    };
    format!("x      [{support}]\nP(X=x) [{masses}]\n{summary}")
}

fn sample(rng: &mut StdRng) -> Vec<(i64, i64)> {
    let n = rng.random_range(3..=7);
    let mut support: Vec<i64> = (-9..=12).collect();
    support.shuffle(rng);
    support.truncate(n);
    support.sort_unstable();
    support
        .into_iter()
        .map(|x| (x, rng.random_range(1..=12)))
        .collect()
}

fn moments(distribution: &[(i64, i64)]) -> (i64, i64, i64) {
    distribution.iter().fold((0, 0, 0), |(w, m, s), (x, mass)| {
        (w + mass, m + x * mass, s + x * x * mass)
    })
}

fn fraction(n: i64, d: i64) -> String {
    let (mut a, mut b) = (n.abs(), d);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    if d / a == 1 {
        (n / a).to_string()
    } else {
        format!("{}/{}", n / a, d / a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn moments_match_pairwise_variance_identity_and_real_support_bounds() {
        let mut rng = StdRng::seed_from_u64(778);
        for _ in 0..2_000 {
            let distribution = sample(&mut rng);
            let (w, m, s) = moments(&distribution);
            assert!(distribution.windows(2).all(|pair| pair[0].0 < pair[1].0));
            let pairwise: i64 = distribution
                .iter()
                .enumerate()
                .flat_map(|(i, (x, a))| {
                    distribution[i + 1..]
                        .iter()
                        .map(move |(y, b)| a * b * (x - y).pow(2))
                })
                .sum();
            assert_eq!(w * s - m * m, pairwise);
            assert!(pairwise > 0);
            assert!(distribution.first().unwrap().0 * w <= m);
            assert!(m <= distribution.last().unwrap().0 * w);
        }
        assert_eq!(moments(&[(-1, 1), (0, 2), (1, 1)]), (4, 0, 2));
        assert_eq!(fraction(-4, 12), "-1/3");
        assert_eq!(fraction(0, 12), "0");
    }
}
