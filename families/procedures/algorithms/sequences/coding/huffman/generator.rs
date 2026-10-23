use rand::{Rng, rngs::StdRng, seq::SliceRandom};
use std::fmt::Write;

#[derive(Debug)]
struct Node {
    weight: usize,
    symbols: String,
    children: Option<(usize, usize)>,
}

#[derive(Debug)]
struct Merge {
    left: String,
    right: String,
    left_weight: usize,
    right_weight: usize,
}

fn codebook(frequencies: &[(u8, usize)]) -> (Vec<(u8, String)>, Vec<Merge>) {
    assert!(frequencies.len() >= 2);
    let mut nodes: Vec<_> = frequencies
        .iter()
        .map(|&(symbol, weight)| Node {
            weight,
            symbols: (symbol as char).to_string(),
            children: None,
        })
        .collect();
    let mut queue: Vec<_> = (0..nodes.len()).collect();
    let mut merges = Vec::new();
    while queue.len() > 1 {
        queue.sort_by(|&a, &b| {
            nodes[a]
                .weight
                .cmp(&nodes[b].weight)
                .then(nodes[a].symbols.cmp(&nodes[b].symbols))
        });
        let left = queue.remove(0);
        let right = queue.remove(0);
        let mut symbols: Vec<_> = nodes[left]
            .symbols
            .chars()
            .chain(nodes[right].symbols.chars())
            .collect();
        symbols.sort();
        merges.push(Merge {
            left: nodes[left].symbols.clone(),
            right: nodes[right].symbols.clone(),
            left_weight: nodes[left].weight,
            right_weight: nodes[right].weight,
        });
        queue.push(nodes.len());
        nodes.push(Node {
            weight: nodes[left].weight + nodes[right].weight,
            symbols: symbols.into_iter().collect(),
            children: Some((left, right)),
        });
    }
    fn visit(nodes: &[Node], index: usize, bits: String, codes: &mut Vec<(u8, String)>) {
        if let Some((left, right)) = nodes[index].children {
            visit(nodes, left, format!("{bits}0"), codes);
            visit(nodes, right, format!("{bits}1"), codes);
        } else {
            codes.push((nodes[index].symbols.as_bytes()[0], bits));
        }
    }
    let mut codes = Vec::new();
    visit(&nodes, queue[0], String::new(), &mut codes);
    codes.sort_by_key(|item| item.0);
    (codes, merges)
}

fn instance(rng: &mut StdRng, regime: usize) -> (Vec<(u8, usize)>, Vec<u8>) {
    let n = rng.random_range(4..=7);
    let counts = match regime {
        0 => (0..n).map(|_| rng.random_range(1..=8)).collect::<Vec<_>>(),
        1 => vec![rng.random_range(2..=5); n],
        2 => {
            let mut counts: Vec<_> = (0..n).map(|_| rng.random_range(1..=3)).collect();
            counts[0] = rng.random_range(18..=28);
            counts
        }
        3 => {
            let mut counts = vec![1, 1];
            for i in 2..n {
                counts.push(counts[i - 1] + counts[i - 2]);
            }
            counts
        }
        _ => {
            let common = rng.random_range(3..=6);
            (0..n).map(|i| common + usize::from(i % 3 == 0)).collect()
        }
    };
    let mut symbols = *b"ABCDEFGH";
    symbols.shuffle(rng);
    let mut frequencies: Vec<_> = counts
        .into_iter()
        .enumerate()
        .map(|(i, count)| (symbols[i], count))
        .collect();
    frequencies.sort_by_key(|item| item.0);
    let mut message: Vec<_> = frequencies
        .iter()
        .flat_map(|&(symbol, count)| std::iter::repeat_n(symbol, count))
        .collect();
    message.shuffle(rng);
    (frequencies, message)
}

