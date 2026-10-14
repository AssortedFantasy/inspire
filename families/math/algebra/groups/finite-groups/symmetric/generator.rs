use rand::{Rng, rngs::StdRng, seq::SliceRandom};

// Internal images are zero-based; rendered images and cycles are one-based.
fn compose(p: &[usize], q: &[usize]) -> Vec<usize> {
    q.iter().map(|&image| p[image]).collect()
}

fn inverse(p: &[usize]) -> Vec<usize> {
    let mut result = vec![0; p.len()];
    for (point, &image) in p.iter().enumerate() {
        result[image] = point;
    }
    result
}

fn cycles(p: &[usize]) -> Vec<Vec<usize>> {
    let mut seen = vec![false; p.len()];
    let mut result = Vec::new();
    for start in 0..p.len() {
        if seen[start] {
            continue;
        }
        let mut cycle = Vec::new();
        let mut point = start;
        while !seen[point] {
            seen[point] = true;
            cycle.push(point);
            point = p[point];
        }
        result.push(cycle);
    }
    result
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn order(cycles: &[Vec<usize>]) -> usize {
    cycles
        .iter()
        .fold(1, |a, cycle| a / gcd(a, cycle.len()) * cycle.len())
}

fn is_even(p: &[usize]) -> bool {
    (p.len() - cycles(p).len()).is_multiple_of(2)
}

fn one_line(p: &[usize]) -> String {
    format!(
        "[{}]",
        p.iter()
            .map(|x| (x + 1).to_string())
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn cycle_notation(cycles: &[Vec<usize>]) -> String {
    let parts: Vec<_> = cycles
        .iter()
        .filter(|c| c.len() > 1)
        .map(|c| {
            format!(
                "({})",
                c.iter()
                    .map(|x| (x + 1).to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        })
        .collect();
    if parts.is_empty() {
        "()".to_string()
    } else {
        parts.join("")
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let n = rng.random_range(3..=9);
    let identity: Vec<_> = (0..n).collect();
    let mut p = identity.clone();
    let mut q = identity.clone();
    p.shuffle(rng);
    q.shuffle(rng);
    let p_cycles = cycles(&p);
    let cycle_lengths = p_cycles
        .iter()
        .map(|c| c.len().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "S_{n}: all permutations of {{1, ..., {n}}}, a nonabelian group of order {n}! = {}.\n\
         One-line notation lists the images [p(1), ..., p({n})]. Product p*q means p after q: (p*q)(i) = p(q(i)).\n\
         Identity: {}.\n\
         p = {} = {}; q = {} = {}.\n\
         p*q = {} = {}. The inverse of p is {} = {}.\n\
         Fixed points are omitted from cycle notation; () denotes the identity.\n\
         The cycle lengths of p, including fixed points, are [{cycle_lengths}]. Its order is their least common multiple, {}.\n\
         p has {} parity, since n minus the number of cycles (including fixed points) is {}.",
        (1..=n).product::<usize>(),
        one_line(&identity),
        one_line(&p),
        cycle_notation(&p_cycles),
        one_line(&q),
        cycle_notation(&cycles(&q)),
        one_line(&compose(&p, &q)),
        cycle_notation(&cycles(&compose(&p, &q))),
        one_line(&inverse(&p)),
        cycle_notation(&cycles(&inverse(&p))),
        order(&p_cycles),
        if is_even(&p) { "even" } else { "odd" },
        n - p_cycles.len()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    #[test]
    fn composition_convention_and_cycle_direction() {
        let p = vec![1, 0, 2];
        let q = vec![0, 2, 1];
        assert_eq!(compose(&p, &q), vec![1, 2, 0]);
        assert_eq!(cycle_notation(&cycles(&compose(&p, &q))), "(1 2 3)");
        assert_ne!(compose(&p, &q), compose(&q, &p));
        assert_eq!(cycle_notation(&cycles(&[0, 1, 2])), "()");
    }

    #[test]
    fn exhaustive_s3_group_axioms() {
        let elements: Vec<Vec<usize>> = vec![
            vec![0, 1, 2],
            vec![0, 2, 1],
            vec![1, 0, 2],
            vec![1, 2, 0],
            vec![2, 0, 1],
            vec![2, 1, 0],
        ];
        let identity = &elements[0];
        for p in &elements {
            assert_eq!(compose(p, identity), *p);
            assert_eq!(compose(identity, p), *p);
            assert_eq!(compose(p, &inverse(p)), *identity);
            assert_eq!(compose(&inverse(p), p), *identity);
            for q in &elements {
                assert!(elements.contains(&compose(p, q)));
                for r in &elements {
                    assert_eq!(compose(&compose(p, q), r), compose(p, &compose(q, r)));
                }
            }
        }
    }

    #[test]
    fn seeded_cycle_order_inverse_and_parity_invariants() {
        let mut rng = StdRng::seed_from_u64(20261013);
        for n in 3..=9 {
            let identity: Vec<_> = (0..n).collect();
            for _ in 0..128 {
                let mut p = identity.clone();
                let mut q = identity.clone();
                p.shuffle(&mut rng);
                q.shuffle(&mut rng);
                let decomposition = cycles(&p);
                let mut restored = vec![usize::MAX; n];
                for cycle in &decomposition {
                    for (i, &point) in cycle.iter().enumerate() {
                        restored[point] = cycle[(i + 1) % cycle.len()];
                    }
                }
                assert_eq!(restored, p);
                assert_eq!(compose(&p, &inverse(&p)), identity);
                assert_eq!(compose(&inverse(&p), &p), identity);
                let expected_order = order(&decomposition);
                let mut power = identity.clone();
                for exponent in 1..=expected_order {
                    power = compose(&power, &p);
                    assert_eq!(power == identity, exponent == expected_order);
                }
                let inversions = (0..n)
                    .map(|i| ((i + 1)..n).filter(|&j| p[i] > p[j]).count())
                    .sum::<usize>();
                assert_eq!(is_even(&p), inversions.is_multiple_of(2));
                assert_eq!(is_even(&compose(&p, &q)), is_even(&p) == is_even(&q));
            }
        }
    }

    #[test]
    fn seeded_outputs_are_varied_and_deterministic() {
        let mut a = StdRng::seed_from_u64(1234);
        let mut b = StdRng::seed_from_u64(1234);
        let mut outputs = HashSet::new();
        for _ in 0..256 {
            let text = generate(&mut a);
            assert_eq!(text, generate(&mut b));
            assert!(!text.ends_with('\n'));
            outputs.insert(text);
        }
        assert!(outputs.len() > 220);
    }
}
