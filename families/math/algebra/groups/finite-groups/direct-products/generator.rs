use rand::{Rng, rngs::StdRng};

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn add(moduli: &[usize], a: &[usize], b: &[usize]) -> Vec<usize> {
    moduli
        .iter()
        .zip(a)
        .zip(b)
        .map(|((&n, &x), &y)| (x + y) % n)
        .collect()
}

fn inverse(moduli: &[usize], a: &[usize]) -> Vec<usize> {
    moduli.iter().zip(a).map(|(&n, &x)| (n - x) % n).collect()
}

fn order(moduli: &[usize], a: &[usize]) -> usize {
    moduli
        .iter()
        .zip(a)
        .map(|(&n, &x)| n / gcd(n, x))
        .fold(1, lcm)
}

fn exponent(moduli: &[usize]) -> usize {
    moduli.iter().copied().fold(1, lcm)
}

fn cyclic(moduli: &[usize]) -> bool {
    moduli
        .iter()
        .enumerate()
        .all(|(i, &n)| moduli[i + 1..].iter().all(|&m| gcd(n, m) == 1))
}

fn tuple(a: &[usize]) -> String {
    format!(
        "({})",
        a.iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    )
}

pub fn generate(rng: &mut StdRng) -> String {
    let rank = rng.random_range(2..=4);
    let moduli: Vec<usize> = (0..rank).map(|_| rng.random_range(2..=16)).collect();
    let a: Vec<usize> = moduli.iter().map(|&n| rng.random_range(0..n)).collect();
    let b: Vec<usize> = moduli.iter().map(|&n| rng.random_range(0..n)).collect();
    let name = moduli
        .iter()
        .map(|n| format!("C_{n}"))
        .collect::<Vec<_>>()
        .join(" x ");
    let coordinate_orders: Vec<_> = moduli
        .iter()
        .zip(&a)
        .map(|(&n, &x)| n / gcd(n, x))
        .collect();
    let cyclicity = if cyclic(&moduli) {
        format!(
            "This group is cyclic: the factor orders are pairwise coprime, and {} generates it.",
            tuple(&vec![1; rank])
        )
    } else {
        "This group is not cyclic: at least two factor orders have a common divisor greater than 1."
            .to_owned()
    };
    format!(
        "G = {name}, a direct product of cyclic additive groups. A coordinate in C_n is a residue 0..n-1.\n\
         Law: (x_1,...,x_{rank}) + (y_1,...,y_{rank}) = ((x_1+y_1) mod n_1,...,(x_{rank}+y_{rank}) mod n_{rank}), with factor orders {}.\n\
         Identity: {}. G is abelian and has {} elements.\n\
         Example: {} + {} = {}; the additive inverse of {} is {}.\n\
         The coordinate orders of {} are {}; its element order is their least common multiple, {}.\n\
         The exponent of G is {} (the least common multiple of the factor orders); {} has this order.\n\
         {cyclicity}",
        tuple(&moduli),
        tuple(&vec![0; rank]),
        moduli.iter().product::<usize>(),
        tuple(&a),
        tuple(&b),
        tuple(&add(&moduli, &a, &b)),
        tuple(&a),
        tuple(&inverse(&moduli, &a)),
        tuple(&a),
        tuple(&coordinate_orders),
        order(&moduli, &a),
        exponent(&moduli),
        tuple(&vec![1; rank]),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    fn elements(moduli: &[usize]) -> Vec<Vec<usize>> {
        let mut values = vec![vec![]];
        for &n in moduli {
            values = values
                .iter()
                .flat_map(|a| {
                    (0..n).map(move |x| {
                        let mut b = a.clone();
                        b.push(x);
                        b
                    })
                })
                .collect();
        }
        values
    }

    #[test]
    fn exhaustive_small_group_laws() {
        for moduli in [
            vec![2, 2],
            vec![2, 3],
            vec![3, 3],
            vec![2, 2, 2],
            vec![2, 3, 2],
            vec![2, 2, 2, 2],
        ] {
            let values = elements(&moduli);
            let zero = vec![0; moduli.len()];
            assert_eq!(values.len(), moduli.iter().product::<usize>());
            for a in &values {
                assert_eq!(add(&moduli, a, &zero), *a);
                assert_eq!(add(&moduli, a, &inverse(&moduli, a)), zero);
                for b in &values {
                    let sum = add(&moduli, a, b);
                    assert!(sum.iter().zip(&moduli).all(|(&x, &n)| x < n));
                    assert_eq!(sum, add(&moduli, b, a));
                    for c in &values {
                        assert_eq!(add(&moduli, &sum, c), add(&moduli, a, &add(&moduli, b, c)));
                    }
                }
            }
        }
    }

    #[test]
    fn exhaustive_small_orders_exponents_and_cyclicity() {
        for rank in 2..=4 {
            for moduli in elements(&vec![3; rank]) {
                let moduli: Vec<_> = moduli.into_iter().map(|n| n + 2).collect();
                let values = elements(&moduli);
                let zero = vec![0; rank];
                let mut observed_exponent = 1;
                let mut has_generator = false;
                for a in &values {
                    let mut sum = a.clone();
                    let mut observed_order = 1;
                    while sum != zero {
                        sum = add(&moduli, &sum, a);
                        observed_order += 1;
                        assert!(observed_order <= values.len());
                    }
                    assert_eq!(order(&moduli, a), observed_order);
                    observed_exponent = lcm(observed_exponent, observed_order);
                    has_generator |= observed_order == values.len();
                }
                assert_eq!(exponent(&moduli), observed_exponent);
                assert_eq!(order(&moduli, &vec![1; rank]), observed_exponent);
                assert_eq!(cyclic(&moduli), has_generator);
            }
        }
    }

    #[test]
    fn full_parameter_cyclicity_criterion() {
        for rank in 2..=4 {
            for moduli in elements(&vec![15; rank]) {
                let moduli: Vec<_> = moduli.into_iter().map(|n| n + 2).collect();
                assert_eq!(
                    cyclic(&moduli),
                    exponent(&moduli) == moduli.iter().product::<usize>()
                );
                assert_eq!(order(&moduli, &vec![1; rank]), exponent(&moduli));
            }
        }
    }

    #[test]
    fn seeded_outputs_are_deterministic_and_varied() {
        let mut first = StdRng::seed_from_u64(20261013);
        let mut second = StdRng::seed_from_u64(20261013);
        let mut outputs = HashSet::new();
        for _ in 0..256 {
            let text = generate(&mut first);
            assert_eq!(text, generate(&mut second));
            assert!(!text.ends_with('\n'));
            outputs.insert(text);
        }
        assert!(outputs.len() > 250);
    }
}
