use rand::{Rng, rngs::StdRng};

const NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];
const SCALES: [(&str, &[u8]); 3] = [
    ("whole-tone", &[0, 2, 4, 6, 8, 10]),
    (
        "octatonic, half-step / whole-step",
        &[0, 1, 3, 4, 6, 7, 9, 10],
    ),
    (
        "octatonic, whole-step / half-step",
        &[0, 2, 3, 5, 6, 8, 9, 11],
    ),
];

pub fn generate(rng: &mut StdRng) -> String {
    let root: u8 = rng.random_range(0..12);
    let (name, intervals) = SCALES[rng.random_range(0..SCALES.len())];
    let notes: Vec<_> = intervals
        .iter()
        .map(|i| NAMES[((root + i) % 12) as usize])
        .collect();
    format!(
        "{} {name} scale\nAscending pitch classes: {}.\nSemitones above starting pitch: {intervals:?}. Twelve-tone equal temperament; pitch-class labels use sharps.",
        NAMES[root as usize],
        notes.join(" – ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn symmetric_scales_have_periodic_interval_cycles() {
        for (_, scale) in SCALES {
            let gaps: Vec<_> = (0..scale.len())
                .map(|i| (scale[(i + 1) % scale.len()] + 12 - scale[i]) % 12)
                .collect();
            assert_eq!(gaps.iter().sum::<u8>(), 12);
            assert!(gaps.iter().enumerate().all(|(i, gap)| *gap == gaps[i % 2]));
        }
    }
}
