use rand::{Rng, rngs::StdRng};

pub fn generate(rng: &mut StdRng) -> String {
    match rng.random_range(0..3) {
        0 => unit_beta(rng.random_range(2..=8), rng.random_range(2..=8)),
        1 => endpoint_beta(
            rng.random_range(2..=5),
            rng.random_range(2..=5),
            rng.random_range(1..=3),
            rng.random_range(2..=3),
        ),
        _ => trigonometric_beta(rng.random_range(2..=8), rng.random_range(2..=8)),
    }
}

fn unit_beta(m: i64, n: i64) -> String {
    let denominator = beta_denominator(m, n);
    format!(
        "Beta integral: integral from 0 to 1 of x^{}*(1-x)^{} dx = 1/{denominator}.\nThis is B({m},{n}) = ({}! * {}!)/{}!. The integrand is nonnegative, continuous, and vanishes at both endpoints.",
        m - 1,
        n - 1,
        m - 1,
        n - 1,
        m + n - 1,
    )
}

fn endpoint_beta(m: i64, n: i64, left: i64, width: i64) -> String {
    let right = left + width;
    let scale = width.pow((m + n - 1) as u32);
    let answer = fraction(scale, beta_denominator(m, n));
    format!(
        "Affine beta integral: integral from {left} to {right} of (x-{left})^{}*({right}-x)^{} dx = {answer}.\nWith t=(x-{left})/{width} and dx={width} dt, this is {width}^{}*B({m},{n}). The integrand is nonnegative, continuous, and vanishes at both endpoints.",
        m - 1,
        n - 1,
        m + n - 1,
    )
}

// NIST DLMF 5.12.1 and 5.12.2: https://dlmf.nist.gov/5.12
fn trigonometric_beta(m: i64, n: i64) -> String {
    let denominator = 2 * beta_denominator(m, n);
    format!(
        "Trigonometric beta integral: integral from 0 to pi/2 of sin(theta)^{}*cos(theta)^{} dtheta = 1/{denominator}.\nThe substitution t=sin(theta)^2 gives dt=2*sin(theta)*cos(theta) dtheta, so this is B({m},{n})/2. The integrand is nonnegative, continuous, and vanishes at both endpoints.",
        2 * m - 1,
        2 * n - 1,
    )
}

fn beta_denominator(m: i64, n: i64) -> i64 {
    (m + n - 1) * binomial(m + n - 2, m - 1)
}

fn binomial(n: i64, k: i64) -> i64 {
    (1..=k.min(n - k)).fold(1, |value, i| value * (n + 1 - i) / i)
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.abs()
}