fn encode(message: &[u8], codes: &[(u8, String)]) -> String {
    message
        .iter()
        .map(|symbol| {
            codes
                .iter()
                .find(|item| item.0 == *symbol)
                .unwrap()
                .1
                .as_str()
        })
        .collect()
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..5);
    let (frequencies, message) = instance(rng, regime);
    let (codes, merges) = codebook(&frequencies);
    let mut out = format!(
        "Message {}\nHuffman merges (left 0, right 1)",
        String::from_utf8_lossy(&message)
    );
    for merge in merges {
        write!(
            out,
            "\n{}({}) + {}({}) → {}",
            merge.left,
            merge.left_weight,
            merge.right,
            merge.right_weight,
            merge.left_weight + merge.right_weight
        )
        .unwrap();
    }
    out.push_str("\n\nSymbol  Count  Code");
    for ((symbol, count), (_, bits)) in frequencies.iter().zip(&codes) {
        write!(out, "\n{}       {:>2}     {}", *symbol as char, count, bits).unwrap();
    }
    let bits = encode(&message, &codes);
    let fixed_width = usize::BITS - (codes.len() - 1).leading_zeros();
    write!(
        out,
        "\n\n{bits}\nPayload {} bits\nFixed-width payload {} bits",
        bits.len(),
        message.len() * fixed_width as usize
    )
    .unwrap();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn optimal_cost(weights: &[usize]) -> usize {
        // Enumerate every binary root split of every symbol subset.
        let size = 1_usize << weights.len();
        let mut costs = vec![0; size];
        for mask in 1..size {
            if mask.count_ones() == 1 {
                continue;
            }
            let sum: usize = weights
                .iter()
                .enumerate()
                .filter(|(i, _)| mask & (1 << i) != 0)
                .map(|(_, weight)| weight)
                .sum();
            let anchor = mask.isolate_lowest_one();
            let mut part = (mask - 1) & mask;
            let mut best = usize::MAX;
            while part > 0 {
                if part & anchor != 0 {
                    best = best.min(costs[part] + costs[mask ^ part]);
                }
                part = (part - 1) & mask;
            }
            costs[mask] = sum + best;
        }
        costs[size - 1]
    }

    #[test]
    fn prefix_free_optimal_codes_and_exact_roundtrip() {
        let mut rng = StdRng::seed_from_u64(20261023);
        for regime in 0..5 {
            for _ in 0..100 {
                let (frequencies, message) = instance(&mut rng, regime);
                let (codes, merges) = codebook(&frequencies);
                for (i, (_, code)) in codes.iter().enumerate() {
                    assert!(!code.is_empty());
                    for (j, (_, other)) in codes.iter().enumerate() {
                        if i != j {
                            assert!(!other.starts_with(code));
                        }
                    }
                }
                let weights: Vec<_> = frequencies.iter().map(|item| item.1).collect();
                let cost: usize = frequencies
                    .iter()
                    .zip(&codes)
                    .map(|((_, count), (_, code))| count * code.len())
                    .sum();
                assert_eq!(cost, optimal_cost(&weights));
                assert_eq!(
                    cost,
                    merges.iter().map(|m| m.left_weight + m.right_weight).sum()
                );
                let max_len = codes.iter().map(|item| item.1.len()).max().unwrap();
                assert_eq!(
                    codes
                        .iter()
                        .map(|item| 1_usize << (max_len - item.1.len()))
                        .sum::<usize>(),
                    1 << max_len
                );
                let encoded = encode(&message, &codes);
                assert_eq!(cost, encoded.len());
                let mut pending = String::new();
                let mut decoded = Vec::new();
                for bit in encoded.chars() {
                    pending.push(bit);
                    if let Some((symbol, _)) = codes.iter().find(|item| item.1 == pending) {
                        decoded.push(*symbol);
                        pending.clear();
                    }
                }
                assert!(pending.is_empty());
                assert_eq!(decoded, message);
                for (symbol, count) in frequencies {
                    assert_eq!(decoded.iter().filter(|&&x| x == symbol).count(), count);
                }
            }
        }
    }
}
