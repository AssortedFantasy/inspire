use rand::{Rng, rngs::StdRng};
#[allow(dead_code)]
mod calendar;

pub fn generate(rng: &mut StdRng) -> String {
    format!(
        "{}Z",
        calendar::timestamp(
            rng.random_range(0..calendar::DAYS),
            rng.random_range(0..86_400)
        )
    )
}
