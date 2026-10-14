use rand::{Rng, rngs::StdRng};

type Element = (usize, usize);
const PARAMETERS: [usize; 6] = [2, 4, 8, 16, 32, 64];

fn product(m: usize, (i, j): Element, (k, l): Element) -> Element {
    let modulus = 2 * m;
    let signed_k = if j == 0 { k } else { modulus - k };
    ((i + signed_k + m * j * l) % modulus, (j + l) % 2)
}

fn inverse(m: usize, (i, j): Element) -> Element {
    (
        if j == 0 {
            (2 * m - i) % (2 * m)
        } else {
            (i + m) % (2 * m)
        },
        j,
    )
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn order(m: usize, (i, j): Element) -> usize {
    if j == 0 { 2 * m / gcd(2 * m, i) } else { 4 }
}

fn display((i, j): Element) -> String {
    format!("({i},{j})")
}

pub fn generate(rng: &mut StdRng) -> String {
    let m = PARAMETERS[rng.random_range(0..PARAMETERS.len())];
    let n = 4 * m;
    let a = (rng.random_range(0..2 * m), rng.random_range(0..2));
    let b = (rng.random_range(0..2 * m), rng.random_range(0..2));
    let special = if m == 2 {
        " This is the quaternion group Q_8."
    } else {
        ""
    };
    format!(
        "Generalized quaternion group Q_{n}, of order {n}; the subscript denotes the group order.{special}\n\
         Presentation: r^{}=e, s^2=r^{m}, srs^(-1)=r^(-1).\n\
         Every element has a unique coordinate (i,j)=r^i s^j, with 0<=i<{}, j in {{0,1}}.\n\
         Law: (i,j)(k,l)=((i+(-1)^j k+{m}jl) mod {}, (j+l) mod 2); identity (0,0).\n\
         For a={} and b={}, ab={}, ba={}. Inverses: a^(-1)={}, b^(-1)={}.\n\
         Element orders: ord(a)={}, ord(b)={}; generally ord((i,0))={}/gcd({},i), and ord((i,1))=4.\n\
         The center is {{(0,0),({m},0)}}. Its nonidentity element ({m},0) is the unique involution.\n\
         The cyclic subgroup <r> has order {} and index 2. Every element outside it squares to ({m},0).\n\
         Noncommuting witness: rs=(1,1), sr=({},1); Q_{n} is nonabelian.",
        2 * m,
        2 * m,
        2 * m,
        display(a),
        display(b),
        display(product(m, a, b)),
        display(product(m, b, a)),
        display(inverse(m, a)),
        display(inverse(m, b)),
        order(m, a),
        order(m, b),
        2 * m,
        2 * m,
        2 * m,
        2 * m - 1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    fn elements(m: usize) -> Vec<Element> {
        (0..2 * m).flat_map(|i| [(i, 0), (i, 1)]).collect()
    }

    #[test]
    fn exhaustive_small_group_axioms() {
        for m in [2, 4] {
            let group = elements(m);
            assert_eq!(group.len(), 4 * m);
            for &a in &group {
                assert_eq!(product(m, a, (0, 0)), a);
                assert_eq!(product(m, (0, 0), a), a);
                for &b in &group {
                    assert!(group.contains(&product(m, a, b)));
                    for &c in &group {
                        assert_eq!(
                            product(m, product(m, a, b), c),
                            product(m, a, product(m, b, c))
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn orders_inverses_center_and_involutions_across_range() {
        for m in PARAMETERS {
            let group = elements(m);
            let mut center = Vec::new();
            let mut involutions = Vec::new();
            for &a in &group {
                assert_eq!(product(m, a, inverse(m, a)), (0, 0));
                assert_eq!(product(m, inverse(m, a), a), (0, 0));
                let mut power = (0, 0);
                let mut actual_order = 0;
                loop {
                    actual_order += 1;
                    power = product(m, power, a);
                    if power == (0, 0) {
                        break;
                    }
                    assert!(actual_order <= 4 * m);
                }
                assert_eq!(order(m, a), actual_order);
                if actual_order == 2 {
                    involutions.push(a);
                }
                if group.iter().all(|&b| product(m, a, b) == product(m, b, a)) {
                    center.push(a);
                }
                if a.1 == 1 {
                    assert_eq!(product(m, a, a), (m, 0));
                }
            }
            assert_eq!(center, [(0, 0), (m, 0)]);
            assert_eq!(involutions, [(m, 0)]);
            assert_eq!(product(m, (0, 1), (0, 1)), (m, 0));
            assert_eq!(
                product(m, product(m, (0, 1), (1, 0)), inverse(m, (0, 1))),
                (2 * m - 1, 0)
            );
        }
    }

    #[test]
    fn seeded_samples_cover_all_group_orders() {
        let mut first = StdRng::seed_from_u64(20261013);
        let mut second = StdRng::seed_from_u64(20261013);
        let mut orders = HashSet::new();
        let mut outputs = HashSet::new();
        for _ in 0..256 {
            let text = generate(&mut first);
            assert_eq!(text, generate(&mut second));
            assert!(!text.ends_with('\n'));
            orders.insert(
                text.lines()
                    .next()
                    .unwrap()
                    .split(',')
                    .next()
                    .unwrap()
                    .to_owned(),
            );
            outputs.insert(text);
        }
        assert_eq!(orders.len(), PARAMETERS.len());
        assert!(outputs.len() > 200);
    }
}
