use rand::{Rng, rngs::StdRng};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Q(i64, i64);

impl Q {
    fn new(mut n: i64, mut d: i64) -> Self {
        assert_ne!(d, 0);
        if d < 0 {
            n = -n;
            d = -d;
        }
        let (mut a, mut b) = (n.abs(), d);
        while b != 0 {
            (a, b) = (b, a % b);
        }
        Self(n / a, d / a)
    }
    fn add(self, rhs: Self) -> Self {
        Self::new(self.0 * rhs.1 + rhs.0 * self.1, self.1 * rhs.1)
    }
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.0 * rhs.1 - rhs.0 * self.1, self.1 * rhs.1)
    }
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.0 * rhs.0, self.1 * rhs.1)
    }
    fn scale(self, n: i64) -> Self {
        Self::new(self.0 * n, self.1)
    }
}

impl fmt::Display for Q {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.1 == 1 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}/{}", self.0, self.1)
        }
    }
}

type Point = [i64; 2];
type Exact = [Q; 2];

fn subtract(a: Point, b: Point) -> Point {
    [a[0] - b[0], a[1] - b[1]]
}

fn dot(a: Point, b: Point) -> i64 {
    a[0] * b[0] + a[1] * b[1]
}

fn cross(a: Point, b: Point) -> i64 {
    a[0] * b[1] - a[1] * b[0]
}

fn exact(p: Point) -> Exact {
    p.map(|x| Q::new(x, 1))
}

fn center_string([x, y]: Exact) -> String {
    format!("({x}, {y})")
}

fn sample(rng: &mut StdRng, regime: usize) -> [Point; 3] {
    let (b, x, y) = match regime {
        0 => {
            let a = rng.random_range(3..=7);
            (
                2 * a,
                a + if rng.random_bool(0.5) { 1 } else { -1 },
                rng.random_range(a + 1..=2 * a),
            )
        }
        1 => {
            let a = rng.random_range(3..=9);
            (2 * a, a, rng.random_range(1..a))
        }
        2 => (rng.random_range(2..=14), 0, rng.random_range(2..=14)),
        3 => {
            let h = rng.random_range(2..=7);
            (h * h + rng.random_range(0..=2), 1, h)
        }
        4 => {
            let models = [(14, 5, 12), (21, 5, 12), (17, 7, 24), (25, 7, 24)];
            models[rng.random_range(0..models.len())]
        }
        5 => {
            let b = rng.random_range(9..=20);
            (b, rng.random_range(2..b - 1), 1)
        }
        _ => {
            let b = rng.random_range(3..=15);
            (b, rng.random_range(1..b), 0)
        }
    };
    let turn = rng.random_range(0..4);
    let reflection = if rng.random_bool(0.5) { -1 } else { 1 };
    let shift = [0; 2].map(|_| rng.random_range(-9..=9));
    [[0, 0], [b, 0], [x, y]].map(|[x, y]| {
        let y = y * reflection;
        let [x, y] = match turn {
            0 => [x, y],
            1 => [-y, x],
            2 => [-x, -y],
            _ => [y, -x],
        };
        [x + shift[0], y + shift[1]]
    })
}

fn centers([a, b, c]: [Point; 3]) -> Option<(Exact, Exact, Exact)> {
    let u = subtract(b, a);
    let v = subtract(c, a);
    let d = 2 * cross(u, v);
    if d == 0 {
        return None;
    }
    let uu = dot(u, u);
    let vv = dot(v, v);
    let o = [
        Q::new(uu * v[1] - vv * u[1], d).add(Q::new(a[0], 1)),
        Q::new(vv * u[0] - uu * v[0], d).add(Q::new(a[1], 1)),
    ];
    let g = [0, 1].map(|i| Q::new(a[i] + b[i] + c[i], 3));
    let h = [0, 1].map(|i| Q::new(a[i] + b[i] + c[i], 1).sub(o[i].scale(2)));
    Some((o, g, h))
}

