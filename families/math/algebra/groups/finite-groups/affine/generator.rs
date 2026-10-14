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
    let x = (rng.random_range(1..p), rng.random_range(0..p));
    let y = (rng.random_range(1..p), rng.random_range(0..p));
    let xy = product(p, x, y);
    let inv = inverse(p, x);
    let fixed = if x == (1, 0) {
        format!("Every one of the {p} field elements is fixed by x.")
    } else if x.0 == 1 {
        "The nonzero translation x has no fixed points.".to_string()
    } else {
        format!(
            "The unique fixed point of x is t={}; shifting this point to zero conjugates x to multiplication by {}.",
            fixed_point(p, x),
            x.0
        )
    };
    format!(
        "The affine group AGL(1,{p}) consists of maps t↦at+b on F_{p}, represented by pairs (a,b) with a≠0. All arithmetic is modulo {p}. Its order is {}, its identity is (1,0), and multiplication means composition with the right-hand map applied first: (a,b)(c,d)=(ac,ad+b).\n\
         For x=({},{}) and y=({},{}), xy=({},{}), x⁻¹=({},{}), and the order of x is {}. {fixed}\n\
         Translations (1,b) form a normal cyclic subgroup of order {p}; dilations (a,0) form a cyclic subgroup of order {}. Thus AGL(1,{p})≅(F_{p},+)⋊F_{p}^×, with dilations acting by multiplication. The group is nonabelian: (2,0)(1,1)=(2,2), whereas (1,1)(2,0)=(2,1).",
        p * (p - 1),
        x.0,
        x.1,
        y.0,
        y.1,
        xy.0,
        xy.1,
        inv.0,
        inv.1,
        order(p, x),
        p - 1
    )
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
                if text.starts_with(&format!("The affine group AGL(1,{p})")) {
                    fields.insert(p);
                }
            }
            outputs.insert(text);
        }
        assert_eq!(fields.len(), PRIMES.len());
        assert!(outputs.len() > 200);
    }
}
