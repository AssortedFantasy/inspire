use rand::{Rng, rngs::StdRng, seq::SliceRandom};

fn outcomes(max: usize, moves: &[usize]) -> Vec<bool> {
    let mut win = vec![false; max + 1];
    for n in 1..=max {
        win[n] = moves.iter().any(|&m| m <= n && !win[n - m]);
    }
    win
}

fn winning_removals(heap: usize, moves: &[usize], win: &[bool]) -> Vec<usize> {
    moves
        .iter()
        .copied()
        .filter(|&removed| removed <= heap && !win[heap - removed])
        .collect()
}

fn outcome_cycle(moves: &[usize]) -> (usize, Vec<bool>) {
    let width = *moves.iter().max().unwrap();
    let mut win = outcomes(width - 1, moves);
    let mut state = win.iter().enumerate().fold(0, |state, (index, &value)| {
        state | (usize::from(value) << index)
    });
    let mut seen = vec![None; 1 << width];
    // A complete width-bit window determines every subsequent outcome.
    // The next heap is at least width, so every allowed removal is legal.
    for start in 0..=1 << width {
        if let Some(first) = seen[state] {
            return (first, win[first..start].to_vec());
        }
        seen[state] = Some(start);
        let next = moves.iter().any(|&removed| !win[start + width - removed]);
        win.push(next);
        state = (state >> 1) | (usize::from(next) << (width - 1));
    }
    unreachable!("a repeated state must occur within the finite state space")
}

fn removals_text(moves: &[usize]) -> String {
    moves
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(" ")
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
    let winning = winning_removals(tokens, &moves, &win);
    let (start, cycle) = outcome_cycle(&moves);
    format!(
        "Subtraction (normal play)\nHeap: {tokens}\nRemovals: {}\n{}-position\nWinning removals: {}\nOutcome cycle from heap {start}: {} (period {})",
        removals_text(&moves),
        if win[tokens] { "N" } else { "P" },
        if winning.is_empty() {
            "none".into()
        } else {
            removals_text(&winning)
        },
        cycle
            .iter()
            .map(|&value| if value { 'N' } else { 'P' })
            .collect::<String>(),
        cycle.len(),
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
        assert_eq!(
            outcome_cycle(&[1, 2, 3]),
            (0, vec![false, true, true, true])
        );
        assert_eq!(
            outcome_cycle(&[2, 3]),
            (0, vec![false, false, true, true, true])
        );
    }

    #[test]
    fn every_allowed_set_has_a_proven_cycle_and_exact_winning_removals() {
        for mask in 1u8..128 {
            if !(2..=4).contains(&mask.count_ones()) {
                continue;
            }
            let moves = (1..=7)
                .filter(|removed| mask & (1 << (removed - 1)) != 0)
                .collect::<Vec<_>>();
            let win = outcomes(4096, &moves);
            let (start, cycle) = outcome_cycle(&moves);
            let width = *moves.last().unwrap();
            assert!(start + cycle.len() <= 1 << width);
            // Equal complete states, rather than agreement on a short prefix,
            // establish that the deterministic recurrence repeats forever.
            assert_eq!(
                win[start..start + width],
                win[start + cycle.len()..start + cycle.len() + width]
            );
            for heap in start..win.len() {
                assert_eq!(win[heap], cycle[(heap - start) % cycle.len()]);
            }
            for heap in 0..=40 {
                let winning = winning_removals(heap, &moves, &win);
                assert_eq!(!winning.is_empty(), win[heap]);
                for &removed in &moves {
                    assert_eq!(
                        winning.contains(&removed),
                        removed <= heap && !win[heap - removed]
                    );
                }
            }
            assert!((8..=40).any(|heap| win[heap]));
            assert!((8..=40).any(|heap| !win[heap]));
        }
    }
}
