use rand::{Rng, rngs::StdRng};

const PRIMES: [usize; 6] = [2, 3, 5, 7, 11, 13];
type Element = (usize, usize, usize);
const IDENTITY: Element = (0, 0, 0);

fn product(p: usize, (a, b, c): Element, (u, v, w): Element) -> Element {
    ((a + u) % p, (b + v) % p, (c + w + a * v) % p)
}

fn inverse(p: usize, (a, b, c): Element) -> Element {
    ((p - a) % p, (p - b) % p, (a * b + p - c) % p)
}

fn commutator(p: usize, (a, b, _): Element, (u, v, _): Element) -> Element {
    (0, 0, (a * v + p * p - u * b) % p)
}

fn order(p: usize, x: Element) -> usize {
    if x == IDENTITY {
        1
    } else if p == 2 && x.0 * x.1 == 1 {
        4
    } else {
        p
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let p = PRIMES[rng.random_range(0..PRIMES.len())];
    let mut element = || {
        (
            rng.random_range(0..p),
            rng.random_range(0..p),
            rng.random_range(0..p),
        )
    };
    let x = element();
    let y = element();
    let xy = product(p, x, y);
    let inv = inverse(p, x);
    let comm = commutator(p, x, y);
    let exponent = if p == 2 { 4 } else { p };
    format!(
        "The finite Heisenberg group H(F_{p}) consists of matrices [[1,a,c],[0,1,b],[0,0,1]], written as triples (a,b,c) over F_{p}. All arithmetic is modulo {p}. Its order is {}, its identity is (0,0,0), and its multiplication is (a,b,c)(u,v,w)=(a+u,b+v,c+w+av).\n\
         For x=({},{},{}) and y=({},{},{}), xy=({},{},{}), x⁻¹=({},{},{}), and x has order {}. With the convention [x,y]=xyx⁻¹y⁻¹, [x,y]=({},{},{}); in general this commutator is (0,0,av-ub).\n\
         The center and commutator subgroup both equal {{(0,0,c): c∈F_{p}}}, a cyclic group of order {p}. The quotient by the center is C_{p}×C_{p}, via (a,b,c)↦(a,b). The group has nilpotency class 2 and exponent {exponent}. It is nonabelian: (1,0,0)(0,1,0)=(1,1,1), whereas (0,1,0)(1,0,0)=(1,1,0).",
        p.pow(3),
        x.0,
        x.1,
        x.2,
        y.0,
        y.1,
        y.2,
        xy.0,
        xy.1,
        xy.2,
        inv.0,
        inv.1,
        inv.2,
        order(p, x),
        comm.0,
        comm.1,
        comm.2
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    fn elements(p: usize) -> Vec<Element> {
        (0..p)
            .flat_map(|a| (0..p).flat_map(move |b| (0..p).map(move |c| (a, b, c))))
            .collect()
    }

    fn matrix((a, b, c): Element) -> [[usize; 3]; 3] {
        [[1, a, c], [0, 1, b], [0, 0, 1]]
    }

    #[test]
    fn exhaustive_small_group_axioms_and_matrix_law() {
        for p in [2, 3] {
            let es = elements(p);
            for &x in &es {
                assert_eq!(product(p, IDENTITY, x), x);
                assert_eq!(product(p, x, IDENTITY), x);
                for &y in &es {
                    let xy = product(p, x, y);
                    assert!(es.contains(&xy));
                    let mx = matrix(x);
                    let my = matrix(y);
                    let mxy = matrix(xy);
                    for (row, expected_row) in mx.iter().zip(mxy.iter()) {
                        for (j, expected) in expected_row.iter().enumerate() {
                            assert_eq!(
                                *expected,
                                row.iter()
                                    .enumerate()
                                    .map(|(k, value)| value * my[k][j])
                                    .sum::<usize>()
                                    % p
                            );
                        }
                    }
                    for &z in &es {
                        assert_eq!(product(p, xy, z), product(p, x, product(p, y, z)));
                    }
                }
            }
        }
    }

    #[test]
    fn inverses_orders_center_and_commutators_for_every_prime() {
        for p in PRIMES {
            for x in elements(p) {
                assert_eq!(product(p, x, inverse(p, x)), IDENTITY);
                assert_eq!(product(p, inverse(p, x), x), IDENTITY);
                let n = order(p, x);
                let mut power = IDENTITY;
                for k in 1..=n {
                    power = product(p, power, x);
                    assert_eq!(power == IDENTITY, k == n);
                }
                let central = [(1, 0, 0), (0, 1, 0)]
                    .into_iter()
                    .all(|y| product(p, x, y) == product(p, y, x));
                assert_eq!(central, x.0 == 0 && x.1 == 0);
                for y in [(1, 0, 0), (0, 1, 0), (p - 1, p - 1, p - 1), (p / 2, 1, 0)] {
                    assert_eq!(
                        commutator(p, x, y),
                        product(
                            p,
                            product(p, product(p, x, y), inverse(p, x)),
                            inverse(p, y)
                        )
                    );
                }
            }
            for c in 0..p {
                assert_eq!(commutator(p, (c, 0, 0), (0, 1, 0)), (0, 0, c));
            }
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
                if text.starts_with(&format!("The finite Heisenberg group H(F_{p})")) {
                    fields.insert(p);
                }
            }
            outputs.insert(text);
        }
        assert_eq!(fields.len(), PRIMES.len());
        assert!(outputs.len() > 200);
    }
}
