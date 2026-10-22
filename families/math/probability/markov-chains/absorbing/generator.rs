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
        let sign = if d < 0 { -1 } else { 1 };
        Self {
            n: n / a * sign,
            d: d / a * sign,
        }
    }

    fn sub(self, other: Self) -> Self {
        Self::new(self.n * other.d - other.n * self.d, self.d * other.d)
    }

    fn mul(self, other: Self) -> Self {
        Self::new(self.n * other.n, self.d * other.d)
    }

    fn div(self, other: Self) -> Self {
        Self::new(self.n * other.d, self.d * other.n)
    }

    fn text(self) -> String {
        if self.d == 1 {
            self.n.to_string()
        } else {
            format!("{}/{}", self.n, self.d)
        }
    }
}

struct Chain {
    denominator: i64,
    rows: Vec<Vec<i64>>,
}

pub fn generate(rng: &mut StdRng) -> String {
    let chain = sample(rng);
    let answers = solve(&chain);
    let n = chain.rows.len();
    let names: Vec<_> = (0..n)
        .map(|i| char::from(b'A' + i as u8).to_string())
        .chain(["U".into(), "V".into()])
        .collect();
    let mut lines = vec![format!("P [{}]", names.join(", "))];
    for (i, row) in chain.rows.iter().enumerate() {
        lines.push(format!(
            "{} [{}]",
            names[i],
            row.iter()
                .map(|&w| Fraction::new(w, chain.denominator).text())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    for absorbing in 0..2 {
        lines.push(format!(
            "{} [{}]",
            names[n + absorbing],
            (0..n + 2)
                .map(|j| if j == n + absorbing { "1" } else { "0" })
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if rng.random_bool(0.5) {
        lines.push("\nAbsorption probabilities [U, V]".into());
        for (i, answer) in answers.iter().enumerate() {
            lines.push(format!(
                "{} [{}, {}]",
                names[i],
                answer[0].text(),
                answer[1].text()
            ));
        }
    } else {
        lines.push("\nExpected steps to absorption".into());
        for (i, answer) in answers.iter().enumerate() {
            lines.push(format!("{} {}", names[i], answer[2].text()));
        }
    }
    lines.join("\n")
}

fn sample(rng: &mut StdRng) -> Chain {
    let n = rng.random_range(2..=4);
    let denominator = rng.random_range(6..=12);
    let rows = (0..n)
        .map(|i| {
            let mut weights = vec![0; n + 2];
            let exit = rng.random_range(1..=denominator / 2);
            weights[n + i % 2] = exit;
            let targets: Vec<_> = (0..n).filter(|_| rng.random_bool(0.5)).collect();
            let targets = if targets.is_empty() {
                vec![rng.random_range(0..n)]
            } else {
                targets
            };
            for _ in exit..denominator {
                weights[targets[rng.random_range(0..targets.len())]] += 1;
            }
            weights
        })
        .collect();
    Chain { denominator, rows }
}

fn solve(chain: &Chain) -> Vec<Vec<Fraction>> {
    let n = chain.rows.len();
    let mut matrix: Vec<Vec<_>> = chain
        .rows
        .iter()
        .enumerate()
        .map(|(i, row)| {
            (0..n)
                .map(|j| Fraction::new(if i == j { chain.denominator } else { 0 } - row[j], 1))
                .chain([
                    Fraction::new(row[n], 1),
                    Fraction::new(row[n + 1], 1),
                    Fraction::new(chain.denominator, 1),
                ])
                .collect()
        })
        .collect();
    for pivot in 0..n {
        let row = (pivot..n).find(|&i| matrix[i][pivot].n != 0).unwrap();
        matrix.swap(pivot, row);
        let divisor = matrix[pivot][pivot];
        for value in &mut matrix[pivot][pivot..] {
            *value = value.div(divisor);
        }
        let pivot_row = matrix[pivot].clone();
        for (i, row) in matrix.iter_mut().enumerate() {
            if i == pivot {
                continue;
            }
            let multiplier = row[pivot];
            for (value, &source) in row[pivot..].iter_mut().zip(&pivot_row[pivot..]) {
                *value = value.sub(multiplier.mul(source));
            }
        }
    }
    matrix.into_iter().map(|row| row[n..].to_vec()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn exact_answers_obey_first_step_equations_and_probability_bounds() {
        let mut rng = StdRng::seed_from_u64(296);
        for _ in 0..2_000 {
            let chain = sample(&mut rng);
            let n = chain.rows.len();
            let answers = solve(&chain);
            for (i, answer) in answers.iter().enumerate() {
                assert_eq!(chain.rows[i].iter().sum::<i64>(), chain.denominator);
                assert!(chain.rows[i][n..].iter().sum::<i64>() > 0);
                assert_eq!(
                    answer[0].n * answer[1].d + answer[1].n * answer[0].d,
                    answer[0].d * answer[1].d
                );
                assert!(answer[..2].iter().all(|p| 0 <= p.n && p.n <= p.d));
                assert!(answer[2].n >= answer[2].d);
                assert!(answer[2].n <= chain.denominator * answer[2].d);
                for k in 0..3 {
                    let constant = if k < 2 {
                        chain.rows[i][n + k]
                    } else {
                        chain.denominator
                    };
                    let mut residual = answer[k]
                        .mul(Fraction::new(chain.denominator, 1))
                        .sub(Fraction::new(constant, 1));
                    for (j, other) in answers.iter().enumerate() {
                        residual = residual.sub(other[k].mul(Fraction::new(chain.rows[i][j], 1)));
                    }
                    assert_eq!(residual.n, 0);
                }
            }
        }
    }

    #[test]
    fn independent_probability_flow_matches_every_answer() {
        let mut rng = StdRng::seed_from_u64(319);
        for _ in 0..100 {
            let chain = sample(&mut rng);
            let n = chain.rows.len();
            let answers = solve(&chain);
            for (start, answer) in answers.iter().enumerate() {
                let mut mass = vec![0.0; n];
                mass[start] = 1.0;
                let (mut u, mut v, mut steps) = (0.0, 0.0, 0.0);
                for _ in 0..500 {
                    steps += mass.iter().sum::<f64>();
                    let mut next = vec![0.0; n];
                    for (i, &weight) in mass.iter().enumerate() {
                        let scale = weight / chain.denominator as f64;
                        u += scale * chain.rows[i][n] as f64;
                        v += scale * chain.rows[i][n + 1] as f64;
                        for (j, value) in next.iter_mut().enumerate() {
                            *value += scale * chain.rows[i][j] as f64;
                        }
                    }
                    mass = next;
                }
                for (actual, expected) in [u, v, steps].iter().zip(answer) {
                    assert!((actual - expected.n as f64 / expected.d as f64).abs() < 1e-10);
                }
            }
        }
    }
}
