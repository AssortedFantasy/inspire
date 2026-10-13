use rand::{Rng, rngs::StdRng};

fn pattern(pulses: usize, steps: usize, rotation: usize) -> Vec<bool> {
    (0..steps)
        .map(|i| (((i + rotation) % steps) * pulses) % steps < pulses)
        .collect()
}

pub fn generate(rng: &mut StdRng) -> String {
    let steps = rng.random_range(5..=24);
    let pulses = rng.random_range(2..steps);
    let rotation = rng.random_range(0..steps);
    let pattern = pattern(pulses, steps, rotation);
    let diagram: String = pattern
        .iter()
        .map(|&on| if on { 'x' } else { '.' })
        .collect();
    let onsets: Vec<_> = pattern
        .iter()
        .enumerate()
        .filter_map(|(i, &on)| on.then_some(i))
        .collect();
    format!(
        "Evenly spaced cyclic rhythm: {pulses} attacks in {steps} equal subdivisions.\n{diagram}\nZero-based onset indices: {onsets:?}. x = attack; . = silence. The pattern repeats cyclically; adjacent inter-onset gaps differ by at most one subdivision."
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_rotation_preserves_count_and_balanced_cyclic_gaps() {
        for steps in 5..=24 {
            for pulses in 2..steps {
                for rotation in 0..steps {
                    let p = pattern(pulses, steps, rotation);
                    let onsets: Vec<_> = p
                        .iter()
                        .enumerate()
                        .filter_map(|(i, &on)| on.then_some(i))
                        .collect();
                    assert_eq!(onsets.len(), pulses);
                    let gaps: Vec<_> = (0..pulses)
                        .map(|i| (onsets[(i + 1) % pulses] + steps - onsets[i]) % steps)
                        .collect();
                    assert!(gaps.iter().max().unwrap() - gaps.iter().min().unwrap() <= 1);
                }
            }
        }
    }
}
