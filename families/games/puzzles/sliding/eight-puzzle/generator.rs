use rand::{Rng, rngs::StdRng};

fn neighbors(blank: usize) -> Vec<usize> {
    (0..9)
        .filter(|&i| {
            i / 3 == blank / 3 && i.abs_diff(blank) == 1
                || i % 3 == blank % 3 && i.abs_diff(blank) == 3
        })
        .collect()
}
fn puzzle(rng: &mut StdRng) -> ([u8; 9], Vec<usize>) {
    let mut board = [1, 2, 3, 4, 5, 6, 7, 8, 0];
    let mut blank = 8;
    let mut previous = 9;
    let mut history = Vec::new();
    for _ in 0..rng.random_range(20..=80) {
        let moves: Vec<_> = neighbors(blank)
            .into_iter()
            .filter(|&i| i != previous)
            .collect();
        let next = moves[rng.random_range(0..moves.len())];
        board.swap(blank, next);
        previous = blank;
        blank = next;
        history.push(next);
    }
    if board == [1, 2, 3, 4, 5, 6, 7, 8, 0] {
        let next = neighbors(blank)[0];
        board.swap(blank, next);
        history.push(next);
    }
    (board, history)
}
pub fn generate(rng: &mut StdRng) -> String {
    let (board, _) = puzzle(rng);
    let mut text = String::from("Eight-puzzle");
    for row in board.chunks(3) {
        text.push('\n');
        text.push_str(
            &row.iter()
                .map(|&v| if v == 0 { ".".into() } else { v.to_string() })
                .collect::<Vec<_>>()
                .join(" "),
        );
    }
    text
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn legal_scrambles_have_even_inversion_parity() {
        let mut rng = StdRng::seed_from_u64(4);
        for _ in 0..2000 {
            let (board, moves) = puzzle(&mut rng);
            let mut replay = [1, 2, 3, 4, 5, 6, 7, 8, 0];
            let mut blank = 8;
            for next in moves {
                assert!(neighbors(blank).contains(&next));
                replay.swap(blank, next);
                blank = next;
            }
            assert_eq!(board, replay);
            assert_ne!(board, [1, 2, 3, 4, 5, 6, 7, 8, 0]);
            let inversions = (0..9)
                .map(|i| {
                    (i + 1..9)
                        .filter(|&j| board[i] != 0 && board[j] != 0 && board[i] > board[j])
                        .count()
                })
                .sum::<usize>();
            assert_eq!(inversions % 2, 0);
        }
    }
}
