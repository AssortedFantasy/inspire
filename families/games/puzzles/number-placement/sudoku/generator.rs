use rand::{Rng, rngs::StdRng, seq::SliceRandom};

const ALL: u16 = 0b11_1111_1110;
const FILL_BUDGET: usize = 10_000;
const COUNT_BUDGET: usize = 4_000;

struct Solver {
    grid: [u8; 81],
    rows: [u16; 9],
    columns: [u16; 9],
    boxes: [u16; 9],
}

impl Solver {
    fn new(grid: [u8; 81]) -> Option<Self> {
        let mut solver = Self {
            grid: [0; 81],
            rows: [0; 9],
            columns: [0; 9],
            boxes: [0; 9],
        };
        for (i, digit) in grid.into_iter().enumerate() {
            if digit > 9 {
                return None;
            }
            if digit != 0 {
                if solver.candidates(i) & (1 << digit) == 0 {
                    return None;
                }
                solver.set(i, digit);
            }
        }
        Some(solver)
    }

    fn box_index(i: usize) -> usize {
        i / 27 * 3 + i % 9 / 3
    }

    fn candidates(&self, i: usize) -> u16 {
        ALL & !(self.rows[i / 9] | self.columns[i % 9] | self.boxes[Self::box_index(i)])
    }

    fn set(&mut self, i: usize, digit: u8) {
        self.grid[i] = digit;
        let bit = 1 << digit;
        self.rows[i / 9] |= bit;
        self.columns[i % 9] |= bit;
        self.boxes[Self::box_index(i)] |= bit;
    }

    fn unset(&mut self, i: usize, digit: u8) {
        self.grid[i] = 0;
        let mask = !(1 << digit);
        self.rows[i / 9] &= mask;
        self.columns[i % 9] &= mask;
        self.boxes[Self::box_index(i)] &= mask;
    }

    fn best(&self) -> Option<(usize, u16)> {
        let mut best = None;
        let mut size = 10;
        for (i, &value) in self.grid.iter().enumerate() {
            if value != 0 {
                continue;
            }
            let mask = self.candidates(i);
            let count = mask.count_ones();
            if count < size {
                best = Some((i, mask));
                size = count;
                if count <= 1 {
                    break;
                }
            }
        }
        best
    }

    fn fill(&mut self, rng: &mut StdRng, budget: &mut usize) -> bool {
        if *budget == 0 {
            return false;
        }
        *budget -= 1;
        let Some((i, mask)) = self.best() else {
            return true;
        };
        let mut choices: Vec<u8> = (1..=9).filter(|&digit| mask & (1 << digit) != 0).collect();
        choices.shuffle(rng);
        for digit in choices {
            self.set(i, digit);
            if self.fill(rng, budget) {
                return true;
            }
            self.unset(i, digit);
        }
        false
    }

    // Count to two. A depleted budget is never interpreted as uniqueness.
    fn count(&mut self, budget: &mut usize, solutions: &mut u8) -> bool {
        if *budget == 0 {
            return false;
        }
        *budget -= 1;
        let Some((i, mut mask)) = self.best() else {
            *solutions += 1;
            return true;
        };
        while mask != 0 {
            let digit = mask.trailing_zeros() as u8;
            mask &= mask - 1;
            self.set(i, digit);
            let completed = self.count(budget, solutions);
            self.unset(i, digit);
            if !completed {
                return false;
            }
            if *solutions >= 2 {
                return true;
            }
        }
        true
    }
}

fn count_solutions(grid: [u8; 81], budget: usize) -> Option<u8> {
    let Some(mut solver) = Solver::new(grid) else {
        return Some(0);
    };
    let mut remaining = budget;
    let mut count = 0;
    solver.count(&mut remaining, &mut count).then_some(count)
}

fn fallback_grid(rng: &mut StdRng) -> [u8; 81] {
    // Guaranteed completion if the randomized search consumes its budget.
    let mut digits: Vec<u8> = (1..=9).collect();
    digits.shuffle(rng);
    let mut bands = [0, 1, 2];
    let mut stacks = [0, 1, 2];
    bands.shuffle(rng);
    stacks.shuffle(rng);
    let mut rows = Vec::new();
    let mut columns = Vec::new();
    for group in bands {
        let mut inside = [0, 1, 2];
        inside.shuffle(rng);
        rows.extend(inside.map(|i| group * 3 + i));
    }
    for group in stacks {
        let mut inside = [0, 1, 2];
        inside.shuffle(rng);
        columns.extend(inside.map(|i| group * 3 + i));
    }
    std::array::from_fn(|i| {
        let row = rows[i / 9];
        let column = columns[i % 9];
        digits[(row * 3 + row / 3 + column) % 9]
    })
}

