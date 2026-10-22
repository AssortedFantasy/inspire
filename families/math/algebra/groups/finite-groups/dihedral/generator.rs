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
    match (a, b) {
        (0, 0) => "e".into(),
        (0, 1) => "s".into(),
        (1, 0) => "r".into(),
        (1, 1) => "rs".into(),
        (_, 0) => format!("r^{a}"),
        _ => format!("r^{a}s"),
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let n = rng.random_range(3..=48);
    let x = (rng.random_range(0..n), rng.random_range(0..2));
    let y = (rng.random_range(0..n), rng.random_range(0..2));
    let heading = format!("D_{n} = ⟨r,s | r^{n}=s²=e, srs=r⁻¹⟩");
    match rng.random_range(0..4) {
        0 => {
            let xy = product(n, x, y);
            let yx = product(n, y, x);
            let calculation = if x == y {
                format!("({})² = {}", display(x), display(xy))
            } else if xy == yx {
                format!(
                    "({})({}) = ({})({}) = {}",
                    display(x),
                    display(y),
                    display(y),
                    display(x),
                    display(xy)
                )
            } else {
                format!(
                    "({})({}) = {}\n({})({}) = {}",
                    display(x),
                    display(y),
                    display(xy),
                    display(y),
                    display(x),
                    display(yx)
                )
            };
            format!(
                "{heading}\n{calculation}\n({})⁻¹ = {}",
                display(x),
                display(inverse(n, x))
            )
        }
        1 => {
            let mut class: Vec<_> = (0..n)
                .map(|k| product(n, product(n, (k, 0), x), inverse(n, (k, 0))))
                .chain((0..n).map(|k| product(n, product(n, (k, 1), x), inverse(n, (k, 1)))))
                .collect();
            class.sort_unstable();
            class.dedup();
            format!(
                "{heading}\nCl({}) = {{{}}}\nZ(D_{n}) = {{{}}}",
                display(x),
                class
                    .into_iter()
                    .map(display)
                    .collect::<Vec<_>>()
                    .join(", "),
                center(n)
                    .into_iter()
                    .map(display)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
        2 => {
            let vertices = (0..n).map(|t| t.to_string()).collect::<Vec<_>>().join(" ");
            let images = (0..n)
                .map(|t| ((x.0 + if x.1 == 0 { t } else { (n - t) % n }) % n).to_string())
                .collect::<Vec<_>>()
                .join(" ");
            format!(
                "D_{n} ↷ Z/{n}Z\nr(t) = t+1\ns(t) = −t\n{} · [{vertices}] = [{images}]\nord({}) = {}",
                display(x),
                display(x),
                order(n, x)
            )
        }
        _ => {
            let centralizer = (0..n)
                .flat_map(|a| [(a, 0), (a, 1)])
                .filter(|&y| product(n, x, y) == product(n, y, x))
                .map(display)
                .collect::<Vec<_>>();
            let members = if centralizer.len() == 2 * n {
                format!("D_{n}")
            } else if centralizer.len() > 4 {
                "⟨r⟩".to_owned()
            } else {
                format!("{{{}}}", centralizer.join(", "))
            };
            format!(
                "{heading}\nC_D{n}({}) = {members}\n|C_D{n}({})| = {}",
                display(x),
                display(x),
                centralizer.len()
            )
        }
    }
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
    fn polygon_action_matches_group_product() {
        for n in 3..=12 {
            let apply = |(a, b): Element, t| (a + if b == 0 { t } else { (n - t) % n }) % n;
            for x in elements(n) {
                for y in elements(n) {
                    for t in 0..n {
                        assert_eq!(apply(product(n, x, y), t), apply(x, apply(y, t)));
                    }
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
            outputs.insert(output);
        }
        assert!(outputs.len() > 200);
    }
}
