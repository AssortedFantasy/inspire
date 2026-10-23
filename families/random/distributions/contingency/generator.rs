use rand::{Rng, rngs::StdRng, seq::SliceRandom};

pub fn generate(rng: &mut StdRng) -> String {
    let table = if rng.random_bool(0.6) {
        sample_two(rng)
    } else {
        sample_three(rng)
    };
    let (rows, columns, total) = margins(&table);
    let mut lines = vec![format!(
        "       {}   total",
        (1..=columns.len())
            .map(|j| format!("{:>4}", format!("C{j}")))
            .collect::<Vec<_>>()
            .join(" ")
    )];
    for (i, row) in table.iter().enumerate() {
        lines.push(format!(
            "R{}     {}   {:>5}",
            i + 1,
            row.iter()
                .map(|v| format!("{v:>4}"))
                .collect::<Vec<_>>()
                .join(" "),
            rows[i]
        ));
    }
    lines.push(format!(
        "total  {}   {total:>5}",
        columns
            .iter()
            .map(|v| format!("{v:>4}"))
            .collect::<Vec<_>>()
            .join(" ")
    ));
    lines.push(format!(
        "Pearson χ² = {}\ndf = {}",
        fraction(chi_square(&table)),
        (rows.len() - 1) * (columns.len() - 1)
    ));
    if table.len() == 2 {
        lines.push(format!("Fisher two-sided p = {}", fraction(fisher(&table))));
    }
    lines.join("\n")
}

fn sample_two(rng: &mut StdRng) -> Vec<Vec<u32>> {
    let regime = rng.random_range(0..4);
    if regime == 0 {
        let [a, b, c, d] = std::array::from_fn(|_| rng.random_range(1..=4));
        return vec![vec![a * c, a * d], vec![b * c, b * d]];
    }
    let n: u32 = rng.random_range(8..=60);
    let row = rng.random_range(1..n);
    let column = rng.random_range(1..n);
    let lo = (row + column).saturating_sub(n);
    let hi = row.min(column);
    let a = match regime {
        1 => {
            if rng.random_bool(0.5) {
                lo
            } else {
                hi
            }
        }
        2 => ((row * column + n / 2) / n).clamp(lo, hi),
        _ => rng.random_range(lo..=hi),
    };
    vec![vec![a, row - a], vec![column - a, n + a - row - column]]
}

fn sample_three(rng: &mut StdRng) -> Vec<Vec<u32>> {
    let regime = rng.random_range(0..4);
    let mut table = vec![vec![0; 3]; 3];
    if regime <= 1 {
        let a: [u32; 3] = std::array::from_fn(|_| rng.random_range(1..=3));
        let b: [u32; 3] = std::array::from_fn(|_| rng.random_range(1..=3));
        for i in 0..3 {
            for j in 0..3 {
                table[i][j] = a[i] * b[j];
            }
        }
        if regime == 1 {
            for _ in 0..rng.random_range(1..=5) {
                let mut rows = [0, 1, 2];
                let mut columns = [0, 1, 2];
                rows.shuffle(rng);
                columns.shuffle(rng);
                let [r, s, _] = rows;
                let [c, d, _] = columns;
                let amount = rng.random_range(0..=table[r][d].min(table[s][c]));
                table[r][c] += amount;
                table[s][d] += amount;
                table[r][d] -= amount;
                table[s][c] -= amount;
            }
        }
    } else if regime == 2 {
        for (i, row) in table.iter_mut().enumerate() {
            row[i] = rng.random_range(3..=16);
        }
        for i in 0..3 {
            table[i][(i + 1) % 3] = rng.random_range(0..=2);
        }
    } else {
        let mut rows: Vec<_> = (0..3).map(|_| rng.random_range(1..=15)).collect();
        let total: u32 = rows.iter().sum();
        let first = rng.random_range(1..total - 1);
        let second = rng.random_range(1..total - first);
        let mut columns = [first, second, total - first - second];
        for i in 0..3 {
            for j in 0..3 {
                let amount = rows[i].min(columns[j]);
                table[i][j] = amount;
                rows[i] -= amount;
                columns[j] -= amount;
            }
        }
    }
    table.shuffle(rng);
    let mut order = [0, 1, 2];
    order.shuffle(rng);
    for row in &mut table {
        *row = order.iter().map(|&j| row[j]).collect();
    }
    table
}

