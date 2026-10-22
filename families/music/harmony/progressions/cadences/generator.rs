use rand::{Rng, rngs::StdRng};

mod spelling {
    include!("../../spelling.rs");
}

const MAJOR: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];
const MINOR: [u8; 7] = [0, 2, 3, 5, 7, 8, 10];
const SUFFIXES: [&str; 7] = ["", "m", "dim", "7", "m7", "maj7", "m7b5"];
type Chord = (usize, i8, usize);
type Pattern = (bool, &'static str, &'static [Chord]);
const PATTERNS: [Pattern; 16] = [
    (
        false,
        "authentic cadence",
        &[(0, 0, 0), (3, 0, 0), (4, 0, 3), (0, 0, 0)],
    ),
    (
        false,
        "authentic cadence",
        &[(0, 0, 0), (1, 0, 4), (4, 0, 3), (0, 0, 0)],
    ),
    (
        false,
        "plagal cadence",
        &[(0, 0, 0), (5, 0, 1), (3, 0, 0), (0, 0, 0)],
    ),
    (
        false,
        "deceptive cadence",
        &[(0, 0, 0), (3, 0, 0), (4, 0, 3), (5, 0, 1)],
    ),
    (
        false,
        "half cadence",
        &[(0, 0, 0), (5, 0, 1), (1, 0, 1), (4, 0, 0)],
    ),
    (
        false,
        "minor plagal cadence",
        &[(0, 0, 0), (3, 0, 0), (3, 0, 1), (0, 0, 0)],
    ),
    (
        false,
        "secondary dominant",
        &[(0, 0, 0), (5, 0, 3), (1, 0, 4), (4, 0, 3), (0, 0, 0)],
    ),
    (
        false,
        "secondary dominant",
        &[(0, 0, 0), (1, 0, 3), (4, 0, 3), (0, 0, 0)],
    ),
    (
        false,
        "backdoor cadence",
        &[(0, 0, 0), (3, 0, 4), (6, -1, 3), (0, 0, 5)],
    ),
    (
        true,
        "authentic cadence",
        &[(0, 0, 1), (3, 0, 1), (4, 0, 3), (0, 0, 1)],
    ),
    (
        true,
        "authentic cadence",
        &[(0, 0, 1), (1, 0, 6), (4, 0, 3), (0, 0, 1)],
    ),
    (
        true,
        "deceptive cadence",
        &[(0, 0, 1), (3, 0, 1), (4, 0, 3), (5, 0, 0)],
    ),
    (
        true,
        "Neapolitan cadence",
        &[(0, 0, 1), (1, -1, 0), (4, 0, 3), (0, 0, 1)],
    ),
    (
        true,
        "Andalusian cadence",
        &[(0, 0, 1), (6, 0, 0), (5, 0, 0), (4, 0, 0)],
    ),
    (
        true,
        "plagal cadence",
        &[(0, 0, 1), (5, 0, 0), (3, 0, 1), (0, 0, 1)],
    ),
    (
        true,
        "Picardy third",
        &[(0, 0, 1), (3, 0, 1), (4, 0, 3), (0, 0, 0)],
    ),
];

fn chord_root(tonic: spelling::Pitch, scale: [u8; 7], chord: Chord) -> spelling::Pitch {
    let (degree, alteration, _) = chord;
    tonic.transpose(
        (scale[degree] as i8 + alteration).rem_euclid(12) as u8,
        degree,
    )
}

pub fn generate(rng: &mut StdRng) -> String {
    let (minor, name, pattern) = PATTERNS[rng.random_range(0..PATTERNS.len())];
    let scale = if minor { MINOR } else { MAJOR };
    let tonic = spelling::root(rng.random_range(0..12), &scale, &[0, 1, 2, 3, 4, 5, 6]);
    let chords: Vec<_> = pattern
        .iter()
        .map(|&chord| {
            format!(
                "{}{}",
                chord_root(tonic, scale, chord).name(),
                SUFFIXES[chord.2]
            )
        })
        .collect();
    format!(
        "{} {} — {name}\n| {} |",
        tonic.name(),
        if minor { "minor" } else { "major" },
        chords.join(" | ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cadential_roots_resolve_to_the_named_degrees() {
        for (minor, name, pattern) in PATTERNS {
            let end = &pattern[pattern.len() - 2..];
            match name {
                "authentic cadence" | "Picardy third" | "Neapolitan cadence" => {
                    assert_eq!(end[0].0, 4);
                    assert_eq!(end[1].0, 0);
                }
                "plagal cadence" | "minor plagal cadence" => {
                    assert_eq!(end[0].0, 3);
                    assert_eq!(end[1].0, 0);
                }
                "deceptive cadence" => {
                    assert_eq!(end[0].0, 4);
                    assert_eq!(end[1].0, 5);
                }
                "half cadence" | "Andalusian cadence" => assert_eq!(end[1].0, 4),
                "backdoor cadence" => {
                    assert_eq!(end[0], (6, -1, 3));
                    assert_eq!(end[1].0, 0);
                }
                _ => assert_eq!(end[1].0, 0),
            }
            let scale = if minor { MINOR } else { MAJOR };
            for pc in 0..12 {
                let tonic = spelling::root(pc, &scale, &[0, 1, 2, 3, 4, 5, 6]);
                for &chord in pattern {
                    let expected = (pc as i8 + scale[chord.0] as i8 + chord.1).rem_euclid(12) as u8;
                    assert_eq!(chord_root(tonic, scale, chord).pc(), expected);
                }
            }
        }
    }
}
