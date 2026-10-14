use rand::{Rng, rngs::StdRng, seq::SliceRandom};

pub fn generate(rng: &mut StdRng) -> String {
    let relation = sample(rng);
    let n = relation.len();
    let opens = open_sets(&relation);
    let covers: Vec<_> = (0..n)
        .flat_map(|i| (0..n).map(move |j| (i, j)))
        .filter(|&(i, j)| {
            i != j
                && relation[i][j]
                && !(0..n).any(|k| k != i && k != j && relation[i][k] && relation[k][j])
        })
        .map(|(i, j)| format!("{i}<{j}"))
        .collect();
    let cover_text = if covers.is_empty() {
        "none".into()
    } else {
        covers.join(", ")
    };
    let open_text = opens
        .iter()
        .map(|&mask| set(mask, n))
        .collect::<Vec<_>>()
        .join(", ");
    let neighborhoods = relation
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let mask = row
                .iter()
                .enumerate()
                .fold(0, |mask, (j, &included)| mask | (u32::from(included) << j));
            format!("U({i})={}", set(mask, n))
        })
        .collect::<Vec<_>>()
        .join("; ");
    let discrete = covers.is_empty();
    format!(
        "Finite topology on X={{0,...,{}}} induced by a partial order. Strict covering relations: {cover_text}; the order adds reflexivity and transitive consequences.\nOpen sets are upper sets: x in O and x<=y imply y in O. Complete list: {open_text}.\nMinimal open neighborhoods: {neighborhoods}.\nThe space is T0 and compact. T1: {}; Hausdorff: {}; discrete: {}. In a finite space these latter three properties are equivalent.",
        n - 1,
        if discrete { "yes" } else { "no" },
        if discrete { "yes" } else { "no" },
        if discrete { "yes" } else { "no" },
    )
}

fn sample(rng: &mut StdRng) -> Vec<Vec<bool>> {
    let n = rng.random_range(3..=5);
    let mut labels: Vec<_> = (0..n).collect();
    labels.shuffle(rng);
    let mut relation = vec![vec![false; n]; n];
    for (i, row) in relation.iter_mut().enumerate() {
        row[i] = true;
    }
    for i in 0..n {
        for j in i + 1..n {
            relation[labels[i]][labels[j]] = rng.random_ratio(1, 3);
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                relation[i][j] |= relation[i][k] && relation[k][j];
            }
        }
    }
    relation
}

fn open_sets(relation: &[Vec<bool>]) -> Vec<u32> {
    let n = relation.len();
    (0..1 << n)
        .filter(|&mask| {
            (0..n).all(|i| {
                mask & (1 << i) == 0 || (0..n).all(|j| !relation[i][j] || mask & (1 << j) != 0)
            })
        })
        .collect()
}

fn set(mask: u32, n: usize) -> String {
    format!(
        "{{{}}}",
        (0..n)
            .filter(|i| mask & (1 << i) != 0)
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn topology_axioms_and_order_recovery_hold_for_all_printed_subsets() {
        let mut rng = StdRng::seed_from_u64(999);
        for _ in 0..500 {
            let relation = sample(&mut rng);
            let n = relation.len();
            let opens = open_sets(&relation);
            assert!(opens.contains(&0) && opens.contains(&((1 << n) - 1)));
            for &a in &opens {
                for &b in &opens {
                    assert!(opens.contains(&(a & b)));
                    assert!(opens.contains(&(a | b)));
                }
            }
            for (i, row) in relation.iter().enumerate() {
                for (j, &ordered) in row.iter().enumerate() {
                    let every_neighborhood_contains_j = opens
                        .iter()
                        .filter(|&&mask| mask & (1 << i) != 0)
                        .all(|&mask| mask & (1 << j) != 0);
                    assert_eq!(ordered, every_neighborhood_contains_j);
                    if i != j {
                        assert!(!(relation[i][j] && relation[j][i]));
                    }
                }
            }
        }
    }
}
