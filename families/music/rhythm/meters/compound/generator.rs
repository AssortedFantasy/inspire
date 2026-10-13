use rand::{Rng, rngs::StdRng};

const CELLS: [&[u8]; 4] = [&[3], &[1, 1, 1], &[2, 1], &[1, 2]];

fn bar(rng: &mut StdRng, beats: usize) -> Vec<Vec<(u8, bool)>> {
    (0..beats)
        .map(|beat| {
            CELLS[rng.random_range(0..CELLS.len())]
                .iter()
                .enumerate()
                .map(|(i, &duration)| (duration, beat == 0 && i == 0 || rng.random_bool(0.9)))
                .collect()
        })
        .collect()
}

pub fn generate(rng: &mut StdRng) -> String {
    let beats = rng.random_range(2..=4);
    let tempo = rng.random_range(48..=112);
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
        "Four-bar rhythm in {}/8; dotted quarter note = {tempo} BPM.\n{}\nLegend: n = note, r = rest; numbers are eighth-note units; / separates dotted-quarter beats and | separates bars.",
        beats * 3,
        bars.join(" | ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn compound_beats_and_bars_are_filled_exactly() {
        assert!(CELLS.iter().all(|cell| cell.iter().sum::<u8>() == 3));
        let mut rng = StdRng::seed_from_u64(9);
        for beats in 2..=4 {
            for _ in 0..500 {
                let groups = bar(&mut rng, beats);
                assert_eq!(
                    groups
                        .iter()
                        .flatten()
                        .map(|(d, _)| *d as usize)
                        .sum::<usize>(),
                    3 * beats
                );
                assert!(groups[0][0].1);
            }
        }
    }
}
