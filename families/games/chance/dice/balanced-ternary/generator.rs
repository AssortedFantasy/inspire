use rand::{Rng, rngs::StdRng};
pub fn generate(rng: &mut StdRng) -> String {
    let values: Vec<i32> = (0..4).map(|_| rng.random_range(-1..=1)).collect();
    let symbols = values
        .iter()
        .map(|v| match v {
            -1 => "-",
            0 => "0",
            _ => "+",
        })
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "Four balanced dice (-, 0, + each equally likely): [{symbols}] => {:+}",
        values.iter().sum::<i32>()
    )
}
