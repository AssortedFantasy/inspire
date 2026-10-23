use rand::{Rng, rngs::StdRng, seq::SliceRandom};
use std::fmt::Write;

fn step(cells: &[u8], rule: u8, ring: bool) -> Vec<u8> {
    let n = cells.len();
    (0..n)
        .map(|i| {
            let left = if i > 0 {
                cells[i - 1]
            } else if ring {
                cells[n - 1]
            } else {
                0
            };
            let right = if i + 1 < n {
                cells[i + 1]
            } else if ring {
                cells[0]
            } else {
                0
            };
            (rule >> (4 * left + 2 * cells[i] + right)) & 1
        })
        .collect()
}

fn picture(cells: &[u8]) -> String {
    cells
        .iter()
        .map(|&x| if x == 0 { '.' } else { '#' })
        .collect()
}

fn initial(rng: &mut StdRng, n: usize) -> Vec<u8> {
    match rng.random_range(0..4) {
        0 => {
            let mut cells = vec![0; n];
            cells[n / 2] = 1;
            if rng.random_bool(0.5) {
                cells[n / 2 + rng.random_range(2..=6)] = 1;
            }
            cells
        }
        1 => {
            let period = rng.random_range(2..=7);
            let motif: Vec<u8> = (0..period).map(|_| rng.random_range(0..=1)).collect();
            let mut cells: Vec<_> = (0..n).map(|i| motif[i % period]).collect();
            cells[n / 2] ^= 1;
            cells
        }
        2 => {
            let edge = rng.random_range(n / 3..2 * n / 3);
            (0..n).map(|i| u8::from(i >= edge)).collect()
        }
        _ => (0..n).map(|_| u8::from(rng.random_bool(0.5))).collect(),
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..8);
    let rules: &[u8] = match regime {
        0 => &[8, 128],
        1 => &[51, 170],
        2 => &[90, 150],
        3 | 4 => &[30, 45],
        5 | 6 => &[54, 110],
        _ => &[184, 232],
    };
    let rule = rules[rng.random_range(0..rules.len())];
    let n = if regime == 7 {
        40
    } else {
        rng.random_range(33..=47)
    };
    let ring = regime == 7 || rng.random_bool(0.6);
    let mut cells = initial(rng, n);
    if regime == 7 {
        let count = (n as i32 / 2 + rng.random_range(-2..=2)) as usize;
        cells = (0..n).map(|i| u8::from(i < count)).collect();
        cells.shuffle(rng);
    }
    let paired = (2..7).contains(&regime) && rng.random_bool(0.25);
    let mut other = cells.clone();
    if paired {
        other[n / 2] ^= 1;
    }
    let boundary = if ring { "ring" } else { "zero exterior" };
    let mut text = format!("Rule {rule}, {n} cells, {boundary}");
    if paired {
        text.push_str("\nSingle-cell perturbation at the center\n t  Original");
        write!(text, "{}Perturbed  Δ", " ".repeat(n - 6)).unwrap();
    }
    let mut history = Vec::new();
    let mut recurrence = None;
    for t in 0..16 {
        if recurrence.is_none() {
            recurrence = history
                .iter()
                .position(|old| old == &cells)
                .map(|old| (old, t));
        }
        history.push(cells.clone());
        write!(text, "\n{t:>2}  {}", picture(&cells)).unwrap();
        if paired {
            let difference = cells.iter().zip(&other).filter(|(a, b)| a != b).count();
            write!(text, "  {}  {difference}", picture(&other)).unwrap();
            other = step(&other, rule, ring);
        }
        if !paired && recurrence.is_some() {
            break;
        }
        cells = step(&cells, rule, ring);
    }
    if !paired && let Some((start, end)) = recurrence {
        write!(text, "\nPeriod {} from t={start}", end - start).unwrap();
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wolfram_neighborhood_numbering() {
        for rule in 0..=255 {
            for neighborhood in 0..8 {
                let cells = [neighborhood >> 2, (neighborhood >> 1) & 1, neighborhood & 1];
                assert_eq!(step(&cells, rule, false)[1], (rule >> neighborhood) & 1);
            }
        }
    }

    #[test]
    fn additive_rules_match_their_xor_laws() {
        for state in 0u16..512 {
            let cells: Vec<_> = (0..9).map(|i| ((state >> i) & 1) as u8).collect();
            for (rule, include_center) in [(90, false), (150, true)] {
                let next = step(&cells, rule, true);
                for i in 0..9 {
                    assert_eq!(
                        next[i],
                        cells[(i + 8) % 9]
                            ^ cells[(i + 1) % 9]
                            ^ if include_center { cells[i] } else { 0 }
                    );
                }
            }
        }
    }

    #[test]
    fn traffic_conserves_particles_and_moves_isolated_cars_right() {
        for state in 0u16..256 {
            let cells: Vec<_> = (0..8).map(|i| ((state >> i) & 1) as u8).collect();
            assert_eq!(
                cells.iter().sum::<u8>(),
                step(&cells, 184, true).iter().sum::<u8>()
            );
        }
        assert_eq!(step(&[0, 1, 0, 0, 0], 184, true), [0, 0, 1, 0, 0]);
    }

    #[test]
    fn complement_rule_is_period_two_and_zero_boundary_absorbs_shift() {
        let cells = [1, 0, 1, 1, 0];
        assert_eq!(step(&step(&cells, 51, true), 51, true), cells);
        let mut shifted = cells.to_vec();
        for _ in 0..cells.len() {
            shifted = step(&shifted, 170, false);
        }
        assert_eq!(shifted, [0; 5]);
    }
}
