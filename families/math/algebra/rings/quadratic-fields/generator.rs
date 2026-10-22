use rand::{Rng, rngs::StdRng};

type Element = (i32, i32);
const PRIMES: [i32; 6] = [2, 3, 5, 7, 11, 13];

fn polynomials(p: i32) -> Vec<(i32, i32)> {
    (0..p)
        .flat_map(|b| (0..p).map(move |c| (b, c)))
        .filter(|&(b, c)| (0..p).all(|x| (x * x + b * x + c) % p != 0))
        .collect()
}

fn add(p: i32, x: Element, y: Element) -> Element {
    ((x.0 + y.0) % p, (x.1 + y.1) % p)
}

fn mul(p: i32, b: i32, c: i32, x: Element, y: Element) -> Element {
    (
        (x.0 * y.0 - c * x.1 * y.1).rem_euclid(p),
        (x.0 * y.1 + x.1 * y.0 - b * x.1 * y.1).rem_euclid(p),
    )
}

#[cfg(test)]
fn pow(p: i32, b: i32, c: i32, mut x: Element, mut n: i32) -> Element {
    let mut result = (1, 0);
    while n > 0 {
        if n % 2 == 1 {
            result = mul(p, b, c, result, x);
        }
        x = mul(p, b, c, x, x);
        n /= 2;
    }
    result
}

fn conjugate(p: i32, b: i32, x: Element) -> Element {
    ((x.0 - b * x.1).rem_euclid(p), (-x.1).rem_euclid(p))
}

fn norm(p: i32, b: i32, c: i32, x: Element) -> i32 {
    (x.0 * x.0 - b * x.0 * x.1 + c * x.1 * x.1).rem_euclid(p)
}

fn inverse(p: i32, b: i32, c: i32, x: Element) -> Element {
    let norm = norm(p, b, c, x);
    let scale = (1..p).find(|&a| a * norm % p == 1).unwrap();
    let conjugate = conjugate(p, b, x);
    (scale * conjugate.0 % p, scale * conjugate.1 % p)
}

fn order(p: i32, b: i32, c: i32, x: Element) -> i32 {
    let mut power = (1, 0);
    for n in 1..p * p {
        power = mul(p, b, c, power, x);
        if power == (1, 0) {
            return n;
        }
    }
    unreachable!("nonzero elements of a finite field have finite order")
}

fn display((a, d): Element) -> String {
    match (a, d) {
        (_, 0) => a.to_string(),
        (0, 1) => "α".to_owned(),
        (0, _) => format!("{d}α"),
        (_, 1) => format!("{a}+α"),
        _ => format!("{a}+{d}α"),
    }
}

fn polynomial(b: i32, c: i32) -> String {
    let linear = match b {
        0 => String::new(),
        1 => "+t".to_owned(),
        _ => format!("+{b}t"),
    };
    format!("t²{linear}+{c}")
}

