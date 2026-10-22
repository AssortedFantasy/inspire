use rand::{Rng, rngs::StdRng};

mod spelling {
    include!("../../spelling.rs");
}

const CHORDS: [(&str, [u8; 4]); 7] = [
    ("major seventh", [0, 4, 7, 11]),
    ("dominant seventh", [0, 4, 7, 10]),
    ("minor seventh", [0, 3, 7, 10]),
    ("half-diminished seventh", [0, 3, 6, 10]),
    ("diminished seventh", [0, 3, 6, 9]),
    ("minor-major seventh", [0, 3, 7, 11]),
    ("augmented major seventh", [0, 4, 8, 11]),
];

fn voicing(root: u8, intervals: [u8; 4], inversion: usize, drop_two: bool) -> [u8; 4] {
    let mut notes = [0, 1, 2, 3]
        .map(|i| root + intervals[(i + inversion) % 4] + if i + inversion >= 4 { 12 } else { 0 });
    if drop_two {
        notes[2] -= 12;
        notes.sort();
    }
    notes
}

pub fn generate(rng: &mut StdRng) -> String {
    let root = rng.random_range(48..60);
    let (quality, intervals) = CHORDS[rng.random_range(0..CHORDS.len())];
    let inversion = rng.random_range(0..4);
    let drop_two = rng.random_bool(0.5);
    let tonic = spelling::root(root % 12, &intervals, &[0, 2, 4, 6]);
    let pitches = [0, 1, 2, 3].map(|i| tonic.transpose(intervals[i], i * 2));
    let labels: Vec<_> = voicing(root, intervals, inversion, drop_two)
        .into_iter()
        .map(|note| {
            pitches
                .iter()
                .find(|pitch| pitch.pc() == note % 12)
                .unwrap()
                .at(note)
        })
        .collect();
    let position = if drop_two {
        "drop-2 voicing"
    } else {
        [
            "root position",
            "first inversion",
            "second inversion",
            "third inversion",
        ][inversion]
    };
    format!(
        "{} {quality}, {position}\n{}",
        tonic.name(),
        labels.join(" – ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voicings_keep_every_chord_tone_once() {
        for root in 48..60 {
            for (_, intervals) in CHORDS {
                for inversion in 0..4 {
                    for drop_two in [false, true] {
                        let notes = voicing(root, intervals, inversion, drop_two);
                        assert!(notes.windows(2).all(|w| w[0] < w[1]));
                        let mut actual = notes.map(|n| (n + 12 - root) % 12);
                        actual.sort();
                        assert_eq!(actual, intervals);
                    }
                }
            }
        }
    }
}
