use rand::{Rng, rngs::StdRng};
fn roll(rng: &mut StdRng, count: usize, sides: u16) -> Vec<u16> {
    (0..count).map(|_| rng.random_range(1..=sides)).collect()
}

fn list(values: &[u16]) -> String {
    values
        .iter()
        .map(u16::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

fn kept(values: &[u16], count: usize, high: bool) -> Vec<u16> {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    if high {
        sorted.reverse();
    }
    sorted.truncate(count);
    sorted
}

pub fn generate(rng: &mut StdRng) -> String {
    let sides = [6, 8, 10, 12, 20][rng.random_range(0..5)];
    let count = rng.random_range(3..=7);
    let keep = rng.random_range(1..count);
    let high = rng.random();
    let values = roll(rng, count, sides);
    let selected = kept(&values, keep, high);
    format!(
        "{count}d{sides}, keep {keep} {}: [{}] => [{}] = {}",
        if high { "highest" } else { "lowest" },
        list(&values),
        list(&selected),
        selected.iter().sum::<u16>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selection_retains_multiplicity() {
        assert_eq!(kept(&[2, 6, 2, 5, 6], 3, true), [6, 6, 5]);
        assert_eq!(kept(&[2, 6, 2, 5, 6], 3, false), [2, 2, 5]);
    }
}
