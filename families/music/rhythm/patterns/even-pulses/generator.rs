use rand::{Rng, rngs::StdRng};

fn pattern(pulses: usize, steps: usize, rotation: usize) -> Vec<bool> {
    (0..steps)
        .map(|i| (((i + rotation) % steps) * pulses) % steps < pulses)
        .collect()
}

pub fn generate(rng: &mut StdRng) -> String {
    let steps = [8, 12, 16, 16, 24, 5, 7, 9, 11, 13][rng.random_range(0..10)];
    let pulses = rng.random_range((steps / 4).max(2)..=steps / 2 + 1);
    let rotation = rng.random_range(0..steps);
    let pattern = pattern(pulses, steps, rotation);
    let group = if steps.is_multiple_of(4) {
        4
    } else if steps.is_multiple_of(3) {
        3
    } else {
        steps
    };
    let diagram = pattern
        .chunks(group)
        .map(|chunk| {
            chunk
                .iter()
                .map(|&on| if on { 'x' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ");
    format!("{pulses} hits / {steps} steps\n|: {diagram} :|")
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
