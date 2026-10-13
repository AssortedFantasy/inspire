use rand::{Rng, rngs::StdRng};

const NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];
const TRIADS: [(&str, [u8; 3]); 4] = [
    ("major", [0, 4, 7]),
    ("minor", [0, 3, 7]),
    ("diminished", [0, 3, 6]),
    ("augmented", [0, 4, 8]),
];

fn voicing(root: u8, intervals: [u8; 3], inversion: usize) -> Vec<u8> {
    (0..3)
        .map(|i| root + intervals[(i + inversion) % 3] + if i + inversion >= 3 { 12 } else { 0 })
        .collect()
}

pub fn generate(rng: &mut StdRng) -> String {
    let root = rng.random_range(48..60);
    let (quality, intervals) = TRIADS[rng.random_range(0..4)];
    let inversion = rng.random_range(0..3);
    let notes = voicing(root, intervals, inversion);
    let labels: Vec<_> = notes
        .iter()
        .map(|&m| format!("{}{}", NAMES[(m % 12) as usize], m / 12 - 1))
        .collect();
    format!(
        "{} {quality} triad, {}\nVoicing, low to high: {}.\nMIDI note numbers: {notes:?}; C4 = 60. Twelve-tone equal temperament; pitch labels use sharps.",
        NAMES[(root % 12) as usize],
        ["root position", "first inversion", "second inversion"][inversion],
        labels.join(" – ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn inversion_keeps_chord_members_and_moves_the_bass() {
        for root in 48..60 {
            for (_, intervals) in TRIADS {
                for inversion in 0..3 {
                    let notes = voicing(root, intervals, inversion);
                    assert!(notes.windows(2).all(|w| w[0] < w[1]));
                    assert_eq!((notes[0] - root) % 12, intervals[inversion]);
                    let mut actual: Vec<_> = notes.iter().map(|n| (n - root) % 12).collect();
                    actual.sort();
                    assert_eq!(actual, intervals);
                }
            }
        }
    }
}
