use rand::{Rng, rngs::StdRng};

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    let (mut x, mut y) = (a, b);
    while y != 0 {
        (x, y) = (y, x % y);
    }
    a / x * b
}

fn tensor_moduli(m: &[u32], n: &[u32]) -> Vec<u32> {
    m.iter()
        .flat_map(|&x| n.iter().map(move |&y| gcd(x, y)))
        .collect()
}

fn image(m: &[u32], n: &[u32], a: &[u32], b: &[u32]) -> Vec<u32> {
    m.iter()
        .zip(a)
        .flat_map(|(&x, &u)| n.iter().zip(b).map(move |(&y, &v)| u * v % gcd(x, y)))
        .collect()
}

fn element_order(moduli: &[u32], values: &[u32]) -> u64 {
    moduli
        .iter()
        .zip(values)
        .map(|(&n, &a)| u64::from(n / gcd(n, a)))
        .fold(1, lcm)
}

fn tuple(values: &[u32]) -> String {
    format!(
        "({})",
        values
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn module(moduli: &[u32]) -> String {
    let factors: Vec<_> = moduli
        .iter()
        .filter(|&&n| n > 1)
        .map(|n| format!("Z/{n}Z"))
        .collect();
    if factors.is_empty() {
        "0".to_owned()
    } else {
        factors.join(" ⊕ ")
    }
}

fn coordinates(moduli: &[u32], values: &[u32]) -> String {
    tuple(
        &moduli
            .iter()
            .zip(values)
            .filter_map(|(&n, &v)| (n > 1).then_some(v))
            .collect::<Vec<_>>(),
    )
}

pub fn generate(rng: &mut StdRng) -> String {
    let (mut m, mut n) = (vec![6, 10], vec![15, 21]);
    for _ in 0..64 {
        let x: Vec<_> = (0..rng.random_range(1..=3))
            .map(|_| rng.random_range(2..=48))
            .collect();
        let y: Vec<_> = (0..rng.random_range(1..=3))
            .map(|_| rng.random_range(2..=48))
            .collect();
        if tensor_moduli(&x, &y).iter().any(|&d| d > 1) {
            (m, n) = (x, y);
            break;
        }
    }
    let moduli = tensor_moduli(&m, &n);
    let (mut a, mut b) = (vec![1; m.len()], vec![1; n.len()]);
    for _ in 0..64 {
        let x: Vec<_> = m.iter().map(|&d| rng.random_range(0..d)).collect();
        let y: Vec<_> = n.iter().map(|&d| rng.random_range(0..d)).collect();
        if image(&m, &n, &x, &y).iter().any(|&v| v != 0) {
            (a, b) = (x, y);
            break;
        }
    }
    let values = image(&m, &n, &a, &b);
    let k = rng.random_range(2..=9);
    let scaled_a: Vec<_> = a.iter().zip(&m).map(|(&v, &d)| k * v % d).collect();
    let scaled_values = image(&m, &n, &scaled_a, &b);
    let context = format!("M = {}\nN = {}", module(&m), module(&n));
    let tensor = format!("M ⊗_Z N ≅ {}", module(&moduli));
    match rng.random_range(0..3) {
        0 => format!(
            "({}) ⊗_Z ({}) ≅ {}",
            module(&m),
            module(&n),
            module(&moduli)
        ),
        1 => format!(
            "{context}\na = {}, b = {}\n{tensor}\na ⊗ b ↦ {}\nord(a ⊗ b) = {}",
            tuple(&a),
            tuple(&b),
            coordinates(&moduli, &values),
            element_order(&moduli, &values)
        ),
        _ => format!(
            "{context}\na = {}, b = {}\n{tensor}\n{k}(a ⊗ b) = {} ⊗ {} ↦ {}",
            tuple(&a),
            tuple(&b),
            tuple(&scaled_a),
            tuple(&b),
            coordinates(&moduli, &scaled_values)
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn cyclic_tensor_map_is_well_defined_and_has_exact_generator_order() {
        for m in 2..=48 {
            for n in 2..=48 {
                let g = gcd(m, n);
                // The tensor of the two cyclic generators is killed by m and n,
                // and the product map to Z/gZ attains all g residue classes.
                assert_eq!(m % g, 0);
                assert_eq!(n % g, 0);
                for a in 0..m {
                    for b in 0..n {
                        assert_eq!(
                            image(&[m], &[n], &[a + m], &[b]),
                            image(&[m], &[n], &[a], &[b])
                        );
                        assert_eq!(
                            image(&[m], &[n], &[a], &[b + n]),
                            image(&[m], &[n], &[a], &[b])
                        );
                    }
                }
                let images: std::collections::HashSet<_> =
                    (0..m).map(|a| image(&[m], &[n], &[a], &[1])[0]).collect();
                assert_eq!(images.len() as u32, g);
                assert_eq!(element_order(&[g], &[1 % g]), u64::from(g));
            }
        }
    }

    #[test]
    fn bilinearity_and_orders_agree_with_repeated_addition() {
        let (m, n) = (vec![6, 10], vec![15, 21]);
        let ds = tensor_moduli(&m, &n);
        assert_eq!(ds, [3, 3, 5, 1]);
        for a0 in 0..6 {
            for a1 in 0..10 {
                for b0 in 0..15 {
                    let a = [a0, a1];
                    let b = [b0, 1];
                    let values = image(&m, &n, &a, &b);
                    let order = element_order(&ds, &values);
                    for k in 1..=order as u32 {
                        let scaled: Vec<_> = a.iter().zip(&m).map(|(&v, &d)| k * v % d).collect();
                        let mapped = image(&m, &n, &scaled, &b);
                        let repeated: Vec<_> =
                            values.iter().zip(&ds).map(|(&v, &d)| k * v % d).collect();
                        assert_eq!(mapped, repeated);
                        let scaled_b: Vec<_> = b.iter().zip(&n).map(|(&v, &d)| k * v % d).collect();
                        assert_eq!(image(&m, &n, &a, &scaled_b), repeated);
                        assert_eq!(mapped.iter().all(|&v| v == 0), u64::from(k) == order);
                    }
                    let a_plus_one = [(a0 + 1) % 6, (a1 + 1) % 10];
                    let plus = image(&m, &n, &a_plus_one, &b);
                    let ones = image(&m, &n, &[1, 1], &b);
                    assert_eq!(
                        plus,
                        values
                            .iter()
                            .zip(ones)
                            .zip(&ds)
                            .map(|((&v, w), &d)| (v + w) % d)
                            .collect::<Vec<_>>()
                    );
                    let b_plus_one = [(b0 + 1) % 15, 2];
                    let plus = image(&m, &n, &a, &b_plus_one);
                    let ones = image(&m, &n, &a, &[1, 1]);
                    assert_eq!(
                        plus,
                        values
                            .iter()
                            .zip(ones)
                            .zip(&ds)
                            .map(|((&v, w), &d)| (v + w) % d)
                            .collect::<Vec<_>>()
                    );
                }
            }
        }
    }

    #[test]
    fn maximal_tensor_size_fits_and_fallback_is_nonzero() {
        let moduli = tensor_moduli(&[48; 3], &[48; 3]);
        let size: u64 = moduli.iter().map(|&d| u64::from(d)).product();
        assert_eq!(size, 48_u64.pow(9));
        assert_eq!(element_order(&moduli, &[1; 9]), 48);
        let fallback = image(&[6, 10], &[15, 21], &[1, 1], &[1, 1]);
        assert_eq!(fallback, [1, 1, 1, 0]);
        assert_eq!(element_order(&[3, 3, 5, 1], &fallback), 15);
    }

    #[test]
    fn seeded_nonzero_examples_are_diverse() {
        let mut rng = StdRng::seed_from_u64(42);
        let outputs: std::collections::HashSet<_> = (0..400).map(|_| generate(&mut rng)).collect();
        assert!(outputs.len() > 390);
    }
}
