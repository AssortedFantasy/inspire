use rand::{Rng, rngs::StdRng};

const NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];
const CHORDS: [(&str, [u8; 4]); 6] = [
    ("major seventh", [0, 4, 7, 11]),
    ("dominant seventh", [0, 4, 7, 10]),
    ("minor seventh", [0, 3, 7, 10]),
    ("half-diminished seventh", [0, 3, 6, 10]),
    ("diminished seventh", [0, 3, 6, 9]),
    ("minor-major seventh", [0, 3, 7, 11]),
];

pub fn generate(rng: &mut StdRng) -> String {
    let root: u8 = rng.random_range(0..12);
    let (quality, intervals) = CHORDS[rng.random_range(0..CHORDS.len())];
    let notes: Vec<_> = intervals
        .iter()
        .map(|i| NAMES[((root + i) % 12) as usize])
        .collect();
    format!(
        "{} {quality} chord\nPitch classes in root order: {}.\nSemitones above root: {intervals:?}. Twelve-tone equal temperament; pitch-class labels use sharps.",
        NAMES[root as usize],
        notes.join(" – ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn seventh_chord_quality_definitions_are_distinct() {
        for (index, (_, chord)) in CHORDS.iter().enumerate() {
            assert_eq!(chord[0], 0);
            assert!(chord.windows(2).all(|w| w[0] < w[1]));
            assert!(!CHORDS[..index].iter().any(|(_, other)| other == chord));
        }
        assert_eq!(CHORDS[1].1, [0, 4, 7, 10]);
        assert_eq!(CHORDS[3].1, [0, 3, 6, 10]);
    }
}
