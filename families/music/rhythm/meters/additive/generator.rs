use rand::{Rng, rngs::StdRng};

const GROUPINGS: [&[u8]; 10] = [
    &[2, 3],
    &[3, 2],
    &[2, 2, 3],
    &[2, 3, 2],
    &[3, 2, 2],
    &[3, 3, 2],
    &[2, 3, 3],
    &[3, 2, 2, 3],
    &[2, 2, 3, 3],
    &[3, 3, 3, 2],
];

fn bar(rng: &mut StdRng, groups: &[u8]) -> Vec<Vec<(u8, bool)>> {
    groups
        .iter()
        .map(|&units| {
            if rng.random_bool(0.25) {
                vec![(units, true)]
            } else {
                (0..units)
                    .map(|i| (1, i == 0 || rng.random_bool(0.8)))
                    .collect()
            }
        })
        .collect()
}

pub fn generate(rng: &mut StdRng) -> String {
    let groups = GROUPINGS[rng.random_range(0..GROUPINGS.len())];
    let numerator: u8 = groups.iter().sum();
    let tempo = rng.random_range(160..=280);
    let grouping = groups
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join("+");
    let bars: Vec<_> = (0..4)
        .map(|_| {
            bar(rng, groups)
                .iter()
                .map(|group| {
                    group
                        .iter()
                        .map(|&(duration, note)| {
                            format!("{}{}", if note { "n" } else { "r" }, duration)
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect::<Vec<_>>()
                .join(" / ")
        })
        .collect();
    format!(
        "Four-bar additive rhythm in {numerator}/8, grouped {grouping}; eighth note = {tempo} BPM.\n{}\nLegend: n = note, r = rest; numbers are eighth-note units; / separates accented groups and | separates bars.",
        bars.join(" | ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn additive_groups_preserve_the_meter_and_have_attacks() {
        let mut rng = StdRng::seed_from_u64(10);
        for groups in GROUPINGS {
            for _ in 0..500 {
                let result = bar(&mut rng, groups);
                for (&expected, actual) in groups.iter().zip(result) {
                    assert_eq!(actual.iter().map(|(d, _)| *d).sum::<u8>(), expected);
                    assert!(actual[0].1);
                }
            }
        }
    }
}
