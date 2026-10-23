use rand::{Rng, rngs::StdRng, seq::SliceRandom};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
enum Method {
    Hamilton,
    Jefferson,
    Webster,
    Hill,
    Adams,
}

const METHODS: [Method; 5] = [
    Method::Hamilton,
    Method::Jefferson,
    Method::Webster,
    Method::Hill,
    Method::Adams,
];

struct Instance {
    populations: Vec<u64>,
    house: u64,
    grow: bool,
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..5);
    let instance = sample(rng, regime);
    let p = &instance.populations;
    let h = instance.house;
    let total: u64 = p.iter().sum();
    let allocations: Vec<_> = METHODS.iter().map(|&m| allocate(p, h, m)).collect();
    let next: Vec<_> = if instance.grow {
        METHODS.iter().map(|&m| allocate(p, h + 1, m)).collect()
    } else {
        Vec::new()
    };
    let population: Vec<_> = p.iter().map(u64::to_string).collect();
    let quota: Vec<_> = p.iter().map(|&x| decimal_quota(x * h, total)).collect();
    let widths: Vec<_> = population
        .iter()
        .zip(&quota)
        .map(|(a, b)| a.len().max(b.len()).max(5) + 2)
        .collect();
    let mut lines = vec![if instance.grow {
        format!("House {h} → {} seats", h + 1)
    } else {
        format!("House {h} seats")
    }];
    lines.push(row(
        "State",
        &(0..p.len()).map(label).collect::<Vec<_>>(),
        &widths,
    ));
    lines.push(row("Population", &population, &widths));
    lines.push(row(&format!("Quota at {h} ≈"), &quota, &widths));
    lines.push(String::new());
    for (index, method) in METHODS.iter().enumerate() {
        let values = allocations[index]
            .iter()
            .enumerate()
            .map(|(state, seats)| {
                if instance.grow {
                    format!("{seats}→{}", next[index][state])
                } else {
                    seats.to_string()
                }
            })
            .collect::<Vec<_>>();
        lines.push(row(method.name(), &values, &widths));
    }
    if instance.grow {
        for state in 0..p.len() {
            if allocations[0][state] > next[0][state] {
                lines.push(format!("{} loses a seat under Hamilton.", label(state)));
            }
        }
    }
    lines.push("\nMinimum one seat for Huntington–Hill and Adams.".into());
    lines.push("Exact ties go to the earlier label.".into());
    lines.join("\n")
}

fn sample(rng: &mut StdRng, regime: usize) -> Instance {
    let (mut populations, house, grow) = match regime {
        0 => {
            let n = rng.random_range(3..=6);
            let populations = (0..n).map(|_| rng.random_range(100..=9_999)).collect();
            (populations, rng.random_range(n * 2..=n * 8), false)
        }
        1 => {
            let scale = rng.random_range(1_000..=10_000);
            if rng.random_bool(0.5) {
                let k = rng.random_range(0..=6);
                let radius = scale / (100 * (k + 1));
                // At 9k+4, the smallest state has the largest remainder.
                // At 9k+5, both larger states overtake it without a tie.
                let p = [5, 3, 1].map(|x| jitter(rng, x * scale, radius)).to_vec();
                (p, 9 * k + 4, true)
            } else {
                let k = rng.random_range(0..=1);
                let radius = scale / (300 * (k + 1));
                // Both five-part states lose their remainder seats.
                let p = [15, 15, 9, 5, 5, 2]
                    .map(|x| jitter(rng, x * scale, radius))
                    .to_vec();
                (p, 51 * k + 25, true)
            }
        }
        2 => {
            let scale = rng.random_range(1_000..=10_000);
            if rng.random_bool(0.5) {
                // Four distinct divisor allocations, away from their boundaries.
                let p = vec![
                    60 * scale + rng.random_range(1..=scale / 10),
                    25 * scale + rng.random_range(1..=scale / 10),
                    15 * scale - rng.random_range(1..=scale / 10),
                ];
                (p, 10, false)
            } else {
                let n = rng.random_range(3..=6);
                let h = rng.random_range(2 * n..=3 * n);
                let radius = scale / (10 * h);
                let mut p = vec![jitter(rng, (h + 1) * scale, radius)];
                p.extend((1..n).map(|_| jitter(rng, scale, radius)));
                // Jefferson gives every seat to the largest state;
                // Webster gives each smaller state at least one.
                (p, h, false)
            }
        }
        3 => {
            let n = rng.random_range(3..=6);
            let p = vec![rng.random_range(100..=9_999); n as usize];
            let h = n * rng.random_range(1..=5) + rng.random_range(1..n);
            (p, h, false)
        }
        _ => {
            let n = rng.random_range(3..=6);
            let seats: Vec<_> = (0..n).map(|_| rng.random_range(1..=9)).collect();
            let scale = rng.random_range(100..=999);
            let h = seats.iter().sum();
            (seats.iter().map(|&s| s * scale).collect(), h, false)
        }
    };
    populations.shuffle(rng);
    Instance {
        populations,
        house,
        grow,
    }
}

