use rand::{Rng, rngs::StdRng};
use std::collections::BTreeMap;

type Partition = Vec<usize>;

fn partitions(n: usize) -> Vec<Partition> {
    fn visit(left: usize, limit: usize, prefix: &mut Partition, out: &mut Vec<Partition>) {
        if left == 0 {
            out.push(prefix.clone());
            return;
        }
        for part in (1..=left.min(limit)).rev() {
            prefix.push(part);
            visit(left - part, part, prefix, out);
            prefix.pop();
        }
    }
    let mut out = Vec::new();
    visit(n, n, &mut Vec::new(), &mut out);
    out
}

fn conjugate(p: &[usize]) -> Partition {
    (1..=p.first().copied().unwrap_or(0))
        .map(|col| p.iter().filter(|&&row| row >= col).count())
        .collect()
}

fn rank(p: &[usize]) -> isize {
    p.first().copied().unwrap_or(0) as isize - p.len() as isize
}

fn durfee(p: &[usize]) -> usize {
    p.iter()
        .enumerate()
        .take_while(|&(i, &part)| part > i)
        .count()
}

fn decompose(p: &[usize]) -> (usize, Partition, Partition) {
    let d = durfee(p);
    let right = p[..d]
        .iter()
        .map(|&part| part - d)
        .filter(|&part| part > 0)
        .collect();
    (d, right, p[d..].to_vec())
}

fn multiplicities(p: &[usize]) -> BTreeMap<usize, usize> {
    let mut counts = BTreeMap::new();
    for &part in p {
        *counts.entry(part).or_insert(0) += 1;
    }
    counts
}

fn binary_groups(part: usize, mut count: usize) -> Partition {
    let mut groups = Vec::new();
    let mut power = 1;
    while count > 0 {
        if count % 2 == 1 {
            groups.push(part * power);
        }
        count /= 2;
        power *= 2;
    }
    groups.reverse();
    groups
}

fn odd_to_distinct(p: &[usize]) -> Partition {
    let mut result: Vec<_> = multiplicities(p)
        .into_iter()
        .flat_map(|(part, count)| binary_groups(part, count))
        .collect();
    result.sort_unstable_by(|a, b| b.cmp(a));
    result
}

#[cfg(test)]
fn distinct_to_odd(p: &[usize]) -> Partition {
    let mut result = Vec::new();
    for &part in p {
        let mut odd = part;
        let mut copies = 1;
        while odd % 2 == 0 {
            odd /= 2;
            copies *= 2;
        }
        result.extend(std::iter::repeat_n(odd, copies));
    }
    result.sort_unstable_by(|a, b| b.cmp(a));
    result
}

fn hooks(p: &[usize]) -> Vec<Vec<usize>> {
    let q = conjugate(p);
    p.iter()
        .enumerate()
        .map(|(row, &length)| {
            (0..length)
                .map(|col| length - col + q[col] - row - 1)
                .collect()
        })
        .collect()
}

fn tableaux(p: &[usize]) -> (u128, u128) {
    let factorial = (1..=p.iter().sum::<usize>())
        .map(|n| n as u128)
        .product::<u128>();
    let product = hooks(p)
        .iter()
        .flatten()
        .map(|&hook| hook as u128)
        .product();
    (factorial / product, product)
}

