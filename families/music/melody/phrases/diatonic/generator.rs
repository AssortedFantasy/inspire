use rand::{Rng, rngs::StdRng};

const NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];
const SCALES: [(&str, [u8; 7]); 3] = [
    ("major", [0, 2, 4, 5, 7, 9, 11]),
    ("natural minor", [0, 2, 3, 5, 7, 8, 10]),
    ("Dorian", [0, 2, 3, 5, 7, 9, 10]),
];

fn contour(rng: &mut StdRng) -> [usize; 8] {
    let second = rng.random_range(1..=2);
    let third = second + rng.random_range(1..=2);
    let summit = (third + 1).max(4);
    [0, second, third, summit, 3, 2, 1, 0]
}

pub fn generate(rng: &mut StdRng) -> String {
    let tonic = rng.random_range(60..72);
    let (mode, scale) = SCALES[rng.random_range(0..SCALES.len())];
    let degrees = contour(rng);
    let notes = degrees.map(|d| tonic + scale[d]);
    let labels: Vec<_> = notes
        .iter()
        .map(|&n| format!("{}{}", NAMES[(n % 12) as usize], n / 12 - 1))
        .collect();
    let tempo = rng.random_range(72..=120);
    format!(
        "Original two-bar melodic arch in {} {mode}, 4/4; quarter note = {tempo} BPM.\n{} | {}\nEvery pitch lasts one quarter note. Scale degrees: {:?}; MIDI pitches: {notes:?}. C4 = MIDI 60; pitch labels use sharps in twelve-tone equal temperament.",
        NAMES[(tonic % 12) as usize],
        labels[..4].join(" "),
        labels[4..].join(" "),
        degrees.map(|d| d + 1)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn melodic_arch_returns_to_tonic_without_large_leaps() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..1000 {
            let degrees = contour(&mut rng);
            assert_eq!(degrees[0], 0);
            assert_eq!(degrees[7], 0);
            assert!(degrees[..4].windows(2).all(|w| w[0] < w[1]));
            assert!(degrees[3..].windows(2).all(|w| w[0] > w[1]));
            for (_, scale) in SCALES {
                let pitches = degrees.map(|d| scale[d]);
                assert!(pitches.windows(2).all(|w| w[0].abs_diff(w[1]) <= 4));
            }
        }
    }
}
