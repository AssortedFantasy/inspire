use rand::{Rng, rngs::StdRng, seq::SliceRandom};

struct Market {
    left: Vec<Vec<usize>>,
    right: Vec<Vec<usize>>,
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..4);
    let market = sample(rng, regime);
    let left_optimal = deferred_acceptance(&market.left, &market.right);
    let right_proposing = deferred_acceptance(&market.right, &market.left);
    let mut right_optimal = vec![0; market.left.len()];
    for (r, &l) in right_proposing.iter().enumerate() {
        right_optimal[l] = r;
    }
    let stable = stable_matchings(&market);
    let mut lines = vec!["Preferences".to_owned()];
    for (l, preferences) in market.left.iter().enumerate() {
        lines.push(format!(
            "{}  {}",
            left_name(l),
            preferences
                .iter()
                .map(|&r| right_name(r))
                .collect::<Vec<_>>()
                .join(" > ")
        ));
    }
    lines.push(String::new());
    for (r, preferences) in market.right.iter().enumerate() {
        lines.push(format!(
            "{}  {}",
            right_name(r),
            preferences
                .iter()
                .map(|&l| left_name(l))
                .collect::<Vec<_>>()
                .join(" > ")
        ));
    }
    if left_optimal == right_optimal {
        lines.push(format!(
            "\nUnique stable matching\n{}",
            pairs(&left_optimal)
        ));
    } else {
        lines.push(format!("\nLetters propose {}", pairs(&left_optimal)));
        lines.push(format!("Numbers propose {}", pairs(&right_optimal)));
        lines.push(format!("{} stable matchings", stable.len()));
    }
    lines.join("\n")
}

fn sample(rng: &mut StdRng, regime: usize) -> Market {
    let n = if regime == 2 {
        rng.random_range(4..=6)
    } else {
        rng.random_range(3..=6)
    };
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut cycle = (0..n).collect::<Vec<_>>();
    if regime == 2 {
        let split = n / 2;
        for (i, next) in cycle.iter_mut().enumerate() {
            *next = if i < split {
                (i + 1) % split
            } else {
                split + (i + 1 - split) % (n - split)
            };
        }
    } else {
        for (i, next) in cycle.iter_mut().enumerate() {
            *next = (i + 1) % n;
        }
    }
    for i in 0..n {
        let mut l: Vec<_> = (0..n).collect();
        let mut r = l.clone();
        l.shuffle(rng);
        r.shuffle(rng);
        match regime {
            0 => l = (0..n).collect(),
            1 | 2 => {
                l.retain(|&j| j != i && j != cycle[i]);
                l.splice(0..0, [i, cycle[i]]);
                let previous = cycle.iter().position(|&j| j == i).unwrap();
                r.retain(|&j| j != previous && j != i);
                r.splice(0..0, [previous, i]);
            }
            _ => (),
        }
        left.push(l);
        right.push(r);
    }
    let mut left_labels: Vec<_> = (0..n).collect();
    let mut right_labels = left_labels.clone();
    left_labels.shuffle(rng);
    right_labels.shuffle(rng);
    let mut renamed_left = vec![Vec::new(); n];
    let mut renamed_right = renamed_left.clone();
    for i in 0..n {
        renamed_left[left_labels[i]] = left[i].iter().map(|&j| right_labels[j]).collect();
        renamed_right[right_labels[i]] = right[i].iter().map(|&j| left_labels[j]).collect();
    }
    Market {
        left: renamed_left,
        right: renamed_right,
    }
}

fn ranks(preferences: &[Vec<usize>]) -> Vec<Vec<usize>> {
    preferences
        .iter()
        .map(|p| {
            let mut rank = vec![0; p.len()];
            for (i, &partner) in p.iter().enumerate() {
                rank[partner] = i;
            }
            rank
        })
        .collect()
}

