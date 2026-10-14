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
    let sides = [4, 6, 8, 10, 12, 20, 100][rng.random_range(0..7)];
    let count = rng.random_range(1..=6);
    let modifier: i32 = rng.random_range(-3..=5);
    let values = roll(rng, count, sides);
    let total = values.iter().map(|&v| i32::from(v)).sum::<i32>() + modifier;
    let suffix = if modifier == 0 {
        String::new()
    } else {
        format!("{modifier:+}")
    };
    format!("{count}d{sides}{suffix}: [{}] => {total}", list(&values))
}
