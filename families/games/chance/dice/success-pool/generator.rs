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

pub fn generate(rng: &mut StdRng) -> String {
    let count = rng.random_range(3..=12);
    let sides = [6, 10][rng.random_range(0..2)];
    let target = rng.random_range((sides / 2 + 1)..=sides);
    let values = roll(rng, count, sides);
    let successes = values.iter().filter(|&&v| v >= target).count();
    format!(
        "{count}d{sides} pool; each die >= {target} is one success: [{}] => {successes} successes",
        list(&values)
    )
}
