use rand::{Rng, rngs::StdRng};

pub fn generate(rng: &mut StdRng) -> String {
    Problem::sample(rng).render()
}

enum Problem {
    AffinePower {
        scale: i32,
        shift: i32,
        coefficient: i32,
        power: i32,
    },
    SparsePolynomial {
        high_coefficient: i32,
        low_coefficient: i32,
        constant: i32,
        high_degree: i32,
        low_degree: i32,
    },
}

fn signed(rng: &mut StdRng, maximum: i32) -> i32 {
    rng.random_range(1..=maximum) * if rng.random() { 1 } else { -1 }
}

fn fraction(numerator: i32, denominator: i32) -> String {
    let mut a = numerator.abs();
    let mut b = denominator;
    while b != 0 {
        (a, b) = (b, a % b);
    }
    let (n, d) = (numerator / a, denominator / a);
    if d == 1 {
        n.to_string()
    } else {
        format!("({n}/{d})")
    }
}

fn product(numerator: i32, denominator: i32, factor: &str) -> String {
    if numerator < 0 {
        return format!("-{}", product(-numerator, denominator, factor));
    }
    if numerator == denominator {
        factor.into()
    } else {
        format!("{}*{factor}", fraction(numerator, denominator))
    }
}

fn polynomial(terms: &[(i32, i32, i32)]) -> String {
    let mut text = String::new();
    for &(numerator, denominator, degree) in terms {
        if numerator == 0 {
            continue;
        }
        if text.is_empty() {
            if numerator < 0 {
                text.push('-');
            }
        } else {
            text.push_str(if numerator < 0 { " - " } else { " + " });
        }
        let factor = match degree {
            0 => String::new(),
            1 => "x".into(),
            _ => format!("x^{degree}"),
        };
        text.push_str(&if degree == 0 {
            fraction(numerator.abs(), denominator)
        } else {
            product(numerator.abs(), denominator, &factor)
        });
    }
    text
}

impl Problem {
    fn sample(rng: &mut StdRng) -> Self {
        if rng.random() {
            Self::AffinePower {
                scale: rng.random_range(1..=6),
                shift: rng.random_range(-5..=5),
                coefficient: signed(rng, 9),
                power: rng.random_range(2..=6),
            }
        } else {
            let high_degree = rng.random_range(2..=6);
            Self::SparsePolynomial {
                high_coefficient: signed(rng, 9),
                low_coefficient: signed(rng, 6),
                constant: signed(rng, 5),
                high_degree,
                low_degree: rng.random_range(1..high_degree),
            }
        }
    }

    fn render(&self) -> String {
        let (integrand, answer) = match *self {
            Self::AffinePower {
                scale,
                shift,
                coefficient,
                power,
            } => {
                let linear = polynomial(&[(scale, 1, 1), (shift, 1, 0)]);
                (
                    product(coefficient, 1, &format!("({linear})^{power}")),
                    product(
                        coefficient,
                        scale * (power + 1),
                        &format!("({linear})^{}", power + 1),
                    ),
                )
            }
            Self::SparsePolynomial {
                high_coefficient,
                low_coefficient,
                constant,
                high_degree,
                low_degree,
            } => (
                polynomial(&[
                    (high_coefficient, 1, high_degree),
                    (low_coefficient, 1, low_degree),
                    (constant, 1, 0),
                ]),
                polynomial(&[
                    (high_coefficient, high_degree + 1, high_degree + 1),
                    (low_coefficient, low_degree + 1, low_degree + 1),
                    (constant, 1, 1),
                ]),
            ),
        };
        format!(
            "Integral: integral of {integrand} dx\nAnswer: {answer} + C\nDomain: all real x; C is constant on each such interval."
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    // Evaluate the two mathematical expressions separately from their rendering.
    fn values(problem: &Problem, x: f64) -> (f64, f64) {
        match *problem {
            Problem::AffinePower {
                scale,
                shift,
                coefficient,
                power,
            } => {
                let u = f64::from(scale) * x + f64::from(shift);
                (
                    f64::from(coefficient) * u.powi(power),
                    f64::from(coefficient) * u.powi(power + 1) / f64::from(scale * (power + 1)),
                )
            }
            Problem::SparsePolynomial {
                high_coefficient,
                low_coefficient,
                constant,
                high_degree,
                low_degree,
            } => {
                let (a, b, c) = (
                    f64::from(high_coefficient),
                    f64::from(low_coefficient),
                    f64::from(constant),
                );
                (
                    a * x.powi(high_degree) + b * x.powi(low_degree) + c,
                    a * x.powi(high_degree + 1) / f64::from(high_degree + 1)
                        + b * x.powi(low_degree + 1) / f64::from(low_degree + 1)
                        + c * x,
                )
            }
        }
    }

    #[test]
    fn differentiate_every_answer_across_random_parameters() {
        let mut rng = StdRng::seed_from_u64(861);
        let mut counts = [0; 2];
        for _ in 0..400 {
            let p = Problem::sample(&mut rng);
            counts[usize::from(matches!(p, Problem::SparsePolynomial { .. }))] += 1;
            for x in [-1.5_f64, -0.75, -0.25, 0.0, 0.25, 0.75, 1.5] {
                let h = 1e-5;
                let derivative = (values(&p, x + h).1 - values(&p, x - h).1) / (2.0 * h);
                let value = values(&p, x).0;
                assert!(
                    (derivative - value).abs() < 2e-5 * value.abs().max(1.0),
                    "{}",
                    p.render()
                );
            }
        }
        assert!(counts.into_iter().all(|count| count > 150));
    }

    #[test]
    fn renderings_match_the_checked_formulas() {
        let affine = Problem::AffinePower {
            scale: 1,
            shift: -2,
            coefficient: -3,
            power: 2,
        };
        assert_eq!(
            affine.render(),
            "Integral: integral of -3*(x - 2)^2 dx\nAnswer: -(x - 2)^3 + C\nDomain: all real x; C is constant on each such interval."
        );
        let sparse = Problem::SparsePolynomial {
            high_coefficient: -6,
            low_coefficient: 3,
            constant: -1,
            high_degree: 4,
            low_degree: 2,
        };
        assert_eq!(
            sparse.render(),
            "Integral: integral of -6*x^4 + 3*x^2 - 1 dx\nAnswer: -(6/5)*x^5 + x^3 - x + C\nDomain: all real x; C is constant on each such interval."
        );
        assert_eq!(polynomial(&[(1, 1, 1), (0, 1, 0)]), "x");
    }

    #[test]
    fn fractions_reduce_and_outputs_are_complete() {
        assert_eq!(fraction(-6, 9), "(-2/3)");
        assert_eq!(fraction(0, 4), "0");
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..80 {
            let text = generate(&mut rng);
            assert!(text.contains("Domain: all real x"));
            assert!(text.contains(" + C"));
            assert!(!text.ends_with('\n'));
            assert!(!text.contains(" + -"));
        }
    }
}
