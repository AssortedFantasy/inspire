use rand::{Rng, rngs::StdRng, seq::SliceRandom};

pub fn generate(rng: &mut StdRng) -> String {
    let relation = sample(rng);
    let n = relation.len();
    let cover_text = covers(&relation)
        .iter()
        .map(|&(i, j)| format!("{} < {}", label(i), label(j)))
        .collect::<Vec<_>>()
        .join(", ");
    let antichains = antichains(&relation);
    let width = antichains.iter().map(|m| m.count_ones()).max().unwrap() as usize;
    let mut counts = vec![0; width + 1];
    for mask in &antichains {
        counts[mask.count_ones() as usize] += 1;
    }
    let maximum = antichains
        .iter()
        .filter(|m| m.count_ones() as usize == width)
        .map(|&m| set(m, n))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "P = {}\nCovers = {{{cover_text}}}\nLinear extensions = {}\nAntichains by size = {:?}\nMaximum antichains = {{{maximum}}}",
        set((1 << n) - 1, n),
        extensions(&relation),
        counts
    )
}

fn label(i: usize) -> char {
    (b'a' + i as u8) as char
}

fn set(mask: u32, n: usize) -> String {
    format!(
        "{{{}}}",
        (0..n)
            .filter(|&i| mask & (1 << i) != 0)
            .map(|i| label(i).to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn close(mut relation: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let n = relation.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                relation[i][j] |= relation[i][k] && relation[k][j];
            }
        }
    }
    relation
}

fn sample(rng: &mut StdRng) -> Vec<Vec<bool>> {
    let regime = rng.random_range(0..8);
    let n = match regime {
        2 => 8,
        3 => 2 * rng.random_range(3..=4),
        4 => 4,
        _ => rng.random_range(4..=8),
    };
    let mut relation = vec![vec![false; n]; n];
    match regime {
        0 => {
            for (i, row) in relation.iter_mut().enumerate().take(n - 1) {
                row[i + 1] = true;
            }
        }
        1 => {}
        2 => {
            for (i, row) in relation.iter_mut().enumerate() {
                for (j, value) in row.iter_mut().enumerate() {
                    *value = i != j && i & j == i;
                }
            }
        }
        3 => {
            let half = n / 2;
            for (i, row) in relation.iter_mut().enumerate().take(half) {
                row[half + i] = true;
                row[half + (i + 1) % half] = true;
            }
        }
        4 => {
            relation[0][2] = true;
            relation[1][2] = true;
            relation[1][3] = true;
        }
        5 => {
            // Recursive series/parallel composition of a shuffled split tree.
            fn compose(relation: &mut [Vec<bool>], lo: usize, hi: usize, rng: &mut StdRng) {
                if hi - lo <= 1 {
                    return;
                }
                let split = rng.random_range(lo + 1..hi);
                compose(relation, lo, split, rng);
                compose(relation, split, hi, rng);
                if rng.random_bool(0.5) {
                    for row in &mut relation[lo..split] {
                        for value in &mut row[split..hi] {
                            *value = true;
                        }
                    }
                }
            }
            compose(&mut relation, 0, n, rng);
        }
        6 => {
            // A fence changes direction at every edge.
            for i in 0..n - 1 {
                if i % 2 == 0 {
                    relation[i][i + 1] = true;
                } else {
                    relation[i + 1][i] = true;
                }
            }
        }
        _ => {
            let probability = if rng.random_bool(0.5) { 0.2 } else { 0.65 };
            for (i, row) in relation.iter_mut().enumerate() {
                for value in &mut row[i + 1..] {
                    *value = rng.random_bool(probability);
                }
            }
        }
    }
    let relation = close(relation);
    let mut labels: Vec<_> = (0..n).collect();
    labels.shuffle(rng);
    (0..n)
        .map(|i| (0..n).map(|j| relation[labels[i]][labels[j]]).collect())
        .collect()
}

