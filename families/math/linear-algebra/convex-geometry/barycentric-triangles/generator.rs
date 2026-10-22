use rand::{Rng, rngs::StdRng};

type Point = [i64; 2];

pub fn generate(rng: &mut StdRng) -> String {
    let (vertices, weights) = sample(rng);
    let total: i64 = weights.iter().sum();
    let numerator = weighted_point(vertices, weights);
    let combination = weights
        .into_iter()
        .zip(['A', 'B', 'C'])
        .filter_map(|(n, label)| {
            if n == 0 {
                return None;
            }
            let coefficient = if n.abs() == total {
                String::new()
            } else {
                format!("{} ", fraction(n.abs(), total))
            };
            Some((n < 0, format!("{coefficient}{label}")))
        })
        .enumerate()
        .map(|(i, (negative, term))| {
            format!(
                "{}{term}",
                match (i == 0, negative) {
                    (true, false) => "",
                    (true, true) => "-",
                    (false, false) => " + ",
                    (false, true) => " - ",
                }
            )
        })
        .collect::<String>();
    format!(
        "A={:?}\nB={:?}\nC={:?}\nP=({}, {})\nP={combination}",
        vertices[0],
        vertices[1],
        vertices[2],
        fraction(numerator[0], total),
        fraction(numerator[1], total),
    )
}

fn sample(rng: &mut StdRng) -> ([Point; 3], [i64; 3]) {
    let [x, y] = [0; 2].map(|_| rng.random_range(-6..=6));
    let a = rng.random_range(2..=12);
    let b = rng.random_range(-8..=8);
    let c = rng.random_range(2..=12);
    let shear = rng.random_range(-3..=3);
    let mut weights = [0; 3].map(|_| rng.random_range(1..=9));
    match rng.random_range(0..4) {
        0 => weights[rng.random_range(0..3)] = 0,
        1 => {
            let i = rng.random_range(0..3);
            weights[i] = -rng.random_range(1..=9);
            weights[(i + 1) % 3] += 9;
        }
        _ => {}
    }
    (
        [[x, y], [x + a, y + shear * a], [x + b, y + shear * b + c]],
        weights,
    )
}

fn weighted_point(vertices: [Point; 3], weights: [i64; 3]) -> Point {
    [0, 1].map(|i| (0..3).map(|j| weights[j] * vertices[j][i]).sum())
}

#[cfg(test)]
fn subtract(a: Point, b: Point) -> Point {
    [a[0] - b[0], a[1] - b[1]]
}

#[cfg(test)]
fn cross(a: Point, b: Point) -> i64 {
    a[0] * b[1] - a[1] * b[0]
}

fn fraction(n: i64, d: i64) -> String {
    let (mut a, mut b) = (n.abs(), d);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    if d / a == 1 {
        (n / a).to_string()
    } else {
        format!("{}/{}", n / a, d / a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn signed_subtriangle_areas_independently_recover_barycentric_weights() {
        let mut rng = StdRng::seed_from_u64(463);
        let mut locations = [false; 3];
        for _ in 0..2_000 {
            let (vertices, weights) = sample(&mut rng);
            let location = if weights.iter().any(|&w| w < 0) {
                2
            } else if weights.contains(&0) {
                1
            } else {
                0
            };
            locations[location] = true;
            let total: i64 = weights.iter().sum();
            assert!(total > 0);
            let numerator = weighted_point(vertices, weights);
            let det = cross(
                subtract(vertices[1], vertices[0]),
                subtract(vertices[2], vertices[0]),
            );
            assert!(det > 0);
            let relative = vertices.map(|v| subtract(v.map(|n| n * total), numerator));
            for i in 0..3 {
                let opposite = cross(relative[(i + 1) % 3], relative[(i + 2) % 3]);
                assert_eq!(opposite, weights[i] * total * det);
                assert_eq!(opposite.signum(), weights[i].signum());
            }
        }
        assert!(locations.into_iter().all(|present| present));
    }
}
