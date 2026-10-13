use rand::{Rng, rngs::StdRng};

const NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];
const MAJOR: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];
const ROMAN: [&str; 7] = ["I", "ii", "iii", "IV", "V", "vi", "vii°"];
const PATTERNS: [(&str, &[usize]); 7] = [
    ("authentic close", &[0, 3, 4, 0]),
    ("authentic close", &[0, 1, 4, 0]),
    ("plagal close", &[0, 5, 3, 0]),
    ("deceptive close", &[0, 3, 4, 5]),
    ("half close", &[0, 5, 1, 4]),
    ("authentic close", &[0, 5, 1, 4, 0]),
    ("half close", &[0, 3, 1, 4]),
];

fn chord(tonic: u8, degree: usize) -> [u8; 3] {
    [0, 2, 4].map(|offset| (tonic + MAJOR[(degree + offset) % 7]) % 12)
}

pub fn generate(rng: &mut StdRng) -> String {
    let tonic = rng.random_range(0..12);
    let (kind, pattern) = PATTERNS[rng.random_range(0..PATTERNS.len())];
    let romans: Vec<_> = pattern.iter().map(|&d| ROMAN[d]).collect();
    let chords: Vec<_> = pattern
        .iter()
        .map(|&d| {
            let labels: Vec<_> = chord(tonic, d)
                .into_iter()
                .map(|pc| NAMES[pc as usize])
                .collect();
            format!("{} [{}]", ROMAN[d], labels.join(", "))
        })
        .collect();
    format!(
        "{} major: {kind}\n{}\n{}. Each chord lasts one bar. Brackets show root, third and fifth as pitch classes, with sharp labels in twelve-tone equal temperament; voicing is unspecified.",
        NAMES[tonic as usize],
        romans.join(" – "),
        chords.join(" → ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn diatonic_triad_qualities_and_cadential_endpoints_are_correct() {
        let qualities = [[4, 7], [3, 7], [3, 7], [4, 7], [4, 7], [3, 7], [3, 6]];
        for tonic in 0..12 {
            for (degree, expected) in qualities.iter().enumerate() {
                let c = chord(tonic, degree);
                assert_eq!(
                    [(c[1] + 12 - c[0]) % 12, (c[2] + 12 - c[0]) % 12],
                    *expected
                );
            }
        }
        for (name, p) in PATTERNS {
            let final_pair = &p[p.len() - 2..];
            match name {
                "authentic close" => assert_eq!(final_pair, [4, 0]),
                "plagal close" => assert_eq!(final_pair, [3, 0]),
                "deceptive close" => assert_eq!(final_pair, [4, 5]),
                _ => assert_eq!(*p.last().unwrap(), 4),
            }
        }
    }
}
