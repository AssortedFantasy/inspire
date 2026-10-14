use rand::{Rng, rngs::StdRng};

pub fn generate(rng: &mut StdRng) -> String {
    let (u, x, kernel) = sample(rng);
    let d = dot(u, u);
    let product = dot(u, x);
    let matrix = (0..3)
        .map(|i| {
            format!(
                "[{}]",
                (0..3)
                    .map(|j| fraction(u[i] * u[j], d))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let image = u.map(|v| fraction(product * v, d)).join(", ");
    format!(
        "Orthogonal projection onto the line span({u:?}) in R^3 with the standard dot product.\nP=[{matrix}]; P(v)=(u dot v)/(u dot u)*u, where u={u:?} and u dot u={d}.\nExample: P({x:?})=[{image}].\nKernel basis: {:?}, {:?}. Rank 1; trace 1; eigenvalues 1,0,0. The matrix is symmetric and satisfies P^2=P.",
        kernel[0], kernel[1]
    )
}

fn dot(a: [i64; 3], b: [i64; 3]) -> i64 {
    a.into_iter().zip(b).map(|(x, y)| x * y).sum()
}

fn sample(rng: &mut StdRng) -> ([i64; 3], [i64; 3], [[i64; 3]; 2]) {
    let mut u = [0; 3].map(|_| rng.random_range(-6..=6));
    // Choose a nonzero pivot directly if the all-zero draw occurs.
    if u == [0; 3] {
        u[rng.random_range(0..3)] = rng.random_range(1..=6);
    }
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

    fn integers(text: &str) -> [i64; 3] {
        text.trim_matches(['[', ']'])
            .split(", ")
            .map(|v| v.parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn scaled_fraction(text: &str, scale: i64) -> i64 {
        let (n, d) = text.split_once('/').unwrap_or((text, "1"));
        let n: i64 = n.parse().unwrap();
        let d: i64 = d.parse().unwrap();
        assert!(d > 0);
        assert_eq!(scale % d, 0);
        n * (scale / d)
    }

    #[test]
    fn printed_matrix_vector_and_certificates_are_exact() {
        let mut rng = StdRng::seed_from_u64(9_014);
        for _ in 0..2_000 {
            let output = generate(&mut rng);
            assert!(!output.ends_with('\n'));
            let lines: Vec<_> = output.lines().collect();
            let (matrix, rest) = lines[1]
                .strip_prefix("P=[")
                .unwrap()
                .split_once("]; ")
                .unwrap();
            let (u_text, d_text) = rest
                .split_once("where u=")
                .unwrap()
                .1
                .split_once(" and u dot u=")
                .unwrap();
            let u = integers(u_text);
            let d: i64 = d_text.trim_end_matches('.').parse().unwrap();
            assert!((1..=108).contains(&d));
            assert!(u.iter().all(|v| (-6..=6).contains(v)));
            assert_eq!(d, dot(u, u));
            let rows: Vec<_> = matrix.trim_matches(['[', ']']).split("], [").collect();
            assert_eq!(rows.len(), 3);
            let p: [[i64; 3]; 3] = std::array::from_fn(|i| {
                rows[i]
                    .split(", ")
                    .map(|v| scaled_fraction(v, d))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            });
            assert_eq!((0..3).map(|i| p[i][i]).sum::<i64>(), d);
            assert!(p.iter().flatten().any(|&v| v != 0));
            for i in 0..3 {
                assert_eq!(dot(p[i], u), d * u[i]);
                for j in 0..3 {
                    assert_eq!(p[i][j], p[j][i]);
                    assert_eq!(p[i][j], u[i] * u[j]);
                    assert_eq!((0..3).map(|k| p[i][k] * p[k][j]).sum::<i64>(), d * p[i][j]);
                    for k in 0..3 {
                        for l in 0..3 {
                            assert_eq!(p[i][k] * p[j][l], p[i][l] * p[j][k]);
                        }
                    }
                }
            }
            let (x_text, image_text) = lines[2]
                .strip_prefix("Example: P(")
                .unwrap()
                .split_once(")=")
                .unwrap();
            let x = integers(x_text);
            assert!(x.iter().all(|v| (-8..=8).contains(v)));
            let image: [i64; 3] = image_text
                .trim_end_matches('.')
                .trim_matches(['[', ']'])
                .split(", ")
                .map(|v| scaled_fraction(v, d))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            for i in 0..3 {
                assert_eq!(image[i], dot(p[i], x));
            }
            let residual = std::array::from_fn(|i| d * x[i] - image[i]);
            assert_eq!(dot(u, residual), 0);
            let (first, second) = lines[3]
                .strip_prefix("Kernel basis: ")
                .unwrap()
                .split_once(". Rank 1;")
                .unwrap()
                .0
                .split_once("], [")
                .unwrap();
            let kernel = [integers(first), integers(second)];
            for v in kernel {
                for row in p {
                    assert_eq!(dot(row, v), 0);
                }
            }
            assert!((0..3).any(|i| {
                (0..3).any(|j| kernel[0][i] * kernel[1][j] != kernel[0][j] * kernel[1][i])
            }));
            assert!(lines[3].contains("trace 1; eigenvalues 1,0,0."));
        }
    }

    #[test]
    fn fractions_cover_supported_numerators_and_denominators() {
        for d in 1..=108 {
            for n in -864..=864 {
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
