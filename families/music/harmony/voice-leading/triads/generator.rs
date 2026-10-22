use rand::{Rng, rngs::StdRng};

mod spelling {
    include!("../../spelling.rs");
}

const MAJOR: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];
const ROMAN: [&str; 7] = ["I", "ii", "iii", "IV", "V", "vi", "vii°"];
const CONNECTIONS: [(usize, usize); 6] = [(1, 4), (4, 0), (3, 0), (1, 3), (5, 3), (0, 5)];
const PHRASES: [&[usize]; 7] = [
    &[1, 4, 0],
    &[0, 5, 1, 4, 0],
    &[0, 3, 4, 0],
    &[0, 5, 3, 4],
    &[0, 2, 5, 1],
    &[5, 3, 1, 4],
    &[0, 3, 0],
];

fn members(tonic: u8, degree: usize) -> [u8; 3] {
    [0, 2, 4].map(|i| (tonic + MAJOR[(degree + i) % 7]) % 12)
}

fn candidates(members: [u8; 3]) -> Vec<[u8; 3]> {
    let mut out = Vec::new();
    for bass in 48..=72 {
        if !members.contains(&(bass % 12)) {
            continue;
        }
        for middle in bass + 1..bass + 12 {
            for top in middle + 1..bass + 12 {
                let pcs = [bass % 12, middle % 12, top % 12];
                if pcs.iter().all(|pc| members.contains(pc))
                    && pcs[0] != pcs[1]
                    && pcs[1] != pcs[2]
                    && pcs[0] != pcs[2]
                {
                    out.push([bass, middle, top]);
                }
            }
        }
    }
    out
}

fn distance(a: [u8; 3], b: [u8; 3]) -> u16 {
    a.into_iter()
        .zip(b)
        .map(|(x, y)| u16::from(x.abs_diff(y)))
        .sum()
}

fn closest(source: [u8; 3], target: [u8; 3]) -> [u8; 3] {
    candidates(target)
        .into_iter()
        .min_by_key(|&v| distance(source, v))
        .unwrap()
}

fn label(notes: [u8; 3], tonic: spelling::Pitch) -> String {
    let pitches = [0, 1, 2, 3, 4, 5, 6].map(|degree| tonic.transpose(MAJOR[degree], degree));
    notes
        .into_iter()
        .map(|note| {
            pitches
                .iter()
                .find(|pitch| pitch.pc() == note % 12)
                .unwrap()
                .at(note)
        })
        .collect::<Vec<_>>()
        .join(" – ")
}

pub fn generate(rng: &mut StdRng) -> String {
    let tonic = rng.random_range(0..12);
    let phrase: Vec<_> = if rng.random_bool(0.35) {
        let (from, to) = CONNECTIONS[rng.random_range(0..CONNECTIONS.len())];
        vec![from, to]
    } else {
        PHRASES[rng.random_range(0..PHRASES.len())].to_vec()
    };
    let pitches = spelling::root(tonic, &MAJOR, &[0, 1, 2, 3, 4, 5, 6]);
    let sources = candidates(members(tonic, phrase[0]));
    let mut notes = sources[rng.random_range(0..sources.len())];
    let mut lines = vec![format!("{} major", pitches.name())];
    for (index, &degree) in phrase.iter().enumerate() {
        if index > 0 {
            notes = closest(notes, members(tonic, degree));
        }
        lines.push(format!("{:<4} {}", ROMAN[degree], label(notes, pitches)));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn voice_order_membership_and_global_minimum_hold() {
        for tonic in 0..12 {
            for (from, to) in CONNECTIONS {
                let targets = candidates(members(tonic, to));
                assert!(!targets.is_empty());
                for source in candidates(members(tonic, from)) {
                    let target = closest(source, members(tonic, to));
                    assert!(target[0] < target[1] && target[1] < target[2]);
                    assert!(target[2] - target[0] < 12);
                    assert!(
                        targets
                            .iter()
                            .all(|&v| distance(source, target) <= distance(source, v))
                    );
                }
            }
        }
    }
}
