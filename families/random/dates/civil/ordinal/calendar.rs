//! Bounded proleptic Gregorian arithmetic; no timezone database or runtime clock.
pub const DAYS: u32 = 219_146;
pub const MONTHS: [&str; 12] = [
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
pub const WEEKDAYS: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

pub fn leap(year: u32) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

pub fn month_days(year: u32, month: u32) -> u32 {
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

fn before_year(year: u32) -> u32 {
    let y = year - 1;
    365 * y + y / 4 - y / 100 + y / 400
}

pub fn ordinal(year: u32, month: u32, day: u32) -> u32 {
    before_year(year) - before_year(1600)
        + (1..month).map(|m| month_days(year, m)).sum::<u32>()
        + day
        - 1
}

pub fn calendar(ordinal: u32) -> (u32, u32, u32) {
    assert!(ordinal < DAYS);
    let absolute = before_year(1600) + ordinal;
    let (mut low, mut high) = (1600, 2200);
    while low + 1 < high {
        let middle = (low + high) / 2;
        if before_year(middle) <= absolute {
            low = middle;
        } else {
            high = middle;
        }
    }
    let mut day = absolute - before_year(low);
    let mut month = 1;
    while day >= month_days(low, month) {
        day -= month_days(low, month);
        month += 1;
    }
    (low, month, day + 1)
}

pub fn weekday(ordinal: u32) -> usize {
    ((ordinal + 5) % 7) as usize
}

pub fn iso(ordinal: u32) -> String {
    let (y, m, d) = calendar(ordinal);
    format!("{y:04}-{m:02}-{d:02}")
}

pub fn timestamp(ordinal: u32, second: u32) -> String {
    format!(
        "{}T{:02}:{:02}:{:02}",
        iso(ordinal),
        second / 3600,
        second / 60 % 60,
        second % 60
    )
}

pub fn iso_week(ordinal: u32) -> (u32, u32, u32) {
    fn weeks(year: u32) -> u32 {
        let jan1 = before_year(year) % 7;
        if jan1 == 3 || (jan1 == 2 && leap(year)) {
            53
        } else {
            52
        }
    }
    let (year, month, day) = calendar(ordinal);
    let day_of_year = (1..month).map(|m| month_days(year, m)).sum::<u32>() + day;
    let day_of_week = weekday(ordinal) as u32 + 1;
    let week = (day_of_year + 10 - day_of_week) / 7;
    if week == 0 {
        (year - 1, weeks(year - 1), day_of_week)
    } else if week > weeks(year) {
        (year + 1, 1, day_of_week)
    } else {
        (year, week, day_of_week)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exhaustive_calendar_bijection() {
        let mut leap_days = 0;
        for day in 0..DAYS {
            let (y, m, d) = calendar(day);
            assert_eq!(ordinal(y, m, d), day);
            assert!((1..=month_days(y, m)).contains(&d));
            leap_days += u32::from(m == 2 && d == 29);
        }
        assert_eq!(leap_days, 146);
        assert_eq!(calendar(DAYS - 1), (2199, 12, 31));
        assert!(!leap(1700) && !leap(1900) && !leap(2100) && leap(2000));
    }
    #[test]
    fn iso_year_boundaries() {
        assert_eq!(iso_week(ordinal(2016, 1, 1)), (2015, 53, 5));
        assert_eq!(iso_week(ordinal(2019, 12, 30)), (2020, 1, 1));
        assert_eq!(iso_week(0), (1599, 52, 6));
        assert_eq!(WEEKDAYS[weekday(ordinal(2000, 1, 1))], "Saturday");
    }
}
