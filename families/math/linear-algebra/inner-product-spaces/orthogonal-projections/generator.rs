use rand::{Rng, rngs::StdRng};

pub fn generate(rng: &mut StdRng) -> String {
    let (u, x, _) = sample(rng);
    let d = dot(u, u);
    let plane = rng.random_bool(0.5);
    let numerators = projection(u, plane);
    let along = numerators.map(|row| dot(row, x));
    let across = std::array::from_fn(|i| d * x[i] - along[i]);
    let vector = |v: [i64; 3]| format!("[{}]", v.map(|n| fraction(n, d)).join(", "));
    let target = if plane {
        format!("L = {{v in R^3 : {u:?} dot v = 0}}")
    } else {
        format!("L = span{{{u:?}}}")
    };
    format!(
        "{target}\nx = x_parallel + x_perp\n{x:?} = {} + {}\nx_parallel dot x_perp = 0",
        vector(along),
        vector(across)
    )
}

fn projection(u: [i64; 3], plane: bool) -> [[i64; 3]; 3] {
    let d = dot(u, u);
    std::array::from_fn(|i| {
        std::array::from_fn(|j| {
            let entry = u[i] * u[j];
            if plane {
                if i == j { d - entry } else { -entry }
            } else {
                entry
            }
        })
    })
}

fn dot(a: [i64; 3], b: [i64; 3]) -> i64 {
    a.into_iter().zip(b).map(|(x, y)| x * y).sum()
}

fn sample(rng: &mut StdRng) -> ([i64; 3], [i64; 3], [[i64; 3]; 2]) {
    let mut u: [i64; 3] = [0; 3].map(|_| rng.random_range(-6..=6));
    // Choose a nonzero pivot directly if the all-zero draw occurs.
    if u == [0; 3] {
        u[rng.random_range(0..3)] = rng.random_range(1..=6);
    }
    let gcd = u.into_iter().fold(0, |mut a: i64, value| {
        let mut b = value.abs();
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    });
    let sign = u.into_iter().find(|&value| value != 0).unwrap().signum();
    u = u.map(|value| value / gcd * sign);
    let x = [0; 3].map(|_| rng.random_range(-8..=8));
    let pivot = u.iter().position(|&v| v != 0).unwrap();
    let mut kernel = [[0; 3]; 2];
    let mut row = 0;
    for j in 0..3 {
        if j != pivot {
            kernel[row][pivot] = -u[j];
            kernel[row][j] = u[pivot];
            row += 1;
        }
    }
    (u, x, kernel)
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
    fn complementary_line_and_plane_projections_are_exact() {
        let mut rng = StdRng::seed_from_u64(9_014);
        for _ in 0..2_000 {
            let (u, x, _) = sample(&mut rng);
            let d = dot(u, u);
            let line = projection(u, false);
            let plane = projection(u, true);
            for (p, rank) in [(line, 1), (plane, 2)] {
                assert_eq!((0..3).map(|i| p[i][i]).sum::<i64>(), rank * d);
                for i in 0..3 {
                    for j in 0..3 {
                        assert_eq!(p[i][j], p[j][i]);
                        assert_eq!((0..3).map(|k| p[i][k] * p[k][j]).sum::<i64>(), d * p[i][j]);
                    }
                }
            }
            for i in 0..3 {
                for j in 0..3 {
                    assert_eq!(line[i][j] + plane[i][j], if i == j { d } else { 0 });
                    assert_eq!((0..3).map(|k| line[i][k] * plane[k][j]).sum::<i64>(), 0);
                }
            }
            let along = line.map(|row| dot(row, x));
            let across = plane.map(|row| dot(row, x));
            assert_eq!(
                std::array::from_fn::<_, 3, _>(|i| along[i] + across[i]),
                x.map(|v| d * v)
            );
            assert_eq!(dot(along, across), 0);
            assert_eq!(dot(u, across), 0);
            assert_eq!(dot(along, along) + dot(across, across), d * d * dot(x, x));
        }
    }

    #[test]
    fn fractions_cover_supported_numerators_and_denominators() {
        for d in 1..=108 {
            for n in -1728..=1728 {
                let text = fraction(n, d);
                let (a, b) = text.split_once('/').unwrap_or((&text, "1"));
                let a: i64 = a.parse().unwrap();
                let b: i64 = b.parse().unwrap();
                assert_eq!(a * d, n * b);
                assert!(b > 0);
                assert!((2..=b).all(|factor| a % factor != 0 || b % factor != 0));
                if n == 0 {
                    assert_eq!(text, "0");
                }
            }
        }
    }

    #[test]
    fn all_zero_draw_uses_bounded_nonzero_fallback() {
        let seed = (0..10_000)
            .find(|&seed| {
                let mut rng = StdRng::seed_from_u64(seed);
                [0; 3].map(|_| rng.random_range(-6_i64..=6)) == [0; 3]
            })
            .expect("fixed seed range includes an all-zero initial direction");
        let (u, _, kernel) = sample(&mut StdRng::seed_from_u64(seed));
        assert_eq!(u.iter().filter(|&&v| v != 0).count(), 1);
        assert!(u.iter().all(|v| (0..=6).contains(v)));
        assert!(dot(u, u) > 0);
        for v in kernel {
            assert_ne!(v, [0; 3]);
            assert_eq!(dot(u, v), 0);
        }
    }

    #[test]
    fn exact_projection_kernel_and_residual_orthogonality() {
        let mut rng = StdRng::seed_from_u64(187);
        for _ in 0..2_000 {
            let (u, x, kernel) = sample(&mut rng);
            let d = dot(u, u);
            assert!(d > 0);
            for i in 0..3 {
                for j in 0..3 {
                    let product: i64 = (0..3).map(|k| u[i] * u[k] * u[k] * u[j]).sum();
                    assert_eq!(product, d * u[i] * u[j]);
                }
            }
            let residual = [0, 1, 2].map(|i| d * x[i] - dot(u, x) * u[i]);
            assert_eq!(dot(u, residual), 0);
            assert_eq!(dot(u, kernel[0]), 0);
            assert_eq!(dot(u, kernel[1]), 0);
            let cross = [
                kernel[0][1] * kernel[1][2] - kernel[0][2] * kernel[1][1],
                kernel[0][2] * kernel[1][0] - kernel[0][0] * kernel[1][2],
                kernel[0][0] * kernel[1][1] - kernel[0][1] * kernel[1][0],
            ];
            assert_ne!(cross, [0; 3]);
        }
    }
}