fn covers(relation: &[Vec<bool>]) -> Vec<(usize, usize)> {
    let n = relation.len();
    (0..n)
        .flat_map(|i| {
            (0..n).filter_map(move |j| {
                (relation[i][j] && !(0..n).any(|k| relation[i][k] && relation[k][j]))
                    .then_some((i, j))
            })
        })
        .collect()
}

fn antichains(relation: &[Vec<bool>]) -> Vec<u32> {
    let n = relation.len();
    (0..1 << n)
        .filter(|&mask| {
            (0..n).all(|i| {
                mask & (1 << i) == 0 || (0..n).all(|j| mask & (1 << j) == 0 || !relation[i][j])
            })
        })
        .collect()
}

fn extensions(relation: &[Vec<bool>]) -> u64 {
    let n = relation.len();
    let predecessors: Vec<u32> = (0..n)
        .map(|j| {
            (0..n)
                .filter(|&i| relation[i][j])
                .fold(0, |m, i| m | 1 << i)
        })
        .collect();
    let mut counts = vec![0u64; 1 << n];
    counts[0] = 1;
    for subset in 0..1 << n {
        for (j, &before) in predecessors.iter().enumerate() {
            if subset & (1 << j) == 0 && before & !(subset as u32) == 0 {
                counts[subset | 1 << j] += counts[subset];
            }
        }
    }
    counts[(1 << n) - 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn brute_extensions(relation: &[Vec<bool>], order: &mut Vec<usize>) -> u64 {
        let n = relation.len();
        if order.len() == n {
            return u64::from((0..n).all(|i| (i + 1..n).all(|j| !relation[order[j]][order[i]])));
        }
        let mut total = 0;
        for i in 0..n {
            if !order.contains(&i) {
                order.push(i);
                total += brute_extensions(relation, order);
                order.pop();
            }
        }
        total
    }

    #[test]
    fn covers_reconstruct_the_order_and_counts_match_permutation_enumeration() {
        let mut rng = StdRng::seed_from_u64(162);
        for _ in 0..120 {
            let relation = sample(&mut rng);
            let n = relation.len();
            let mut reduced = vec![vec![false; n]; n];
            for (i, j) in covers(&relation) {
                reduced[i][j] = true;
            }
            assert_eq!(close(reduced), relation);
            for (i, row) in relation.iter().enumerate() {
                assert!(!row[i]);
                for j in 0..n {
                    assert!(!(row[j] && relation[j][i]));
                }
            }
            assert_eq!(
                extensions(&relation),
                brute_extensions(&relation, &mut Vec::new())
            );
            let ideals: Vec<u32> = (0..1 << n)
                .filter(|&m| {
                    (0..n).all(|j| {
                        m & (1 << j) == 0 || (0..n).all(|i| !relation[i][j] || m & (1 << i) != 0)
                    })
                })
                .collect();
            // Ideals and antichains are in bijection through maximal elements.
            let mut maxima: Vec<u32> = ideals
                .iter()
                .map(|&m| {
                    (0..n)
                        .filter(|&i| {
                            m & (1 << i) != 0
                                && !(0..n).any(|j| m & (1 << j) != 0 && relation[i][j])
                        })
                        .fold(0, |a, i| a | 1 << i)
                })
                .collect();
            maxima.sort_unstable();
            assert_eq!(maxima, antichains(&relation));
        }
    }

    #[test]
    fn standard_counts_are_exact() {
        let mut boolean = vec![vec![false; 8]; 8];
        for (i, row) in boolean.iter_mut().enumerate() {
            for (j, value) in row.iter_mut().enumerate() {
                *value = i != j && i & j == i;
            }
        }
        assert_eq!(extensions(&boolean), 48);
        assert_eq!(antichains(&boolean).len(), 20);
        assert_eq!(extensions(&vec![vec![false; 8]; 8]), 40320);
        let mut chain = vec![vec![false; 8]; 8];
        for (i, row) in chain.iter_mut().enumerate().take(7) {
            row[i + 1] = true;
        }
        assert_eq!(extensions(&close(chain)), 1);
    }
}
