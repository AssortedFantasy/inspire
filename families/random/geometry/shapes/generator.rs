use rand::{Rng, rngs::StdRng};

type Point = (i64, i64);

pub fn generate(rng: &mut StdRng) -> String {
    let polygon = sample(rng);
    let path = polygon
        .iter()
        .enumerate()
        .map(|(i, (x, y))| format!("{} {x} {y}", if i == 0 { "M" } else { "L" }))
        .collect::<Vec<_>>()
        .join(" ");
    let area = area_twice(&polygon).abs();
    let area = if area % 2 == 0 {
        (area / 2).to_string()
    } else {
        format!("{}.5", area / 2)
    };
    let perimeter: f64 = edges(&polygon)
        .map(|(a, b)| (((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)) as f64).sqrt())
        .sum();
    let perimeter = format!("{perimeter:.2}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string();
    let reflex = (0..polygon.len())
        .filter(|&i| {
            cross(
                polygon[(i + polygon.len() - 1) % polygon.len()],
                polygon[i],
                polygon[(i + 1) % polygon.len()],
            ) < 0
        })
        .count();
    format!(
        "<path d=\"{path} Z\"/>\nArea = {area}\nPerimeter ≈ {perimeter}\nReflex vertices = {reflex}"
    )
}

fn sample(rng: &mut StdRng) -> Vec<Point> {
    let mut polygon = match rng.random_range(0..5) {
        0 => {
            let points: Vec<_> = (0..rng.random_range(8..=24))
                .map(|_| (rng.random_range(0..=30), rng.random_range(0..=30)))
                .chain([(0, 0), (30, 0), (0, 30)])
                .collect();
            convex_hull(points)
        }
        1 => {
            let rays: Vec<Point> = if rng.random_bool(0.5) {
                vec![
                    (1, 0),
                    (1, 1),
                    (0, 1),
                    (-1, 1),
                    (-1, 0),
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                ]
            } else {
                vec![
                    (2, 0),
                    (2, 1),
                    (1, 2),
                    (0, 2),
                    (-1, 2),
                    (-2, 1),
                    (-2, 0),
                    (-2, -1),
                    (-1, -2),
                    (0, -2),
                    (1, -2),
                    (2, -1),
                ]
            };
            rays.iter()
                .enumerate()
                .map(|(i, &(x, y))| {
                    let radius = if i % 2 == 0 {
                        rng.random_range(12..=20)
                    } else {
                        rng.random_range(1..=3)
                    };
                    (x * radius, y * radius)
                })
                .collect()
        }
        2 => {
            let n = rng.random_range(2..=6);
            let mut x = vec![0];
            let heights: Vec<_> = (0..n)
                .map(|i| {
                    if i % 2 == 0 {
                        rng.random_range(2..=5)
                    } else {
                        rng.random_range(7..=12)
                    }
                })
                .collect();
            for _ in 0..n {
                x.push(x.last().unwrap() + rng.random_range(1..=7));
            }
            let mut points = vec![(0, 0), (x[n], 0), (x[n], heights[n - 1])];
            for i in (0..n).rev() {
                points.push((x[i], heights[i]));
                if i > 0 {
                    points.push((x[i], heights[i - 1]));
                }
            }
            points
        }
        3 => {
            let width = rng.random_range(12..=30);
            let height = rng.random_range(12..=30);
            let left = rng.random_range(1..=width / 3);
            let right = width - rng.random_range(1..=width / 3);
            let floor = if rng.random_bool(0.5) {
                1
            } else {
                rng.random_range(2..=height / 2)
            };
            vec![
                (0, 0),
                (width, 0),
                (width, height),
                (right, height),
                (right, floor),
                (left, floor),
                (left, height),
                (0, height),
            ]
        }
        _ => {
            let width = rng.random_range(15..=50);
            let tip = rng.random_range(1..width);
            vec![(0, 0), (width, 0), (tip, 1)]
        }
    };
    let rotation = rng.random_range(0..4);
    let reflect = rng.random_bool(0.5);
    for point in &mut polygon {
        if reflect {
            point.0 = -point.0;
        }
        for _ in 0..rotation {
            *point = (-point.1, point.0);
        }
    }
    if area_twice(&polygon) < 0 {
        polygon.reverse();
    }
    let min_x = polygon.iter().map(|p| p.0).min().unwrap();
    let min_y = polygon.iter().map(|p| p.1).min().unwrap();
    for point in &mut polygon {
        point.0 -= min_x;
        point.1 -= min_y;
    }
    polygon
}

fn cross(a: Point, b: Point, c: Point) -> i64 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}