fn margins(table: &[Vec<u32>]) -> (Vec<u32>, Vec<u32>, u32) {
    let rows: Vec<u32> = table.iter().map(|r| r.iter().sum()).collect();
    let columns = (0..table[0].len())
        .map(|j| table.iter().map(|r| r[j]).sum())
        .collect();
    let total = rows.iter().sum();
    (rows, columns, total)
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn reduce((n, d): (u128, u128)) -> (u128, u128) {
    let g = gcd(n, d);
    (n / g, d / g)
}

fn fraction(value: (u128, u128)) -> String {
    let (n, d) = reduce(value);
    if d == 1 {
        n.to_string()
    } else {
        format!("{n}/{d}")
    }
}

fn chi_square(table: &[Vec<u32>]) -> (u128, u128) {
    let (rows, columns, total) = margins(table);
    let mut sum = (0, 1);
    for (i, row) in table.iter().enumerate() {
        for (j, &observed) in row.iter().enumerate() {
            let difference = (i128::from(observed) * i128::from(total)
                - i128::from(rows[i]) * i128::from(columns[j]))
            .unsigned_abs();
            let denominator = u128::from(total) * u128::from(rows[i]) * u128::from(columns[j]);
            let divisor = gcd(sum.1, denominator);
            sum = reduce((
                sum.0 * (denominator / divisor) + difference * difference * (sum.1 / divisor),
                sum.1 * (denominator / divisor),
            ));
        }
    }
    sum
}

fn choose(n: u32, k: u32) -> u128 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    (1..=k).fold(1, |value, i| value * u128::from(n - k + i) / u128::from(i))
}

fn fisher(table: &[Vec<u32>]) -> (u128, u128) {
    let (rows, columns, total) = margins(table);
    let weight = |a| choose(columns[0], a) * choose(columns[1], rows[0] - a);
    let observed = weight(table[0][0]);
    let lo = (rows[0] + columns[0]).saturating_sub(total);
    let hi = rows[0].min(columns[0]);
    let numerator = (lo..=hi).map(weight).filter(|&w| w <= observed).sum();
    reduce((numerator, choose(total, rows[0])))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn exact_fisher_agrees_with_enumerated_label_assignments() {
        for n in 4u32..=10 {
            for r in 1..n {
                for c in 1..n {
                    let lo = (r + c).saturating_sub(n);
                    let hi = r.min(c);
                    let mut frequencies = vec![0u128; n as usize + 1];
                    for subset in 0u32..1 << n {
                        if subset.count_ones() == r {
                            let overlap = (subset & ((1 << c) - 1)).count_ones();
                            frequencies[overlap as usize] += 1;
                        }
                    }
                    let assignments: u128 = frequencies.iter().sum();
                    assert_eq!(assignments, choose(n, r));
                    for a in lo..=hi {
                        let table = vec![vec![a, r - a], vec![c - a, n + a - r - c]];
                        let expected = frequencies
                            .iter()
                            .filter(|&&f| f <= frequencies[a as usize])
                            .sum();
                        assert_eq!(fisher(&table), reduce((expected, assignments)));
                        let determinant = i128::from(a) * i128::from(table[1][1])
                            - i128::from(table[0][1]) * i128::from(table[1][0]);
                        assert_eq!(
                            chi_square(&table),
                            reduce((
                                u128::from(n) * determinant.unsigned_abs().pow(2),
                                u128::from(r)
                                    * u128::from(n - r)
                                    * u128::from(c)
                                    * u128::from(n - c)
                            ))
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn generated_tables_have_positive_margins_and_exact_statistics() {
        let mut rng = StdRng::seed_from_u64(590);
        for _ in 0..2000 {
            for table in [sample_two(&mut rng), sample_three(&mut rng)] {
                let (rows, columns, n) = margins(&table);
                assert!(rows.iter().chain(&columns).all(|&x| x > 0));
                let approximate: f64 = table
                    .iter()
                    .enumerate()
                    .flat_map(|(i, row)| {
                        let rows = &rows;
                        let columns = &columns;
                        row.iter().enumerate().map(move |(j, &value)| {
                            let expected =
                                f64::from(rows[i]) * f64::from(columns[j]) / f64::from(n);
                            (f64::from(value) - expected).powi(2) / expected
                        })
                    })
                    .sum();
                let (a, b) = chi_square(&table);
                assert!((a as f64 / b as f64 - approximate).abs() < 1e-10);
                if table.len() == 2 {
                    let (a, b) = fisher(&table);
                    assert!(a > 0 && a <= b);
                }
            }
        }
        assert_eq!(fisher(&[vec![1, 9], vec![11, 3]]), (41, 14858));
    }
}
