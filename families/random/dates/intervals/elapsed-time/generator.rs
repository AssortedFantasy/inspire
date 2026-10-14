use rand::{Rng, rngs::StdRng};
#[allow(dead_code)]
mod calendar;

fn duration(seconds: u32) -> String {
    format!(
        "P{}DT{}H{}M{}S",
        seconds / 86_400,
        seconds / 3600 % 24,
        seconds / 60 % 60,
        seconds % 60
    )
}

pub fn generate(rng: &mut StdRng) -> String {
    let length = match rng.random_range(0..3) {
        0 => rng.random_range(1..=3600),
        1 => rng.random_range(3601..=86_400),
        _ => rng.random_range(86_401..=2_592_000),
    };
    let start = rng.random_range(0..(u64::from(calendar::DAYS) - 31) * 86_400);
    let end = start + u64::from(length);
    format!(
        "UTC elapsed-time interval: {}Z/{}Z; duration {} ({length} seconds; leap seconds excluded).",
        calendar::timestamp((start / 86_400) as u32, (start % 86_400) as u32),
        calendar::timestamp((end / 86_400) as u32, (end % 86_400) as u32),
        duration(length)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duration_is_exact_without_calendar_months() {
        for seconds in [1, 59, 60, 3600, 86_400, 86_401, 2_592_000] {
            let rendered = duration(seconds);
            let parts: Vec<u32> = rendered
                .split(['P', 'D', 'T', 'H', 'M', 'S'])
                .filter(|part| !part.is_empty())
                .map(|part| part.parse().unwrap())
                .collect();
            assert_eq!(
                parts[0] * 86_400 + parts[1] * 3600 + parts[2] * 60 + parts[3],
                seconds
            );
        }
    }
}