fn edges(polygon: &[Point]) -> impl Iterator<Item = (Point, Point)> + '_ {
    polygon
        .iter()
        .copied()
        .zip(polygon.iter().copied().cycle().skip(1))
        .take(polygon.len())
}

fn area_twice(polygon: &[Point]) -> i64 {
    edges(polygon).map(|(a, b)| a.0 * b.1 - a.1 * b.0).sum()
}

fn convex_hull(mut points: Vec<Point>) -> Vec<Point> {
    points.sort_unstable();
    points.dedup();
    fn half(points: impl Iterator<Item = Point>) -> Vec<Point> {
        let mut chain = Vec::new();
        for p in points {
            while chain.len() >= 2 && cross(chain[chain.len() - 2], chain[chain.len() - 1], p) <= 0
            {
                chain.pop();
            }
            chain.push(p);
        }
        chain.pop();
        chain
    }
    let mut hull = half(points.iter().copied());
    hull.extend(half(points.iter().rev().copied()));
    hull
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    fn intersect(a: Point, b: Point, c: Point, d: Point) -> bool {
        fn on(a: Point, b: Point, c: Point) -> bool {
            cross(a, b, c) == 0
                && (a.0.min(b.0)..=a.0.max(b.0)).contains(&c.0)
                && (a.1.min(b.1)..=a.1.max(b.1)).contains(&c.1)
        }
        on(a, b, c)
            || on(a, b, d)
            || on(c, d, a)
            || on(c, d, b)
            || (cross(a, b, c).signum() * cross(a, b, d).signum() < 0
                && cross(c, d, a).signum() * cross(c, d, b).signum() < 0)
    }

    fn scanline_area(polygon: &[Point]) -> f64 {
        let mut levels: Vec<_> = polygon.iter().map(|p| p.1).collect();
        levels.sort_unstable();
        levels.dedup();
        levels
            .windows(2)
            .map(|level| {
                let y = (level[0] + level[1]) as f64 / 2.;
                let mut intersections: Vec<f64> = edges(polygon)
                    .filter(|(a, b)| (a.1.min(b.1) as f64) < y && y < a.1.max(b.1) as f64)
                    .map(|(a, b)| {
                        a.0 as f64 + (y - a.1 as f64) * (b.0 - a.0) as f64 / (b.1 - a.1) as f64
                    })
                    .collect();
                intersections.sort_by(f64::total_cmp);
                assert_eq!(intersections.len() % 2, 0);
                intersections
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .map(|pair| pair[1] - pair[0])
                    .sum::<f64>()
                    * (level[1] - level[0]) as f64
            })
            .sum()
    }

    #[test]
    fn every_regime_is_simple_and_shoelace_matches_independent_slicing() {
        let mut rng = StdRng::seed_from_u64(401);
        let mut vertex_counts = HashSet::new();
        let mut concave = 0;
        for _ in 0..10_000 {
            let p = sample(&mut rng);
            let n = p.len();
            vertex_counts.insert(n);
            assert_eq!(p.iter().collect::<HashSet<_>>().len(), n);
            assert!(area_twice(&p) > 0);
            for i in 0..n {
                assert_ne!(cross(p[(i + n - 1) % n], p[i], p[(i + 1) % n]), 0);
                for j in i + 1..n {
                    if j != i + 1 && !(i == 0 && j == n - 1) {
                        assert!(!intersect(p[i], p[(i + 1) % n], p[j], p[(j + 1) % n]));
                    }
                }
            }
            concave +=
                usize::from((0..n).any(|i| cross(p[(i + n - 1) % n], p[i], p[(i + 1) % n]) < 0));
            assert!((scanline_area(&p) - area_twice(&p) as f64 / 2.).abs() < 1e-8);
        }
        assert!(concave > 4000);
        assert!(vertex_counts.len() >= 8);
    }

    #[test]
    fn hull_discards_duplicates_interior_points_and_flat_vertices() {
        let hull = convex_hull(vec![(0, 0), (2, 0), (4, 0), (4, 3), (0, 3), (2, 1), (0, 0)]);
        assert_eq!(hull, vec![(0, 0), (4, 0), (4, 3), (0, 3)]);
        assert_eq!(area_twice(&hull), 24);
        assert_eq!(scanline_area(&hull), 12.);
    }
}
