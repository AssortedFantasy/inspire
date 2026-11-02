use rand::{Rng, rngs::StdRng, seq::SliceRandom};
use std::fmt::Write;

#[derive(Debug)]
struct Step {
    index: usize,
    before: usize,
    fallbacks: Vec<usize>,
    reached: usize,
    resume: usize,
    found: Option<usize>,
}

fn prefix(pattern: &[u8]) -> Vec<usize> {
    let mut table = vec![0; pattern.len()];
    for i in 1..pattern.len() {
        let mut q = table[i - 1];
        while q > 0 && pattern[q] != pattern[i] {
            q = table[q - 1];
        }
        if pattern[q] == pattern[i] {
            q += 1;
        }
        table[i] = q;
    }
    table
}

fn search(text: &[u8], pattern: &[u8], table: &[usize]) -> (Vec<usize>, Vec<Step>) {
    assert!(!pattern.is_empty());
    let mut q = 0;
    let mut matches = Vec::new();
    let mut trace = Vec::new();
    for (index, &symbol) in text.iter().enumerate() {
        let before = q;
        let mut fallbacks = Vec::new();
        while q > 0 && pattern[q] != symbol {
            q = table[q - 1];
            fallbacks.push(q);
        }
        if pattern[q] == symbol {
            q += 1;
        }
        let reached = q;
        let found = if q == pattern.len() {
            let start = index + 1 - pattern.len();
            matches.push(start);
            q = table[q - 1];
            Some(start)
        } else {
            None
        };
        trace.push(Step {
            index,
            before,
            fallbacks,
            reached,
            resume: q,
            found,
        });
    }
    (matches, trace)
}

fn instance(rng: &mut StdRng, regime: usize) -> (Vec<u8>, Vec<u8>) {
    let mut alphabet = *b"ACGT";
    alphabet.shuffle(rng);
    let (a, b, c) = (alphabet[0], alphabet[1], alphabet[2]);
    match regime {
        0 => {
            let pattern: Vec<_> = (0..rng.random_range(4..=8))
                .map(|_| alphabet[rng.random_range(0..4)])
                .collect();
            let mut text: Vec<_> = (0..rng.random_range(18..=26))
                .map(|_| alphabet[rng.random_range(0..4)])
                .collect();
            if rng.random_bool(0.6) {
                let start = rng.random_range(0..=text.len() - pattern.len());
                text[start..start + pattern.len()].copy_from_slice(&pattern);
            }
            (text, pattern)
        }
        1 => {
            let pattern = vec![a; rng.random_range(3..=6)];
            let mut text = vec![a; rng.random_range(12..=20)];
            let middle = text.len() / 2;
            if rng.random_bool(0.5) {
                text[middle] = b;
            }
            (text, pattern)
        }
        2 => {
            let unit = if rng.random_bool(0.5) {
                vec![a, b]
            } else {
                vec![a, a, b]
            };
            let pattern = unit.repeat(rng.random_range(2..=3));
            let text = unit.repeat(rng.random_range(5..=7));
            (text, pattern)
        }
        3 => {
            let root = [a, b].repeat(rng.random_range(1..=2));
            let mut pattern = root.clone();
            pattern.push(c);
            pattern.extend(&root);
            let mut text = root.repeat(2);
            text.push(b);
            text.extend(&pattern);
            text.extend(&root);
            (text, pattern)
        }
        _ => {
            let mut pattern = vec![a; rng.random_range(3..=7)];
            pattern.push(b);
            (vec![a; rng.random_range(15..=25)], pattern)
        }
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..5);
    let (text, pattern) = instance(rng, regime);
    let table = prefix(&pattern);
    let (matches, trace) = search(&text, &pattern, &table);
    let mut out = format!(
        "Text    {}\nPattern {}\nKMP prefix lengths {table:?}\n\ni char matched length / fallback / resume after match",
        String::from_utf8_lossy(&text),
        String::from_utf8_lossy(&pattern)
    );
    for step in trace {
        let fallbacks = step
            .fallbacks
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join("→");
        let found = step
            .found
            .map(|_| format!("resume {}", step.resume))
            .unwrap_or_default();
        let row = format!(
            "{} {} {}→{}  {}  {}",
            step.index, text[step.index] as char, step.before, step.reached, fallbacks, found
        );
        write!(out, "\n{}", row.trim_end()).unwrap();
    }
    write!(out, "\nMatches at {matches:?}").unwrap();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn check(text: &[u8], pattern: &[u8]) {
        let table = prefix(pattern);
        for (i, &value) in table.iter().enumerate() {
            let expected = (0..=i)
                .rev()
                .find(|&length| pattern[..length] == pattern[i + 1 - length..i + 1])
                .unwrap();
            assert_eq!(value, expected);
        }
        let (matches, trace) = search(text, pattern, &table);
        let expected: Vec<_> = text
            .windows(pattern.len())
            .enumerate()
            .filter_map(|(i, window)| (window == pattern).then_some(i))
            .collect();
        assert_eq!(matches, expected);
        for step in trace {
            let consumed = &text[..=step.index];
            let expected = (0..pattern.len())
                .rev()
                .find(|&length| consumed.ends_with(&pattern[..length]))
                .unwrap();
            assert_eq!(step.resume, expected);
            assert!(step.fallbacks.windows(2).all(|pair| pair[0] > pair[1]));
        }
    }

    fn binary(code: usize, length: usize) -> Vec<u8> {
        (0..length)
            .map(|i| b'A' + ((code >> i) & 1) as u8)
            .collect()
    }

    #[test]
    fn exhaustive_matches_borders_and_resume_suffixes() {
        for p_len in 1..=5 {
            for p_bits in 0..1 << p_len {
                let pattern = binary(p_bits, p_len);
                for t_len in 0..=7 {
                    for t_bits in 0..1 << t_len {
                        check(&binary(t_bits, t_len), &pattern);
                    }
                }
            }
        }
        let mut rng = StdRng::seed_from_u64(271828);
        for regime in 0..5 {
            for _ in 0..100 {
                let (text, pattern) = instance(&mut rng, regime);
                check(&text, &pattern);
            }
        }
    }
}
