use rand::{Rng, rngs::StdRng};

fn winning_move(heaps: &[u8]) -> Option<(usize, u8)> {
    let xor = heaps.iter().fold(0, |a, &b| a ^ b);
    heaps.iter().enumerate().find_map(|(i, &heap)| {
        let next = heap ^ xor;
        (next < heap).then_some((i, next))
    })
}
fn position(rng: &mut StdRng) -> Vec<u8> {
    let mut heaps: Vec<u8> = (0..rng.random_range(2..=6))
        .map(|_| rng.random_range(1..=15))
        .collect();
    if rng.random_ratio(1, 3) {
        heaps.pop();
        let last = heaps.iter().fold(0, |a, &b| a ^ b);
        if last != 0 {
            heaps.push(last);
        }
    } else if winning_move(&heaps).is_none() {
        heaps[0] ^= 16;
    }
    heaps
}

pub fn generate(rng: &mut StdRng) -> String {
    format!(
        "Nim\nHeaps: {}",
        position(rng)
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn samples_include_both_strategic_classes() {
        let mut rng = StdRng::seed_from_u64(432);
        let mut losing = 0;
        for _ in 0..1000 {
            let heaps = position(&mut rng);
            assert!((2..=6).contains(&heaps.len()));
            assert!(heaps.iter().all(|&heap| (1..=31).contains(&heap)));
            losing += usize::from(winning_move(&heaps).is_none());
        }
        assert!((250..420).contains(&losing));
    }
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
