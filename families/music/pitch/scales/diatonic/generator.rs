use rand::{Rng, rngs::StdRng};

mod spelling {
    include!("../../../harmony/spelling.rs");
}

const MODES: [&str; 7] = [
    "major",
    "Dorian",
    "Phrygian",
    "Lydian",
    "Mixolydian",
    "natural minor",
    "Locrian",
];
const MAJOR: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];

fn intervals(mode: usize) -> Vec<u8> {
    (0..7)
        .map(|i| (MAJOR[(i + mode) % 7] + 12 - MAJOR[mode]) % 12)
        .collect()
}

pub fn generate(rng: &mut StdRng) -> String {
    let mode = rng.random_range(0..7);
    let steps = intervals(mode);
    let tonic = spelling::root(rng.random_range(0..12), &steps, &[0, 1, 2, 3, 4, 5, 6]);
    let notes: Vec<_> = steps
        .iter()
        .enumerate()
        .map(|(degree, &interval)| tonic.transpose(interval, degree).name())
        .collect();
    format!("{} {}\n{}", tonic.name(), MODES[mode], notes.join(" – "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modes_rotate_the_diatonic_step_pattern() {
        for mode in 0..7 {
            let steps = intervals(mode);
            assert_eq!(steps[0], 0);
            assert!(steps.windows(2).all(|w| w[0] < w[1]));
            let gaps: Vec<_> = (0..7)
                .map(|i| (steps[(i + 1) % 7] + 12 - steps[i]) % 12)
                .collect();
            assert_eq!(gaps.iter().filter(|&&g| g == 1).count(), 2);
            assert_eq!(gaps.iter().filter(|&&g| g == 2).count(), 5);
        }
    }
}
