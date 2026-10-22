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

fn matrix((a, b, c): Element) -> String {
    format!("[[1,{a},{c}],[0,1,{b}],[0,0,1]]")
}

fn tuple((a, b, c): Element) -> String {
    format!("({a},{b},{c})")
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
    let heading = format!("UT₃(F_{p})");
    match rng.random_range(0..4) {
        0 => {
            let xy = product(p, x, y);
            let yx = product(p, y, x);
            let products = if xy == yx {
                format!("XY = YX = {}", matrix(xy))
            } else {
                format!("XY = {}\nYX = {}", matrix(xy), matrix(yx))
            };
            format!(
                "{heading}\nX = {}\nY = {}\n{products}",
                matrix(x),
                matrix(y)
            )
        }
        1 => format!(
            "{heading}\nX = {}\nY = {}\nXYX⁻¹Y⁻¹ = {}",
            matrix(x),
            matrix(y),
            matrix(commutator(p, x, y))
        ),
        2 => {
            let class = if x.0 == 0 && x.1 == 0 {
                vec![x]
            } else {
                (0..p).map(|c| (x.0, x.1, c)).collect()
            };
            format!(
                "{heading}\n(a,b,c) ↔ [[1,a,c],[0,1,b],[0,0,1]]\nCl({}) = {{{}}}\n|C({})| = {}",
                tuple(x),
                class
                    .iter()
                    .copied()
                    .map(tuple)
                    .collect::<Vec<_>>()
                    .join(", "),
                tuple(x),
                p.pow(3) / class.len()
            )
        }
        _ => {
            let mut powers = vec!["X = ".to_owned() + &matrix(x)];
            let mut power = x;
            for k in 2..=order(p, x) {
                power = product(p, power, x);
                powers.push(format!("X^{k} = {}", matrix(power)));
            }
            format!(
                "{heading}\n{}\nX⁻¹ = {}",
                powers.join("\n"),
                matrix(inverse(p, x))
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
    fn conjugacy_classes_and_centralizer_sizes() {
        for p in [2, 3, 5] {
            let es = elements(p);
            for &x in &es {
                let actual: HashSet<_> = es
                    .iter()
                    .map(|&g| product(p, product(p, g, x), inverse(p, g)))
                    .collect();
                let expected: HashSet<_> = if x.0 == 0 && x.1 == 0 {
                    [x].into_iter().collect()
                } else {
                    (0..p).map(|c| (x.0, x.1, c)).collect()
                };
                assert_eq!(actual, expected);
                let centralizer = es
                    .iter()
                    .filter(|&&g| product(p, x, g) == product(p, g, x))
                    .count();
                assert_eq!(actual.len() * centralizer, es.len());
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
                if text.starts_with(&format!("UT₃(F_{p})")) {
                    fields.insert(p);
                }
            }
            outputs.insert(text);
        }
        assert_eq!(fields.len(), PRIMES.len());
        assert!(outputs.len() > 200);
    }
}
