use rand::{Rng, rngs::StdRng};

type Matrix = [[i64; 2]; 2];

pub fn generate(rng: &mut StdRng) -> String {
    let (a, basis, inverse, eigenvalues) = sample(rng);
    describe(a, basis, inverse, eigenvalues)
}

fn describe(a: Matrix, basis: Matrix, inverse: Matrix, eigenvalues: [i64; 2]) -> String {
    let [l, m] = eigenvalues;
    let square = multiply(a, a);
    let factor = |n: i64| match n.cmp(&0) {
        std::cmp::Ordering::Greater => format!("(t - {n})"),
        std::cmp::Ordering::Less => format!("(t + {})", -n),
        std::cmp::Ordering::Equal => "t".into(),
    };
    format!(
        "Integer matrix A={a:?}, diagonalizable over Q.\nEigenvalues: {l} and {m}; corresponding eigenvectors: [{},{}] and [{},{}].\nEigenbasis matrix S={basis:?}, with det(S)=1 and S^(-1)={inverse:?}; A=S*diag({l},{m})*S^(-1).\nCharacteristic polynomial: {}*{}; trace {}; determinant {}. A^2={square:?}. For every integer k>=1, A^k=S*diag(({l})^k,({m})^k)*S^(-1), and A^0=I.",
        basis[0][0],
        basis[1][0],
        basis[0][1],
        basis[1][1],
        factor(l),
        factor(m),
        l + m,
        l * m,
    )
}

fn multiply(a: Matrix, b: Matrix) -> Matrix {
    [0, 1].map(|i| [0, 1].map(|j| (0..2).map(|k| a[i][k] * b[k][j]).sum()))
}

fn sample(rng: &mut StdRng) -> (Matrix, Matrix, Matrix, [i64; 2]) {
    let t = rng.random_range(-3..=3);
    let u = rng.random_range(-3..=3);
    let l = rng.random_range(-6..=8);
    let mut m = rng.random_range(-6..=7);
    if m >= l {
        m += 1;
    }
    construct(t, u, l, m)
}

fn construct(t: i64, u: i64, l: i64, m: i64) -> (Matrix, Matrix, Matrix, [i64; 2]) {
    let basis = [[1, t], [u, 1 + t * u]];
    let inverse = [[1 + t * u, -t], [-u, 1]];
    let a = multiply(multiply(basis, [[l, 0], [0, m]]), inverse);
    (a, basis, inverse, [l, m])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn all_parameters_have_bounded_entries_and_exact_positive_powers() {
        let mut cases = 0;
        for t in -3..=3 {
            for u in -3..=3 {
                for l in -6..=8 {
                    for m in -6..=8 {
                        if l == m {
                            continue;
                        }
                        let (a, s, inverse, _) = construct(t, u, l, m);
                        assert!(s.into_iter().flatten().all(|n| n.abs() <= 10));
                        assert!(inverse.into_iter().flatten().all(|n| n.abs() <= 10));
                        assert!(a.into_iter().flatten().all(|n| n.abs() <= 420));
                        assert_eq!(multiply(s, inverse), [[1, 0], [0, 1]]);
                        let mut power = [[1, 0], [0, 1]];
                        for k in 1..=6 {
                            power = multiply(power, a);
                            assert_eq!(
                                power,
                                multiply(multiply(s, [[l.pow(k), 0], [0, m.pow(k)]]), inverse)
                            );
                        }
                        cases += 1;
                    }
                }
            }
        }
        assert_eq!(cases, 10_290);
    }

    #[test]
    fn emitted_negative_and_zero_powers_have_unambiguous_domain() {
        let (a, s, inverse, eigenvalues) = construct(3, -3, -6, 0);
        let text = describe(a, s, inverse, eigenvalues);
        assert!(
            text.starts_with("Integer matrix A=[[48, 18], [-144, -54]], diagonalizable over Q.")
        );
        assert!(text.contains("Characteristic polynomial: (t + 6)*t; trace -6; determinant 0."));
        assert!(
            text.contains("For every integer k>=1, A^k=S*diag((-6)^k,(0)^k)*S^(-1), and A^0=I.")
        );
        assert!(!text.ends_with('\n'));
        assert!(!text.contains('\0'));
    }

    #[test]
    fn eigenpairs_inverse_and_cayley_hamilton_are_exact() {
        let mut rng = StdRng::seed_from_u64(833);
        for _ in 0..2_000 {
            let (a, s, inverse, [l, m]) = sample(&mut rng);
            assert_ne!(l, m);
            assert_eq!(multiply(s, inverse), [[1, 0], [0, 1]]);
            assert_eq!(a[0][0] + a[1][1], l + m);
            assert_eq!(a[0][0] * a[1][1] - a[0][1] * a[1][0], l * m);
            for (j, eigenvalue) in [l, m].into_iter().enumerate() {
                for i in 0..2 {
                    assert_eq!(a[i][0] * s[0][j] + a[i][1] * s[1][j], eigenvalue * s[i][j]);
                }
            }
            let square = multiply(a, a);
            for i in 0..2 {
                for j in 0..2 {
                    let constant = if i == j { l * m } else { 0 };
                    assert_eq!(square[i][j] - (l + m) * a[i][j] + constant, 0);
                }
            }
        }
    }
}
