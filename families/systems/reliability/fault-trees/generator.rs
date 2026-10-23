use rand::{Rng, rngs::StdRng, seq::SliceRandom};

#[derive(Clone, Debug)]
enum Event {
    Basic(usize),
    And(Vec<Event>),
    Or(Vec<Event>),
}

impl Event {
    fn occurs(&self, failed: u16) -> bool {
        match self {
            Self::Basic(i) => failed & (1 << i) != 0,
            Self::And(children) => children.iter().all(|child| child.occurs(failed)),
            Self::Or(children) => children.iter().any(|child| child.occurs(failed)),
        }
    }

    fn cut_sets(&self) -> Vec<u16> {
        match self {
            Self::Basic(i) => vec![1 << i],
            Self::Or(children) => minimal(children.iter().flat_map(Self::cut_sets).collect()),
            Self::And(children) => children.iter().fold(vec![0], |cuts, child| {
                let other = child.cut_sets();
                minimal(
                    cuts.iter()
                        .flat_map(|left| other.iter().map(move |right| left | right))
                        .collect(),
                )
            }),
        }
    }

    fn expression(&self) -> String {
        match self {
            Self::Basic(i) => label(*i).to_string(),
            Self::And(children) | Self::Or(children) => {
                let gate = if matches!(self, Self::And(_)) {
                    "AND"
                } else {
                    "OR"
                };
                let inputs = children
                    .iter()
                    .map(Self::expression)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{gate}({inputs})")
            }
        }
    }
}

