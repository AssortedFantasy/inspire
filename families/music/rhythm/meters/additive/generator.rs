use rand::{Rng, rngs::StdRng};

type Group = Vec<(u8, bool)>;
type Bar = Vec<Group>;

const GROUPINGS: [&[u8]; 12] = [
    &[2, 3],
    &[3, 2],
    &[2, 2, 3],
    &[2, 3, 2],
    &[3, 2, 2],
    &[3, 3, 2],
    &[2, 3, 3],
    &[2, 2, 2, 3],
    &[3, 2, 2, 3],
    &[2, 2, 3, 3],
    &[3, 3, 3, 2],
    &[3, 2, 3, 2, 3],
];
const SHORT: [&[u8]; 6] = [&[4], &[2, 2], &[3, 1], &[1, 3], &[2, 1, 1], &[1, 1, 2]];
const LONG: [&[u8]; 7] = [
    &[6],
    &[2, 2, 2],
    &[4, 2],
    &[2, 4],
    &[3, 1, 2],
    &[2, 1, 1, 2],
    &[2, 2, 1, 1],
];

fn group(rng: &mut StdRng, eighths: u8) -> Group {
    let cells = if eighths == 2 { &SHORT[..] } else { &LONG[..] };
    cells[rng.random_range(0..cells.len())]
        .iter()
        .enumerate()
        .map(|(i, &duration)| (duration, i == 0 || rng.random_bool(0.8)))
        .collect()
}

fn phrase(rng: &mut StdRng, groups: &[u8]) -> Vec<Bar> {
    let motif: Bar = groups.iter().map(|&units| group(rng, units)).collect();
    let mut answer = motif.clone();
    let changed = rng.random_range(0..groups.len());
    answer[changed] = group(rng, groups[changed]);
    if answer[changed] == motif[changed] {
        answer[changed] = if motif[changed].len() == 1 {
            vec![(2, true); groups[changed] as usize]
        } else {
            vec![(2 * groups[changed], true)]
        };
    }
    let mut ending = answer.clone();
    let last = groups.len() - 1;
    ending[last] = vec![(2 * groups[last], true)];
    vec![motif.clone(), answer, motif, ending]
}

fn notation(bar: &Bar) -> String {
    bar.iter()
        .map(|group| {
            group
                .iter()
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
    let groups = GROUPINGS[rng.random_range(0..GROUPINGS.len())];
    let grouping = groups
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join("+");
    let tempo = rng.random_range(160..=280);
    let bars: Vec<_> = phrase(rng, groups).iter().map(notation).collect();
    format!(
        "X:1\nM:({grouping})/8\nL:1/16\nQ:1/8={tempo}\nK:C\n{} |]",
        bars.join(" | ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn additive_phrases_preserve_grouping_and_reprise_the_motif() {
        let mut rng = StdRng::seed_from_u64(10);
        for groups in GROUPINGS {
            for _ in 0..500 {
                let bars = phrase(&mut rng, groups);
                assert_eq!(bars.len(), 4);
                assert_eq!(bars[0], bars[2]);
                assert_ne!(bars[0], bars[1]);
                for bar in &bars {
                    assert_eq!(bar.len(), groups.len());
                    for (&expected, actual) in groups.iter().zip(bar) {
                        assert_eq!(actual.iter().map(|(d, _)| *d).sum::<u8>(), 2 * expected);
                        assert!(actual[0].1);
                    }
                }
            }
        }
    }
}
