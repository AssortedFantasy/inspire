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
    match (i, j) {
        (0, 0) => "e".into(),
        (0, 1) => "s".into(),
        (1, 0) => "r".into(),
        (1, 1) => "rs".into(),
        (_, 0) => format!("r^{i}"),
        _ => format!("r^{i}s"),
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let m = PARAMETERS[rng.random_range(0..PARAMETERS.len())];
    let n = 4 * m;
    let a = (rng.random_range(0..2 * m), rng.random_range(0..2));
    let b = (rng.random_range(0..2 * m), rng.random_range(0..2));
    let heading = format!("Q_{n} = ⟨r,s | r^{}=e, s²=r^{m}, srs⁻¹=r⁻¹⟩", 2 * m);
    match rng.random_range(0..3) {
        0 => {
            let ab = product(m, a, b);
            let ba = product(m, b, a);
            let calculation = if a == b {
                format!("({})² = {}", display(a), display(ab))
            } else if ab == ba {
                format!(
                    "({})({}) = ({})({}) = {}",
                    display(a),
                    display(b),
                    display(b),
                    display(a),
                    display(ab)
                )
            } else {
                format!(
                    "({})({}) = {}\n({})({}) = {}",
                    display(a),
                    display(b),
                    display(ab),
                    display(b),
                    display(a),
                    display(ba)
                )
            };
            format!(
                "{heading}\n{calculation}\n({})⁻¹ = {}\nord({}) = {}",
                display(a),
                display(inverse(m, a)),
                display(a),
                order(m, a)
            )
        }
        1 => {
            let mut class: Vec<_> = (0..2 * m)
                .flat_map(|i| [(i, 0), (i, 1)])
                .map(|g| product(m, product(m, g, a), inverse(m, g)))
                .collect();
            class.sort_unstable();
            class.dedup();
            let members = if class.len() > 12 {
                format!("{}⟨r²⟩", display(a))
            } else {
                format!(
                    "{{{}}}",
                    class
                        .into_iter()
                        .map(display)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            };
            format!(
                "{heading}\nCl({}) = {members}\nZ(Q_{n}) = {{e, r^{m}}}",
                display(a),
            )
        }
        _ => {
            let a = (a.0, 1);
            let square = product(m, a, a);
            let cube = product(m, square, a);
            format!(
                "{heading}\nx = {}\nx² = {}\nx³ = {}\nx⁴ = e\n⟨x⟩ ∩ ⟨r⟩ = {{e, r^{m}}}",
                display(a),
                display(square),
                display(cube)
            )
        }
    }
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
            orders.insert(text.split(" = ").next().unwrap().to_owned());
            outputs.insert(text);
        }
        assert_eq!(orders.len(), PARAMETERS.len());
        assert!(outputs.len() > 150);
    }
}
