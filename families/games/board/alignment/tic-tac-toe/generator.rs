use rand::{Rng, rngs::StdRng};

const LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

fn winner(board: &[u8; 9]) -> Option<u8> {
    LINES.iter().find_map(|line| {
        let value = board[line[0]];
        (value != 0 && line.iter().all(|&i| board[i] == value)).then_some(value)
    })
}

fn position(rng: &mut StdRng) -> ([u8; 9], Vec<usize>) {
    let mut board = [0; 9];
    let mut history = Vec::new();
    let target = rng.random_range(1..=9);
    for turn in 0..target {
        let empty: Vec<_> = (0..9).filter(|&i| board[i] == 0).collect();
        let square = empty[rng.random_range(0..empty.len())];
        board[square] = 1 + (turn % 2) as u8;
        history.push(square);
        if winner(&board).is_some() {
            break;
        }
    }
    (board, history)
}

pub fn generate(rng: &mut StdRng) -> String {
    let (board, history) = position(rng);
    let symbol = |value| match value {
        1 => 'X',
        2 => 'O',
        _ => '.',
    };
    let mut text = String::from("Tic-tac-toe (X starts; . is empty)\n  a b c");
    for row in 0..3 {
        text.push_str(&format!(
            "\n{} {} {} {}",
            row + 1,
            symbol(board[row * 3]),
            symbol(board[row * 3 + 1]),
            symbol(board[row * 3 + 2])
        ));
    }
    let status = match winner(&board) {
        Some(player) => format!("{} wins", symbol(player)),
        None if history.len() == 9 => "draw".to_owned(),
        None => format!("{} to move", if history.len() % 2 == 0 { 'X' } else { 'O' }),
    };
    let moves = history
        .iter()
        .enumerate()
        .map(|(turn, &square)| {
            format!(
                "{}:{}{}",
                if turn % 2 == 0 { 'X' } else { 'O' },
                (b'a' + (square % 3) as u8) as char,
                square / 3 + 1
            )
        })
        .collect::<Vec<_>>()
        .join(" ");
    format!("{text}\n{status}\nMoves: {moves}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    fn reachable(board: [u8; 9], turn: u8, found: &mut HashSet<[u8; 9]>) {
        if !found.insert(board) || winner(&board).is_some() {
            return;
        }
        for square in 0..9 {
            if board[square] == 0 {
                let mut next = board;
                next[square] = turn;
                reachable(next, 3 - turn, found);
            }
        }
    }

    #[test]
    fn samples_are_reachable_and_stop_at_the_first_win() {
        let mut found = HashSet::new();
        reachable([0; 9], 1, &mut found);
        assert_eq!(found.len(), 5478);
        let mut rng = StdRng::seed_from_u64(42);
        let mut outcomes = [false; 4];
        for _ in 0..10_000 {
            let (board, history) = position(&mut rng);
            assert!(found.contains(&board));
            let mut replay = [0; 9];
            for (turn, square) in history.iter().copied().enumerate() {
                assert!(winner(&replay).is_none());
                assert_eq!(replay[square], 0);
                replay[square] = 1 + (turn % 2) as u8;
            }
            assert_eq!(board, replay);
            let outcome = match winner(&board) {
                Some(1) => 0,
                Some(2) => 1,
                None if history.len() == 9 => 2,
                _ => 3,
            };
            outcomes[outcome] = true;
        }
        assert!(outcomes.into_iter().all(|seen| seen));
    }

    #[test]
    fn recognizes_all_eight_winning_lines() {
        for line in LINES {
            for player in 1..=2 {
                let mut board = [0; 9];
                for square in line {
                    board[square] = player;
                }
                assert_eq!(winner(&board), Some(player));
            }
        }
        assert_eq!(winner(&[1, 2, 1, 1, 2, 2, 2, 1, 1]), None);
    }
}
