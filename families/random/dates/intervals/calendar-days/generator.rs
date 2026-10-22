use rand::{Rng, rngs::StdRng};
#[allow(dead_code)]
mod calendar;

pub fn generate(rng: &mut StdRng) -> String {
    let (start, length) = sample(rng);
    match rng.random_range(0..3) {
        0 => format!("{}/{}", calendar::iso(start), calendar::iso(start + length)),
        1 => format!("{}/P{length}D", calendar::iso(start)),
        _ => format!("P{length}D/{}", calendar::iso(start + length)),
    }
}

fn sample(rng: &mut StdRng) -> (u32, u32) {
    let length = match rng.random_range(0..3) {
        0 => rng.random_range(1..=14),
        1 => rng.random_range(15..=180),
        _ => rng.random_range(181..=3660),
    };
    (rng.random_range(0..calendar::DAYS - length), length)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn ordered_intervals_have_exact_calendar_length() {
        let mut rng = StdRng::seed_from_u64(24);
        let mut bands = [false; 3];
        for _ in 0..10_000 {
            let (start, length) = sample(&mut rng);
            assert!(length > 0 && start + length < calendar::DAYS);
            let (y, m, d) = calendar::calendar(start + length);
            assert_eq!(calendar::ordinal(y, m, d) - start, length);
            bands[if length <= 14 {
                0
            } else if length <= 180 {
                1
            } else {
                2
            }] = true;
        }
        assert!(bands.into_iter().all(|seen| seen));
    }
}