fn minimal(mut candidates: Vec<u16>) -> Vec<u16> {
    candidates.sort_unstable_by_key(|mask| (mask.count_ones(), *mask));
    let mut cuts = Vec::new();
    for candidate in candidates {
        if !cuts.iter().any(|&cut| cut & !candidate == 0) {
            cuts.push(candidate);
        }
    }
    cuts
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Fraction {
    numerator: u128,
    denominator: u128,
}

impl Fraction {
    fn new(numerator: u128, denominator: u128) -> Self {
        let mut a = numerator;
        let mut b = denominator;
        while b != 0 {
            (a, b) = (b, a % b);
        }
        Self {
            numerator: numerator / a,
            denominator: denominator / a,
        }
    }

    fn display(self) -> String {
        let value = self.numerator as f64 / self.denominator as f64;
        let approximation = if value < 0.0001 {
            let scientific = format!("{value:.5e}");
            let (mantissa, exponent) = scientific.split_once('e').unwrap();
            format!("{}e{exponent}", trim_decimal(mantissa))
        } else {
            let digits = (5.0 - value.log10().floor()).max(0.0) as usize;
            trim_decimal(&format!("{value:.digits$}")).to_owned()
        };
        format!("{}/{} ≈ {approximation}", self.numerator, self.denominator)
    }
}

fn probability(tree: &Event, probabilities: &[u8]) -> Fraction {
    let denominator = 100_u128.pow(probabilities.len() as u32);
    let numerator = (0..1_u16 << probabilities.len())
        .filter(|&failed| tree.occurs(failed))
        .map(|failed| {
            probabilities
                .iter()
                .enumerate()
                .map(|(i, &p)| u128::from(if failed & (1 << i) != 0 { p } else { 100 - p }))
                .product::<u128>()
        })
        .sum();
    Fraction::new(numerator, denominator)
}

fn label(i: usize) -> char {
    char::from(b'A' + i as u8)
}

fn trim_decimal(text: &str) -> &str {
    if text.contains('.') {
        text.trim_end_matches('0').trim_end_matches('.')
    } else {
        text
    }
}

fn random_tree(rng: &mut StdRng, labels: &[usize]) -> Event {
    if labels.len() == 1 {
        return Event::Basic(labels[0]);
    }
    let split = rng.random_range(1..labels.len());
    let children = vec![
        random_tree(rng, &labels[..split]),
        random_tree(rng, &labels[split..]),
    ];
    if rng.random_bool(0.5) {
        Event::And(children)
    } else {
        Event::Or(children)
    }
}

fn add_shared_event(rng: &mut StdRng, tree: &mut Event, event: usize) {
    match tree {
        Event::Basic(_) => unreachable!("called with a gate"),
        Event::And(children) | Event::Or(children) => {
            let already_direct = children
                .iter()
                .any(|child| matches!(child, Event::Basic(i) if *i == event));
            let gates: Vec<_> = children
                .iter()
                .enumerate()
                .filter_map(|(i, child)| (!matches!(child, Event::Basic(_))).then_some(i))
                .collect();
            if !gates.is_empty() && (already_direct || rng.random_bool(0.65)) {
                let next = gates[rng.random_range(0..gates.len())];
                add_shared_event(rng, &mut children[next], event);
            } else if !already_direct {
                children.push(Event::Basic(event));
            }
        }
    }
}

fn sample_tree(rng: &mut StdRng, regime: usize) -> (Event, usize) {
    match regime {
        0 | 1 => {
            let n = rng.random_range(2..=7);
            let children = (0..n).map(Event::Basic).collect();
            let tree = if regime == 0 {
                Event::Or(children)
            } else {
                Event::And(children)
            };
            (tree, n)
        }
        2 | 3 => {
            let n = rng.random_range(5..=8);
            let banks = rng.random_range(2..=3);
            let mut labels: Vec<_> = (0..n).collect();
            labels.shuffle(rng);
            let mut groups = vec![Vec::new(); banks];
            for (i, event) in labels.into_iter().enumerate() {
                groups[i % banks].push(Event::Basic(event));
            }
            let children = groups
                .into_iter()
                .map(|group| {
                    if regime == 2 {
                        Event::Or(group)
                    } else {
                        Event::And(group)
                    }
                })
                .collect();
            let tree = if regime == 2 {
                Event::And(children)
            } else {
                Event::Or(children)
            };
            (tree, n)
        }
        4 => {
            let trains = rng.random_range(2..=3);
            let mut next = 1;
            let mut children = Vec::new();
            for _ in 0..trains {
                let mut train = vec![Event::Basic(0)];
                for _ in 0..rng.random_range(1..=2) {
                    train.push(Event::Basic(next));
                    next += 1;
                }
                children.push(Event::Or(train));
            }
            (Event::And(children), next)
        }
        5 => {
            let n = rng.random_range(3..=8);
            let mut labels: Vec<_> = (0..n).collect();
            labels.shuffle(rng);
            let children = (0..n)
                .map(|i| {
                    Event::Or(vec![
                        Event::Basic(labels[i]),
                        Event::Basic(labels[(i + 1) % n]),
                    ])
                })
                .collect();
            (Event::And(children), n)
        }
        6 => {
            let n = rng.random_range(4..=8);
            let mut labels: Vec<_> = (0..n).collect();
            labels.shuffle(rng);
            let mut tree = random_tree(rng, &labels);
            for _ in 0..rng.random_range(1..=3) {
                let event = rng.random_range(0..n);
                add_shared_event(rng, &mut tree, event);
            }
            (tree, n)
        }
        _ => {
            let n = rng.random_range(3..=5);
            let threshold = rng.random_range(2..n);
            let children = (1_u16..1 << n)
                .filter(|mask| mask.count_ones() as usize == threshold)
                .map(|mask| {
                    Event::And(
                        (0..n)
                            .filter(|i| mask & (1 << i) != 0)
                            .map(Event::Basic)
                            .collect(),
                    )
                })
                .collect();
            (Event::Or(children), n)
        }
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..8);
    let (tree, n) = sample_tree(rng, regime);
    let rare = rng.random_bool(0.4);
    let mut probabilities: Vec<u8> = (0..n)
        .map(|_| {
            if rare {
                rng.random_range(1..=5)
            } else {
                rng.random_range(8..=45)
            }
        })
        .collect();
    if regime == 4 && rng.random_bool(0.5) {
        probabilities[0] = rng.random_range(15..=35);
    }
    let mut lines = vec![
        format!("TOP = {}", tree.expression()),
        "Independent basic events on one demand".to_owned(),
    ];
    for (i, p) in probabilities.iter().enumerate() {
        let decimal = format!("{:.2}", f64::from(*p) / 100.0);
        lines.push(format!("P({}) = {}", label(i), trim_decimal(&decimal)));
    }
    let cuts = tree
        .cut_sets()
        .iter()
        .map(|mask| {
            let members = (0..n)
                .filter(|i| mask & (1 << i) != 0)
                .map(|i| label(i).to_string())
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{{members}}}")
        })
        .collect::<Vec<_>>()
        .join(", ");
    lines.push(format!("Minimal cut sets: {cuts}"));
    lines.push(format!(
        "P(TOP) = {}",
        probability(&tree, &probabilities).display()
    ));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn shared_cause_is_not_an_independent_copy() {
        // (A OR C) AND (B OR C) = C OR (A AND B).
        let tree = Event::And(vec![
            Event::Or(vec![Event::Basic(0), Event::Basic(2)]),
            Event::Or(vec![Event::Basic(1), Event::Basic(2)]),
        ]);
        assert_eq!(tree.cut_sets(), vec![4, 3]);
        assert_eq!(probability(&tree, &[20, 30, 10]), Fraction::new(154, 1000));
    }

    #[test]
    fn overlapping_cuts_use_their_union_probability() {
        let tree = Event::Or(vec![
            Event::And(vec![Event::Basic(0), Event::Basic(1)]),
            Event::And(vec![Event::Basic(0), Event::Basic(2)]),
        ]);
        assert_eq!(tree.cut_sets(), vec![3, 5]);
        // 0.2 * (0.3 + 0.4 - 0.3 * 0.4), not 0.2*0.3 + 0.2*0.4.
        assert_eq!(probability(&tree, &[20, 30, 40]), Fraction::new(116, 1000));
    }

    #[test]
    fn absorption_and_duplicate_occurrences_are_idempotent() {
        let tree = Event::Or(vec![
            Event::Basic(0),
            Event::And(vec![Event::Basic(0), Event::Basic(1)]),
            Event::And(vec![Event::Basic(0), Event::Basic(0)]),
        ]);
        assert_eq!(tree.cut_sets(), vec![1]);
        assert_eq!(probability(&tree, &[17, 91]), Fraction::new(17, 100));
    }

    // This oracle extracts minimal satisfying assignments directly, independently
    // of the recursive union/product construction used by Event::cut_sets.
    fn minimal_satisfying_assignments(tree: &Event, n: usize) -> Vec<u16> {
        let mut cuts: Vec<_> = (1_u16..1 << n)
            .filter(|&mask| {
                tree.occurs(mask)
                    && (0..n).all(|i| mask & (1 << i) == 0 || !tree.occurs(mask ^ (1 << i)))
            })
            .collect();
        cuts.sort_unstable_by_key(|mask| (mask.count_ones(), *mask));
        cuts
    }

    #[test]
    fn every_regime_has_exact_and_complete_minimal_cuts() {
        for regime in 0..8 {
            for seed in 0..32 {
                let mut rng = StdRng::seed_from_u64(100 * regime as u64 + seed);
                let (tree, n) = sample_tree(&mut rng, regime);
                assert!((2..=8).contains(&n));
                let cuts = tree.cut_sets();
                assert_eq!(cuts, minimal_satisfying_assignments(&tree, n));
                for failed in 0..1_u16 << n {
                    assert_eq!(
                        tree.occurs(failed),
                        cuts.iter().any(|&cut| cut & !failed == 0)
                    );
                }
                let satisfying = (0..1_u16 << n).filter(|&mask| tree.occurs(mask)).count() as u128;
                assert_eq!(
                    probability(&tree, &vec![50; n]),
                    Fraction::new(satisfying, 1 << n)
                );
            }
        }
    }

    #[test]
    fn heterogeneous_probabilities_match_inclusion_exclusion() {
        for regime in 0..8 {
            let mut rng = StdRng::seed_from_u64(93 + regime as u64);
            let (tree, n) = sample_tree(&mut rng, regime);
            let cuts = tree.cut_sets();
            let probabilities: Vec<u8> = (0..n).map(|i| 7 + 9 * i as u8).collect();
            // Merge equal intersections while expanding inclusion-exclusion,
            // keeping the oracle bounded by the 2^n possible event unions.
            let mut coefficients = std::collections::BTreeMap::<u16, i128>::new();
            for cut in cuts {
                let mut changes = vec![(cut, 1)];
                for (&union, &coefficient) in &coefficients {
                    changes.push((union | cut, -coefficient));
                }
                for (union, coefficient) in changes {
                    *coefficients.entry(union).or_default() += coefficient;
                }
                coefficients.retain(|_, coefficient| *coefficient != 0);
            }
            let union_mass = coefficients
                .into_iter()
                .map(|(union, coefficient)| {
                    let mass = probabilities
                        .iter()
                        .enumerate()
                        .map(|(i, &p)| i128::from(if union & (1 << i) != 0 { p } else { 100 }))
                        .product::<i128>();
                    coefficient * mass
                })
                .sum::<i128>();
            assert_eq!(
                probability(&tree, &probabilities),
                Fraction::new(union_mass as u128, 100_u128.pow(n as u32))
            );
        }
    }

    #[test]
    fn deterministic_probability_endpoints_are_supported() {
        let mut rng = StdRng::seed_from_u64(7);
        for regime in 0..8 {
            let (tree, n) = sample_tree(&mut rng, regime);
            assert_eq!(probability(&tree, &vec![0; n]), Fraction::new(0, 1));
            assert_eq!(probability(&tree, &vec![100; n]), Fraction::new(1, 1));
        }
    }
}
