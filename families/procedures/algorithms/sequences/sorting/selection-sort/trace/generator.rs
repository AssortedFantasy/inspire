use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

fn execute(input: &[i32]) -> (Vec<i32>, Vec<(usize, Vec<i32>)>) {
    let mut data = input.to_vec();
    let mut passes = Vec::new();
    for i in 0..data.len().saturating_sub(1) {
        let minimum = (i..data.len()).min_by_key(|&j| (data[j], j)).unwrap();
        data.swap(i, minimum);
        passes.push((minimum, data.clone()));
    }
    (data, passes)
}
pub fn generate(rng: &mut StdRng) -> String {
    let input: Vec<_> = (0..rng.random_range(5..=9))
        .map(|_| rng.random_range(-9..=19))
        .collect();
    let (_, passes) = execute(&input);
    let mut text = format!("Selection sort\n{input:?}");
    for (i, (minimum, state)) in passes.iter().enumerate() {
        if i == *minimum {
            write!(text, "\nKeep {i}     → {state:?}").unwrap();
        } else {
            write!(text, "\nSwap {i} ↔ {minimum} → {state:?}").unwrap();
        }
    }
    text
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn passes_place_the_next_order_statistic() {
        let mut rng = StdRng::seed_from_u64(282);
        for _ in 0..1000 {
            let input: Vec<_> = (0..rng.random_range(0..=12))
                .map(|_| rng.random_range(-4..=4))
                .collect();
            let (result, passes) = execute(&input);
            let mut expected = input.clone();
            expected.sort();
            assert_eq!(result, expected);
            let mut previous = input;
            for (i, (j, state)) in passes.iter().enumerate() {
                assert_eq!(
                    *j,
                    (i..previous.len())
                        .min_by_key(|&k| (previous[k], k))
                        .unwrap()
                );
                previous.swap(i, *j);
                assert_eq!(*state, previous);
                assert_eq!(state[..=i], expected[..=i]);
            }
        }
    }
}
