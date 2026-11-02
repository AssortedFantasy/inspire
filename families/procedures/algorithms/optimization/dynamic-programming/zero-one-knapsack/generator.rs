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
    let mut text = format!("0/1 knapsack\nCapacity {capacity}\nItem  Weight  Value");
    for (i, (weight, value)) in items.iter().enumerate() {
        write!(
            text,
            "\n{}      {weight:>2}     {value:>2}",
            (b'A' + i as u8) as char
        )
        .unwrap();
    }
    // The final frontier retains each capacity threshold where the optimum
    // changes, without repeating unchanged cells for every item prefix.
    write!(text, "\nCapacity → best value").unwrap();
    for (c, &value) in table[items.len()].iter().enumerate() {
        if c == 0 || value != table[items.len()][c - 1] {
            write!(text, "\n{c} → {value}").unwrap();
        }
    }
    let names: Vec<_> = selected.iter().map(|&i| (b'A' + i as u8) as char).collect();
    let weight: usize = selected.iter().map(|&i| items[i].0).sum();
    write!(
        text,
        "\nTake {}\nWeight {weight}\nValue {}",
        names
            .iter()
            .map(char::to_string)
            .collect::<Vec<_>>()
            .join(" + "),
        table[items.len()][capacity]
    )
    .unwrap();
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
