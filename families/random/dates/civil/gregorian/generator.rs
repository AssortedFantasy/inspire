use rand::{Rng, rngs::StdRng};
#[allow(dead_code)]
mod calendar;

pub fn generate(rng: &mut StdRng) -> String {
    let ordinal = rng.random_range(0..calendar::DAYS);
    let (year, month, day) = calendar::calendar(ordinal);
    format!(
        "Proleptic Gregorian date: {}, {day} {} {year} ({}).",
        calendar::WEEKDAYS[calendar::weekday(ordinal)],
        calendar::MONTHS[(month - 1) as usize],
        calendar::iso(ordinal)
    )
}
