struct Opening {
    name: String,
    eco: String,
    moves: Vec<String>,
}

fn openings() -> Vec<Opening> {
    let records: Vec<serde_json::Value> = serde_json::from_str(include_str!(
        "../families/games/chess-openings/records.json"
    ))
    .unwrap();
    records
        .into_iter()
        .map(|record| {
            let (label, score) = record["text"].as_str().unwrap().split_once('\n').unwrap();
            let (name, eco) = label.rsplit_once(" (").unwrap();
            Opening {
                name: name.to_owned(),
                eco: eco.strip_suffix(')').unwrap().to_owned(),
                moves: score
                    .split_whitespace()
                    .filter(|token| !token.ends_with('.'))
                    .map(str::to_owned)
                    .collect(),
            }
        })
        .collect()
}

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
        if shape && board[target].is_none() && (kind == 'N' || clear_path(board, from, target)) {
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
    let openings = openings();
    assert!(openings.len() >= 15);
    for opening in openings {
        let mut board = initial();
        for (ply, san) in opening.moves.iter().enumerate() {
            play(&mut board, san, ply % 2 == 0);
        }
        assert!(!opening.name.is_empty() && !opening.eco.is_empty());
    }
}

#[test]
fn output_uses_numbered_algebraic_move_pairs() {
    let openings = openings();
    for seed in 0..128 {
        let result = std::process::Command::new(env!("CARGO_BIN_EXE_inspire"))
            .args([
                "--family",
                "games/chess-openings",
                "--seed",
                &seed.to_string(),
            ])
            .output()
            .unwrap();
        assert!(result.status.success());
        let output = String::from_utf8(result.stdout).unwrap();
        let output = output.trim_end();
        assert!(output.contains('\n'));
        let (label, moves) = output.split_once('\n').unwrap();
        assert!(moves.starts_with("1. "));
        assert!(!moves.ends_with('\n'));
        let opening = openings
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
