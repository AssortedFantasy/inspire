use rand::{Rng, rngs::StdRng};

fn nim_value(heaps: &[u8]) -> u8 {
    heaps.iter().fold(0, |a, &b| a ^ b)
}

fn winning(heaps: &[u8], misere: bool) -> bool {
    if misere && heaps.iter().all(|&h| h <= 1) {
        heaps.iter().filter(|&&h| h == 1).count() % 2 == 0
    } else {
        nim_value(heaps) != 0
    }
}

fn winning_moves(heaps: &[u8], misere: bool) -> Vec<(usize, u8)> {
    let mut moves = Vec::new();
    let mut next = heaps.to_vec();
    for (i, &heap) in heaps.iter().enumerate() {
        for remainder in 0..heap {
            next[i] = remainder;
            if !winning(&next, misere) {
                moves.push((i, remainder));
            }
        }
        next[i] = heap;
    }
    moves
}

fn position(rng: &mut StdRng, misere: bool) -> Vec<u8> {
    let n = rng.random_range(2..=6);
    if misere {
        match rng.random_range(0..4) {
            0 => return vec![1; n],
            1 => {
                let mut heaps = vec![1; n];
                heaps[rng.random_range(0..n)] = rng.random_range(2..=15);
                return heaps;
            }
            _ => (),
        }
    }
    let mut heaps: Vec<u8> = (0..n).map(|_| rng.random_range(1..=15)).collect();
    if rng.random_ratio(1, 3) {
        heaps.pop();
        let last = nim_value(&heaps);
        if last != 0 {
            heaps.push(last);
        }
    } else if nim_value(&heaps) == 0 {
        heaps[0] ^= 16;
    }
    heaps
}

pub fn generate(rng: &mut StdRng) -> String {
    let misere = rng.random_bool(0.5);
    let heaps = position(rng, misere);
    let mut text = format!(
        "{}\nHeaps: {}\n{}-position",
        if misere {
            "Misère Nim (last counter loses)"
        } else {
            "Nim (normal play)"
        },
        heaps
            .iter()
            .enumerate()
            .map(|(i, h)| format!("H{}={h}", i + 1))
            .collect::<Vec<_>>()
            .join(" "),
        if winning(&heaps, misere) { "N" } else { "P" },
    );
    if !misere {
        text.push_str(&format!("\nNim-value: {}", nim_value(&heaps)));
    }
    let moves = winning_moves(&heaps, misere);
    if moves.is_empty() {
        text.push_str("\nWinning reductions: none");
    } else {
        text.push_str(&format!(
            "\nWinning reductions: {}",
            moves
                .iter()
                .map(|&(i, h)| format!("H{} {}→{h}", i + 1, heaps[i]))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashMap;

    fn minimax(heaps: &[u8], misere: bool, memo: &mut HashMap<Vec<u8>, bool>) -> bool {
        let mut key: Vec<_> = heaps.iter().copied().filter(|&h| h != 0).collect();
        key.sort_unstable();
        if let Some(&value) = memo.get(&key) {
            return value;
        }
        if key.is_empty() {
            return misere;
        }
        let mut result = false;
        for (i, &heap) in key.iter().enumerate() {
            for remainder in 0..heap {
                let mut child = key.clone();
                child[i] = remainder;
                result |= !minimax(&child, misere, memo);
            }
        }
        memo.insert(key, result);
        result
    }

    #[test]
    fn both_conventions_and_all_moves_match_independent_game_tree() {
        for misere in [false, true] {
            let mut memo = HashMap::new();
            for a in 0..=6 {
                for b in 0..=6 {
                    for c in 0..=6 {
                        for d in 0..=6 {
                            let heaps = [a, b, c, d];
                            assert_eq!(winning(&heaps, misere), minimax(&heaps, misere, &mut memo));
                            let reported = winning_moves(&heaps, misere);
                            for (i, &heap) in heaps.iter().enumerate() {
                                for remainder in 0..heap {
                                    let mut child = heaps;
                                    child[i] = remainder;
                                    assert_eq!(
                                        reported.contains(&(i, remainder)),
                                        !minimax(&child, misere, &mut memo)
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn endgame_flip_and_strategic_classes_are_exposed() {
        assert!(winning(&[1], false));
        assert!(!winning(&[1], true));
        assert!(!winning(&[1, 1], false));
        assert!(winning(&[1, 1], true));
        assert_eq!(winning_moves(&[1, 1, 7], true), [(2, 1)]);
        assert_eq!(winning_moves(&[1, 7], true), [(1, 0)]);
        let mut rng = StdRng::seed_from_u64(432);
        for misere in [false, true] {
            let mut losing = 0;
            let mut boundary = 0;
            for _ in 0..1000 {
                let heaps = position(&mut rng, misere);
                assert!((1..=6).contains(&heaps.len()));
                assert!(heaps.iter().all(|&h| (1..=31).contains(&h)));
                losing += usize::from(!winning(&heaps, misere));
                boundary += usize::from(heaps.iter().filter(|&&h| h > 1).count() <= 1);
            }
            assert!((150..450).contains(&losing));
            if misere {
                assert!(boundary > 400);
            }
        }
    }
}