fn line(o: Exact, h: Exact) -> String {
    let dx = h[0].sub(o[0]);
    let dy = h[1].sub(o[1]);
    let constant = dy.mul(o[0]).sub(dx.mul(o[1]));
    let denominators = dy.1 * dx.1 * constant.1;
    let mut coefficients = [
        dy.0 * (denominators / dy.1),
        -dx.0 * (denominators / dx.1),
        constant.0 * (denominators / constant.1),
    ];
    let divisor = coefficients.into_iter().fold(0_i64, |a, b| {
        let (mut a, mut b) = (a.abs(), b.abs());
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    });
    let sign = if coefficients[0] < 0 || (coefficients[0] == 0 && coefficients[1] < 0) {
        -1
    } else {
        1
    };
    for coefficient in &mut coefficients {
        *coefficient = *coefficient / divisor * sign;
    }
    let [a, b, c] = coefficients;
    let mut terms = Vec::new();
    for (coefficient, variable) in [(a, "x"), (b, "y")] {
        if coefficient != 0 {
            let body = if coefficient.abs() == 1 {
                variable.to_string()
            } else {
                format!("{}{variable}", coefficient.abs())
            };
            terms.push((coefficient < 0, body));
        }
    }
    let expression = terms
        .into_iter()
        .enumerate()
        .map(|(i, (negative, body))| {
            format!(
                "{}{body}",
                match (i == 0, negative) {
                    (true, false) => "",
                    (true, true) => "-",
                    (false, false) => " + ",
                    (false, true) => " - ",
                }
            )
        })
        .collect::<String>();
    format!("{expression} = {c}")
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..7);
    let vertices = sample(rng, regime);
    let [a, b, c] = vertices;
    let mut output = format!(
        "A={}\nB={}\nC={}",
        center_string(exact(a)),
        center_string(exact(b)),
        center_string(exact(c))
    );
    let Some((o, g, h)) = centers(vertices) else {
        let g = [0, 1].map(|i| Q::new(a[i] + b[i] + c[i], 3));
        return format!(
            "{output}\nCollinear\nG={}\nNo circumcircle",
            center_string(g)
        );
    };
    let angles = (0..3)
        .map(|i| {
            dot(
                subtract(vertices[(i + 1) % 3], vertices[i]),
                subtract(vertices[(i + 2) % 3], vertices[i]),
            )
        })
        .collect::<Vec<_>>();
    let kind = if angles.contains(&0) {
        "Right"
    } else if angles.iter().any(|&d| d < 0) {
        "Obtuse"
    } else {
        "Acute"
    };
    output.push_str(&format!("\n{kind}"));
    match rng.random_range(0..3) {
        0 => output.push_str(&format!(
            "\nO={}\nG={}\nH={}\nEuler line: {}",
            center_string(o),
            center_string(g),
            center_string(h),
            line(o, h)
        )),
        1 => {
            let radius_squared = [0, 1]
                .map(|i| o[i].sub(Q::new(a[i], 1)))
                .into_iter()
                .map(|x| x.mul(x))
                .fold(Q::new(0, 1), Q::add);
            output.push_str(&format!(
                "\nCircumcenter O={}\nR²={radius_squared}\nOrthocenter H={}",
                center_string(o),
                center_string(h)
            ));
        }
        _ => {
            let n = [0, 1].map(|i| Q::new(o[i].add(h[i]).0, o[i].add(h[i]).1 * 2));
            let radius_squared = [0, 1]
                .map(|i| o[i].sub(Q::new(a[i], 1)))
                .into_iter()
                .map(|x| x.mul(x))
                .fold(Q::new(0, 1), Q::add);
            let r = Q::new(radius_squared.0, radius_squared.1 * 4);
            output.push_str(&format!(
                "\nNine-point center N={}\nr²={r}\nEuler line: {}",
                center_string(n),
                line(o, h)
            ));
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn norm(v: Exact) -> Q {
        v.into_iter().map(|x| x.mul(x)).fold(Q::new(0, 1), Q::add)
    }

    #[test]
    fn centers_satisfy_independent_bisector_altitude_and_circle_equations() {
        let mut rng = StdRng::seed_from_u64(432);
        let mut right_boundary = [false; 3];
        for regime in 0..7 {
            for _ in 0..300 {
                let vertices = sample(&mut rng, regime);
                let Some((o, g, h)) = centers(vertices) else {
                    assert_eq!(regime, 6);
                    assert_eq!(
                        cross(
                            subtract(vertices[1], vertices[0]),
                            subtract(vertices[2], vertices[0])
                        ),
                        0
                    );
                    continue;
                };
                let r = norm([0, 1].map(|i| o[i].sub(Q::new(vertices[0][i], 1))));
                let n = [0, 1].map(|i| {
                    let q = o[i].add(h[i]);
                    Q::new(q.0, q.1 * 2)
                });
                for i in 0..3 {
                    let v = vertices[i];
                    let side = subtract(vertices[(i + 1) % 3], vertices[(i + 2) % 3]);
                    let altitude_dot = [0, 1]
                        .map(|j| h[j].sub(Q::new(v[j], 1)).scale(side[j]))
                        .into_iter()
                        .fold(Q::new(0, 1), Q::add);
                    assert_eq!(altitude_dot, Q::new(0, 1));
                    assert_eq!(norm([0, 1].map(|j| o[j].sub(Q::new(v[j], 1)))), r);
                    let midpoint = [0, 1]
                        .map(|j| Q::new(vertices[(i + 1) % 3][j] + vertices[(i + 2) % 3][j], 2));
                    assert_eq!(
                        norm([0, 1].map(|j| n[j].sub(midpoint[j]))),
                        Q::new(r.0, r.1 * 4)
                    );
                }
                let og = [0, 1].map(|i| g[i].sub(o[i]));
                let gh = [0, 1].map(|i| h[i].sub(g[i]));
                assert_eq!(gh, og.map(|x| x.scale(2)));
                if regime == 3 {
                    let a = dot(
                        subtract(vertices[0], vertices[2]),
                        subtract(vertices[1], vertices[2]),
                    );
                    right_boundary[(a.signum() + 1) as usize] = true;
                }
            }
        }
        assert!(right_boundary.into_iter().all(|x| x));
    }

    #[test]
    fn right_triangle_centers_and_heronian_area_are_exact() {
        let (o, _, h) = centers([[0, 0], [6, 0], [0, 8]]).unwrap();
        assert_eq!(o, exact([3, 4]));
        assert_eq!(h, exact([0, 0]));
        let triangle = [[0, 0], [14, 0], [5, 12]];
        let mut lengths = Vec::new();
        for i in 0..3 {
            let v = subtract(triangle[i], triangle[(i + 1) % 3]);
            lengths.push(dot(v, v));
        }
        lengths.sort();
        assert_eq!(lengths, [169, 196, 225]);
        assert_eq!(
            cross(
                subtract(triangle[1], triangle[0]),
                subtract(triangle[2], triangle[0])
            ) / 2,
            84
        );
    }
}
