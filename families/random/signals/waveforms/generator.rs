use rand::{Rng, rngs::StdRng, seq::SliceRandom};

pub fn generate(rng: &mut StdRng) -> String {
    let x = sample(rng);
    if rng.random_bool(0.5) {
        format!(
            "x = {}\nH_{} x = {}",
            vector(&x, 1),
            x.len(),
            vector(&hadamard(&x), 1)
        )
    } else {
        let levels = x.len().ilog2();
        format!(
            "x = {}\nHaar [a{levels}, d{levels}, …, d1] = {}",
            vector(&x, 1),
            vector(&haar(&x), 16)
        )
    }
}

fn sample(rng: &mut StdRng) -> Vec<i64> {
    let n = if rng.random_bool(0.5) { 8 } else { 16 };
    let amplitude = rng.random_range(1..=7);
    match rng.random_range(0..7) {
        0 => {
            let edge = rng.random_range(1..n);
            let baseline = rng.random_range(-4..=4);
            (0..n)
                .map(|i| baseline + if i < edge { 0 } else { amplitude })
                .collect()
        }
        1 => {
            let mut x = vec![0; n];
            x[rng.random_range(0..n)] = amplitude * if rng.random_bool(0.5) { 1 } else { -1 };
            x
        }
        2 => (0..n)
            .map(|i| if i % 2 == 0 { amplitude } else { -amplitude })
            .collect(),
        3 => {
            let slope = if rng.random_bool(0.5) { 1 } else { -1 };
            (0..n).map(|i| slope * (i as i64 - n as i64 / 2)).collect()
        }
        4 => {
            let mut coefficients = vec![0; n];
            coefficients[0] = rng.random_range(-3..=3);
            let mut positions: Vec<_> = (1..n).collect();
            positions.shuffle(rng);
            for &position in &positions[..rng.random_range(1..=3)] {
                coefficients[position] =
                    rng.random_range(1..=5) * if rng.random_bool(0.5) { 1 } else { -1 };
            }
            inverse_haar(&coefficients)
        }
        5 => {
            let start = rng.random_range(1..n - 1);
            let end = rng.random_range(start + 1..=n);
            (0..n)
                .map(|i| {
                    if (start..end).contains(&i) {
                        amplitude
                    } else {
                        0
                    }
                })
                .collect()
        }
        _ => (0..n).map(|_| rng.random_range(-5..=5)).collect(),
    }
}

fn hadamard(x: &[i64]) -> Vec<i64> {
    let mut y = x.to_vec();
    let mut width = 1;
    while width < y.len() {
        for block in y.chunks_mut(2 * width) {
            for i in 0..width {
                let (a, b) = (block[i], block[i + width]);
                block[i] = a + b;
                block[i + width] = a - b;
            }
        }
        width *= 2;
    }
    y
}

// Sixteen is a common exact denominator for every supported Haar level.
fn haar(x: &[i64]) -> Vec<i64> {
    let mut y: Vec<_> = x.iter().map(|v| v * 16).collect();
    let mut width = y.len();
    while width > 1 {
        let mut next = vec![0; width];
        for (i, pair) in y[..width].as_chunks::<2>().0.iter().enumerate() {
            next[i] = (pair[0] + pair[1]) / 2;
            next[i + width / 2] = (pair[0] - pair[1]) / 2;
        }
        y[..width].copy_from_slice(&next);
        width /= 2;
    }
    y
}

fn inverse_haar(coefficients: &[i64]) -> Vec<i64> {
    let mut y = coefficients.to_vec();
    let mut width = 1;
    while width < y.len() {
        let mut next = vec![0; 2 * width];
        for i in 0..width {
            next[2 * i] = y[i] + y[i + width];
            next[2 * i + 1] = y[i] - y[i + width];
        }
        y[..2 * width].copy_from_slice(&next);
        width *= 2;
    }
    y
}

fn vector(values: &[i64], denominator: i64) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|&v| {
                let mut divisor = denominator;
                while divisor > 1 && v % divisor != 0 {
                    divisor /= 2;
                }
                if denominator / divisor == 1 {
                    (v / divisor).to_string()
                } else {
                    format!("{}/{}", v / divisor, denominator / divisor)
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn butterfly_matches_the_sylvester_matrix_and_parseval() {
        let mut rng = StdRng::seed_from_u64(809);
        for _ in 0..1000 {
            let x = sample(&mut rng);
            let y = hadamard(&x);
            for (k, &value) in y.iter().enumerate() {
                let direct: i64 = x
                    .iter()
                    .enumerate()
                    .map(|(i, &v)| if (i & k).count_ones() % 2 == 0 { v } else { -v })
                    .sum();
                assert_eq!(value, direct);
            }
            assert_eq!(
                hadamard(&y),
                x.iter().map(|v| v * x.len() as i64).collect::<Vec<_>>()
            );
            assert_eq!(
                y.iter().map(|v| v * v).sum::<i64>(),
                x.len() as i64 * x.iter().map(|v| v * v).sum::<i64>()
            );
        }
    }

    #[test]
    fn haar_is_exact_and_each_coefficient_matches_its_support() {
        let mut rng = StdRng::seed_from_u64(817);
        for _ in 0..1000 {
            let x = sample(&mut rng);
            let y = haar(&x);
            assert_eq!(
                inverse_haar(&y),
                x.iter().map(|v| v * 16).collect::<Vec<_>>()
            );
            assert_eq!(y[0] * x.len() as i64, 16 * x.iter().sum::<i64>());
            let mut count = 1;
            while count < x.len() {
                let support = x.len() / count;
                for i in 0..count {
                    let chunk = &x[i * support..(i + 1) * support];
                    let signed_sum = chunk[..support / 2].iter().sum::<i64>()
                        - chunk[support / 2..].iter().sum::<i64>();
                    assert_eq!(y[count + i] * support as i64, 16 * signed_sum);
                }
                count *= 2;
            }
        }
    }
}
