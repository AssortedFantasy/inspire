use rand::{Rng, rngs::StdRng};
#[allow(dead_code)]
mod calendar;

pub fn generate(rng: &mut StdRng) -> String {
    let ordinal = rng.random_range(0..calendar::DAYS);
    let (year, month, day) = calendar::calendar(ordinal);
    match rng.random_range(0..3) {
        0 => calendar::iso(ordinal),
        1 => format!(
            "{} ({})",
            calendar::iso(ordinal),
            calendar::WEEKDAYS[calendar::weekday(ordinal)]
        ),
        _ => format!("{day} {} {year}", calendar::MONTHS[(month - 1) as usize]),
    }
}
