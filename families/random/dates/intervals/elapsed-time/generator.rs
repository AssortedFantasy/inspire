use rand::{Rng, rngs::StdRng};
#[allow(dead_code)]
mod calendar;

fn duration(seconds: u32) -> String {
    let mut text = String::from("P");
    let days = seconds / 86_400;
    if days != 0 {
        text.push_str(&format!("{days}D"));
    }
    let hours = seconds / 3600 % 24;
    let minutes = seconds / 60 % 60;
    let remainder = seconds % 60;
    if hours != 0 || minutes != 0 || remainder != 0 || days == 0 {
        text.push('T');
        if hours != 0 {
            text.push_str(&format!("{hours}H"));
        }
        if minutes != 0 {
            text.push_str(&format!("{minutes}M"));
        }
        if remainder != 0 || (hours == 0 && minutes == 0) {
            text.push_str(&format!("{remainder}S"));
        }
    }
    text
}

pub fn generate(rng: &mut StdRng) -> String {
    let length = match rng.random_range(0..3) {
        0 => rng.random_range(1..=3600),
        1 => rng.random_range(3601..=86_400),
        _ => rng.random_range(86_401..=2_592_000),
    };
    let start = rng.random_range(0..(u64::from(calendar::DAYS) - 31) * 86_400);
    let end = start + u64::from(length);
    let first = format!(
        "{}Z",
        calendar::timestamp((start / 86_400) as u32, (start % 86_400) as u32)
    );
    let last = format!(
        "{}Z",
        calendar::timestamp((end / 86_400) as u32, (end % 86_400) as u32)
    );
    match rng.random_range(0..3) {
        0 => format!("{first}/{last}"),
        1 => format!("{first}/{}", duration(length)),
        _ => format!("{}/{last}", duration(length)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duration_preserves_unit_boundaries_without_zero_components() {
        for (seconds, expected) in [
            (0, "PT0S"),
            (1, "PT1S"),
            (59, "PT59S"),
            (60, "PT1M"),
            (3600, "PT1H"),
            (86_400, "P1D"),
            (86_401, "P1DT1S"),
            (90_061, "P1DT1H1M1S"),
            (2_592_000, "P30D"),
        ] {
            assert_eq!(duration(seconds), expected);
        }
    }
}