fn deferred_acceptance(proposers: &[Vec<usize>], receivers: &[Vec<usize>]) -> Vec<usize> {
    let n = proposers.len();
    let receiver_ranks = ranks(receivers);
    let mut next = vec![0; n];
    let mut held: Vec<Option<usize>> = vec![None; n];
    let mut free: Vec<_> = (0..n).collect();
    while let Some(p) = free.pop() {
        let r = proposers[p][next[p]];
        next[p] += 1;
        match held[r] {
            None => held[r] = Some(p),
            Some(previous) if receiver_ranks[r][p] < receiver_ranks[r][previous] => {
                held[r] = Some(p);
                free.push(previous);
            }
            Some(_) => free.push(p),
        }
    }
    let mut partners = vec![0; n];
    for (r, p) in held.into_iter().enumerate() {
        partners[p.unwrap()] = r;
    }
    partners
}

fn permutations(values: &mut [usize], start: usize, output: &mut Vec<Vec<usize>>) {
    if start == values.len() {
        output.push(values.to_vec());
        return;
    }
    for i in start..values.len() {
        values.swap(start, i);
        permutations(values, start + 1, output);
        values.swap(start, i);
    }
}

fn stable_matchings(market: &Market) -> Vec<Vec<usize>> {
    let n = market.left.len();
    let left_ranks = ranks(&market.left);
    let right_ranks = ranks(&market.right);
    let mut all = Vec::new();
    permutations(&mut (0..n).collect::<Vec<_>>(), 0, &mut all);
    all.into_iter()
        .filter(|matching| {
            let mut inverse = vec![0; n];
            for (l, &r) in matching.iter().enumerate() {
                inverse[r] = l;
            }
            (0..n).all(|l| {
                (0..n).all(|r| {
                    left_ranks[l][r] >= left_ranks[l][matching[l]]
                        || right_ranks[r][l] >= right_ranks[r][inverse[r]]
                })
            })
        })
        .collect()
}

fn left_name(i: usize) -> String {
    char::from(b'A' + i as u8).to_string()
}
fn right_name(i: usize) -> String {
    (i + 1).to_string()
}
fn pairs(matching: &[usize]) -> String {
    matching
        .iter()
        .enumerate()
        .map(|(l, &r)| format!("{}–{}", left_name(l), right_name(r)))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn extremes_are_stable_and_dominate_every_enumerated_stable_matching() {
        let mut rng = StdRng::seed_from_u64(338);
        for regime in 0..4 {
            for _ in 0..100 {
                let market = sample(&mut rng, regime);
                let l_opt = deferred_acceptance(&market.left, &market.right);
                let r_opt = deferred_acceptance(&market.right, &market.left);
                let stable = stable_matchings(&market);
                assert!(stable.contains(&l_opt));
                let mut inverse = vec![0; r_opt.len()];
                for (r, &l) in r_opt.iter().enumerate() {
                    inverse[l] = r;
                }
                assert!(stable.contains(&inverse));
                for matching in &stable {
                    for (l, preferences) in market.left.iter().enumerate() {
                        let rank = |r| preferences.iter().position(|&x| x == r).unwrap();
                        assert!(rank(l_opt[l]) <= rank(matching[l]));
                        assert!(rank(matching[l]) <= rank(inverse[l]));
                    }
                    for (r, preferences) in market.right.iter().enumerate() {
                        let l = matching.iter().position(|&x| x == r).unwrap();
                        assert!(
                            preferences.iter().position(|&x| x == r_opt[r])
                                <= preferences.iter().position(|&x| x == l)
                        );
                    }
                }
                match regime {
                    0 => assert_eq!(stable.len(), 1),
                    1 => assert_eq!(stable.len(), 2),
                    2 => assert_eq!(stable.len(), 4),
                    _ => (),
                }
            }
        }
    }

    #[test]
    fn enumerated_matchings_have_no_blocking_pair_by_direct_preference_search() {
        let mut rng = StdRng::seed_from_u64(8128);
        for _ in 0..100 {
            let market = sample(&mut rng, 3);
            for matching in stable_matchings(&market) {
                for (l, preferences) in market.left.iter().enumerate() {
                    for &r in preferences.iter().take_while(|&&r| r != matching[l]) {
                        let current = matching.iter().position(|&x| x == r).unwrap();
                        assert!(
                            !market.right[r]
                                .iter()
                                .take_while(|&&x| x != current)
                                .any(|&x| x == l)
                        );
                    }
                }
            }
        }
    }
}