pub fn generate(rng: &mut StdRng) -> String {
    let p = PRIMES[rng.random_range(0..PRIMES.len())];
    let candidates = polynomials(p);
    let (b, c) = candidates[rng.random_range(0..candidates.len())];
    let x = (rng.random_range(0..p), rng.random_range(1..p));
    let y = (rng.random_range(0..p), rng.random_range(1..p));
    let trace = (2 * x.0 - b * x.1).rem_euclid(p);
    let defining = polynomial(b, c);
    let model = format!("K = F_{p}[t]/({defining}), α = [t]");
    match rng.random_range(0..4) {
        0 => format!(
            "{model}\n({}) + ({}) = {}\n({})({}) = {}",
            display(x),
            display(y),
            display(add(p, x, y)),
            display(x),
            display(y),
            display(mul(p, b, c, x, y))
        ),
        1 => format!(
            "{model}\nx = {}\nx^{p} = {}\nTr(x) = {trace}\nN(x) = {}",
            display(x),
            display(conjugate(p, b, x)),
            norm(p, b, c, x)
        ),
        2 => format!(
            "{model}\n({})^(-1) = {}\nord({}) = {} in K^× ≅ C_{}",
            display(x),
            display(inverse(p, b, c, x)),
            display(x),
            order(p, b, c, x),
            p * p - 1
        ),
        _ => {
            let minimal_linear = (-trace).rem_euclid(p);
            let minimal_constant = norm(p, b, c, x);
            format!(
                "{model}\nx = {}\nmin_{{F_{p}}}(x) = {}\nRoots in K = {{{}, {}}}",
                display(x),
                polynomial(minimal_linear, minimal_constant),
                display(x),
                display(conjugate(p, b, x))
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn elements(p: i32) -> Vec<Element> {
        (0..p).flat_map(|a| (0..p).map(move |d| (a, d))).collect()
    }

    #[test]
    fn canonical_polynomial_display() {
        assert_eq!(polynomial(0, 2), "t²+2");
        assert_eq!(polynomial(1, 1), "t²+t+1");
        assert_eq!(polynomial(3, 4), "t²+3t+4");
    }

    #[test]
    fn every_quadratic_model_has_field_inverses_and_correct_frobenius() {
        for p in PRIMES {
            let models = polynomials(p);
            assert_eq!(models.len() as i32, p * (p - 1) / 2);
            for (b, c) in models {
                let mut maximum_order = 0;
                assert_eq!(
                    mul(p, b, c, (0, 1), (0, 1)),
                    ((-c).rem_euclid(p), (-b).rem_euclid(p))
                );
                for x in elements(p) {
                    assert_eq!(pow(p, b, c, x, p), conjugate(p, b, x));
                    assert_eq!(pow(p, b, c, x, p * p), x);
                    assert_eq!(
                        add(p, x, conjugate(p, b, x)),
                        ((2 * x.0 - b * x.1).rem_euclid(p), 0)
                    );
                    assert_eq!(pow(p, b, c, x, p + 1), (norm(p, b, c, x), 0));
                    let trace = (2 * x.0 - b * x.1).rem_euclid(p);
                    let quadratic = add(
                        p,
                        mul(p, b, c, x, x),
                        mul(p, b, c, ((-trace).rem_euclid(p), 0), x),
                    );
                    assert_eq!(add(p, quadratic, (norm(p, b, c, x), 0)), (0, 0));
                    if x.1 != 0 {
                        assert_ne!(conjugate(p, b, x), x);
                    }

                    if x != (0, 0) {
                        assert_ne!(norm(p, b, c, x), 0);
                        assert_eq!(mul(p, b, c, x, inverse(p, b, c, x)), (1, 0));
                        let n = order(p, b, c, x);
                        maximum_order = maximum_order.max(n);
                        assert_eq!((p * p - 1) % n, 0);
                        assert_eq!(pow(p, b, c, x, n), (1, 0));
                        for d in 1..n {
                            if n % d == 0 {
                                assert_ne!(pow(p, b, c, x, d), (1, 0));
                            }
                        }
                    }
                }
                assert_eq!(maximum_order, p * p - 1);
            }
        }
    }

    #[test]
    fn exhaustive_small_fields_satisfy_ring_laws() {
        for p in [2, 3] {
            for (b, c) in polynomials(p) {
                let all = elements(p);
                for &x in &all {
                    assert_eq!(mul(p, b, c, x, (1, 0)), x);
                    for &y in &all {
                        assert_eq!(mul(p, b, c, x, y), mul(p, b, c, y, x));
                        for &z in &all {
                            assert_eq!(
                                mul(p, b, c, mul(p, b, c, x, y), z),
                                mul(p, b, c, x, mul(p, b, c, y, z))
                            );
                            assert_eq!(
                                mul(p, b, c, x, add(p, y, z)),
                                add(p, mul(p, b, c, x, y), mul(p, b, c, x, z))
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn seeded_models_and_examples_are_diverse() {
        let mut rng = StdRng::seed_from_u64(42);
        let outputs: std::collections::HashSet<_> = (0..400).map(|_| generate(&mut rng)).collect();
        assert!(outputs.len() > 300);
    }
}
