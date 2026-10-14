use shakmaty::{Chess, Position, san::San};
use std::path::{Path, PathBuf};

fn corpora(path: &Path, found: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            corpora(&path, found);
        } else if path.file_name().unwrap() == "records.json" {
            found.push(path);
        }
    }
}
fn records() -> Vec<(PathBuf, serde_json::Value)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("families/games/board/chess/openings");
    let mut paths = Vec::new();
    corpora(&root, &mut paths);
    paths.sort();
    paths
        .into_iter()
        .flat_map(|path| {
            let rows: Vec<serde_json::Value> =
                serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
            rows.into_iter().map(move |row| (path.clone(), row))
        })
        .collect()
}
fn replay(text: &str) -> usize {
    let (label, score) = text.split_once('\n').unwrap();
    let (name, eco) = label.rsplit_once(" (").unwrap();
    assert!(!name.is_empty());
    let eco = eco.strip_suffix(')').unwrap();
    assert!(
        eco.len() == 3
            && matches!(eco.as_bytes()[0], b'A'..=b'E')
            && eco.as_bytes()[1..].iter().all(u8::is_ascii_digit)
    );
    let mut position = Chess::default();
    let mut ply = 0;
    for token in score.split_whitespace() {
        if token.ends_with('.') {
            assert_eq!(token, format!("{}.", ply / 2 + 1));
            continue;
        }
        assert!(
            !position.is_game_over(),
            "move after terminal position: {label}, {token}"
        );
        let san = San::from_ascii(token.as_bytes()).unwrap();
        let chess_move = san
            .to_move(&position)
            .unwrap_or_else(|error| panic!("{label}, ply {ply}, {token}: {error}"));
        assert_eq!(
            San::from_move(&position, chess_move).to_string(),
            token.trim_end_matches(['+', '#']),
            "noncanonical SAN: {label}"
        );
        position.play_unchecked(chess_move);
        if token.ends_with('#') {
            assert!(position.is_checkmate(), "false mate suffix: {label}");
        } else if token.ends_with('+') {
            assert!(
                position.is_check() && !position.is_checkmate(),
                "false check suffix: {label}"
            );
        } else {
            assert!(
                !position.is_check(),
                "missing check suffix: {label}, {token}"
            );
        }
        ply += 1;
    }
    assert!(ply > 0);
    ply
}
#[test]
fn every_imported_opening_is_a_complete_legal_canonical_line() {
    let rows = records();
    assert_eq!(rows.len(), 3865);
    let mut ids = std::collections::HashSet::new();
    let mut long = 0;
    for (_, row) in rows {
        assert!(ids.insert(row["id"].as_str().unwrap().to_owned()));
        if replay(row["text"].as_str().unwrap()) >= 20 {
            long += 1;
        }
    }
    assert!(long > 100, "book should include substantial variations");
}
#[test]
fn all_twenty_one_baseline_lines_are_preserved() {
    let texts: std::collections::HashSet<_> = records()
        .into_iter()
        .map(|(_, row)| row["text"].as_str().unwrap().to_owned())
        .collect();
    let baseline = [
        "Italian Game (C50)\n1. e4 e5 2. Nf3 Nc6 3. Bc4",
        "Ruy Lopez (C60)\n1. e4 e5 2. Nf3 Nc6 3. Bb5",
        "Nimzo-Larsen Attack: Modern Variation (A01)\n1. b3 e5 2. Bb2 Nc6",
        "Four Knights Game (C47)\n1. e4 e5 2. Nf3 Nc6 3. Nc3 Nf6",
        "Sicilian Defense: Staunton-Cochrane Variation (B20)\n1. e4 c5 2. c4",
        "French Defense (C00)\n1. e4 e6",
        "Caro-Kann Defense (B10)\n1. e4 c6",
        "Pirc Defense: Austrian Attack (B09)\n1. e4 d6 2. d4 Nf6 3. Nc3 g6 4. f4",
        "King's Indian Defense: Normal Variation, Rare Defenses (E90)\n1. d4 Nf6 2. c4 g6 3. Nc3 Bg7 4. e4 d6 5. Nf3",
        "Queen's Gambit Declined (D30)\n1. d4 d5 2. c4 e6",
        "Slav Defense (D10)\n1. d4 d5 2. c4 c6",
        "Nimzo-Indian Defense: Rubinstein System (E40)\n1. d4 Nf6 2. c4 e6 3. Nc3 Bb4 4. e3",
        "Queen's Indian Defense: Fianchetto Variation, Traditional Line (E15)\n1. d4 Nf6 2. c4 e6 3. Nf3 b6 4. g3 Bb7",
        "Grünfeld Defense (D80)\n1. d4 Nf6 2. c4 g6 3. Nc3 d5",
        "Dutch Defense (A80)\n1. d4 f5",
        "English Opening: King's English Variation (A20)\n1. c4 e5",
        "Réti Opening (A09)\n1. Nf3 d5 2. c4",
        "Bird Opening: Dutch Variation (A03)\n1. f4 d5",
        "Benoni Defense (A56)\n1. d4 Nf6 2. c4 c5",
        "Benko Gambit (A57)\n1. d4 Nf6 2. c4 c5 3. d5 b5",
        "Catalan Opening (E00)\n1. d4 Nf6 2. c4 e6 3. g3 d5",
    ];
    for text in baseline {
        assert!(texts.contains(text), "missing baseline line: {text}");
    }
}
#[test]
fn sampled_labels_and_moves_match_the_stored_line() {
    let rows = records();
    let mut leaves = std::collections::BTreeMap::new();
    for (path, row) in rows {
        leaves
            .entry(path)
            .or_insert_with(std::collections::HashSet::new)
            .insert(row["text"].as_str().unwrap().to_owned());
    }
    for (path, texts) in leaves {
        let family = path
            .parent()
            .unwrap()
            .strip_prefix(Path::new(env!("CARGO_MANIFEST_DIR")).join("families"))
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        for seed in 0..4 {
            let output = std::process::Command::new(env!("CARGO_BIN_EXE_inspire"))
                .args(["--family", &family, "--seed", &seed.to_string()])
                .output()
                .unwrap();
            assert!(output.status.success());
            let text = String::from_utf8(output.stdout).unwrap();
            assert!(texts.contains(text.trim_end()));
            replay(text.trim_end());
        }
    }
}
