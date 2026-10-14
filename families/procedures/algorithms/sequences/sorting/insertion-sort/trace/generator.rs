use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

fn execute(input: &[i32]) -> (Vec<i32>, Vec<Vec<i32>>) {
    let mut data = input.to_vec();
    let mut passes = Vec::new();
    for i in 1..data.len() {
        let key = data[i];
        let mut j = i;
        while j > 0 && data[j - 1] > key {
            data[j] = data[j - 1];
            j -= 1;
        }
        data[j] = key;
        passes.push(data.clone());
    }
    (data, passes)
}

pub fn generate(rng: &mut StdRng) -> String {
    let input: Vec<_> = (0..rng.random_range(5..=9))
        .map(|_| rng.random_range(-9..=19))
        .collect();
    let (_, passes) = execute(&input);
    let mut text = format!(
        "Insertion-sort execution\nInput: {input:?}\nAscending order; each pass inserts the next key into the sorted prefix."
    );
    for (i, state) in passes.iter().enumerate() {
        write!(
            text,
            "\nInsert key at original prefix length {}: {state:?}",
            i + 2
        )
        .unwrap();
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn every_pass_preserves_items_and_grows_sorted_prefix() {
        let mut rng = StdRng::seed_from_u64(281);
        for _ in 0..1000 {
            let input: Vec<_> = (0..rng.random_range(0..=12))
                .map(|_| rng.random_range(-4..=4))
                .collect();
            let (result, passes) = execute(&input);
            let mut expected = input.clone();
            expected.sort();
            assert_eq!(result, expected);
            for (i, state) in passes.iter().enumerate() {
                assert!(state[..i + 2].windows(2).all(|pair| pair[0] <= pair[1]));
                let mut items = state.clone();
                items.sort();
                assert_eq!(items, expected);
                assert_eq!(state[i + 2..], input[i + 2..]);
            }
        }
    }
}
