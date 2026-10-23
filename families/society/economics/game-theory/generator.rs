use rand::{Rng, rngs::StdRng};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Fraction {
    n: i64,
    d: i64,
}

impl Fraction {
    fn new(n: i64, d: i64) -> Self {
        assert_ne!(d, 0);
        let (mut a, mut b) = (n.abs(), d.abs());
        while b != 0 {
            (a, b) = (b, a % b);
        }
        let sign = d.signum();
        Self {
            n: n / a * sign,
            d: d / a * sign,
        }
    }
    fn text(self) -> String {
        if self.d == 1 {
            self.n.to_string()
        } else {
            format!("{}/{}", self.n, self.d)
        }
    }
}

struct Game {
    title: &'static str,
    row: Vec<&'static str>,
    col: Vec<&'static str>,
    cells: Vec<Vec<[i64; 2]>>,
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..6);
    let game = sample(rng, regime);
    let pure = pure_equilibria(&game);
    let mixed = mixed_equilibrium(&game);
    let mut lines = vec![
        game.title.to_owned(),
        format!(
            "{:>12}{}",
            "",
            game.col
                .iter()
                .map(|c| format!("{c:>15}"))
                .collect::<String>()
        ),
    ];
    for (r, row) in game.cells.iter().enumerate() {
        lines.push(format!(
            "{:>12}{}",
            game.row[r],
            row.iter()
                .map(|p| format!("{:>15}", format!("({}, {})", p[0], p[1])))
                .collect::<String>()
        ));
    }
    lines.push(format!("\nPure Nash {}", cell_names(&game, &pure)));
    if let Some((row, col)) = mixed {
        lines.push(format!("Mixed Nash\nRow {}", distribution(&game.row, &row)));
        lines.push(format!("Column {}", distribution(&game.col, &col)));
    }
    let efficient = pareto(&game);
    lines.push(format!(
        "Pareto-efficient cells {}",
        if efficient.len() == game.row.len() * game.col.len() {
            format!("all {}", efficient.len())
        } else {
            cell_names(&game, &efficient)
        }
    ));
    if regime == 1 {
        let reward = game.cells[0][0][0];
        let safe = game.cells[1][1][0];
        let dominant = match reward.cmp(&(2 * safe)) {
            std::cmp::Ordering::Greater => "(Stag, Stag)",
            std::cmp::Ordering::Less => "(Hare, Hare)",
            std::cmp::Ordering::Equal => "tie",
        };
        lines.push(format!("Risk-dominant {dominant}"));
    }
    lines.join("\n")
}

fn sample(rng: &mut StdRng, regime: usize) -> Game {
    let (title, actions, cells) = match regime {
        0 => {
            let sucker = rng.random_range(-5..=0);
            let punishment = sucker + rng.random_range(1..=5);
            let reward = punishment + rng.random_range(1..=6);
            let temptation = reward + rng.random_range(1..=7);
            (
                "Prisoner's dilemma",
                ["Cooperate", "Defect"],
                vec![
                    vec![[reward, reward], [sucker, temptation]],
                    vec![[temptation, sucker], [punishment, punishment]],
                ],
            )
        }
        1 => {
            let reward = rng.random_range(5..=20);
            let safe = rng.random_range(1..reward);
            (
                "Stag hunt",
                ["Stag", "Hare"],
                vec![
                    vec![[reward, reward], [0, safe]],
                    vec![[safe, 0], [safe, safe]],
                ],
            )
        }
        2 => {
            let value = 2 * rng.random_range(1..=8);
            let cost = value + 2 * rng.random_range(1..=12);
            (
                "Hawk–dove",
                ["Hawk", "Dove"],
                vec![
                    vec![[(value - cost) / 2, (value - cost) / 2], [value, 0]],
                    vec![[0, value], [value / 2, value / 2]],
                ],
            )
        }
        3 => {
            let a = rng.random_range(3..=15);
            let b = rng.random_range(1..a);
            let c = rng.random_range(3..=15);
            let d = rng.random_range(1..c);
            (
                "Coordination with opposed preferences",
                ["Theater", "Concert"],
                vec![vec![[a, d], [0, 0]], vec![[0, 0], [b, c]]],
            )
        }
        4 => {
            let [a, b, c, d] = std::array::from_fn(|_| rng.random_range(1..=12));
            (
                "Unequal-stakes matching pennies",
                ["Heads", "Tails"],
                vec![vec![[a, -a], [-b, b]], vec![[-c, c], [d, -d]]],
            )
        }
        _ => {
            let [a, b, c] = std::array::from_fn(|_| rng.random_range(1..=12));
            return Game {
                title: "Weighted rock–paper–scissors",
                row: vec!["Rock", "Paper", "Scissors"],
                col: vec!["Rock", "Paper", "Scissors"],
                cells: vec![
                    vec![[0, 0], [-a, a], [b, -b]],
                    vec![[a, -a], [0, 0], [-c, c]],
                    vec![[-b, b], [c, -c], [0, 0]],
                ],
            };
        }
    };
    Game {
        title,
        row: actions.to_vec(),
        col: actions.to_vec(),
        cells,
    }
}

