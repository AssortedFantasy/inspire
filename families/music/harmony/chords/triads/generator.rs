use rand::{Rng, rngs::StdRng};

mod spelling {
    include!("../../spelling.rs");
}

const TRIADS: [(&str, [u8; 3]); 4] = [
    ("major", [0, 4, 7]),
    ("minor", [0, 3, 7]),
    ("diminished", [0, 3, 6]),
    ("augmented", [0, 4, 8]),
];

fn voicing(root: u8, intervals: [u8; 3], inversion: usize, open: bool) -> Vec<u8> {
    let mut notes: Vec<_> = (0..3)
        .map(|i| root + intervals[(i + inversion) % 3] + if i + inversion >= 3 { 12 } else { 0 })
        .collect();
    if open {
        notes[1] += 12;
        notes.swap(1, 2);
    }
    notes
}

pub fn generate(rng: &mut StdRng) -> String {
    let root = rng.random_range(48..60);
    let (quality, intervals) = TRIADS[rng.random_range(0..TRIADS.len())];
    let inversion = rng.random_range(0..3);
    let open = rng.random_bool(0.35);
    let tonic = spelling::root(root % 12, &intervals, &[0, 2, 4]);
    let pitches = [0, 1, 2].map(|i| tonic.transpose(intervals[i], i * 2));
    let labels: Vec<_> = voicing(root, intervals, inversion, open)
        .into_iter()
        .map(|note| {
            pitches
                .iter()
                .find(|pitch| pitch.pc() == note % 12)
                .unwrap()
                .at(note)
        })
        .collect();
    format!(
        "{} {quality}, {}{}\n{}",
        tonic.name(),
        ["root position", "first inversion", "second inversion"][inversion],
        if open { ", open voicing" } else { "" },
        labels.join(" – ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voicing_preserves_chord_members_and_inversion() {
        for root in 48..60 {
            for (_, intervals) in TRIADS {
                for inversion in 0..3 {
                    for open in [false, true] {
                        let notes = voicing(root, intervals, inversion, open);
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
}
