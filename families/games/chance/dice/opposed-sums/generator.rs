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
    let a_count = rng.random_range(2..=6);
    let b_count = rng.random_range(2..=6);
    let a = roll(rng, a_count, 6);
    let b = roll(rng, b_count, 6);
    let a_total = a.iter().sum::<u16>();
    let b_total = b.iter().sum::<u16>();
    let result = match a_total.cmp(&b_total) {
        std::cmp::Ordering::Greater => "A wins",
        std::cmp::Ordering::Less => "B wins",
        std::cmp::Ordering::Equal => "tie",
    };
    format!(
        "Opposed sums: A {a_count}d6 [{}] = {a_total}; B {b_count}d6 [{}] = {b_total}; {result}",
        list(&a),
        list(&b)
    )
}
