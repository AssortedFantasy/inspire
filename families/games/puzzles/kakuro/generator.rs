use rand::{Rng, rngs::StdRng, seq::SliceRandom};

#[derive(Clone)]
struct Run {
    cells: Vec<usize>,
    sum: u8,
    clue: usize,
    across: bool,
}

struct Board {
    width: usize,
    white: Vec<bool>,
    runs: Vec<Run>,
    membership: Vec<Vec<usize>>,
}

impl Board {
    fn new(pattern: &str, values: &[u8]) -> Self {
        let rows: Vec<_> = pattern.split('/').collect();
        let width = rows[0].len() + 1;
        let mut white = vec![false; width * (rows.len() + 1)];
        for (r, row) in rows.iter().enumerate() {
            for (c, ch) in row.bytes().enumerate() {
                white[(r + 1) * width + c + 1] = ch == b'1';
            }
        }
        let mut runs = Vec::new();
        for (i, &is_white) in white.iter().enumerate() {
            if !is_white {
                continue;
            }
            for (step, across) in [(1, true), (width, false)] {
                if white[i - step] {
                    continue;
                }
                let mut cells = Vec::new();
                let mut j = i;
                while j < white.len() && white[j] && (!across || j / width == i / width) {
                    cells.push(j);
                    j += step;
                }
                assert!((2..=9).contains(&cells.len()));
                runs.push(Run {
                    cells,
                    sum: 0,
                    clue: i - step,
                    across,
                });
            }
        }
        let mut board = Self {
            width,
            membership: vec![Vec::new(); white.len()],
            white,
            runs,
        };
        for (j, run) in board.runs.iter().enumerate() {
            for &i in &run.cells {
                board.membership[i].push(j);
            }
        }
        board.set_sums(values);
        board
    }

    fn set_sums(&mut self, values: &[u8]) {
        let mut grid = vec![0; self.white.len()];
        for (i, &value) in self.cells().zip(values) {
            grid[i] = value;
        }
        for run in &mut self.runs {
            run.sum = run.cells.iter().map(|&i| grid[i]).sum();
        }
    }

    fn cells(&self) -> impl Iterator<Item = usize> + '_ {
        self.white
            .iter()
            .enumerate()
            .filter_map(|(i, &w)| w.then_some(i))
    }
}

struct Solver<'a> {
    board: &'a Board,
    grid: Vec<u8>,
    used: Vec<u16>,
    sums: Vec<u8>,
    left: Vec<usize>,
}

impl<'a> Solver<'a> {
    fn new(board: &'a Board) -> Self {
        Self {
            board,
            grid: vec![0; board.white.len()],
            used: vec![0; board.runs.len()],
            sums: vec![0; board.runs.len()],
            left: board.runs.iter().map(|r| r.cells.len()).collect(),
        }
    }

    fn candidates(&self, i: usize) -> u16 {
        let mut result = 0;
        'digit: for d in 1..=9 {
            for &j in &self.board.membership[i] {
                let bit = 1 << d;
                if self.used[j] & bit != 0 {
                    continue 'digit;
                }
                let n = self.left[j] - 1;
                let rest = i16::from(self.board.runs[j].sum) - i16::from(self.sums[j]) - d;
                let available: Vec<i16> = (1..=9)
                    .filter(|&v| v != d && self.used[j] & (1 << v) == 0)
                    .collect();
                let low: i16 = available.iter().take(n).sum();
                let high: i16 = available.iter().rev().take(n).sum();
                if available.len() < n || rest < low || rest > high {
                    continue 'digit;
                }
            }
            result |= 1 << d;
        }
        result
    }

    fn set(&mut self, i: usize, d: u8) {
        self.grid[i] = d;
        for &j in &self.board.membership[i] {
            self.used[j] |= 1 << d;
            self.sums[j] += d;
            self.left[j] -= 1;
        }
    }

    fn unset(&mut self, i: usize, d: u8) {
        self.grid[i] = 0;
        for &j in &self.board.membership[i] {
            self.used[j] ^= 1 << d;
            self.sums[j] -= d;
            self.left[j] += 1;
        }
    }

    fn count(&mut self, budget: &mut usize, solutions: &mut u8) -> bool {
        if *budget == 0 {
            return false;
        }
        *budget -= 1;
        let best = self
            .board
            .cells()
            .filter(|&i| self.grid[i] == 0)
            .map(|i| (i, self.candidates(i)))
            .min_by_key(|&(_, mask)| mask.count_ones());
        let Some((i, mut mask)) = best else {
            *solutions += 1;
            return true;
        };
        while mask != 0 {
            let d = mask.trailing_zeros() as u8;
            mask &= mask - 1;
            self.set(i, d);
            let finished = self.count(budget, solutions);
            self.unset(i, d);
            if !finished {
                return false;
            }
            if *solutions >= 2 {
                break;
            }
        }
        true
    }
}

