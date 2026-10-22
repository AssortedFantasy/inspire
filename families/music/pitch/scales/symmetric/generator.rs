use rand::{Rng, rngs::StdRng};

mod spelling {
    include!("../../../harmony/spelling.rs");
}

const NAMES: [&str; 12] = [
    "C", "Db", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
];
const SCALES: [(&str, &[u8], &[usize]); 4] = [
    ("whole-tone", &[0, 2, 4, 6, 8, 10], &[0, 1, 2, 3, 4, 6]),
    (
        "half-whole diminished",
        &[0, 1, 3, 4, 6, 7, 9, 10],
        &[0, 1, 2, 2, 4, 4, 5, 6],
    ),
    (
        "whole-half diminished",
        &[0, 2, 3, 5, 6, 8, 9, 11],
        &[0, 1, 2, 3, 4, 5, 5, 6],
    ),
    (
        "augmented hexatonic",
        &[0, 3, 4, 7, 8, 11],
        &[0, 2, 2, 4, 4, 6],
    ),
];

pub fn generate(rng: &mut StdRng) -> String {
    let root: u8 = rng.random_range(0..12);
    let (name, intervals, degrees) = SCALES[rng.random_range(0..SCALES.len())];
    let tonic = spelling::root(root, &[0, 2, 4, 5, 7, 9, 11], &[0, 1, 2, 3, 4, 5, 6]);
    let notes: Vec<_> = intervals
        .iter()
        .zip(degrees)
        .map(|(&interval, &degree)| {
            let pitch = tonic.transpose(interval, degree);
            let name = pitch.name();
            // Symmetric collections cross diatonic spellings; keep them readable.
            if name.len() > 2 || matches!(name.as_str(), "E#" | "B#" | "Cb" | "Fb") {
                NAMES[pitch.pc() as usize].to_owned()
            } else {
                name
            }
        })
        .collect();
    format!("{} {name}\n{}", tonic.name(), notes.join(" – "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symmetric_scales_have_periodic_interval_cycles() {
        for (_, scale, degrees) in SCALES {
            assert_eq!(scale.len(), degrees.len());
            let gaps: Vec<_> = (0..scale.len())
                .map(|i| (scale[(i + 1) % scale.len()] + 12 - scale[i]) % 12)
                .collect();
            assert_eq!(gaps.iter().sum::<u8>(), 12);
            assert!(gaps.iter().enumerate().all(|(i, gap)| *gap == gaps[i % 2]));
        }
    }
}
