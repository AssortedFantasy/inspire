use rand::{Rng, rngs::StdRng};

type Beat = Vec<(u8, bool)>;
type Bar = Vec<Beat>;

const CELLS: [&[u8]; 8] = [
    &[4],
    &[2, 2],
    &[3, 1],
    &[1, 3],
    &[2, 1, 1],
    &[1, 1, 2],
    &[1, 2, 1],
    &[1, 1, 1, 1],
];
const FEELS: [[usize; 5]; 3] = [[0, 1, 1, 4, 5], [0, 1, 2, 2, 3], [1, 4, 5, 6, 7]];

fn beat(rng: &mut StdRng, feel: &[usize]) -> Beat {
    CELLS[feel[rng.random_range(0..feel.len())]]
        .iter()
        .map(|&duration| (duration, rng.random_bool(0.85)))
        .collect()
}

fn phrase(rng: &mut StdRng, beats: usize) -> Vec<Bar> {
    let feel = FEELS[rng.random_range(0..FEELS.len())];
    let mut motif: Bar = (0..beats).map(|_| beat(rng, &feel)).collect();
    motif[0][0].1 = true;
    let mut answer = motif.clone();
    let changed = rng.random_range(0..beats);
    answer[changed] = beat(rng, &feel);
    answer[0][0].1 = true;
    if answer[changed] == motif[changed] {
        answer[changed] = if motif[changed].len() == 1 {
            vec![(2, true), (2, true)]
        } else {
            vec![(4, true)]
        };
    }
    let mut ending = answer.clone();
    ending[beats - 1] = vec![(4, true)];
    vec![motif.clone(), answer, motif, ending]
}

fn notation(bar: &Bar) -> String {
    bar.iter()
        .map(|beat| {
            beat.iter()
                .map(|&(duration, sounding)| {
                    let note = if sounding { 'C' } else { 'z' };
                    if duration == 1 {
                        note.to_string()
                    } else {
                        format!("{note}{duration}")
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn generate(rng: &mut StdRng) -> String {
    let beats = rng.random_range(2..=4);
    let tempo = rng.random_range(64..=144);
    let bars: Vec<_> = phrase(rng, beats).iter().map(notation).collect();
    format!(
        "X:1\nM:{beats}/4\nL:1/16\nQ:1/4={tempo}\nK:C\n{} |]",
        bars.join(" | ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn phrases_fill_the_meter_and_reprise_the_motif() {
        assert!(CELLS.iter().all(|cell| cell.iter().sum::<u8>() == 4));
        let mut rng = StdRng::seed_from_u64(8);
        for beats in 2..=4 {
            for _ in 0..500 {
                let bars = phrase(&mut rng, beats);
                assert_eq!(bars.len(), 4);
                assert_eq!(bars[0], bars[2]);
                assert_ne!(bars[0], bars[1]);
                for bar in &bars {
                    assert_eq!(bar.len(), beats);
                    assert!(
                        bar.iter()
                            .all(|b| b.iter().map(|(d, _)| *d).sum::<u8>() == 4)
                    );
                    assert!(bar[0][0].1);
                }
                assert_eq!(bars[3][beats - 1], vec![(4, true)]);
            }
        }
    }
}