fn jitter(rng: &mut StdRng, center: u64, radius: u64) -> u64 {
    rng.random_range(center - radius..=center + radius)
}

impl Method {
    fn name(self) -> &'static str {
        match self {
            Self::Hamilton => "Hamilton",
            Self::Jefferson => "Jefferson",
            Self::Webster => "Webster",
            Self::Hill => "Huntington–Hill",
            Self::Adams => "Adams",
        }
    }

    fn minimum(self) -> u64 {
        u64::from(matches!(self, Self::Hill | Self::Adams))
    }

    fn priority(self, population: u64, seats: u64) -> (u128, u128) {
        let p = u128::from(population);
        let s = u128::from(seats);
        match self {
            Self::Jefferson => (p, s + 1),
            Self::Webster => (p, 2 * s + 1),
            Self::Hill => (p * p, s * (s + 1)),
            Self::Adams => (p, s),
            Self::Hamilton => unreachable!(),
        }
    }
}

fn allocate(populations: &[u64], house: u64, method: Method) -> Vec<u64> {
    if matches!(method, Method::Hamilton) {
        let total: u64 = populations.iter().sum();
        let mut seats: Vec<_> = populations.iter().map(|&p| p * house / total).collect();
        let mut order: Vec<_> = (0..populations.len()).collect();
        order.sort_by(|&a, &b| {
            (populations[b] * house % total)
                .cmp(&(populations[a] * house % total))
                .then(a.cmp(&b))
        });
        let remaining = house - seats.iter().sum::<u64>();
        for &state in order.iter().take(remaining as usize) {
            seats[state] += 1;
        }
        seats
    } else {
        let mut seats = vec![method.minimum(); populations.len()];
        for _ in seats.iter().sum::<u64>()..house {
            let mut winner = 0;
            for state in 1..populations.len() {
                if compare(
                    method.priority(populations[state], seats[state]),
                    method.priority(populations[winner], seats[winner]),
                ) == Ordering::Greater
                {
                    winner = state;
                }
            }
            seats[winner] += 1;
        }
        seats
    }
}

fn compare((a, b): (u128, u128), (c, d): (u128, u128)) -> Ordering {
    (a * d).cmp(&(c * b))
}

fn decimal_quota(numerator: u64, denominator: u64) -> String {
    let rounded = (1_000 * numerator + denominator / 2) / denominator;
    let text = format!("{}.{:03}", rounded / 1_000, rounded % 1_000);
    text.trim_end_matches('0').trim_end_matches('.').into()
}

fn label(state: usize) -> String {
    char::from(b'A' + state as u8).to_string()
}

