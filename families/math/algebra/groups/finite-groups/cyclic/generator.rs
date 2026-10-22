use rand::{Rng, rngs::StdRng};

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn add(n: usize, a: usize, b: usize) -> usize {
    (a + b) % n
}

fn order(n: usize, a: usize) -> usize {
    n / gcd(n, a)
}

fn subgroup(n: usize, size: usize) -> Vec<usize> {
    (0..size).map(|i| i * (n / size)).collect()
}

fn display(values: &[usize]) -> String {
    if values.len() > 12 {
        return format!(
            "{}, {}, {}, …, {}",
            values[0],
            values[1],
            values[2],
            values.last().unwrap()
        );
    }
    values
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn generate(rng: &mut StdRng) -> String {
    let n = rng.random_range(2..=96);
    let a = rng.random_range(0..n);
    match rng.random_range(0..32) {
        1..=11 => format!(
            "Z/{n}Z\n⟨{a}⟩ = {{{}}}\nord({a}) = {}",
            display(&subgroup(n, order(n, a))),
            order(n, a)
        ),
        12..=21 => {
            let divisors: Vec<_> = (1..n).filter(|d| n % d == 0).collect();
            let size = divisors[rng.random_range(0..divisors.len())];
            let index = n / size;
            format!(
                "π: Z/{n}Z → Z/{index}Z\nπ(x) = x mod {index}\nker π = {{{}}}\n(Z/{n}Z)/ker π ≅ Z/{index}Z",
                display(&subgroup(n, size))
            )
        }
        22..=31 => {
            let kernel: Vec<_> = (0..n).filter(|&x| a * x % n == 0).collect();
            let image = subgroup(n, order(n, a));
            format!(
                "φ: Z/{n}Z → Z/{n}Z\nφ(x) = {a}x mod {n}\nker φ = {{{}}}\nim φ = {{{}}}",
                display(&kernel),
                display(&image)
            )
        }
        _ => {
            let n = rng.random_range(2..=9);
            let mut rows = vec![
                format!("Z/{n}Z"),
                format!("+ | {}", display(&(0..n).collect::<Vec<_>>())),
            ];
            for a in 0..n {
                let row: Vec<_> = (0..n).map(|b| add(n, a, b)).collect();
                rows.push(format!("{a} | {}", display(&row)));
            }
            rows.join("\n")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    #[test]
    fn exhaustive_group_laws() {
        for n in 2..=16 {
            for a in 0..n {
                assert_eq!(add(n, a, 0), a);
                assert_eq!(add(n, a, (n - a) % n), 0);
                for b in 0..n {
                    assert!(add(n, a, b) < n);
                    assert_eq!(add(n, a, b), add(n, b, a));
                    for c in 0..n {
                        assert_eq!(add(n, add(n, a, b), c), add(n, a, add(n, b, c)));
                    }
                }
            }
        }
    }

    #[test]
    fn exhaustive_element_orders_and_generated_subgroups() {
        for n in 2..=96 {
            for a in 0..n {
                let mut orbit = HashSet::new();
                let mut value = 0;
                while orbit.insert(value) {
                    value = add(n, value, a);
                }
                assert_eq!(value, 0);
                assert_eq!(orbit.len(), order(n, a));
                assert_eq!(orbit, subgroup(n, order(n, a)).into_iter().collect());
                assert_eq!(order(n, a) == n, gcd(n, a) == 1);
            }
        }
    }

    #[test]
    fn exhaustive_divisor_subgroups_and_quotients() {
        for n in 2..=96 {
            for size in (1..=n).filter(|d| n % d == 0) {
                let members = subgroup(n, size);
                assert_eq!(members.len(), size);
                assert_eq!(order(n, (n / size) % n), size);
                let index = n / size;
                for &a in &members {
                    assert!(members.contains(&((n - a) % n)));
                    for &b in &members {
                        assert!(members.contains(&add(n, a, b)));
                    }
                }
                // Reduction modulo the index has exactly this subgroup as kernel.
                for a in 0..n {
                    assert_eq!(members.contains(&a), a % index == 0);
                    for b in 0..n {
                        assert_eq!(add(n, a, b) % index, (a % index + b % index) % index);
                    }
                }
            }
        }
    }

    #[test]
    fn multiplication_endomorphism_kernel_and_image() {
        for n in 2..=96 {
            for a in 0..n {
                let kernel: Vec<_> = (0..n).filter(|&x| a * x % n == 0).collect();
                let image: HashSet<_> = (0..n).map(|x| a * x % n).collect();
                assert_eq!(kernel, subgroup(n, gcd(n, a)));
                assert_eq!(image, subgroup(n, order(n, a)).into_iter().collect());
                assert_eq!(kernel.len() * image.len(), n);
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
        assert!(outputs.len() > 170);
    }
}
