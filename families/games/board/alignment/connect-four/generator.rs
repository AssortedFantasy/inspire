use rand::{Rng, rngs::StdRng};

fn winner(board: &[u8; 42]) -> Option<u8> {
    for row in 0i32..6 {
        for col in 0i32..7 {
            for (dr, dc) in [(0, 1), (1, 0), (1, 1), (1, -1)] {
                let end = (row + 3 * dr, col + 3 * dc);
                if !(0..6).contains(&end.0) || !(0..7).contains(&end.1) {
                    continue;
                }
                let player = board[(row * 7 + col) as usize];
                if player != 0
                    && (1..4).all(|i| board[((row + i * dr) * 7 + col + i * dc) as usize] == player)
                {
                    return Some(player);
                }
            }
        }
    }
    None
}

fn position(rng: &mut StdRng) -> ([u8; 42], Vec<usize>) {
    let mut board = [0; 42];
    let mut heights = [0; 7];
    let mut moves = Vec::new();
    let target = rng.random_range(4..=42);
    for turn in 0..target {
        let available: Vec<_> = (0..7).filter(|&c| heights[c] < 6).collect();
        let col = available[rng.random_range(0..available.len())];
        board[heights[col] * 7 + col] = 1 + (turn % 2) as u8;
        heights[col] += 1;
        moves.push(col);
        if winner(&board).is_some() {
            break;
        }
    }
    (board, moves)
}

pub fn generate(rng: &mut StdRng) -> String {
    let (board, moves) = position(rng);
    let symbol = |p| match p {
        1 => 'X',
        2 => 'O',
        _ => '.',
    };
    let mut text = String::from(
        "Connect Four (X starts; columns 1-7; gravity down).\nFour contiguous pieces horizontally, vertically, or diagonally win.",
    );
    for row in (0..6).rev() {
        text.push('\n');
        text.push_str(
            &(0..7)
                .map(|c| symbol(board[row * 7 + c]).to_string())
                .collect::<Vec<_>>()
                .join(" "),
        );
    }
    let status = match winner(&board) {
        Some(p) => format!("{} wins", symbol(p)),
        None if moves.len() == 42 => "draw".into(),
        _ => format!("{} to move", if moves.len() % 2 == 0 { 'X' } else { 'O' }),
    };
    format!(
        "{text}\n1 2 3 4 5 6 7\n{status}\nColumns played: {}",
        moves
            .iter()
            .map(|c| (c + 1).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn replay_stops_at_first_win_and_respects_gravity() {
        let mut rng = StdRng::seed_from_u64(4);
        for _ in 0..2000 {
            let (board, moves) = position(&mut rng);
            let mut replay = [0; 42];
            let mut heights = [0; 7];
            for (turn, col) in moves.iter().copied().enumerate() {
                assert!(winner(&replay).is_none());
                assert!(heights[col] < 6);
                replay[heights[col] * 7 + col] = 1 + (turn % 2) as u8;
                heights[col] += 1;
            }
            assert_eq!(board, replay);
        }
    }
    #[test]
    fn all_four_line_directions() {
        for (dr, dc) in [(0, 1), (1, 0), (1, 1), (1, -1)] {
            let mut board = [0; 42];
            let col = if dc < 0 { 3 } else { 0 };
            for i in 0..4 {
                board[(i * dr * 7 + col + i * dc) as usize] = 2;
            }
            assert_eq!(winner(&board), Some(2));
        }
    }
}
