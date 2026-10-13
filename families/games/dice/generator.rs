use rand::{Rng, rngs::StdRng};

fn roll(rng: &mut StdRng, count: usize, sides: u16) -> Vec<u16> {
    (0..count).map(|_| rng.random_range(1..=sides)).collect()
}

fn list(values: &[u16]) -> String {
    values
        .iter()
        .map(u16::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

fn kept(values: &[u16], count: usize, high: bool) -> Vec<u16> {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    if high {
        sorted.reverse();
    }
    sorted.truncate(count);
    sorted
}

pub fn generate(rng: &mut StdRng) -> String {
    match rng.random_range(0..7) {
        0 => {
            let sides = [4, 6, 8, 10, 12, 20, 100][rng.random_range(0..7)];
            let count = rng.random_range(1..=6);
            let modifier: i32 = rng.random_range(-3..=5);
            let values = roll(rng, count, sides);
            let total = values.iter().map(|&v| i32::from(v)).sum::<i32>() + modifier;
            let suffix = if modifier == 0 {
                String::new()
            } else {
                format!("{modifier:+}")
            };
            format!("{count}d{sides}{suffix}: [{}] => {total}", list(&values))
        }
        1 => {
            let sides = [6, 8, 10, 12, 20][rng.random_range(0..5)];
            let count = rng.random_range(3..=7);
            let keep = rng.random_range(1..count);
            let high = rng.random();
            let values = roll(rng, count, sides);
            let selected = kept(&values, keep, high);
            format!(
                "{count}d{sides}, keep {keep} {}: [{}] => [{}] = {}",
                if high { "highest" } else { "lowest" },
                list(&values),
                list(&selected),
                selected.iter().sum::<u16>()
            )
        }
        2 => {
            let count = rng.random_range(3..=12);
            let sides = [6, 10][rng.random_range(0..2)];
            let target = rng.random_range((sides / 2 + 1)..=sides);
            let values = roll(rng, count, sides);
            let successes = values.iter().filter(|&&v| v >= target).count();
            format!(
                "{count}d{sides} pool; each die >= {target} is one success: [{}] => {successes} successes",
                list(&values)
            )
        }
        3 => {
            let values: Vec<i32> = (0..4).map(|_| rng.random_range(-1..=1)).collect();
            let symbols = values
                .iter()
                .map(|v| match v {
                    -1 => "-",
                    0 => "0",
                    _ => "+",
                })
                .collect::<Vec<_>>()
                .join(" ");
            format!(
                "Four balanced dice (-, 0, + each equally likely): [{symbols}] => {:+}",
                values.iter().sum::<i32>()
            )
        }
        4 => {
            let tens: u16 = rng.random_range(0..10) * 10;
            let units: u16 = rng.random_range(0..10);
            let total = percentile(tens, units);
            format!("Percentile dice (00 + 0 means 100): [{tens:02}, {units}] => {total}")
        }
        5 => {
            let a_count = rng.random_range(2..=6);
            let b_count = rng.random_range(2..=6);
            let a = roll(rng, a_count, 6);
            let b = roll(rng, b_count, 6);
            let a_total = a.iter().sum::<u16>();
            let b_total = b.iter().sum::<u16>();
            let result = match a_total.cmp(&b_total) {
                std::cmp::Ordering::Greater => "A wins",
                std::cmp::Ordering::Less => "B wins",
                std::cmp::Ordering::Equal => "tie",
            };
            format!(
                "Opposed sums: A {a_count}d6 [{}] = {a_total}; B {b_count}d6 [{}] = {b_total}; {result}",
                list(&a),
                list(&b)
            )
        }
        _ => {
            let sides = [4, 6, 8, 10, 12, 20, 100][rng.random_range(0..7)];
            let value = rng.random_range(1..=sides);
            format!("1d{sides}: [{value}] => {value}")
        }
    }
}

fn percentile(tens: u16, units: u16) -> u16 {
    if tens + units == 0 { 100 } else { tens + units }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn percentile_maps_all_pairs_bijectively() {
        let mut values = Vec::new();
        for tens in (0..100).step_by(10) {
            for units in 0..10 {
                values.push(percentile(tens, units));
            }
        }
        values.sort_unstable();
        assert_eq!(values, (1..=100).collect::<Vec<_>>());
    }

    #[test]
    fn keeps_extrema_including_ties() {
        let values = [2, 6, 2, 5, 6];
        assert_eq!(kept(&values, 3, true), [6, 6, 5]);
        assert_eq!(kept(&values, 3, false), [2, 2, 5]);
    }

    #[test]
    fn rolls_stay_in_range_and_outputs_reproduce() {
        let mut rng = StdRng::seed_from_u64(31);
        for sides in [4, 6, 8, 10, 12, 20, 100] {
            let values = roll(&mut rng, 1000, sides);
            assert!(values.iter().all(|v| (1..=sides).contains(v)));
            assert!(values.contains(&1) && values.contains(&sides));
        }
        for seed in 0..1000 {
            let text = generate(&mut StdRng::seed_from_u64(seed));
            assert_eq!(text, generate(&mut StdRng::seed_from_u64(seed)));
            assert!(!text.is_empty() && !text.ends_with('\n'));
        }
    }
}
