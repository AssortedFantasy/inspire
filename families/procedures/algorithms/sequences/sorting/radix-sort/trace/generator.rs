use rand::{Rng, rngs::StdRng, seq::SliceRandom};
use std::fmt::Write;

type Item = (u32, char);

#[derive(Debug)]
struct Pass {
    place: u32,
    buckets: Vec<Vec<Item>>,
    result: Vec<Item>,
}

fn radix_sort(data: &[Item], base: u32, width: u32) -> Vec<Pass> {
    let mut data = data.to_vec();
    let mut passes = Vec::new();
    for digit in 0..width {
        let place = base.pow(digit);
        let mut buckets = vec![Vec::new(); base as usize];
        for &item in &data {
            buckets[((item.0 / place) % base) as usize].push(item);
        }
        data = buckets.iter().flatten().copied().collect();
        passes.push(Pass {
            place,
            buckets,
            result: data.clone(),
        });
    }
    passes
}

fn input(rng: &mut StdRng, base: u32, width: u32, regime: usize) -> Vec<Item> {
    let limit = base.pow(width);
    let n = rng.random_range(7..=11);
    let suffix = rng.random_range(0..base);
    let mut keys: Vec<_> = match regime {
        0 => (0..n).map(|_| rng.random_range(0..limit)).collect(),
        1 => (0..n).map(|_| rng.random_range(0..base)).collect(),
        2 => (0..n)
            .map(|_| rng.random_range(0..limit / base) * base + suffix)
            .collect(),
        3 => {
            let a = rng.random_range(0..limit);
            let b = rng.random_range(0..limit);
            (0..n).map(|i| if i % 3 == 0 { a } else { b }).collect()
        }
        _ => (0..n)
            .map(|i| [0, limit - 1, base - 1, base, limit / 2][i % 5])
            .collect(),
    };
    keys.shuffle(rng);
    keys.into_iter()
        .enumerate()
        .map(|(i, key)| (key, (b'a' + i as u8) as char))
        .collect()
}

fn display(data: &[Item], base: u32, width: u32) -> String {
    data.iter()
        .map(|&(mut key, id)| {
            let mut digits = vec!['0'; width as usize];
            for digit in digits.iter_mut().rev() {
                *digit = char::from_digit(key % base, base)
                    .unwrap()
                    .to_ascii_uppercase();
                key /= base;
            }
            format!("{}({id})", digits.into_iter().collect::<String>())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn generate(rng: &mut StdRng) -> String {
    let base = [2, 10, 16][rng.random_range(0..3)];
    let width = rng.random_range(2..=3);
    let regime = rng.random_range(0..5);
    let data = input(rng, base, width, regime);
    let mut text = format!(
        "LSD radix sort, base {base}, width {width}\nInput {}",
        display(&data, base, width)
    );
    let passes = radix_sort(&data, base, width);
    for pass in &passes {
        write!(text, "\n\nDigit at place {}", pass.place).unwrap();
        for (digit, bucket) in pass
            .buckets
            .iter()
            .enumerate()
            .filter(|(_, b)| !b.is_empty())
        {
            write!(
                text,
                "\n{}  {}",
                char::from_digit(digit as u32, base)
                    .unwrap()
                    .to_ascii_uppercase(),
                bucket
                    .iter()
                    .map(|item| item.1.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
            .unwrap();
        }
    }
    write!(
        text,
        "\nSorted {}",
        display(&passes.last().unwrap().result, base, width)
    )
    .unwrap();
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn every_pass_is_a_stable_sort_by_processed_digits() {
        let mut rng = StdRng::seed_from_u64(314159);
        for base in [2, 10, 16] {
            for width in 2..=3 {
                for regime in 0..5 {
                    for _ in 0..64 {
                        let data = input(&mut rng, base, width, regime);
                        let passes = radix_sort(&data, base, width);
                        let mut previous = data.clone();
                        for pass in &passes {
                            let mut expected = data.clone();
                            expected.sort_by_key(|item| item.0 % (pass.place * base));
                            assert_eq!(pass.result, expected);
                            for (digit, bucket) in pass.buckets.iter().enumerate() {
                                let expected: Vec<_> = previous
                                    .iter()
                                    .copied()
                                    .filter(|item| (item.0 / pass.place) % base == digit as u32)
                                    .collect();
                                assert_eq!(*bucket, expected);
                            }
                            previous = pass.result.clone();
                        }
                        let mut expected = data;
                        expected.sort_by_key(|item| item.0);
                        assert_eq!(passes.last().unwrap().result, expected);
                    }
                }
            }
        }
    }
}
