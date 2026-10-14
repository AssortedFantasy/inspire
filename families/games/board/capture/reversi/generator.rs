use rand::{Rng, rngs::StdRng};

fn initial() -> [u8; 64] {
    let mut board = [0; 64];
    board[27] = 2;
    board[36] = 2;
    board[28] = 1;
    board[35] = 1;
    board
}
fn flips(board: &[u8; 64], square: usize, player: u8) -> Vec<usize> {
    if board[square] != 0 {
        return Vec::new();
    }
    let mut result = Vec::new();
    for (dr, dc) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let (mut row, mut col) = (square as i32 / 8 + dr, square as i32 % 8 + dc);
        let mut line = Vec::new();
        while (0..8).contains(&row) && (0..8).contains(&col) {
            let i = (row * 8 + col) as usize;
            if board[i] == 3 - player {
                line.push(i);
            } else {
                if board[i] == player && !line.is_empty() {
                    result.extend(line);
                }
                break;
            }
            row += dr;
            col += dc;
        }
    }
    result
}
fn legal(board: &[u8; 64], player: u8) -> Vec<usize> {
    (0..64)
        .filter(|&i| !flips(board, i, player).is_empty())
        .collect()
}
fn position(rng: &mut StdRng) -> ([u8; 64], Vec<Option<usize>>, u8) {
    let mut board = initial();
    let mut history = Vec::new();
    let mut player = 1;
    let target = rng.random_range(4..=60);
    let mut placed = 0;
    while placed < target {
        let moves = legal(&board, player);
        if moves.is_empty() {
            if legal(&board, 3 - player).is_empty() {
                break;
            }
            history.push(None);
            player = 3 - player;
            continue;
        }
        let square = moves[rng.random_range(0..moves.len())];
        let captured = flips(&board, square, player);
        board[square] = player;
        for i in captured {
            board[i] = player;
        }
        history.push(Some(square));
        placed += 1;
        player = 3 - player;
    }
    (board, history, player)
}
fn coord(i: usize) -> String {
    format!("{}{}", (b'a' + (i % 8) as u8) as char, i / 8 + 1)
}
pub fn generate(rng: &mut StdRng) -> String {
    let (board, history, player) = position(rng);
    let mut text = String::from(
        "Reversi (B starts; . empty; row 1 at top).\nA move brackets one or more W/B discs in a straight line and flips them. Pass only with no legal move; finish when neither player can move.\n  a b c d e f g h",
    );
    for row in 0..8 {
        text.push_str(&format!(
            "\n{} {}",
            row + 1,
            (0..8)
                .map(|c| match board[row * 8 + c] {
                    1 => "B",
                    2 => "W",
                    _ => ".",
                })
                .collect::<Vec<_>>()
                .join(" ")
        ));
    }
    let b = board.iter().filter(|&&p| p == 1).count();
    let w = board.iter().filter(|&&p| p == 2).count();
    let moves = legal(&board, player);
    let status = if moves.is_empty() && legal(&board, 3 - player).is_empty() {
        format!(
            "finished; {}",
            if b > w {
                "B wins"
            } else if w > b {
                "W wins"
            } else {
                "draw"
            }
        )
    } else if moves.is_empty() {
        format!("{} must pass", if player == 1 { "B" } else { "W" })
    } else {
        format!(
            "{} to move; legal: {}",
            if player == 1 { "B" } else { "W" },
            moves
                .iter()
                .map(|&i| coord(i))
                .collect::<Vec<_>>()
                .join(" ")
        )
    };
    format!(
        "{text}\nDiscs B={b}, W={w}; {status}\nMoves from standard center: {}",
        history
            .iter()
            .map(|m| m.map(coord).unwrap_or_else(|| "pass".into()))
            .collect::<Vec<_>>()
            .join(" ")
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn initial_moves_and_capture() {
        assert_eq!(legal(&initial(), 1), [19, 26, 37, 44]);
        assert_eq!(flips(&initial(), 19, 1), [27]);
    }
    #[test]
    fn histories_replay_with_only_forced_passes() {
        let mut rng = StdRng::seed_from_u64(8);
        for _ in 0..500 {
            let (board, history, next) = position(&mut rng);
            let mut replay = initial();
            let mut p = 1;
            let mut occupied = 4;
            for m in history {
                assert!(!(legal(&replay, 1).is_empty() && legal(&replay, 2).is_empty()));
                if let Some(i) = m {
                    let captured = flips(&replay, i, p);
                    assert!(!captured.is_empty());
                    replay[i] = p;
                    for j in captured {
                        replay[j] = p;
                    }
                    occupied += 1;
                } else {
                    assert!(legal(&replay, p).is_empty());
                }
                p = 3 - p;
            }
            assert_eq!(replay, board);
            assert_eq!(p, next);
            assert_eq!(board.iter().filter(|&&x| x != 0).count(), occupied);
        }
    }
}
