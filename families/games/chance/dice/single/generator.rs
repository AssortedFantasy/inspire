use rand::{Rng, rngs::StdRng};
pub fn generate(rng: &mut StdRng) -> String {
    let sides = [4, 6, 8, 10, 12, 20, 100][rng.random_range(0..7)];
    let value = rng.random_range(1..=sides);
    format!("1d{sides}: [{value}] => {value}")
}
