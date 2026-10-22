use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

type Item = (i32, char);
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
    let mut left: Vec<_> = (0..rng.random_range(3..=5))
        .map(|i| (rng.random_range(0..=9), (b'a' + i) as char))
        .collect();
    let mut right: Vec<_> = (0..rng.random_range(3..=5))
        .map(|i| (rng.random_range(0..=9), (b'p' + i) as char))
        .collect();
    left.sort_by_key(|item| item.0);
    right.sort_by_key(|item| item.0);
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
            let mut expected = left.clone();
            expected.extend(&right);
            expected.sort_by_key(|item| item.0);
            let (actual, sides) = merge(&left, &right);
            assert_eq!(actual, expected);
            assert_eq!(
                sides.iter().filter(|&&side| side == 'L').count(),
                left.len()
            );
            assert_eq!(
                sides.iter().filter(|&&side| side == 'R').count(),
                right.len()
            );
        }
    }
}
