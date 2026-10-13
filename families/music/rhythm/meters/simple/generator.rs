use rand::{Rng, rngs::StdRng};

fn bar(rng: &mut StdRng, beats: usize) -> Vec<Vec<(u8, bool)>> {
    (0..beats)
        .map(|beat| {
            let durations: &[u8] = if rng.random_bool(0.55) { &[2] } else { &[1, 1] };
            durations
                .iter()
                .enumerate()
                .map(|(i, &duration)| (duration, beat == 0 && i == 0 || rng.random_bool(0.85)))
                .collect()
        })
        .collect()
}

pub fn generate(rng: &mut StdRng) -> String {
    let beats = rng.random_range(2..=4);
    let tempo = rng.random_range(64..=144);
    let bars: Vec<_> = (0..4)
        .map(|_| {
            bar(rng, beats)
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
        "Four-bar rhythm in {beats}/4; quarter note = {tempo} BPM.\n{}\nLegend: n = note, r = rest; numbers are eighth-note units; / separates quarter-note beats and | separates bars.",
        bars.join(" | ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn simple_beats_and_bars_are_filled_exactly() {
        let mut rng = StdRng::seed_from_u64(8);
        for beats in 2..=4 {
            for _ in 0..500 {
                let groups = bar(&mut rng, beats);
                assert_eq!(groups.len(), beats);
                assert!(
                    groups
                        .iter()
                        .all(|g| g.iter().map(|(d, _)| *d).sum::<u8>() == 2)
                );
                assert!(groups[0][0].1);
            }
        }
    }
}
