use rand::{Rng, rngs::StdRng, seq::SliceRandom};

type Square = Vec<Vec<usize>>;

fn siamese(n: usize) -> Square {
    let mut square = vec![vec![0; n]; n];
    let (mut row, mut col) = (0, n / 2);
    for value in 1..=n * n {
        square[row][col] = value;
        let next = ((row + n - 1) % n, (col + 1) % n);
        if square[next.0][next.1] == 0 {
            (row, col) = next;
        } else {
            row = (row + 1) % n;
        }
    }
    square
}

fn doubly_even(n: usize) -> Square {
    (0..n)
        .map(|row| {
            (0..n)
                .map(|col| {
                    let value = row * n + col + 1;
                    if row % 4 == col % 4 || row % 4 + col % 4 == 3 {
                        n * n + 1 - value
                    } else {
                        value
                    }
                })
                .collect()
        })
        .collect()
}

fn singly_even(n: usize) -> Square {
    let half = n / 2;
    let base = siamese(half);
    let block = half * half;
    let mut square = vec![vec![0; n]; n];
    for row in 0..half {
        for col in 0..half {
            let value = base[row][col];
            square[row][col] = value;
            square[row][col + half] = value + 2 * block;
            square[row + half][col] = value + 3 * block;
            square[row + half][col + half] = value + block;
        }
    }
    let k = (half - 1) / 2;
    for row in 0..half {
        for col in (0..k).chain(n - k + 1..n) {
            let value = square[row][col];
            square[row][col] = square[row + half][col];
            square[row + half][col] = value;
        }
    }
    for col in [0, k] {
        let value = square[k][col];
        square[k][col] = square[k + half][col];
        square[k + half][col] = value;
    }
    square
}

fn pandiagonal(n: usize, a: usize, b: usize, x: usize, y: usize) -> Square {
    (0..n)
        .map(|row| {
            (0..n)
                .map(|col| n * ((row + a * col + x) % n) + (row + b * col + y) % n + 1)
                .collect()
        })
        .collect()
}

fn permute_symmetric(square: &Square, rng: &mut StdRng) -> Square {
    let n = square.len();
    let mut pairs: Vec<_> = (0..n / 2).collect();
    pairs.shuffle(rng);
    let mut permutation = vec![n / 2; n];
    for (i, pair) in pairs.into_iter().enumerate() {
        let value = if rng.random() { pair } else { n - 1 - pair };
        permutation[i] = value;
        permutation[n - 1 - i] = n - 1 - value;
    }
    (0..n)
        .map(|row| {
            (0..n)
                .map(|col| square[permutation[row]][permutation[col]])
                .collect()
        })
        .collect()
}

fn orient(square: &Square, rotation: usize, reflect: bool, complement: bool) -> Square {
    let n = square.len();
    (0..n)
        .map(|row| {
            (0..n)
                .map(|col| {
                    let (mut r, mut c) = (row, if reflect { n - 1 - col } else { col });
                    for _ in 0..rotation {
                        (r, c) = (c, n - 1 - r);
                    }
                    let value = square[r][c];
                    if complement { n * n + 1 - value } else { value }
                })
                .collect()
        })
        .collect()
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..4);
    let square = match regime {
        0 => siamese([3, 5, 7, 9, 11][rng.random_range(0..5)]),
        1 => doubly_even([4, 8, 12][rng.random_range(0..3)]),
        2 => singly_even([6, 10][rng.random_range(0..2)]),
        _ => {
            let n = [5, 7, 11][rng.random_range(0..3)];
            let a = rng.random_range(2..n - 1);
            let candidates: Vec<_> = (2..n - 1).filter(|&b| b != a).collect();
            let b = candidates[rng.random_range(0..candidates.len())];
            pandiagonal(n, a, b, rng.random_range(0..n), rng.random_range(0..n))
        }
    };
    let square = if regime == 3 {
        square
    } else {
        permute_symmetric(&square, rng)
    };
    let square = orient(&square, rng.random_range(0..4), rng.random(), rng.random());
    let n = square.len();
    let width = (n * n).to_string().len();
    let rows = square
        .iter()
        .map(|row| {
            row.iter()
                .map(|value| format!("{value:>width$}"))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let kind = if regime == 3 { "Pandiagonal" } else { "Normal" };
    let sum = n * (n * n + 1) / 2;
    format!("{kind} magic square of order {n}\n\n{rows}\n\nMagic sum {sum}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn check(square: &Square, broken: bool) {
        let n = square.len();
        let mut entries: Vec<_> = square.iter().flatten().copied().collect();
        entries.sort_unstable();
        assert_eq!(entries, (1..=n * n).collect::<Vec<_>>());
        let target = n * (n * n + 1) / 2;
        for (i, row) in square.iter().enumerate() {
            assert_eq!(row.iter().sum::<usize>(), target);
            assert_eq!((0..n).map(|r| square[r][i]).sum::<usize>(), target);
        }
        assert_eq!((0..n).map(|r| square[r][r]).sum::<usize>(), target);
        assert_eq!((0..n).map(|r| square[r][n - 1 - r]).sum::<usize>(), target);
        if broken {
            for offset in 0..n {
                assert_eq!(
                    (0..n).map(|r| square[r][(r + offset) % n]).sum::<usize>(),
                    target
                );
                assert_eq!(
                    (0..n)
                        .map(|r| square[r][(offset + n - r) % n])
                        .sum::<usize>(),
                    target
                );
            }
        }
    }

    #[test]
    fn all_construction_orders_have_every_normal_magic_property() {
        for n in [3, 5, 7, 9, 11] {
            check(&siamese(n), false);
        }
        for n in [4, 8, 12] {
            check(&doubly_even(n), false);
        }
        for n in [6, 10] {
            check(&singly_even(n), false);
        }
        for n in [5, 7, 11] {
            for a in 2..n - 1 {
                for b in (2..n - 1).filter(|&b| b != a) {
                    for x in 0..n {
                        for y in 0..n {
                            check(&pandiagonal(n, a, b, x, y), true);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn transformations_preserve_all_required_lines() {
        let mut rng = StdRng::seed_from_u64(71);
        for base in [siamese(7), doubly_even(8), singly_even(10)] {
            for _ in 0..24 {
                let square = permute_symmetric(&base, &mut rng);
                for rotation in 0..4 {
                    for reflect in [false, true] {
                        for complement in [false, true] {
                            check(&orient(&square, rotation, reflect, complement), false);
                        }
                    }
                }
            }
        }
        for rotation in 0..4 {
            for reflect in [false, true] {
                for complement in [false, true] {
                    check(
                        &orient(&pandiagonal(7, 2, 4, 3, 5), rotation, reflect, complement),
                        true,
                    );
                }
            }
        }
    }
}