fn pure_equilibria(game: &Game) -> Vec<(usize, usize)> {
    let n = game.row.len();
    (0..n)
        .flat_map(|r| (0..n).map(move |c| (r, c)))
        .filter(|&(r, c)| {
            (0..n).all(|s| game.cells[r][c][0] >= game.cells[s][c][0])
                && (0..n).all(|s| game.cells[r][c][1] >= game.cells[r][s][1])
        })
        .collect()
}

fn mixed_equilibrium(game: &Game) -> Option<(Vec<Fraction>, Vec<Fraction>)> {
    let a = &game.cells;
    if a.len() == 3 {
        let weights = [a[2][1][0], a[0][2][0], a[1][0][0]];
        let total: i64 = weights.iter().sum();
        let strategy: Vec<_> = weights.iter().map(|&w| Fraction::new(w, total)).collect();
        return Some((strategy.clone(), strategy));
    }
    let row_denominator = a[0][0][1] - a[0][1][1] - a[1][0][1] + a[1][1][1];
    let col_denominator = a[0][0][0] - a[0][1][0] - a[1][0][0] + a[1][1][0];
    if row_denominator == 0 || col_denominator == 0 {
        return None;
    }
    let p = Fraction::new(a[1][1][1] - a[1][0][1], row_denominator);
    let q = Fraction::new(a[1][1][0] - a[0][1][0], col_denominator);
    if p.n <= 0 || p.n >= p.d || q.n <= 0 || q.n >= q.d {
        return None;
    }
    Some((
        vec![p, Fraction::new(p.d - p.n, p.d)],
        vec![q, Fraction::new(q.d - q.n, q.d)],
    ))
}

fn pareto(game: &Game) -> Vec<(usize, usize)> {
    let n = game.row.len();
    (0..n)
        .flat_map(|r| (0..n).map(move |c| (r, c)))
        .filter(|&(r, c)| {
            let p = game.cells[r][c];
            !game
                .cells
                .iter()
                .flatten()
                .any(|q| q[0] >= p[0] && q[1] >= p[1] && q != &p)
        })
        .collect()
}

fn cell_names(game: &Game, cells: &[(usize, usize)]) -> String {
    if cells.is_empty() {
        "none".into()
    } else {
        cells
            .iter()
            .map(|&(r, c)| format!("({}, {})", game.row[r], game.col[c]))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn distribution(actions: &[&str], probabilities: &[Fraction]) -> String {
    actions
        .iter()
        .zip(probabilities)
        .map(|(a, p)| format!("{a} {}", p.text()))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn add(a: Fraction, b: Fraction) -> Fraction {
        Fraction::new(a.n * b.d + b.n * a.d, a.d * b.d)
    }

    #[test]
    fn every_mixed_equilibrium_is_normalized_and_makes_all_actions_indifferent() {
        let mut rng = StdRng::seed_from_u64(104729);
        for regime in 0..6 {
            for _ in 0..500 {
                let game = sample(&mut rng, regime);
                let pure = pure_equilibria(&game);
                assert_eq!(pure.len(), [1, 2, 2, 2, 0, 0][regime]);
                let mixed = mixed_equilibrium(&game);
                assert_eq!(mixed.is_some(), regime != 0);
                if let Some((row, col)) = mixed {
                    assert_eq!(
                        row.iter().copied().reduce(add).unwrap(),
                        Fraction::new(1, 1)
                    );
                    assert_eq!(
                        col.iter().copied().reduce(add).unwrap(),
                        Fraction::new(1, 1)
                    );
                    assert!(row.iter().chain(&col).all(|p| p.n > 0 && p.n < p.d));
                    let row_values: Vec<_> = game
                        .cells
                        .iter()
                        .map(|cells| {
                            cells
                                .iter()
                                .zip(&col)
                                .map(|(cell, p)| Fraction::new(cell[0] * p.n, p.d))
                                .reduce(add)
                                .unwrap()
                        })
                        .collect();
                    let col_values: Vec<_> = (0..game.col.len())
                        .map(|c| {
                            game.cells
                                .iter()
                                .zip(&row)
                                .map(|(cells, p)| Fraction::new(cells[c][1] * p.n, p.d))
                                .reduce(add)
                                .unwrap()
                        })
                        .collect();
                    assert!(row_values.iter().all(|v| v == &row_values[0]));
                    assert!(col_values.iter().all(|v| v == &col_values[0]));
                    if regime >= 4 {
                        for cells in &game.cells {
                            for cell in cells {
                                assert_eq!(cell[0] + cell[1], 0);
                            }
                        }
                        assert_eq!(add(row_values[0], col_values[0]), Fraction::new(0, 1));
                    }
                }
            }
        }
    }

    #[test]
    fn stag_hunt_crosses_the_risk_dominance_boundary_and_prisoners_dilemma_is_inefficient() {
        let mut rng = StdRng::seed_from_u64(901);
        let mut risk = [false; 3];
        for _ in 0..500 {
            let game = sample(&mut rng, 1);
            let (row, _) = mixed_equilibrium(&game).unwrap();
            risk[match (2 * row[0].n).cmp(&row[0].d) {
                std::cmp::Ordering::Less => 0,
                std::cmp::Ordering::Equal => 1,
                std::cmp::Ordering::Greater => 2,
            }] = true;
            let dilemma = sample(&mut rng, 0);
            assert_eq!(pure_equilibria(&dilemma), vec![(1, 1)]);
            assert!(!pareto(&dilemma).contains(&(1, 1)));
        }
        assert!(risk.iter().all(|x| *x));
    }
}
