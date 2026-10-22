use rand::{Rng, rngs::StdRng};

struct Groove {
    kick: Vec<bool>,
    snare: Vec<bool>,
    hat: Vec<bool>,
}

const KICKS: [&[usize]; 8] = [
    &[0, 8],
    &[0, 6, 8],
    &[0, 7, 10],
    &[0, 8, 14],
    &[0, 3, 8, 10],
    &[0, 6, 10, 15],
    &[0, 3, 6, 8, 11],
    &[0, 5, 8, 14],
];

fn groove(rng: &mut StdRng) -> Groove {
    let mut kick = vec![false; 32];
    let mut snare = vec![false; 32];
    let mut hat = vec![false; 32];
    let kicks = KICKS[rng.random_range(0..KICKS.len())];
    let backbeats: &'static [usize] = if rng.random_bool(0.25) {
        &[8]
    } else {
        &[4, 12]
    };
    let hat_spacing = if rng.random_bool(0.15) { 1 } else { 2 };
    for bar in 0..2 {
        for &step in kicks {
            kick[bar * 16 + step] = true;
        }
        for &step in backbeats {
            snare[bar * 16 + step] = true;
        }
        for step in (0..16).step_by(hat_spacing) {
            hat[bar * 16 + step] = step.is_multiple_of(4) || rng.random_bool(0.9);
        }
    }
    if rng.random_bool(0.7) {
        let step = rng.random_range(25..32);
        kick[step] = !kick[step];
    }
    if rng.random_bool(0.5) {
        snare[if rng.random_bool(0.5) { 29 } else { 31 }] = true;
    }
    if rng.random_bool(0.6) {
        hat[30] = true;
        hat[31] = true;
    }
    Groove { kick, snare, hat }
}

fn grid(part: &[bool]) -> String {
    part.chunks(16)
        .map(|bar| {
            bar.chunks(4)
                .map(|beat| {
                    beat.iter()
                        .map(|&hit| if hit { 'x' } else { '.' })
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join(" | ")
}

pub fn generate(rng: &mut StdRng) -> String {
    let tempo = rng.random_range(68..=138);
    let groove = groove(rng);
    format!(
        "4/4 · 1/4 = {tempo} · 1/16 grid\nKick   |: {} :|\nSnare  |: {} :|\nHi-hat |: {} :|",
        grid(&groove.kick),
        grid(&groove.snare),
        grid(&groove.hat),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn two_bar_parts_keep_the_downbeat_and_backbeat() {
        let mut rng = StdRng::seed_from_u64(24);
        for _ in 0..1000 {
            let groove = groove(&mut rng);
            for part in [&groove.kick, &groove.snare, &groove.hat] {
                assert_eq!(part.len(), 32);
                assert!(part[..16].iter().any(|&hit| hit));
                assert!(part[16..].iter().any(|&hit| hit));
            }
            assert!(groove.kick[0] && groove.kick[16]);
            assert!(
                groove.snare[8] && groove.snare[24]
                    || [4, 12, 20, 28].iter().all(|&step| groove.snare[step])
            );
        }
    }
}