fn solutions(board: &Board, mut budget: usize) -> Option<u8> {
    let mut count = 0;
    Solver::new(board)
        .count(&mut budget, &mut count)
        .then_some(count)
}

fn fill(rng: &mut StdRng, board: &Board) -> Vec<u8> {
    let domain = match rng.random_range(0..3) {
        0 => 1..=5,
        1 => 5..=9,
        _ => 1..=9,
    };
    let mut grid = vec![0; board.white.len()];
    let mut cells: Vec<_> = board.cells().collect();
    cells.shuffle(rng);
    for i in cells {
        let used = board.membership[i].iter().fold(0u16, |mask, &j| {
            board.runs[j]
                .cells
                .iter()
                .fold(mask, |m, &k| m | (1 << grid[k]))
        });
        let mut choices: Vec<u8> = domain.clone().filter(|&d| used & (1 << d) == 0).collect();
        if choices.is_empty() {
            choices = (1..=9).filter(|&d| used & (1 << d) == 0).collect();
        }
        grid[i] = choices[rng.random_range(0..choices.len())];
    }
    board.cells().map(|i| grid[i]).collect()
}

const LAYOUTS: &[(&str, &[&[u8]])] = &[
    (
        "111/111",
        &[
            &[1, 2, 4, 3, 1, 2],
            &[7, 5, 9, 9, 8, 6],
            &[1, 4, 2, 3, 2, 1],
        ],
    ),
    (
        "111/111/110",
        &[
            &[8, 7, 5, 9, 8, 6, 5, 9],
            &[9, 7, 8, 6, 8, 9, 5, 9],
            &[9, 8, 6, 4, 2, 1, 8, 1],
        ],
    ),
    (
        "0110/1111/1111/0110",
        &[
            &[4, 1, 4, 1, 2, 3, 3, 2, 5, 1, 5, 3],
            &[5, 4, 2, 9, 1, 4, 5, 3, 2, 1, 1, 3],
            &[5, 7, 5, 9, 8, 6, 9, 8, 6, 7, 7, 9],
        ],
    ),
    (
        "1100/1111/1111/0011",
        &[
            &[1, 5, 3, 2, 5, 1, 2, 1, 3, 4, 1, 2],
            &[1, 2, 3, 4, 2, 1, 2, 3, 1, 5, 3, 2],
            &[3, 1, 2, 4, 1, 5, 1, 2, 5, 3, 2, 4],
        ],
    ),
];