fn notation(p: &[usize]) -> String {
    format!(
        "({})",
        p.iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn sum(p: &[usize]) -> usize {
    p.iter().sum()
}

fn ferrers(p: &[usize]) -> String {
    p.iter()
        .map(|&part| "[]".repeat(part))
        .collect::<Vec<_>>()
        .join("\n")
}

fn conjugation_view(all: &[Partition], n: usize, rng: &mut StdRng) -> String {
    let self_conjugate: bool = rng.random();
    let candidates: Vec<_> = all
        .iter()
        .filter(|p| !self_conjugate || **p == conjugate(p))
        .collect();
    let p = candidates[rng.random_range(0..candidates.len())];
    let q = conjugate(p);
    if *p == q {
        let diagonal: Vec<_> = (0..durfee(p)).map(|i| 2 * (p[i] - i) - 1).collect();
        format!(
            "λ = λ′ = {} ⊢ {n}\nDiagonal hooks {}\n\n{}",
            notation(p),
            notation(&diagonal),
            ferrers(p)
        )
    } else {
        format!(
            "λ = {} ⊢ {n}\nλ′ = {}\n\n{}",
            notation(p),
            notation(&q),
            ferrers(p)
        )
    }
}

fn glaisher_view(all: &[Partition], n: usize, rng: &mut StdRng) -> String {
    let candidates: Vec<_> = all
        .iter()
        .filter(|p| {
            p.iter().all(|part| part % 2 == 1) && p.windows(2).any(|pair| pair[0] == pair[1])
        })
        .collect();
    let odd = candidates[rng.random_range(0..candidates.len())];
    let distinct = odd_to_distinct(odd);
    let reverse = rng.random();
    let groups = multiplicities(odd)
        .into_iter()
        .rev()
        .filter(|&(_, count)| count > 1)
        .map(|(part, count)| {
            let grouped = binary_groups(part, count)
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(" + ");
            if reverse {
                format!("{grouped} ↦ {count}·{part}")
            } else {
                format!("{count}·{part} ↦ {grouped}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    if reverse {
        format!(
            "Distinct partition {} ⊢ {n}\nOdd partition {}\n\n{groups}",
            notation(&distinct),
            notation(odd)
        )
    } else {
        format!(
            "Odd partition {} ⊢ {n}\nDistinct partition {}\n\n{groups}",
            notation(odd),
            notation(&distinct)
        )
    }
}

fn durfee_view(all: &[Partition], n: usize, rng: &mut StdRng) -> String {
    let max = (1..=n).take_while(|d| d * d <= n).last().unwrap();
    let d = if rng.random() {
        [1, max][rng.random_range(0..2)]
    } else {
        rng.random_range(1..=max)
    };
    let candidates: Vec<_> = all.iter().filter(|p| durfee(p) == d).collect();
    let p = candidates[rng.random_range(0..candidates.len())];
    let (_, right, below) = decompose(p);
    format!(
        "λ = {} ⊢ {n}\nDurfee square {d}×{d}\nRight arm {}\nLower leg {}\n{n} = {} + {} + {}\n\n{} partitions of {n} have Durfee side {d}",
        notation(p),
        notation(&right),
        notation(&below),
        d * d,
        sum(&right),
        sum(&below),
        candidates.len()
    )
}

fn rank_view(all: &[Partition], n: usize, rng: &mut StdRng) -> String {
    let target = if rng.random_range(0..3) == 0 {
        let sign = if rng.random() { 1 } else { -1 };
        sign * (n as isize - 1)
    } else {
        rng.random_range(-3_i32..=3) as isize
    };
    let candidates: Vec<_> = all.iter().filter(|p| rank(p) == target).collect();
    let p = candidates[rng.random_range(0..candidates.len())];
    let q = conjugate(p);
    let opposite = all.iter().filter(|p| rank(p) == -target).count();
    let counts = if target == 0 {
        format!("N({n}, 0) = {}", candidates.len())
    } else {
        format!(
            "N({n}, {target}) = {}\nN({n}, {}) = {opposite}",
            candidates.len(),
            -target
        )
    };
    format!(
        "λ = {} ⊢ {n}\nrank(λ) = {} − {} = {target}\nλ′ = {}\nrank(λ′) = {}\n\n{counts}",
        notation(p),
        p[0],
        p.len(),
        notation(&q),
        -target
    )
}

fn hook_view(all: &[Partition], n: usize, rng: &mut StdRng) -> String {
    let candidates: Vec<_> = all.iter().filter(|p| p[0] <= 10 && p.len() <= 10).collect();
    let p = candidates[rng.random_range(0..candidates.len())];
    let grid = hooks(p)
        .iter()
        .map(|row| {
            row.iter()
                .map(|hook| format!("{hook:>2}"))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let (count, product) = tableaux(p);
    format!(
        "λ = {} ⊢ {n}\n\nHook lengths\n{grid}\n\nStandard tableaux f^λ = {n}! / {product} = {count}",
        notation(p)
    )
}

pub fn generate(rng: &mut StdRng) -> String {
    let n = rng.random_range(8..=26);
    let all = partitions(n);
    match rng.random_range(0..5) {
        0 => conjugation_view(&all, n, rng),
        1 => glaisher_view(&all, n, rng),
        2 => durfee_view(&all, n, rng),
        3 => rank_view(&all, n, rng),
        _ => hook_view(&all, n, rng),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn bounded_counts(n: usize, largest: usize) -> Vec<usize> {
        let mut counts = vec![0; n + 1];
        counts[0] = 1;
        for part in 1..=largest {
            for total in part..=n {
                counts[total] += counts[total - part];
            }
        }
        counts
    }

    #[test]
    fn enumerator_and_conjugation_agree_with_independent_counts() {
        for n in 0..=26 {
            let all = partitions(n);
            assert_eq!(all.len(), bounded_counts(n, n)[n]);
            assert_eq!(all.iter().collect::<BTreeSet<_>>().len(), all.len());
            for p in all {
                assert_eq!(sum(&p), n);
                assert!(p.iter().all(|&part| part > 0));
                assert!(p.windows(2).all(|pair| pair[0] >= pair[1]));
                let q = conjugate(&p);
                assert_eq!(sum(&q), n);
                assert_eq!(conjugate(&q), p);
                assert_eq!(rank(&q), -rank(&p));
                assert_eq!(durfee(&q), durfee(&p));
                if p == q {
                    let diagonal: Vec<_> = (0..durfee(&p)).map(|i| 2 * (p[i] - i) - 1).collect();
                    assert_eq!(sum(&diagonal), n);
                    assert!(diagonal.iter().all(|part| part % 2 == 1));
                    assert!(diagonal.windows(2).all(|pair| pair[0] > pair[1]));
                }
            }
        }
    }

    #[test]
    fn glaisher_is_a_bijection_between_the_complete_restricted_sets() {
        for n in 0..=26 {
            let all = partitions(n);
            let odd: Vec<_> = all
                .iter()
                .filter(|p| p.iter().all(|part| part % 2 == 1))
                .collect();
            let distinct: BTreeSet<_> = all
                .iter()
                .filter(|p| p.windows(2).all(|pair| pair[0] > pair[1]))
                .cloned()
                .collect();
            let images: BTreeSet<_> = odd.iter().map(|p| odd_to_distinct(p)).collect();
            assert_eq!(images.len(), odd.len());
            assert_eq!(images, distinct);
            for p in odd {
                assert_eq!(distinct_to_odd(&odd_to_distinct(p)), *p);
            }
        }
    }

    #[test]
    fn durfee_decomposition_matches_reconstruction_and_generating_function() {
        for n in 1..=26 {
            let all = partitions(n);
            for p in &all {
                let (d, right, below) = decompose(p);
                let mut rebuilt: Vec<_> = (0..d)
                    .map(|i| d + right.get(i).copied().unwrap_or(0))
                    .collect();
                rebuilt.extend(&below);
                assert_eq!(rebuilt, *p);
                assert_eq!(n, d * d + sum(&right) + sum(&below));
                assert!(right.len() <= d);
                assert!(below.iter().all(|&part| part <= d));
            }
            for d in (1..=n).take_while(|d| d * d <= n) {
                let remainder = n - d * d;
                let counts = bounded_counts(remainder, d);
                let coefficient: usize = (0..=remainder)
                    .map(|i| counts[i] * counts[remainder - i])
                    .sum();
                assert_eq!(all.iter().filter(|p| durfee(p) == d).count(), coefficient);
            }
        }
    }

    fn corner_removal_count(p: &[usize], memo: &mut BTreeMap<Partition, u128>) -> u128 {
        if p.is_empty() {
            return 1;
        }
        if let Some(&count) = memo.get(p) {
            return count;
        }
        let mut count = 0;
        for row in 0..p.len() {
            if row + 1 == p.len() || p[row] > p[row + 1] {
                let mut smaller = p.to_vec();
                smaller[row] -= 1;
                smaller.retain(|&part| part > 0);
                count += corner_removal_count(&smaller, memo);
            }
        }
        memo.insert(p.to_vec(), count);
        count
    }

    #[test]
    fn hook_length_counts_match_independent_corner_removal_recurrence() {
        let mut memo = BTreeMap::new();
        for n in 0..=12 {
            for p in partitions(n) {
                assert_eq!(tableaux(&p).0, corner_removal_count(&p, &mut memo));
                for (row, lengths) in hooks(&p).iter().enumerate() {
                    for (col, &length) in lengths.iter().enumerate() {
                        let cells = p[row] - col
                            + p.iter().skip(row + 1).filter(|&&part| part > col).count();
                        assert_eq!(length, cells);
                    }
                }
            }
        }
        for n in 13..=26 {
            for p in partitions(n) {
                let (count, product) = tableaux(&p);
                let factorial: u128 = (1..=n).map(|v| v as u128).product();
                assert_eq!(count * product, factorial);
            }
        }
    }
}