fn row(name: &str, values: &[String], widths: &[usize]) -> String {
    let mut result = format!("{name:<17}");
    for (value, &width) in values.iter().zip(widths) {
        result.push_str(&format!("{value:>width$}"));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn check_quota(p: &[u64], h: u64, seats: &[u64]) {
        let total: u64 = p.iter().sum();
        for (i, &s) in seats.iter().enumerate() {
            assert!(s == p[i] * h / total || s == (p[i] * h).div_ceil(total));
            if s > p[i] * h / total {
                for j in 0..p.len() {
                    if seats[j] == p[j] * h / total {
                        let ri = p[i] * h % total;
                        let rj = p[j] * h % total;
                        assert!(ri > rj || ri == rj && i < j);
                    }
                }
            }
        }
    }

    // Independent rounding characterization: every state's interval of valid
    // divisors must intersect. Hill uses squared divisors, avoiding square roots.
    fn check_divisor(p: &[u64], seats: &[u64], method: Method) {
        let mut lower = (0_u128, 1_u128);
        let mut upper = None;
        for (&population, &seats) in p.iter().zip(seats) {
            let p = u128::from(population);
            let s = u128::from(seats);
            let (lo, hi) = match method {
                Method::Jefferson => ((p, s + 1), (s > 0).then_some((p, s))),
                Method::Webster => ((2 * p, 2 * s + 1), (s > 0).then(|| (2 * p, 2 * s - 1))),
                Method::Hill => ((p * p, s * (s + 1)), (s > 1).then(|| (p * p, s * (s - 1)))),
                Method::Adams => ((p, s), (s > 1).then(|| (p, s - 1))),
                Method::Hamilton => unreachable!(),
            };
            if lo.0 * lower.1 > lower.0 * lo.1 {
                lower = lo;
            }
            if let Some(hi) = hi
                && upper.is_none_or(|(a, b)| hi.0 * b < a * hi.1)
            {
                upper = Some(hi);
            }
        }
        let (a, b) = upper.unwrap();
        assert!(lower.0 * b <= a * lower.1, "{method:?}: {p:?} {seats:?}");
    }

    #[test]
    fn conservation_quota_rounding_and_house_monotonicity() {
        let mut rng = StdRng::seed_from_u64(8_149);
        for regime in 0..5 {
            for _ in 0..200 {
                let instance = sample(&mut rng, regime);
                for method in METHODS {
                    let p = &instance.populations;
                    let h = instance.house;
                    let seats = allocate(p, h, method);
                    assert_eq!(seats.iter().sum::<u64>(), h);
                    if matches!(method, Method::Hamilton) {
                        check_quota(p, h, &seats);
                    } else {
                        assert!(seats.iter().all(|&s| s >= method.minimum()));
                        check_divisor(p, &seats, method);
                        let next = allocate(p, h + 1, method);
                        assert!(seats.iter().zip(next).all(|(&a, b)| a <= b));
                    }
                }
            }
        }
    }

    #[test]
    fn constructed_regimes_have_their_promised_seat_changes() {
        let mut rng = StdRng::seed_from_u64(19_027);
        for _ in 0..1_000 {
            let instance = sample(&mut rng, 1);
            let a = allocate(&instance.populations, instance.house, Method::Hamilton);
            let b = allocate(&instance.populations, instance.house + 1, Method::Hamilton);
            let losses = a.iter().zip(&b).filter(|(a, b)| a > b).count();
            assert_eq!(losses, if a.len() == 3 { 1 } else { 2 });

            let instance = sample(&mut rng, 2);
            let p = &instance.populations;
            let allocations: Vec<_> = METHODS
                .iter()
                .skip(1)
                .map(|&m| allocate(p, instance.house, m))
                .collect();
            if p.len() == 3 && instance.house == 10 {
                for i in 0..allocations.len() {
                    for j in i + 1..allocations.len() {
                        assert_ne!(allocations[i], allocations[j]);
                    }
                }
            } else {
                assert_eq!(allocations[0].iter().filter(|&&s| s > 0).count(), 1);
                assert!(allocations[1].iter().all(|&s| s > 0));
            }
        }
    }

    #[test]
    fn alabama_parameter_corners_preserve_strict_seat_losses() {
        // With floors fixed, every quota and remainder-order boundary becomes
        // a linear population inequality, so the perturbation-box corners
        // also bound all the interior populations used by these constructions.
        for scale in [1_000, 10_000] {
            for k in 0..=6 {
                let radius = scale / (100 * (k + 1));
                for mask in 0..8 {
                    let p: Vec<_> = [5, 3, 1]
                        .iter()
                        .enumerate()
                        .map(|(i, &x)| {
                            x * scale + if mask & (1 << i) == 0 { 0 } else { 2 * radius } - radius
                        })
                        .collect();
                    assert_eq!(
                        allocate(&p, 9 * k + 4, Method::Hamilton),
                        [5 * k + 2, 3 * k + 1, k + 1]
                    );
                    assert_eq!(
                        allocate(&p, 9 * k + 5, Method::Hamilton),
                        [5 * k + 3, 3 * k + 2, k]
                    );
                }
            }
            for k in 0..=1 {
                let radius = scale / (300 * (k + 1));
                let weights = [15, 15, 9, 5, 5, 2];
                for mask in 0..64 {
                    let p: Vec<_> = weights
                        .iter()
                        .enumerate()
                        .map(|(i, &x)| {
                            x * scale + if mask & (1 << i) == 0 { 0 } else { 2 * radius } - radius
                        })
                        .collect();
                    let before: Vec<_> = weights
                        .iter()
                        .zip([7, 7, 4, 3, 3, 1])
                        .map(|(&w, s)| w * k + s)
                        .collect();
                    let after: Vec<_> = weights
                        .iter()
                        .zip([8, 8, 5, 2, 2, 1])
                        .map(|(&w, s)| w * k + s)
                        .collect();
                    assert_eq!(allocate(&p, 51 * k + 25, Method::Hamilton), before);
                    assert_eq!(allocate(&p, 51 * k + 26, Method::Hamilton), after);
                }
            }
        }
    }

    #[test]
    fn exact_quotas_and_equal_population_ties() {
        let mut rng = StdRng::seed_from_u64(520);
        for _ in 0..200 {
            let instance = sample(&mut rng, 4);
            let p = &instance.populations;
            let total: u64 = p.iter().sum();
            let expected: Vec<_> = p.iter().map(|&p| p * instance.house / total).collect();
            for method in METHODS {
                assert_eq!(allocate(p, instance.house, method), expected);
            }
            let instance = sample(&mut rng, 3);
            let n = instance.populations.len() as u64;
            let expected: Vec<_> = (0..n)
                .map(|i| instance.house / n + u64::from(i < instance.house % n))
                .collect();
            for method in METHODS {
                assert_eq!(
                    allocate(&instance.populations, instance.house, method),
                    expected
                );
            }
        }
    }

    #[test]
    fn census_example_and_exact_hill_priority_tie() {
        let p = [2_560, 3_315, 995, 5_012];
        assert_eq!(allocate(&p, 20, Method::Hamilton), [4, 6, 2, 8]);
        assert_eq!(allocate(&p, 20, Method::Jefferson), [4, 6, 1, 9]);
        assert_eq!(allocate(&p, 20, Method::Webster), [4, 6, 2, 8]);
        assert_eq!(allocate(&p, 20, Method::Hill), [4, 6, 2, 8]);
        // 6/sqrt(8*9) = 1/sqrt(1*2), an exact irrational priority tie.
        assert_eq!(
            compare(Method::Hill.priority(6, 8), Method::Hill.priority(1, 1)),
            Ordering::Equal
        );
        assert_eq!(allocate(&[6, 1], 10, Method::Hill), [9, 1]);
        assert_eq!(allocate(&[1, 6], 10, Method::Hill), [2, 8]);
    }
}
