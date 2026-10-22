use rand::{Rng, rngs::StdRng};

type Beat = Vec<(u8, bool)>;
type Bar = Vec<Beat>;

const CELLS: [&[u8]; 9] = [
    &[6],
    &[2, 2, 2],
    &[4, 2],
    &[2, 4],
    &[3, 1, 2],
    &[2, 1, 1, 2],
    &[1, 1, 2, 2],
    &[2, 2, 1, 1],
    &[3, 3],
];
const FEELS: [[usize; 5]; 3] = [[0, 1, 1, 2, 3], [0, 2, 3, 4, 8], [1, 4, 5, 6, 7]];

fn beat(rng: &mut StdRng, feel: &[usize]) -> Beat {
    CELLS[feel[rng.random_range(0..feel.len())]]
        .iter()
        .map(|&duration| (duration, rng.random_bool(0.88)))
        .collect()
}

fn phrase(rng: &mut StdRng, beats: usize) -> Vec<Bar> {
    let feel = FEELS[rng.random_range(0..FEELS.len())];
    let mut motif: Bar = (0..beats).map(|_| beat(rng, &feel)).collect();
    motif[0][0].1 = true;
    let mut answer = motif.clone();
    answer[beats - 1] = beat(rng, &feel);
    if answer[beats - 1] == motif[beats - 1] {
        answer[beats - 1] = if motif[beats - 1].len() == 1 {
            vec![(4, true), (2, true)]
        } else {
            vec![(6, true)]
        };
    }
    let mut ending = motif.clone();
    ending[beats - 1] = vec![(6, true)];
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
    let tempo = rng.random_range(48..=112);
    let bars: Vec<_> = phrase(rng, beats).iter().map(notation).collect();
    format!(
        "X:1\nM:{}/8\nL:1/16\nQ:3/8={tempo}\nK:C\n{} |]",
        beats * 3,
        bars.join(" | ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn compound_phrases_fill_the_meter_and_reprise_the_motif() {
        assert!(CELLS.iter().all(|cell| cell.iter().sum::<u8>() == 6));
        let mut rng = StdRng::seed_from_u64(9);
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
                            .all(|b| b.iter().map(|(d, _)| *d).sum::<u8>() == 6)
                    );
                    assert!(bar[0][0].1);
                }
                assert_eq!(bars[3][beats - 1], vec![(6, true)]);
            }
        }
    }
}
