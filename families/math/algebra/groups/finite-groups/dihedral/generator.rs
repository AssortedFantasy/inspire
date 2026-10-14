use rand::{Rng, rngs::StdRng};

type Element = (usize, usize);

fn product(n: usize, (a, b): Element, (c, d): Element) -> Element {
    ((a + if b == 0 { c } else { n - c }) % n, (b + d) % 2)
}

fn inverse(n: usize, (a, b): Element) -> Element {
    (if b == 0 { (n - a) % n } else { a }, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn order(n: usize, (a, b): Element) -> usize {
    if b == 0 { n / gcd(n, a) } else { 2 }
}

fn center(n: usize) -> Vec<Element> {
    if n.is_multiple_of(2) {
        vec![(0, 0), (n / 2, 0)]
    } else {
        vec![(0, 0)]
    }
}

fn display((a, b): Element) -> String {
    format!("({a},{b})")
}

pub fn generate(rng: &mut StdRng) -> String {
    let n = rng.random_range(3..=48);
    let x = (rng.random_range(0..n), rng.random_range(0..2));
    let y = (rng.random_range(0..n), rng.random_range(0..2));
    let z = center(n)
        .into_iter()
        .map(display)
        .collect::<Vec<_>>()
        .join(", ");
    let xy = product(n, x, y);
    let yx = product(n, y, x);
    let relation = if xy == yx {
        "commute"
    } else {
        "do not commute"
    };
    format!(
        "D_{n} is the symmetry group of a regular {n}-gon, of order {} (the subscript counts vertices).\n\
         Presentation: r^{n}=e, s^2=e, srs=r^(-1); r is one vertex-step rotation and s a reflection.\n\
         Each element has a unique coordinate (a,b)=r^a s^b, with 0<=a<{n}, b in {{0,1}}.\n\
         Law: (a,b)(c,d)=((a+(-1)^b c) mod {n}, (b+d) mod 2); identity (0,0).\n\
         Example: x={}, y={}; xy={}, yx={}; these elements {relation}.\n\
         Inverses: x^(-1)={}, y^(-1)={}. Orders: ord(x)={}, ord(y)={}.\n\
         In general ord((a,0))={n}/gcd({n},a), and every (a,1) has order 2.\n\
         Noncommuting witness: rs=(1,1), sr=({},1). Hence D_{n} is nonabelian.\n\
         Center: Z(D_{n})={{{z}}}. The rotations form a normal cyclic subgroup of order {n} and index 2.",
        2 * n,
        display(x),
        display(y),
        display(xy),
        display(yx),
        display(inverse(n, x)),
        display(inverse(n, y)),
        order(n, x),
        order(n, y),
        n - 1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    fn elements(n: usize) -> Vec<Element> {
        (0..n).flat_map(|a| [(a, 0), (a, 1)]).collect()
    }

    #[test]
    fn exhaustive_group_axioms() {
        for n in 3..=8 {
            let all = elements(n);
            for &x in &all {
                assert_eq!(product(n, x, (0, 0)), x);
                assert_eq!(product(n, (0, 0), x), x);
                assert_eq!(product(n, x, inverse(n, x)), (0, 0));
                assert_eq!(product(n, inverse(n, x), x), (0, 0));
                for &y in &all {
                    assert!(all.contains(&product(n, x, y)));
                    for &z in &all {
                        assert_eq!(
                            product(n, product(n, x, y), z),
                            product(n, x, product(n, y, z))
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn full_bounds_orders_inverses_and_center() {
        for n in 3..=48 {
            let all = elements(n);
            let actual_center: Vec<_> = all
                .iter()
                .copied()
                .filter(|&x| all.iter().all(|&y| product(n, x, y) == product(n, y, x)))
                .collect();
            assert_eq!(actual_center, center(n));
            assert_ne!(product(n, (1, 0), (0, 1)), product(n, (0, 1), (1, 0)));
            for x in all {
                assert_eq!(product(n, x, inverse(n, x)), (0, 0));
                assert_eq!(product(n, inverse(n, x), x), (0, 0));
                let mut power = (0, 0);
                for k in 1..=order(n, x) {
                    power = product(n, power, x);
                    assert_eq!(power == (0, 0), k == order(n, x));
                }
            }
        }
    }

    #[test]
    fn seeded_outputs_are_varied_and_repeatable() {
        let mut first = StdRng::seed_from_u64(713);
        let mut second = StdRng::seed_from_u64(713);
        let mut outputs = HashSet::new();
        for _ in 0..256 {
            let output = generate(&mut first);
            assert_eq!(output, generate(&mut second));
            assert!(!output.ends_with('\n'));
            assert!(output.contains("Noncommuting witness"));
            outputs.insert(output);
        }
        assert!(outputs.len() > 240);
    }
}
