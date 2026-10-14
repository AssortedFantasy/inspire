use rand::{Rng, rngs::StdRng};

fn winning_move(heaps: &[u8]) -> Option<(usize, u8)> {
    let xor = heaps.iter().fold(0, |a, &b| a ^ b);
    heaps.iter().enumerate().find_map(|(i, &heap)| {
        let next = heap ^ xor;
        (next < heap).then_some((i, next))
    })
}
pub fn generate(rng: &mut StdRng) -> String {
    let heaps: Vec<u8> = (0..rng.random_range(2..=6))
        .map(|_| rng.random_range(1..=15))
        .collect();
    let xor = heaps.iter().fold(0, |a, &b| a ^ b);
    let status = if let Some((i, next)) = winning_move(&heaps) {
        format!(
            "Winning position under optimal play; heap {} can be reduced from {} to {next}, leaving nim-sum 0.",
            i + 1,
            heaps[i]
        )
    } else {
        "Losing position under optimal play; every move leaves a nonzero nim-sum.".into()
    };
    format!(
        "Normal-play Nim. A turn removes one or more tokens from exactly one heap; the player unable to move loses.\nHeaps: {}\nNim-sum (bitwise XOR): {xor}\n{status}",
        heaps
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strategic_move_reaches_zero_and_zero_cannot_remain_zero() {
        for a in 0..16 {
            for b in 0..16 {
                for c in 0..16 {
                    let heaps = [a, b, c];
                    if let Some((i, next)) = winning_move(&heaps) {
                        assert!(next < heaps[i]);
                        let mut reduced = heaps;
                        reduced[i] = next;
                        assert_eq!(reduced.into_iter().fold(0, |a, b| a ^ b), 0);
                    } else {
                        assert_eq!(a ^ b ^ c, 0);
                        for (i, &heap) in heaps.iter().enumerate() {
                            for next in 0..heap {
                                let mut reduced = heaps;
                                reduced[i] = next;
                                assert_ne!(reduced.into_iter().fold(0, |a, b| a ^ b), 0);
                            }
                        }
                    }
                }
            }
        }
    }
}
