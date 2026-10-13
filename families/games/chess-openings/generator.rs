use rand::{Rng, rngs::StdRng};

struct Opening {
    name: &'static str,
    eco: &'static str,
    moves: &'static [&'static str],
}

const OPENINGS: &[Opening] = &[
    Opening {
        name: "Italian Game",
        eco: "C50",
        moves: &["e4", "e5", "Nf3", "Nc6", "Bc4"],
    },
    Opening {
        name: "Ruy Lopez",
        eco: "C60",
        moves: &["e4", "e5", "Nf3", "Nc6", "Bb5"],
    },
    Opening {
        name: "Nimzo-Larsen Attack: Modern Variation",
        eco: "A01",
        moves: &["b3", "e5", "Bb2", "Nc6"],
    },
    Opening {
        name: "Four Knights Game",
        eco: "C47",
        moves: &["e4", "e5", "Nf3", "Nc6", "Nc3", "Nf6"],
    },
    Opening {
        name: "Sicilian Defense: Staunton-Cochrane Variation",
        eco: "B20",
        moves: &["e4", "c5", "c4"],
    },
    Opening {
        name: "French Defense",
        eco: "C00",
        moves: &["e4", "e6"],
    },
    Opening {
        name: "Caro-Kann Defense",
        eco: "B10",
        moves: &["e4", "c6"],
    },
    Opening {
        name: "Pirc Defense: Austrian Attack",
        eco: "B09",
        moves: &["e4", "d6", "d4", "Nf6", "Nc3", "g6", "f4"],
    },
    Opening {
        name: "King's Indian Defense: Normal Variation, Rare Defenses",
        eco: "E90",
        moves: &["d4", "Nf6", "c4", "g6", "Nc3", "Bg7", "e4", "d6", "Nf3"],
    },
    Opening {
        name: "Queen's Gambit Declined",
        eco: "D30",
        moves: &["d4", "d5", "c4", "e6"],
    },
    Opening {
        name: "Slav Defense",
        eco: "D10",
        moves: &["d4", "d5", "c4", "c6"],
    },
    Opening {
        name: "Nimzo-Indian Defense: Rubinstein System",
        eco: "E40",
        moves: &["d4", "Nf6", "c4", "e6", "Nc3", "Bb4", "e3"],
    },
    Opening {
        name: "Queen's Indian Defense: Fianchetto Variation, Traditional Line",
        eco: "E15",
        moves: &["d4", "Nf6", "c4", "e6", "Nf3", "b6", "g3", "Bb7"],
    },
    Opening {
        name: "Grünfeld Defense",
        eco: "D80",
        moves: &["d4", "Nf6", "c4", "g6", "Nc3", "d5"],
    },
    Opening {
        name: "Dutch Defense",
        eco: "A80",
        moves: &["d4", "f5"],
    },
    Opening {
        name: "English Opening: King's English Variation",
        eco: "A20",
        moves: &["c4", "e5"],
    },
    Opening {
        name: "Réti Opening",
        eco: "A09",
        moves: &["Nf3", "d5", "c4"],
    },
    Opening {
        name: "Bird Opening: Dutch Variation",
        eco: "A03",
        moves: &["f4", "d5"],
    },
    Opening {
        name: "Benoni Defense",
        eco: "A56",
        moves: &["d4", "Nf6", "c4", "c5"],
    },
    Opening {
        name: "Benko Gambit",
        eco: "A57",
        moves: &["d4", "Nf6", "c4", "c5", "d5", "b5"],
    },
    Opening {
        name: "Catalan Opening",
        eco: "E00",
        moves: &["d4", "Nf6", "c4", "e6", "g3", "d5"],
    },
];

