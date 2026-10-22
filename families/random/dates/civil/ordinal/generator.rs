use rand::{Rng, rngs::StdRng};
#[allow(dead_code)]
mod calendar;

pub fn generate(rng: &mut StdRng) -> String {
    render(rng.random_range(0..calendar::DAYS))
}

fn render(ordinal: u32) -> String {
    let (year, _, _) = calendar::calendar(ordinal);
    let day = ordinal - calendar::ordinal(year, 1, 1) + 1;
    format!("{year:04}-{day:03}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinal_boundaries_and_century_leap_days() {
        for (year, month, date, day, _) in [
            (1600, 1, 1, 1, 366),
            (1600, 2, 29, 60, 366),
            (1600, 12, 31, 366, 366),
            (1700, 3, 1, 60, 365),
            (1900, 12, 31, 365, 365),
            (2000, 3, 1, 61, 366),
            (2100, 3, 1, 60, 365),
            (2199, 12, 31, 365, 365),
        ] {
            assert_eq!(
                render(calendar::ordinal(year, month, date)),
                format!("{year:04}-{day:03}")
            );
        }
    }
}