fn fraction(numerator: i64, denominator: i64) -> String {
    let factor = gcd(numerator, denominator);
    let (numerator, denominator) = (numerator / factor, denominator / factor);
    if denominator == 1 {
        numerator.to_string()
    } else {
        format!("{numerator}/{denominator}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_2;

    fn gcd128(mut a: i128, mut b: i128) -> i128 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a.abs()
    }

    // Multiply t^(m-1) by (width-t)^(n-1), without using binomial().
    fn polynomial_integral(m: i64, n: i64, width: i64) -> (i128, i128) {
        let mut coefficients = vec![0; m as usize];
        coefficients[m as usize - 1] = 1_i128;
        for _ in 1..n {
            let mut next = vec![0; coefficients.len() + 1];
            for (k, coefficient) in coefficients.iter().enumerate() {
                next[k] += coefficient * i128::from(width);
                next[k + 1] -= coefficient;
            }
            coefficients = next;
        }
        let (mut numerator, mut denominator) = (0, 1);
        for (k, coefficient) in coefficients.into_iter().enumerate() {
            let divisor = k as i128 + 1;
            numerator = numerator * divisor
                + coefficient * i128::from(width).pow(k as u32 + 1) * denominator;
            denominator *= divisor;
            let factor = gcd128(numerator, denominator);
            numerator /= factor;
            denominator /= factor;
        }
        (numerator, denominator)
    }

    fn simpson(f: impl Fn(f64) -> f64, left: f64, right: f64) -> f64 {
        let steps = 1024;
        let h = (right - left) / f64::from(steps);
        let interior: f64 = (1..steps)
            .map(|i| f(left + f64::from(i) * h) * if i % 2 == 0 { 2.0 } else { 4.0 })
            .sum();
        h / 3.0 * (f(left) + interior + f(right))
    }

    #[test]
    fn exact_independent_polynomial_integration_matches_beta_values() {
        for m in 2..=8 {
            for n in 2..=8 {
                let (numerator, denominator) = polynomial_integral(m, n, 1);
                assert_eq!(numerator, 1);
                assert_eq!(denominator, i128::from(beta_denominator(m, n)));
                assert_eq!(beta_denominator(m, n), beta_denominator(n, m));
                let integrand = |x: f64| x.powi(m as i32 - 1) * (1.0 - x).powi(n as i32 - 1);
                assert_eq!(integrand(0.0), 0.0);
                assert_eq!(integrand(1.0), 0.0);
            }
        }
    }

    #[test]
    fn affine_values_jacobians_and_endpoints_cover_all_parameters() {
        for m in 2..=5 {
            for n in 2..=5 {
                for width in 2_i64..=3 {
                    let (numerator, denominator) = polynomial_integral(m, n, width);
                    let scale = width.pow((m + n - 1) as u32);
                    assert_eq!(
                        numerator * i128::from(beta_denominator(m, n)),
                        i128::from(scale) * denominator
                    );
                    assert_eq!(
                        fraction(scale, beta_denominator(m, n)),
                        fraction(numerator as i64, denominator as i64)
                    );
                    for left in 1..=3 {
                        let right = left + width;
                        let integrand = |x: f64| {
                            (x - left as f64).powi(m as i32 - 1)
                                * (right as f64 - x).powi(n as i32 - 1)
                        };
                        assert_eq!(integrand(left as f64), 0.0);
                        assert_eq!(integrand(right as f64), 0.0);
                        let exact = numerator as f64 / denominator as f64;
                        assert!(
                            (simpson(integrand, left as f64, right as f64) / exact - 1.0).abs()
                                < 1e-9
                        );
                        for k in 1..10 {
                            let t = f64::from(k) / 10.0;
                            let transformed =
                                integrand(left as f64 + width as f64 * t) * width as f64;
                            let beta_scaled =
                                scale as f64 * t.powi(m as i32 - 1) * (1.0 - t).powi(n as i32 - 1);
                            assert!((transformed / beta_scaled - 1.0).abs() < 1e-12);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn trigonometric_values_jacobians_and_endpoints_cover_all_parameters() {
        for m in 2..=8 {
            for n in 2..=8 {
                let integrand = |theta: f64| {
                    theta.sin().powi(2 * m as i32 - 1) * theta.cos().powi(2 * n as i32 - 1)
                };
                assert_eq!(integrand(0.0), 0.0);
                assert!(integrand(FRAC_PI_2).abs() < 1e-40);
                let (numerator, denominator) = polynomial_integral(m, n, 1);
                let exact = numerator as f64 / (2.0 * denominator as f64);
                assert!((simpson(integrand, 0.0, FRAC_PI_2) / exact - 1.0).abs() < 1e-9);
                for k in 1..10 {
                    let theta = FRAC_PI_2 * f64::from(k) / 10.0;
                    let t = theta.sin().powi(2);
                    let jacobian = 2.0 * theta.sin() * theta.cos();
                    let transformed =
                        t.powi(m as i32 - 1) * (1.0 - t).powi(n as i32 - 1) * jacobian / 2.0;
                    assert!((integrand(theta) / transformed - 1.0).abs() < 1e-12);
                }
            }
        }
    }
}
