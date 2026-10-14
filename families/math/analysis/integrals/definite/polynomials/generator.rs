use rand::{Rng, rngs::StdRng};

pub fn generate(rng: &mut StdRng) -> String {
    Integral::sample(rng).render()
}

struct Integral {
    terms: Vec<(i64, u32)>,
    lower: i64,
    upper: i64,
}

impl Integral {
    fn sample(rng: &mut StdRng) -> Self {
        let lower = rng.random_range(-3..=2);
        let signed = |rng: &mut StdRng, limit| {
            rng.random_range(1..=limit) * if rng.random() { 1 } else { -1 }
        };
        let terms = if rng.random() {
            vec![(signed(rng, 9), rng.random_range(0..=6))]
        } else {
            vec![
                (signed(rng, 5), rng.random_range(2..=4)),
                (signed(rng, 6), 1),
                (signed(rng, 9), 0),
            ]
        };
        Self {
            terms,
            lower,
            upper: lower + rng.random_range(1..=3.min(3 - lower)),
        }
    }

    fn exact_fraction(&self) -> (i64, i64) {
        let (mut numerator, mut denominator) = (0, 1);
        for &(coefficient, degree) in &self.terms {
            let term_numerator =
                coefficient * (self.upper.pow(degree + 1) - self.lower.pow(degree + 1));
            let term_denominator = i64::from(degree + 1);
            numerator = numerator * term_denominator + term_numerator * denominator;
            denominator *= term_denominator;
        }
        (numerator, denominator)
    }

    fn polynomial(&self) -> String {
        let mut text = String::new();
        for &(coefficient, degree) in &self.terms {
            if text.is_empty() {
                if coefficient < 0 {
                    text.push('-');
                }
            } else {
                text.push_str(if coefficient < 0 { " - " } else { " + " });
            }
            let magnitude = coefficient.abs();
            if degree == 0 {
                text.push_str(&magnitude.to_string());
            } else {
                if magnitude != 1 {
                    text.push_str(&format!("{magnitude}*"));
                }
                text.push('x');
                if degree != 1 {
                    text.push_str(&format!("^{degree}"));
                }
            }
        }
        text
    }

    fn render(&self) -> String {
        let (numerator, denominator) = self.exact_fraction();
        format!(
            "Integral: integral from {} to {} of ({}) dx\nAnswer: {}\nDomain: real x; a proper integral of a polynomial.",
            self.lower,
            self.upper,
            self.polynomial(),
            fraction(numerator, denominator)
        )
    }
}

fn fraction(numerator: i64, denominator: i64) -> String {
    let (mut a, mut b) = (numerator.abs(), denominator);
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn exact_values_match_independent_simpson_quadrature() {
        let mut rng = StdRng::seed_from_u64(43);
        for _ in 0..256 {
            let integral = Integral::sample(&mut rng);
            let (lower, upper) = (integral.lower as f64, integral.upper as f64);
            let h = (upper - lower) / 1000.0;
            let integrand = |x: f64| {
                integral
                    .terms
                    .iter()
                    .map(|&(coefficient, degree)| coefficient as f64 * x.powi(degree as i32))
                    .sum::<f64>()
            };
            let mut sum = integrand(lower) + integrand(upper);
            for i in 1..1000 {
                sum += (if i % 2 == 0 { 2.0 } else { 4.0 }) * integrand(lower + f64::from(i) * h);
            }
            let (numerator, denominator) = integral.exact_fraction();
            let exact = numerator as f64 / denominator as f64;
            assert!((sum * h / 3.0 - exact).abs() < 1e-8 * exact.abs().max(1.0));
        }
    }

    #[test]
    fn symmetry_and_linear_combination_cancel_exactly() {
        for radius in 1..=3 {
            let integral = Integral {
                terms: vec![(5, 5), (-4, 3), (9, 1)],
                lower: -radius,
                upper: radius,
            };
            assert_eq!(integral.exact_fraction().0, 0);
        }
        let integral = Integral {
            terms: vec![(1, 2), (-4, 1), (3, 0)],
            lower: 0,
            upper: 3,
        };
        assert_eq!(integral.exact_fraction().0, 0);
    }

    #[test]
    fn constants_and_linear_terms_match_rectangle_and_trapezoid_areas() {
        for lower in -3..=2 {
            for upper in lower + 1..=3 {
                for coefficient in [-9, -1, 1, 9] {
                    for degree in 0..=1 {
                        let integral = Integral {
                            terms: vec![(coefficient, degree)],
                            lower,
                            upper,
                        };
                        let (numerator, denominator) = integral.exact_fraction();
                        let endpoint_sum = if degree == 0 { 2 } else { lower + upper };
                        assert_eq!(
                            2 * numerator,
                            denominator * coefficient * endpoint_sum * (upper - lower)
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn samples_stay_bounded_and_cover_both_templates_and_interval_signs() {
        let mut rng = StdRng::seed_from_u64(31);
        let mut templates = [false; 2];
        let mut degrees = [false; 7];
        let mut signs = [false; 3];
        for _ in 0..2_000 {
            let integral = Integral::sample(&mut rng);
            assert!((-3..=2).contains(&integral.lower));
            assert!(integral.lower < integral.upper && integral.upper <= 3);
            assert!(integral.upper - integral.lower <= 3);
            assert!(matches!(integral.terms.len(), 1 | 3));
            templates[usize::from(integral.terms.len() == 3)] = true;
            signs[if integral.upper <= 0 {
                0
            } else if integral.lower >= 0 {
                1
            } else {
                2
            }] = true;
            for &(coefficient, degree) in &integral.terms {
                assert!((1..=9).contains(&coefficient.abs()));
                assert!(degree <= 6);
                degrees[degree as usize] = true;
            }
            let (numerator, denominator) = integral.exact_fraction();
            assert!((1..=10).contains(&denominator));
            assert!(numerator.abs() <= 19_683);
            assert!(numerator.abs() <= 2_812 * denominator);
        }
        assert!(templates.into_iter().all(|seen| seen));
        assert!(degrees.into_iter().all(|seen| seen));
        assert!(signs.into_iter().all(|seen| seen));
    }

    #[test]
    fn rendered_fractions_preserve_value_and_have_no_common_divisor() {
        for numerator in -400..=400 {
            for denominator in 1..=10 {
                let text = fraction(numerator, denominator);
                let (n, d) = if let Some(inner) = text.strip_prefix('(') {
                    let (n, d) = inner.trim_end_matches(')').split_once('/').unwrap();
                    (n.parse::<i64>().unwrap(), d.parse::<i64>().unwrap())
                } else {
                    (text.parse::<i64>().unwrap(), 1)
                };
                assert_eq!(n * denominator, numerator * d);
                assert!(d > 0);
                for divisor in 2..=d {
                    assert!(n % divisor != 0 || d % divisor != 0);
                }
            }
        }
    }

    #[test]
    fn fractions_reduce_and_outputs_describe_proper_integrals() {
        assert_eq!(fraction(-6, 9), "(-2/3)");
        assert_eq!(fraction(0, 4), "0");
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..80 {
            let output = generate(&mut rng);
            assert!(output.contains("Answer:"));
            assert!(output.contains("a proper integral of a polynomial"));
            assert!(!output.ends_with('\n'));
        }
    }
}
