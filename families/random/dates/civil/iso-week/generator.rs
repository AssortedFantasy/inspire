use rand::{Rng, rngs::StdRng};
#[allow(dead_code)]
mod calendar;

pub fn generate(rng: &mut StdRng) -> String {
    let ordinal = rng.random_range(0..calendar::DAYS);
    let (year, week, day) = calendar::iso_week(ordinal);
    format!("{year:04}-W{week:02}-{day}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exhaustive_january_fourth_reference() {
        // ISO week 1 is the week containing January 4. This reference uses
        // its Monday as the anchor, independently of the day-of-year formula.
        fn week_one(year: u32) -> i64 {
            let january_fourth = calendar::ordinal(year, 1, 4) as i64;
            january_fourth - (january_fourth + 5).rem_euclid(7)
        }
        for ordinal in 0..calendar::DAYS {
            let (year, _, _) = calendar::calendar(ordinal);
            let day = ordinal as i64;
            let current_start = week_one(year);
            let next_start = week_one(year + 1);
            let (week_year, start) = if day < current_start {
                let previous_january_fourth = calendar::ordinal(year, 1, 4) as i64
                    - if calendar::leap(year - 1) { 366 } else { 365 };
                (
                    year - 1,
                    previous_january_fourth - (previous_january_fourth + 5).rem_euclid(7),
                )
            } else if day >= next_start {
                (year + 1, next_start)
            } else {
                (year, current_start)
            };
            let expected = (
                week_year,
                ((day - start) / 7 + 1) as u32,
                ((day - start) % 7 + 1) as u32,
            );
            assert_eq!(calendar::iso_week(ordinal), expected, "ordinal {ordinal}");
        }
    }

    #[test]
    fn fifty_three_weeks_require_thursday_or_leap_wednesday() {
        for year in 1600..=2199 {
            let january_first = calendar::weekday(calendar::ordinal(year, 1, 1));
            let expected = if january_first == 3 || (january_first == 2 && calendar::leap(year)) {
                53
            } else {
                52
            };
            // December 28 always belongs to the year's final ISO week.
            assert_eq!(
                calendar::iso_week(calendar::ordinal(year, 12, 28)),
                (
                    year,
                    expected,
                    1 + calendar::weekday(calendar::ordinal(year, 12, 28)) as u32
                )
            );
        }
    }

    #[test]
    fn leap_centuries_and_domain_edges() {
        for (date, expected) in [
            ((1600, 1, 1), (1599, 52, 6)),
            ((1600, 12, 31), (1600, 52, 7)),
            ((1700, 1, 1), (1699, 53, 5)),
            ((1800, 1, 1), (1800, 1, 3)),
            ((1900, 1, 1), (1900, 1, 1)),
            ((2000, 1, 1), (1999, 52, 6)),
            ((2100, 1, 1), (2099, 53, 5)),
            ((2199, 12, 31), (2200, 1, 2)),
        ] {
            assert_eq!(
                calendar::iso_week(calendar::ordinal(date.0, date.1, date.2)),
                expected
            );
        }
    }

    #[test]
    fn iso_weeks_change_on_monday() {
        let mut previous = calendar::iso_week(0);
        for ordinal in 1..calendar::DAYS {
            let current = calendar::iso_week(ordinal);
            assert!((1..=53).contains(&current.1));
            assert!((1..=7).contains(&current.2));
            if current.2 != 1 {
                assert_eq!(current, (previous.0, previous.1, previous.2 + 1));
            } else if current.1 == 1 {
                assert_eq!(current.0, previous.0 + 1);
            } else {
                assert_eq!(current, (previous.0, previous.1 + 1, 1));
            }
            previous = current;
        }
    }
}
