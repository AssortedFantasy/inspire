use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

fn execute(items: &[(usize, u32)], capacity: usize) -> (Vec<Vec<u32>>, Vec<usize>) {
    let mut table = vec![vec![0; capacity + 1]; items.len() + 1];
    for (i, &(weight, value)) in items.iter().enumerate() {
        for c in 0..=capacity {
            table[i + 1][c] = table[i][c];
            if weight <= c {
                table[i + 1][c] = table[i + 1][c].max(table[i][c - weight] + value);
            }
        }
    }
    let mut selected = Vec::new();
    let mut c = capacity;
    for i in (1..=items.len()).rev() {
        if table[i][c] != table[i - 1][c] {
            selected.push(i - 1);
            c -= items[i - 1].0;
        }
    }
    selected.reverse();
    (table, selected)
}
fn instance(rng: &mut StdRng) -> (Vec<(usize, u32)>, usize) {
    let items = (0..rng.random_range(4..=7))
        .map(|_| (rng.random_range(2..=9), rng.random_range(3..=20)))
        .collect();
    (items, rng.random_range(8..=17))
}
pub fn generate(rng: &mut StdRng) -> String {
    let (items, capacity) = instance(rng);
    let (table, selected) = execute(&items, capacity);
    let mut text = format!(
        "0/1-knapsack dynamic-programming execution\nCapacity: {capacity}; each item is available once; unused capacity is allowed."
    );
    for (i, (weight, value)) in items.iter().enumerate() {
        write!(
            text,
            "\nItem {}: weight {weight}, value {value}",
            (b'A' + i as u8) as char
        )
        .unwrap();
    }
    write!(text,"\nRows show the best value for capacities 0..{capacity}, after processing each prefix of items.").unwrap();
    for (i, row) in table.iter().enumerate() {
        write!(text, "\nAfter {i} items: {row:?}").unwrap();
    }
    let names: Vec<_> = selected.iter().map(|&i| (b'A' + i as u8) as char).collect();
    let weight: usize = selected.iter().map(|&i| items[i].0).sum();
    write!(text,"\nSelected items: {names:?}; total weight {weight}; optimal value {}.\nTraceback skips an item when including it ties the best value without it.",table[items.len()][capacity]).unwrap();
    text
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn every_dp_prefix_and_capacity_matches_exhaustive_subsets() {
        let mut rng = StdRng::seed_from_u64(288);
        for _ in 0..500 {
            let (items, capacity) = instance(&mut rng);
            let (table, selected) = execute(&items, capacity);
            for (prefix, row) in table.iter().enumerate() {
                for (c, &value_at_capacity) in row.iter().enumerate() {
                    let mut best = 0;
                    for mask in 0usize..1 << prefix {
                        let mut weight = 0;
                        let mut value = 0;
                        for (i, &(w, v)) in items[..prefix].iter().enumerate() {
                            if mask & (1 << i) != 0 {
                                weight += w;
                                value += v;
                            }
                        }
                        if weight <= c {
                            best = best.max(value);
                        }
                    }
                    assert_eq!(value_at_capacity, best);
                }
            }
            assert!(selected.windows(2).all(|pair| pair[0] < pair[1]));
            assert!(selected.iter().map(|&i| items[i].0).sum::<usize>() <= capacity);
            assert_eq!(
                selected.iter().map(|&i| items[i].1).sum::<u32>(),
                table[items.len()][capacity]
            );
        }
    }
}
