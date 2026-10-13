use rand::{Rng, rngs::StdRng};

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const WEEKDAYS: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

fn leap(year: u32) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

fn month_days(year: u32, month: u32) -> u32 {
    match month {
        2 => {
            if leap(year) {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn calendar(mut ordinal: u32) -> (u32, u32, u32) {
    let mut year = 1600;
    loop {
        let length = if leap(year) { 366 } else { 365 };
        if ordinal < length {
            break;
        }
        ordinal -= length;
        year += 1;
    }
    let mut month = 1;
    while ordinal >= month_days(year, month) {
        ordinal -= month_days(year, month);
        month += 1;
    }
    (year, month, ordinal + 1)
}

fn total_days() -> u32 {
    (1600..=2199).map(|y| if leap(y) { 366 } else { 365 }).sum()
}

fn render(ordinal: u32, second: u32, style: u32) -> String {
    let (year, month, day) = calendar(ordinal);
    let (hour, minute, second) = (second / 3600, second / 60 % 60, second % 60);
    let name = MONTHS[(month - 1) as usize];
    match style {
        0 => format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z"),
        1 => format!("{day} {name} {year}, {hour:02}:{minute:02}:{second:02} UTC"),
        2 => format!(
            "{name} {day}, {year} at {}:{minute:02}:{second:02} {} UTC",
            (hour + 11) % 12 + 1,
            if hour < 12 { "AM" } else { "PM" }
        ),
        3 => format!(
            "{}, {day} {name} {year}, {hour:02}:{minute:02}:{second:02} UTC",
            WEEKDAYS[((ordinal + 5) % 7) as usize]
        ),
        _ => format!(
            "{year:04}/{month:02}/{day:02} (year/month/day) {hour:02}:{minute:02}:{second:02} UTC"
        ),
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    render(
        rng.random_range(0..total_days()),
        rng.random_range(0..86400),
        rng.random_range(0..5),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn all_days_are_valid_and_consecutive() {
        let mut previous = (1599, 12, 31);
        let mut leap_days = 0;
        for ordinal in 0..total_days() {
            let current = calendar(ordinal);
            let (y, m, d) = current;
            assert!((1600..=2199).contains(&y));
            assert!((1..=12).contains(&m));
            assert!((1..=month_days(y, m)).contains(&d));
            if d > 1 {
                assert_eq!(current, (previous.0, previous.1, previous.2 + 1));
            } else if m > 1 {
                assert_eq!((y, m), (previous.0, previous.1 + 1));
            } else {
                assert_eq!(y, previous.0 + 1);
            }
            leap_days += usize::from(m == 2 && d == 29);
            previous = current;
        }
        assert_eq!(previous, (2199, 12, 31));
        assert_eq!(leap_days, 146);
        assert!(leap(1600) && leap(2000));
        assert!(!leap(1700) && !leap(1900) && !leap(2100));
    }

    #[test]
    fn formats_disambiguate_dates_and_midnight() {
        assert_eq!(render(0, 0, 0), "1600-01-01T00:00:00Z");
        assert_eq!(render(0, 0, 2), "January 1, 1600 at 12:00:00 AM UTC");
        assert_eq!(render(0, 43200, 2), "January 1, 1600 at 12:00:00 PM UTC");
        assert!(render(0, 86399, 3).starts_with("Saturday,"));
        let mut rng = StdRng::seed_from_u64(8);
        for _ in 0..1000 {
            let sample = generate(&mut rng);
            assert!(sample.ends_with('Z') || sample.ends_with("UTC"));
            assert!(!sample.ends_with('\n'));
        }
    }
}
