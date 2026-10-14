use rand::{Rng, rngs::StdRng};

fn runs(bits: impl Iterator<Item = bool>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut count = 0;
    for bit in bits {
        if bit {
            count += 1;
        } else if count > 0 {
            result.push(count);
            count = 0;
        }
    }
    if count > 0 {
        result.push(count);
    }
    result
}
fn row_runs(row: u8) -> Vec<u8> {
    runs((0..5).map(|i| row & (1 << i) != 0))
}
fn clues(grid: &[u8; 5]) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    (
        grid.iter().map(|&r| row_runs(r)).collect(),
        (0..5)
            .map(|c| runs(grid.iter().map(|r| r & (1 << c) != 0)))
            .collect(),
    )
}
fn count(rows: &[Vec<u8>], cols: &[Vec<u8>]) -> u8 {
    let choices: Vec<Vec<u8>> = rows
        .iter()
        .map(|r| (0..32).filter(|&mask| row_runs(mask) == *r).collect())
        .collect();
    fn visit(
        row: usize,
        grid: &mut [u8; 5],
        choices: &[Vec<u8>],
        cols: &[Vec<u8>],
        solutions: &mut u8,
    ) {
        if *solutions >= 2 {
            return;
        }
        if row == 5 {
            if clues(grid).1 == cols {
                *solutions += 1;
            }
            return;
        }
        for &mask in &choices[row] {
            grid[row] = mask;
            visit(row + 1, grid, choices, cols, solutions);
            if *solutions >= 2 {
                return;
            }
        }
    }
    let mut solutions = 0;
    visit(0, &mut [0; 5], &choices, cols, &mut solutions);
    solutions
}
fn puzzle(rng: &mut StdRng) -> [u8; 5] {
    for _ in 0..32 {
        let grid = std::array::from_fn(|_| rng.random_range(0..32));
        let (rows, cols) = clues(&grid);
        let filled = grid.iter().map(|r| r.count_ones()).sum::<u32>();
        if (7..=18).contains(&filled) && count(&rows, &cols) == 1 {
            return grid;
        }
    }
    // A nontrivial Ferrers shape: rows/columns have forced consecutive prefixes.
    [31, 15, 7, 3, 1]
}
fn list(clues: &[Vec<u8>]) -> String {
    clues
        .iter()
        .map(|r| {
            if r.is_empty() {
                "0".into()
            } else {
                r.iter().map(u8::to_string).collect::<Vec<_>>().join(" ")
            }
        })
        .collect::<Vec<_>>()
        .join(" / ")
}
pub fn generate(rng: &mut StdRng) -> String {
    let grid = puzzle(rng);
    let (rows, cols) = clues(&grid);
    format!(
        "Nonogram 5x5 (unique solution).\nEach clue lists consecutive filled runs; neighboring runs have at least one empty cell; 0 means an empty line.\nRows top to bottom: {}\nColumns left to right: {}",
        list(&rows),
        list(&cols)
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn counting_detects_uniqueness_and_ambiguity() {
        assert_eq!(count(&vec![vec![1]; 5], &vec![vec![1]; 5]), 2);
        let (r, c) = clues(&[31, 15, 7, 3, 1]);
        assert_eq!(count(&r, &c), 1);
        let (r, c) = clues(&[0; 5]);
        assert_eq!(count(&r, &c), 1);
        assert_eq!(row_runs(0b10110), [2, 1]);
    }
    #[test]
    fn every_generated_grid_has_a_unique_clue_solution() {
        let mut rng = StdRng::seed_from_u64(17);
        for _ in 0..200 {
            let grid = puzzle(&mut rng);
            let (r, c) = clues(&grid);
            assert_eq!(count(&r, &c), 1);
        }
    }
}