fn puzzle(rng: &mut StdRng) -> (Board, Vec<u8>) {
    let (pattern, fallbacks) = LAYOUTS[rng.random_range(0..LAYOUTS.len())];
    let fallback = fallbacks[rng.random_range(0..fallbacks.len())];
    let mut board = Board::new(pattern, fallback);
    let mut answer = fallback.to_vec();
    // Search limits are conservative: incomplete counting is never uniqueness.
    for _ in 0..64 {
        let candidate = fill(rng, &board);
        board.set_sums(&candidate);
        if solutions(&board, 6_000) == Some(1) {
            answer = candidate;
            break;
        }
    }
    // Reflections, transposition, and digit complementation preserve uniqueness.
    let mut rows: Vec<Vec<(bool, u8)>> = pattern
        .split('/')
        .map(|row| row.bytes().map(|c| (c == b'1', 0)).collect())
        .collect();
    let mut values = answer.into_iter();
    for row in &mut rows {
        for cell in row.iter_mut().filter(|cell| cell.0) {
            cell.1 = values.next().unwrap();
        }
    }
    if rng.random_bool(0.5) {
        rows.reverse();
    }
    if rng.random_bool(0.5) {
        for row in &mut rows {
            row.reverse();
        }
    }
    if rng.random_bool(0.5) {
        rows = (0..rows[0].len())
            .map(|c| rows.iter().map(|row| row[c]).collect())
            .collect();
    }
    let complement = rng.random_bool(0.5);
    let pattern = rows
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| if cell.0 { '1' } else { '0' })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("/");
    let answer: Vec<_> = rows
        .iter()
        .flatten()
        .filter(|cell| cell.0)
        .map(|cell| if complement { 10 - cell.1 } else { cell.1 })
        .collect();
    (Board::new(&pattern, &answer), answer)
}

pub fn generate(rng: &mut StdRng) -> String {
    let (board, _) = puzzle(rng);
    let mut labels: Vec<_> = board
        .white
        .iter()
        .map(|&white| {
            if white {
                ".".to_string()
            } else {
                "#".to_string()
            }
        })
        .collect();
    for (i, label) in labels.iter_mut().enumerate() {
        let down = board
            .runs
            .iter()
            .find(|r| r.clue == i && !r.across)
            .map(|r| r.sum.to_string())
            .unwrap_or_default();
        let across = board
            .runs
            .iter()
            .find(|r| r.clue == i && r.across)
            .map(|r| r.sum.to_string())
            .unwrap_or_default();
        if !down.is_empty() || !across.is_empty() {
            *label = format!("{down}\\{across}");
        }
    }
    let mut text = String::from("Kakuro (down\\across)");
    for row in labels.chunks(board.width) {
        text.push('\n');
        text.push_str(
            &row.iter()
                .map(|s| format!("{s:>5}"))
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
    fn counts_reject_ambiguity_impossibility_and_budget_exhaustion() {
        let mut board = Board::new("11/11", &[1, 2, 3, 4]);
        assert_eq!(solutions(&board, 1000), Some(1));
        board.set_sums(&[3, 4, 4, 5]);
        assert_eq!(solutions(&board, 1000), Some(2));
        board.runs[0].sum = 2;
        assert_eq!(solutions(&board, 1000), Some(0));
        assert_eq!(solutions(&board, 0), None);
    }

    #[test]
    fn every_bounded_search_fallback_has_exactly_one_solution() {
        for &(pattern, fallbacks) in LAYOUTS {
            for &values in fallbacks {
                assert_eq!(solutions(&Board::new(pattern, values), 100_000), Some(1));
            }
        }
    }

    #[test]
    fn generated_boards_are_unique_and_each_cell_has_two_distinct_digit_runs() {
        let mut rng = StdRng::seed_from_u64(90331);
        let mut distinct = std::collections::HashSet::new();
        for _ in 0..80 {
            let (board, values) = puzzle(&mut rng);
            assert_eq!(solutions(&board, 100_000), Some(1));
            let mut grid = vec![0; board.white.len()];
            for (i, value) in board.cells().zip(values) {
                assert_eq!(board.membership[i].len(), 2);
                assert!((1..=9).contains(&value));
                grid[i] = value;
            }
            for run in &board.runs {
                let mut seen = 0u16;
                let mut sum = 0;
                for &i in &run.cells {
                    assert_eq!(seen & (1 << grid[i]), 0);
                    seen |= 1 << grid[i];
                    sum += grid[i];
                }
                assert_eq!(run.sum, sum);
                assert!(!board.white[run.clue]);
            }
            distinct.insert((board.width, board.white, grid));
        }
        assert!(distinct.len() > 65, "{} distinct puzzles", distinct.len());
    }
}
