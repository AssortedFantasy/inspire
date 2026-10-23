use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

type Item = (i32, char);

#[derive(Clone, Copy, Debug)]
enum Regime {
    Mixed,
    Tied,
    Disjoint,
    Interleaved,
    Unequal,
}

const REGIMES: [Regime; 5] = [
    Regime::Mixed,
    Regime::Tied,
    Regime::Disjoint,
    Regime::Interleaved,
    Regime::Unequal,
];

fn runs(rng: &mut StdRng, regime: Regime) -> (Vec<Item>, Vec<Item>) {
    let (mut left, mut right): (Vec<_>, Vec<_>) = match regime {
        Regime::Mixed => (
            (0..rng.random_range(3..=5))
                .map(|_| rng.random_range(0..=9))
                .collect(),
            (0..rng.random_range(3..=5))
                .map(|_| rng.random_range(0..=9))
                .collect(),
        ),
        Regime::Tied => {
            let key = rng.random_range(0..=9);
            (
                vec![key; rng.random_range(2..=5)],
                vec![key; rng.random_range(2..=5)],
            )
        }
        Regime::Disjoint => {
            let start = rng.random_range(-5..=5);
            let step = rng.random_range(1..=3);
            let low: Vec<_> = (0..rng.random_range(2..=5))
                .map(|i| start + i * step)
                .collect();
            let high_start = low.last().unwrap() + rng.random_range(1..=3);
            let high = (0..rng.random_range(2..=5))
                .map(|i| high_start + i * step)
                .collect();
            (low, high)
        }
        Regime::Interleaved => {
            let count = rng.random_range(3..=5);
            let start = rng.random_range(-5..=5);
            let step = rng.random_range(1..=3);
            (
                (0..count).map(|i| start + 2 * i * step).collect(),
                (0..count).map(|i| start + (2 * i + 1) * step).collect(),
            )
        }
        Regime::Unequal => {
            let count = rng.random_range(6..=9);
            let start = rng.random_range(-5..=5);
            let step = rng.random_range(1..=3);
            let long: Vec<_> = (0..count).map(|i| start + i * step).collect();
            let singleton = vec![long[rng.random_range(1..long.len() - 1)]];
            (singleton, long)
        }
    };
    if rng.random_bool(0.5) {
        std::mem::swap(&mut left, &mut right);
    }
    let tag = |keys: Vec<i32>, first: u8| {
        let mut items: Vec<_> = keys
            .into_iter()
            .enumerate()
            .map(|(i, key)| (key, (first + i as u8) as char))
            .collect();
        items.sort_by_key(|item| item.0);
        items
    };
    (tag(left, b'a'), tag(right, b'p'))
}

fn merge(left: &[Item], right: &[Item]) -> (Vec<Item>, Vec<char>) {
    let (mut i, mut j) = (0, 0);
    let mut result = Vec::new();
    let mut sides = Vec::new();
    while i < left.len() || j < right.len() {
        if j == right.len() || (i < left.len() && left[i].0 <= right[j].0) {
            result.push(left[i]);
            sides.push('L');
            i += 1;
        } else {
            result.push(right[j]);
            sides.push('R');
            j += 1;
        }
    }
    (result, sides)
}
pub fn generate(rng: &mut StdRng) -> String {
    let regime = REGIMES[rng.random_range(0..REGIMES.len())];
    let (left, right) = runs(rng, regime);
    let (result, sides) = merge(&left, &right);
    let display = |items: &[Item]| {
        items
            .iter()
            .map(|&(key, id)| format!("{key}{id}"))
            .collect::<Vec<_>>()
            .join(" ")
    };
    let mut text = format!(
        "Stable merge\nLeft   {}\nRight  {}",
        display(&left),
        display(&right)
    );
    for (item, side) in result.iter().zip(sides) {
        write!(text, "\n{side} → {}{}", item.0, item.1).unwrap();
    }
    write!(text, "\nMerged {}", display(&result)).unwrap();
    text
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn assert_stable_merge(left: &[Item], right: &[Item]) {
        let mut expected = left.to_vec();
        expected.extend(right);
        expected.sort_by_key(|item| item.0);
        let (actual, sides) = merge(left, right);
        assert_eq!(actual, expected);
        for (source, side) in [(left, 'L'), (right, 'R')] {
            let consumed: Vec<_> = actual
                .iter()
                .zip(&sides)
                .filter(|&(_, &origin)| origin == side)
                .map(|(&item, _)| item)
                .collect();
            assert_eq!(consumed, source);
        }
    }

    #[test]
    fn constructed_regimes_have_their_distinct_merge_shapes() {
        let mut rng = StdRng::seed_from_u64(20261022);
        for regime in REGIMES {
            for _ in 0..128 {
                let (left, right) = runs(&mut rng, regime);
                assert!(left.windows(2).all(|pair| pair[0].0 <= pair[1].0));
                assert!(right.windows(2).all(|pair| pair[0].0 <= pair[1].0));
                assert_stable_merge(&left, &right);
                let (merged, sides) = merge(&left, &right);
                match regime {
                    Regime::Mixed => {}
                    Regime::Tied => {
                        assert!(merged.iter().all(|item| item.0 == merged[0].0));
                        assert!(sides[..left.len()].iter().all(|&side| side == 'L'));
                    }
                    Regime::Disjoint => {
                        assert!(
                            left.last().unwrap().0 < right[0].0
                                || right.last().unwrap().0 < left[0].0
                        );
                        assert_eq!(
                            sides.windows(2).filter(|pair| pair[0] != pair[1]).count(),
                            1
                        );
                    }
                    Regime::Interleaved => {
                        assert!(sides.windows(2).all(|pair| pair[0] != pair[1]));
                    }
                    Regime::Unequal => {
                        let (short, long) = if left.len() == 1 {
                            (&left, &right)
                        } else {
                            (&right, &left)
                        };
                        assert_eq!(short.len(), 1);
                        assert!((6..=9).contains(&long.len()));
                        assert!(long[0].0 < short[0].0);
                        assert!(short[0].0 < long.last().unwrap().0);
                        assert!(long.iter().any(|item| item.0 == short[0].0));
                    }
                }
            }
        }
    }

    #[test]
    fn result_equals_stable_sort_of_concatenated_runs() {
        let mut rng = StdRng::seed_from_u64(283);
        for _ in 0..1000 {
            let mut left: Vec<_> = (0..rng.random_range(0..=6))
                .map(|i| (rng.random_range(-2..=2), (b'a' + i) as char))
                .collect();
            let mut right: Vec<_> = (0..rng.random_range(0..=6))
                .map(|i| (rng.random_range(-2..=2), (b'p' + i) as char))
                .collect();
            left.sort_by_key(|item| item.0);
            right.sort_by_key(|item| item.0);
            assert_stable_merge(&left, &right);
        }
    }
}
