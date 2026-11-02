use rand::{Rng, rngs::StdRng, seq::SliceRandom};

type Distribution = Vec<(i64, i64)>;

pub fn generate(rng: &mut StdRng) -> String {
    match rng.random_range(0..3) {
        0 => {
            let p = sample(rng);
            let (w, m, _) = moments(&p);
            let (lo, hi) = medians(&p);
            let median = if lo == hi {
                format!("Median = {lo}")
            } else {
                format!("Medians = [{lo}, {hi}]")
            };
            format!(
                "{}\nE[X] = {}\n{median}\nModes = {}",
                table(&p),
                fraction(m, w),
                modes(&p)
            )
        }
        1 => {
            let p = sample(rng);
            let index = rng.random_range(0..p.len());
            let step = rng.random_range(1..=4);
            let q = spread(&p, index, step);
            let (w, m, _) = moments(&p);
            format!(
                "A\n{}\nB\n{}\nE[A] = E[B] = {}\nVar(A) = {}\nVar(B) = {}\nB is a mean-preserving spread of A",
                table(&p),
                table(&q),
                fraction(m, w),
                variance(&p),
                variance(&q)
            )
        }
        _ => {
            let center = rng.random_range(-5..=5);
            let radius = rng.random_range(1..=5);
            let n = 3 * rng.random_range(2..=6);
            let boundary = 2 * n / 3;
            let (a, b) = match rng.random_range(0..5) {
                0 => (0, 1),
                1 => (boundary - 1, boundary),
                2 => (boundary, boundary + 1),
                3 => (n - 1, n),
                _ => {
                    let a = rng.random_range(1..n - 1);
                    (a, a + 1)
                }
            };
            let p = mass_transfer(center, radius, n, a);
            let q = mass_transfer(center, radius, n, b);
            format!(
                "t = {}\n{}\nt = {}\n{}\nE[X] = {center} in both\nVar(X): {} -> {}\nModes: {} -> {}",
                fraction(a, n),
                table(&p),
                fraction(b, n),
                table(&q),
                variance(&p),
                variance(&q),
                modes(&p),
                modes(&q)
            )
        }
    }
}

fn table(p: &[(i64, i64)]) -> String {
    let w: i64 = p.iter().map(|(_, mass)| mass).sum();
    let support = p
        .iter()
        .map(|(x, _)| x.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let masses = p
        .iter()
        .map(|(_, mass)| fraction(*mass, w))
        .collect::<Vec<_>>()
        .join(", ");
    format!("x      [{support}]\nP(X=x) [{masses}]")
}

fn modes(p: &[(i64, i64)]) -> String {
    let max_weight = p.iter().map(|(_, w)| w).max().unwrap();
    format!(
        "{{{}}}",
        p.iter()
            .filter(|(_, w)| w == max_weight)
            .map(|(x, _)| x.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn medians(p: &[(i64, i64)]) -> (i64, i64) {
    let total: i64 = p.iter().map(|(_, w)| w).sum();
    let mut cumulative = 0;
    for (i, &(x, w)) in p.iter().enumerate() {
        cumulative += w;
        if 2 * cumulative >= total {
            return (
                x,
                if 2 * cumulative == total {
                    p[i + 1].0
                } else {
                    x
                },
            );
        }
    }
    unreachable!()
}

fn variance(p: &[(i64, i64)]) -> String {
    let (w, m, s) = moments(p);
    fraction(w * s - m * m, w * w)
}

fn spread(p: &[(i64, i64)], index: usize, step: i64) -> Distribution {
    let (x, w) = p[index];
    let mut q: std::collections::BTreeMap<_, _> = p.iter().map(|&(x, w)| (x, 2 * w)).collect();
    q.remove(&x);
    *q.entry(x - step).or_default() += w;
    *q.entry(x + step).or_default() += w;
    q.into_iter().collect()
}

fn mass_transfer(center: i64, radius: i64, n: i64, t: i64) -> Distribution {
    [
        (center - radius, t),
        (center, 2 * (n - t)),
        (center + radius, t),
    ]
    .into_iter()
    .filter(|&(_, w)| w > 0)
    .collect()
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
    fn spreads_preserve_mean_and_increase_every_call_payoff() {
        let mut rng = StdRng::seed_from_u64(902);
        for _ in 0..1_000 {
            let p = sample(&mut rng);
            let i = rng.random_range(0..p.len());
            let step = rng.random_range(1..=4);
            let q = spread(&p, i, step);
            let (wp, mp, sp) = moments(&p);
            let (wq, mq, sq) = moments(&q);
            assert_eq!(wq, 2 * wp);
            assert_eq!(mq, 2 * mp);
            assert_eq!(sq - 2 * sp, 2 * p[i].1 * step * step);
            for &(k, _) in p.iter().chain(&q) {
                let call =
                    |d: &[(i64, i64)]| d.iter().map(|&(x, w)| w * (x - k).max(0)).sum::<i64>();
                assert!(call(&q) >= 2 * call(&p));
            }
        }
    }

    #[test]
    fn symmetric_mass_transfer_crosses_mode_and_support_boundaries() {
        for center in -5..=5 {
            for radius in 1..=5 {
                for n in (6..=18).step_by(3) {
                    for t in 0..=n {
                        let p = mass_transfer(center, radius, n, t);
                        let (w, m, s) = moments(&p);
                        assert_eq!(w, 2 * n);
                        assert_eq!(m, w * center);
                        assert_eq!(s - w * center * center, 2 * t * radius * radius);
                        assert_eq!(
                            medians(&p),
                            if t == n {
                                (center - radius, center + radius)
                            } else {
                                (center, center)
                            }
                        );
                        let mode_count = p
                            .iter()
                            .filter(|&&(_, mass)| {
                                mass == *p.iter().map(|(_, mass)| mass).max().unwrap()
                            })
                            .count();
                        assert_eq!(
                            mode_count,
                            if 3 * t == 2 * n {
                                3
                            } else if 3 * t > 2 * n {
                                2
                            } else {
                                1
                            }
                        );
                        assert_eq!(
                            p.len(),
                            if t == 0 {
                                1
                            } else if t == n {
                                2
                            } else {
                                3
                            }
                        );
                    }
                }
            }
        }
    }

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
