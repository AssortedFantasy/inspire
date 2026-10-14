use rand::{Rng, rngs::StdRng};
#[allow(dead_code)]
mod calendar;

// A mathematical fixed offset, not an assertion about any region's civil time.
const OFFSETS: [i32; 29] = [
    -720, -660, -600, -570, -540, -480, -420, -360, -300, -240, -210, -180, -120, -60, 60, 120,
    180, 210, 240, 270, 300, 330, 345, 360, 390, 480, 570, 765, 840,
];

fn render(day: u32, second: u32, offset: i32) -> String {
    let local = i64::from(day) * 86_400 + i64::from(second) + i64::from(offset) * 60;
    let local_day = (local / 86_400) as u32;
    let local_second = (local % 86_400) as u32;
    let magnitude = offset.unsigned_abs();
    format!(
        "{}{}{:02}:{:02} (fixed offset); same instant: {}Z",
        calendar::timestamp(local_day, local_second),
        if offset < 0 { '-' } else { '+' },
        magnitude / 60,
        magnitude % 60,
        calendar::timestamp(day, second)
    )
}

pub fn generate(rng: &mut StdRng) -> String {
    render(
        rng.random_range(1..calendar::DAYS - 1),
        rng.random_range(0..86_400),
        OFFSETS[rng.random_range(0..OFFSETS.len())],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn offset_rolls_across_midnight_both_directions() {
        assert_eq!(
            render(calendar::ordinal(2000, 1, 1), 0, -210),
            "1999-12-31T20:30:00-03:30 (fixed offset); same instant: 2000-01-01T00:00:00Z"
        );
        assert_eq!(
            render(calendar::ordinal(2000, 12, 31), 23 * 3600, 345),
            "2001-01-01T04:45:00+05:45 (fixed offset); same instant: 2000-12-31T23:00:00Z"
        );
    }
    #[test]
    fn all_offsets_keep_boundary_instants_in_range() {
        for offset in OFFSETS {
            for day in [1, calendar::DAYS - 2] {
                for second in [0, 86_399] {
                    let text = render(day, second, offset);
                    assert!(text.contains("same instant:"));
                }
            }
        }
    }
}
