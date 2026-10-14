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
    let tokens = rng.random_range(8..=40);
    let win = outcomes(tokens, &moves);
    let winning: Vec<_> = moves
        .iter()
        .copied()
        .filter(|&m| m <= tokens && !win[tokens - m])
        .collect();
    format!(
        "Normal-play subtraction game. One heap contains {tokens} tokens. Each turn removes exactly one of [{}] tokens when enough remain; a player with no legal move loses.\nPosition under optimal play: {}.\nWinning removals: {}",
        moves
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(", "),
        if win[tokens] { "winning" } else { "losing" },
        if winning.is_empty() {
            "none".into()
        } else {
            winning
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        }
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