fn puzzle(rng: &mut StdRng) -> ([u8; 81], [u8; 81]) {
    let mut solver = Solver::new([0; 81]).unwrap();
    let mut fill_budget = FILL_BUDGET;
    let solution = if solver.fill(rng, &mut fill_budget) {
        solver.grid
    } else {
        fallback_grid(rng)
    };
    let mut grid = solution;
    let target = rng.random_range(30..=42);
    let mut clues = 81;
    let mut pairs: Vec<usize> = (0..=40).collect();
    pairs.shuffle(rng);
    for i in pairs {
        let opposite = 80 - i;
        let removed = if i == opposite { 1 } else { 2 };
        if clues - removed < target {
            continue;
        }
        grid[i] = 0;
        grid[opposite] = 0;
        if count_solutions(grid, COUNT_BUDGET) == Some(1) {
            clues -= removed;
        } else {
            grid[i] = solution[i];
            grid[opposite] = solution[opposite];
        }
    }
    (grid, solution)
}

pub fn generate(rng: &mut StdRng) -> String {
    let (grid, _) = puzzle(rng);
    let mut text = String::from("Sudoku");
    for row in 0..9 {
        if row > 0 && row % 3 == 0 {
            text.push_str("\n------+-------+------");
        }
        text.push('\n');
        for column in 0..9 {
            if column > 0 {
                text.push_str(if column % 3 == 0 { " | " } else { " " });
            }
            let digit = grid[row * 9 + column];
            text.push(if digit == 0 {
                '.'
            } else {
                (b'0' + digit) as char
            });
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn valid_solution(grid: [u8; 81]) {
        for group in 0..9 {
            let mut row: Vec<_> = (0..9).map(|i| grid[group * 9 + i]).collect();
            let mut column: Vec<_> = (0..9).map(|i| grid[i * 9 + group]).collect();
            let mut block: Vec<_> = (0..9)
                .map(|i| grid[(group / 3 * 3 + i / 3) * 9 + group % 3 * 3 + i % 3])
                .collect();
            for values in [&mut row, &mut column, &mut block] {
                values.sort_unstable();
                assert_eq!(*values, (1..=9).collect::<Vec<_>>());
            }
        }
    }

    #[test]
    fn generated_puzzles_preserve_solution_uniqueness_and_symmetry() {
        let mut rng = StdRng::seed_from_u64(23);
        let mut distinct = std::collections::HashSet::new();
        for _ in 0..100 {
            let (grid, solution) = puzzle(&mut rng);
            valid_solution(solution);
            let clues = grid.iter().filter(|&&digit| digit != 0).count();
            assert!((30..=81).contains(&clues));
            for i in 0..81 {
                assert!(grid[i] == 0 || grid[i] == solution[i]);
                assert_eq!(grid[i] == 0, grid[80 - i] == 0);
            }
            assert_eq!(count_solutions(grid, 100_000), Some(1));
            distinct.insert(solution);
        }
        assert_eq!(distinct.len(), 100);
    }

    #[test]
    fn counting_rejects_ambiguity_contradictions_and_budget_exhaustion() {
        assert_eq!(count_solutions([0; 81], 100_000), Some(2));
        assert_eq!(count_solutions([0; 81], 0), None);
        let mut invalid = [0; 81];
        invalid[0] = 1;
        invalid[1] = 1;
        assert_eq!(count_solutions(invalid, 100), Some(0));
        invalid[1] = 10;
        assert_eq!(count_solutions(invalid, 100), Some(0));
        let solution = fallback_grid(&mut StdRng::seed_from_u64(1));
        assert_eq!(count_solutions(solution, 1), Some(1));
        let mut missing_digit = solution;
        for digit in &mut missing_digit {
            if *digit == 1 || *digit == 2 {
                *digit = 0;
            }
        }
        assert_eq!(count_solutions(missing_digit, 100_000), Some(2));
    }

    #[test]
    fn randomized_pattern_fallback_always_obeys_all_units() {
        for seed in 0..100 {
            valid_solution(fallback_grid(&mut StdRng::seed_from_u64(seed)));
        }
    }
}
