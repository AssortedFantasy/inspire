use rand::{Rng, rngs::StdRng};

const PRIMES: [usize; 10] = [3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
type Element = (usize, usize);

fn product(p: usize, (a, b): Element, (c, d): Element) -> Element {
    (a * c % p, (a * d + b) % p)
}

fn reciprocal(p: usize, a: usize) -> usize {
    (1..p).find(|&c| a * c % p == 1).unwrap()
}

fn inverse(p: usize, (a, b): Element) -> Element {
    let c = reciprocal(p, a);
    (c, (p - c * b % p) % p)
}

fn order(p: usize, (a, b): Element) -> usize {
    if a == 1 {
        return if b == 0 { 1 } else { p };
    }
    let mut power = a;
    for k in 1..p {
        if power == 1 {
            return k;
        }
        power = power * a % p;
    }
    unreachable!("nonzero elements of a prime field have finite order")
}

fn fixed_point(p: usize, (a, b): Element) -> usize {
    b * reciprocal(p, (p + 1 - a) % p) % p
}

pub fn generate(rng: &mut StdRng) -> String {
    let p = PRIMES[rng.random_range(0..PRIMES.len())];
    let mut x = (rng.random_range(1..p), rng.random_range(0..p));
    let y = (rng.random_range(1..p), rng.random_range(0..p));
    let map = |(a, b): Element| match (a, b) {
        (1, 0) => "t".to_owned(),
        (1, _) => format!("t+{b}"),
        (_, 0) => format!("{a}t"),
        _ => format!("{a}t+{b}"),
    };
    let heading = format!("AGL(1,{p}) over F_{p}");
    match rng.random_range(0..4) {
        0 => format!(
            "{heading}\nf(t) = {}\ng(t) = {}\n(f∘g)(t) = {}\n(g∘f)(t) = {}\nf⁻¹(t) = {}",
            map(x),
            map(y),
            map(product(p, x, y)),
            map(product(p, y, x)),
            map(inverse(p, x))
        ),
        1 => {
            if x == (1, 0) {
                x.1 = 1;
            }
            let start = if x.0 == 1 {
                rng.random_range(0..p)
            } else {
                (fixed_point(p, x) + rng.random_range(1..p)) % p
            };
            let mut orbit = vec![start.to_string()];
            let mut t = start;
            for _ in 0..order(p, x) {
                t = (x.0 * t + x.1) % p;
                orbit.push(t.to_string());
            }
            format!(
                "{heading}\nf(t) = {}\n{}\nord(f) = {}",
                map(x),
                orbit.join(" → "),
                order(p, x)
            )
        }
        2 => {
            x.0 = rng.random_range(2..p);
            x.1 = rng.random_range(1..p);
            let c = fixed_point(p, x);
            format!(
                "{heading}\nf(t) = {}\nf({c}) = {c}\nT(t) = t+{c}\n(T⁻¹∘f∘T)(t) = {}t",
                map(x),
                x.0
            )
        }
        _ => {
            let comm = product(
                p,
                product(p, product(p, x, y), inverse(p, x)),
                inverse(p, y),
            );
            format!(
                "{heading}\nf(t) = {}\ng(t) = {}\n(f∘g∘f⁻¹∘g⁻¹)(t) = {}",
                map(x),
                map(y),
                map(comm)
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    fn elements(p: usize) -> Vec<Element> {
        (1..p).flat_map(|a| (0..p).map(move |b| (a, b))).collect()
    }

    fn apply(p: usize, (a, b): Element, t: usize) -> usize {
        (a * t + b) % p
    }

    #[test]
    fn exhaustive_small_group_axioms() {
        for p in [3, 5] {
            let es = elements(p);
            for &x in &es {
                assert_eq!(product(p, (1, 0), x), x);
                assert_eq!(product(p, x, (1, 0)), x);
                for &y in &es {
                    let xy = product(p, x, y);
                    assert!(es.contains(&xy));
                    for t in 0..p {
                        assert_eq!(apply(p, xy, t), apply(p, x, apply(p, y, t)));
                    }
                    for &z in &es {
                        assert_eq!(product(p, xy, z), product(p, x, product(p, y, z)));
                    }
                }
            }
        }
    }

    #[test]
    fn inverses_orders_actions_and_fixed_points_across_all_primes() {
        for p in PRIMES {
            for x in elements(p) {
                assert_eq!(product(p, x, inverse(p, x)), (1, 0));
                assert_eq!(product(p, inverse(p, x), x), (1, 0));
                let n = order(p, x);
                let mut power = (1, 0);
                for k in 1..=n {
                    power = product(p, power, x);
                    assert_eq!(power == (1, 0), k == n);
                }
                if x.0 != 1 {
                    assert_eq!((p - 1) % n, 0);
                }
                let fixed: Vec<_> = (0..p).filter(|&t| apply(p, x, t) == t).collect();
                if x == (1, 0) {
                    assert_eq!(fixed.len(), p);
                } else if x.0 == 1 {
                    assert!(fixed.is_empty());
                    assert_eq!(n, p);
                } else {
                    assert_eq!(fixed, vec![fixed_point(p, x)]);
                }
                for y in [(1, 0), (1, 1), (2, p - 1), (p - 1, p / 2)] {
                    for t in 0..p {
                        assert_eq!(apply(p, product(p, x, y), t), apply(p, x, apply(p, y, t)));
                    }
                    let comm = product(
                        p,
                        product(p, product(p, x, y), inverse(p, x)),
                        inverse(p, y),
                    );
                    assert_eq!(
                        comm,
                        (1, (x.1 + x.0 * y.1 + 2 * p * p - y.0 * x.1 - y.1) % p)
                    );
                }
            }
            assert_eq!(product(p, (2, 0), (1, 1)), (2, 2));
            assert_eq!(product(p, (1, 1), (2, 0)), (2, 1));
        }
    }

    #[test]
    fn seeded_samples_cover_all_prime_fields() {
        let mut first = StdRng::seed_from_u64(20261013);
        let mut second = StdRng::seed_from_u64(20261013);
        let mut fields = HashSet::new();
        let mut outputs = HashSet::new();
        for _ in 0..256 {
            let text = generate(&mut first);
            assert_eq!(text, generate(&mut second));
            assert!(!text.ends_with('\n'));
            for p in PRIMES {
                if text.starts_with(&format!("AGL(1,{p}) over F_{p}")) {
                    fields.insert(p);
                }
            }
            outputs.insert(text);
        }
        assert_eq!(fields.len(), PRIMES.len());
        assert!(outputs.len() > 200);
    }
}
