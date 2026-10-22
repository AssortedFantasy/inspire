use rand::{Rng, rngs::StdRng, seq::SliceRandom};

fn outcomes(max: usize, moves: &[usize]) -> Vec<bool> {
    let mut win = vec![false; max + 1];
    for n in 1..=max {
        win[n] = moves.iter().any(|&m| m <= n && !win[n - m]);
    }
    win
}
pub fn generate(rng: &mut StdRng) -> String {
    let mut moves: Vec<usize> = (1..=7).collect();
    moves.shuffle(rng);
    moves.truncate(rng.random_range(2..=4));
    moves.sort_unstable();
    let win = outcomes(40, &moves);
    let desired: bool = rng.random();
    let candidates: Vec<_> = (8..=40).filter(|&n| win[n] == desired).collect();
    let tokens = candidates[rng.random_range(0..candidates.len())];
    format!(
        "Subtraction\nHeap: {tokens}\nRemovals: {}",
        moves
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn known_period_and_terminal_short_heap() {
        let win = outcomes(40, &[1, 2, 3]);
        for (n, value) in win.into_iter().enumerate() {
            assert_eq!(value, n % 4 != 0);
        }
        assert_eq!(outcomes(3, &[2, 3]), [false, false, true, true]);
    }
}
