use rand::{Rng, rngs::StdRng, seq::SliceRandom};

pub fn generate(rng: &mut StdRng) -> String {
    let relation = sample(rng);
    let n = relation.len();
    let opens = open_sets(&relation);
    let universe = (1 << n) - 1;
    let mut output = format!("X={}\n", set(universe, n));
    if rng.random_bool(0.5) {
        let open_text = opens
            .iter()
            .map(|&mask| set(mask, n))
            .collect::<Vec<_>>()
            .join(", ");
        output.push_str(&format!("τ={{{open_text}}}"));
    } else {
        let neighborhoods = relation
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let mask = row
                    .iter()
                    .enumerate()
                    .fold(0, |mask, (j, &included)| mask | (u32::from(included) << j));
                format!("U_min({i})={}", set(mask, n))
            })
            .collect::<Vec<_>>()
            .join("\n");
        output.push_str(&neighborhoods);
    }
    if rng.random_bool(0.5) {
        let subset = rng.random_range(1..universe);
        let (interior, closure) = interior_and_closure(subset, universe, &opens);
        output.push_str(&format!(
            "\nA={}\nint(A)={}\ncl(A)={}",
            set(subset, n),
            set(interior, n),
            set(closure, n)
        ));
    }
    output
}

fn interior_and_closure(subset: u32, universe: u32, opens: &[u32]) -> (u32, u32) {
    let interior = |mask: u32| {
        opens
            .iter()
            .filter(|&&open| open & !mask == 0)
            .fold(0, |union, &open| union | open)
    };
    (interior(subset), universe ^ interior(universe ^ subset))
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
    if mask == 0 {
        return "∅".into();
    }
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
    fn interiors_and_closures_agree_with_neighborhoods_and_closed_sets() {
        let mut rng = StdRng::seed_from_u64(2_519);
        for _ in 0..300 {
            let relation = sample(&mut rng);
            let n = relation.len();
            let universe = (1 << n) - 1;
            let opens = open_sets(&relation);
            for subset in 0..=universe {
                let (interior, closure) = interior_and_closure(subset, universe, &opens);
                let closed_intersection = opens
                    .iter()
                    .map(|&open| universe ^ open)
                    .filter(|&closed| subset & !closed == 0)
                    .fold(universe, |a, b| a & b);
                assert_eq!(closure, closed_intersection);
                assert_eq!(interior & !subset, 0);
                assert_eq!(subset & !closure, 0);
                assert_eq!(interior_and_closure(interior, universe, &opens).0, interior);
                assert_eq!(interior_and_closure(closure, universe, &opens).1, closure);
                for (i, row) in relation.iter().enumerate() {
                    assert_eq!(
                        interior & (1 << i) != 0,
                        row.iter()
                            .enumerate()
                            .all(|(j, &ordered)| !ordered || subset & (1 << j) != 0)
                    );
                    assert_eq!(
                        closure & (1 << i) != 0,
                        row.iter()
                            .enumerate()
                            .any(|(j, &ordered)| ordered && subset & (1 << j) != 0)
                    );
                }
            }
        }
    }

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