pub fn generate(rng: &mut StdRng) -> String {
    let opening = &OPENINGS[rng.random_range(0..OPENINGS.len())];
    let mut score = String::new();
    for (ply, san) in opening.moves.iter().enumerate() {
        if ply % 2 == 0 {
            if !score.is_empty() {
                score.push(' ');
            }
            score.push_str(&format!("{}. ", ply / 2 + 1));
        } else {
            score.push(' ');
        }
        score.push_str(san);
    }
    format!("{} ({})\n{}", opening.name, opening.eco, score)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[derive(Clone, Copy, PartialEq, Eq)]
    struct Piece {
        kind: char,
        white: bool,
    }

    fn square(file: u8, rank: u8) -> usize {
        rank as usize * 8 + file as usize
    }

    fn initial() -> [Option<Piece>; 64] {
        let mut board = [None; 64];
        let back = ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'];
        for file in 0..8 {
            board[square(file, 0)] = Some(Piece {
                kind: back[file as usize],
                white: true,
            });
            board[square(file, 1)] = Some(Piece {
                kind: 'P',
                white: true,
            });
            board[square(file, 6)] = Some(Piece {
                kind: 'P',
                white: false,
            });
            board[square(file, 7)] = Some(Piece {
                kind: back[file as usize],
                white: false,
            });
        }
        board
    }

    fn attacked(board: &[Option<Piece>; 64], target: usize, by_white: bool) -> bool {
        for (from, piece) in board.iter().enumerate() {
            let Some(piece) = piece.filter(|p| p.white == by_white) else {
                continue;
            };
            let df = target as i8 % 8 - from as i8 % 8;
            let dr = target as i8 / 8 - from as i8 / 8;
            let (af, ar) = (df.abs(), dr.abs());
            let attacks = match piece.kind {
                'P' => af == 1 && dr == if by_white { 1 } else { -1 },
                'N' => (af, ar) == (1, 2) || (af, ar) == (2, 1),
                'K' => af <= 1 && ar <= 1,
                'B' => af == ar && af != 0,
                'R' => (df == 0) != (dr == 0),
                'Q' => (af == ar && af != 0) || ((df == 0) != (dr == 0)),
                _ => false,
            };
            let sliding = matches!(piece.kind, 'B' | 'R' | 'Q');
            if attacks && (!sliding || clear_path(board, from, target)) {
                return true;
            }
        }
        false
    }

    fn clear_path(board: &[Option<Piece>; 64], from: usize, to: usize) -> bool {
        let (ff, fr) = ((from % 8) as i8, (from / 8) as i8);
        let (tf, tr) = ((to % 8) as i8, (to / 8) as i8);
        let (sf, sr) = ((tf - ff).signum(), (tr - fr).signum());
        let (mut f, mut r) = (ff + sf, fr + sr);
        while (f, r) != (tf, tr) {
            if board[square(f as u8, r as u8)].is_some() {
                return false;
            }
            f += sf;
            r += sr;
        }
        true
    }

    fn play(board: &mut [Option<Piece>; 64], san: &str, white: bool) {
        let bytes = san.as_bytes();
        assert!(!bytes.is_empty(), "SAN move must not be empty");
        let offset = usize::from(bytes[0].is_ascii_uppercase());
        assert!(
            (offset == 0 && bytes.len() == 2 && bytes[0].is_ascii_lowercase())
                || (offset == 1
                    && bytes.len() == 3
                    && matches!(bytes[0], b'B' | b'K' | b'N' | b'Q' | b'R')),
            "unsupported SAN syntax {san}"
        );
        let kind = if offset == 1 { bytes[0] as char } else { 'P' };
        assert!(bytes[offset].is_ascii_lowercase() && (b'a'..=b'h').contains(&bytes[offset]));
        assert!((b'1'..=b'8').contains(&bytes[offset + 1]));
        let target = square(bytes[offset] - b'a', bytes[offset + 1] - b'1');
        let mut candidates = Vec::new();
        for (from, piece) in board.iter().enumerate() {
            let Some(piece) = piece.filter(|p| p.white == white && p.kind == kind) else {
                continue;
            };
            let (df, dr) = (
                target as i8 % 8 - from as i8 % 8,
                target as i8 / 8 - from as i8 / 8,
            );
            let (af, ar) = (df.abs(), dr.abs());
            let shape = match kind {
                'P' => {
                    df == 0
                        && (dr == if white { 1 } else { -1 }
                            || (dr == if white { 2 } else { -2 }
                                && from / 8 == if white { 1 } else { 6 }))
                }
                'N' => (af, ar) == (1, 2) || (af, ar) == (2, 1),
                'B' => af == ar && af != 0,
                'R' => (df == 0) != (dr == 0),
                'Q' => (af == ar && af != 0) || ((df == 0) != (dr == 0)),
                'K' => af <= 1 && ar <= 1,
                _ => false,
            };
            let start = square((from % 8) as u8, (from / 8) as u8);
            let mut trial = *board;
            trial[start] = None;
            if shape && board[target].is_none() && (kind == 'N' || clear_path(board, from, target))
            {
                trial[target] = Some(piece);
                let king = trial
                    .iter()
                    .position(|p| *p == Some(Piece { kind: 'K', white }))
                    .expect("king");
                if !attacked(&trial, king, !white) {
                    candidates.push(from);
                }
            }
        }
        assert_eq!(candidates.len(), 1, "illegal or ambiguous SAN move {san}");
        board[target] = board[candidates[0]];
        board[candidates[0]] = None;
    }

    #[test]
    fn every_named_line_is_legal_and_has_a_label_matching_its_endpoint() {
        assert!(OPENINGS.len() >= 15);
        for opening in OPENINGS {
            let mut board = initial();
            for (ply, san) in opening.moves.iter().enumerate() {
                play(&mut board, san, ply % 2 == 0);
            }
            assert!(!opening.name.is_empty() && !opening.eco.is_empty());
        }
    }

    #[test]
    fn output_uses_numbered_algebraic_move_pairs() {
        for seed in 0..128 {
            let output = generate(&mut StdRng::seed_from_u64(seed));
            assert!(output.contains('\n'));
            let (label, moves) = output.split_once('\n').unwrap();
            assert!(moves.starts_with("1. "));
            assert!(!moves.ends_with('\n'));
            let opening = OPENINGS
                .iter()
                .find(|opening| label == format!("{} ({})", opening.name, opening.eco))
                .unwrap();
            let mut expected = String::new();
            for (ply, san) in opening.moves.iter().enumerate() {
                if ply % 2 == 0 {
                    if !expected.is_empty() {
                        expected.push(' ');
                    }
                    expected.push_str(&format!("{}. ", ply / 2 + 1));
                } else {
                    expected.push(' ');
                }
                expected.push_str(san);
            }
            assert_eq!(moves, expected, "label must describe the full sampled line");
        }
    }
}
