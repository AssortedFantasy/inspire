use rand::{Rng, rngs::StdRng};

pub fn generate(rng: &mut StdRng) -> String {
    Integral::sample(rng).render()
}

struct Integral {
    terms: Vec<(i64, u32)>,
    roots: Option<Vec<i64>>,
    lower: i64,
    upper: i64,
}

impl Integral {
    fn sample(rng: &mut StdRng) -> Self {
        let lower = rng.random_range(-3..=2);
        let signed = |rng: &mut StdRng, limit| {
            rng.random_range(1..=limit) * if rng.random() { 1 } else { -1 }
        };
        let (terms, roots) = match rng.random_range(0..3) {
            0 => (vec![(signed(rng, 9), rng.random_range(0..=6))], None),
            1 => {
                let count = rng.random_range(2..=4);
                let mut degrees: Vec<u32> = (0..=6).collect();
                let mut terms = Vec::new();
                for _ in 0..count {
                    let degree = degrees.swap_remove(rng.random_range(0..degrees.len()));
                    terms.push((signed(rng, 6), degree));
                }
                terms.sort_by_key(|&(_, degree)| std::cmp::Reverse(degree));
                (terms, None)
            }
            _ => {
                let count = rng.random_range(2..=4);
                let mut available: Vec<i64> = (-2..=2).collect();
                let mut roots = Vec::new();
                let mut coefficients = vec![1];
                for _ in 0..count {
                    let root = available.swap_remove(rng.random_range(0..available.len()));
                    roots.push(root);
                    let mut next = vec![0; coefficients.len() + 1];
                    for (degree, coefficient) in coefficients.into_iter().enumerate() {
                        next[degree] -= root * coefficient;
                        next[degree + 1] += coefficient;
                    }
                    coefficients = next;
                }
                roots.sort_unstable();
                let terms = coefficients
                    .into_iter()
                    .enumerate()
                    .rev()
                    .filter(|&(_, coefficient)| coefficient != 0)
                    .map(|(degree, coefficient)| (coefficient, degree as u32))
                    .collect();
                (terms, Some(roots))
            }
        };
        Self {
            terms,
            roots,
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
            let factor = gcd(numerator, denominator);
            numerator /= factor;
            denominator /= factor;
        }
        (numerator, denominator)
    }

    fn polynomial(&self) -> String {
        if let Some(roots) = &self.roots {
            return roots
                .iter()
                .map(|root| match root.cmp(&0) {
                    std::cmp::Ordering::Less => format!("(x+{})", -root),
                    std::cmp::Ordering::Equal => "x".to_owned(),
                    std::cmp::Ordering::Greater => format!("(x-{root})"),
                })
                .collect::<Vec<_>>()
                .join("*");
        }
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
            "integral from {} to {} of ({}) dx = {}",
            self.lower,
            self.upper,
            self.polynomial(),
            fraction(numerator, denominator)
        )
    }
}

fn fraction(numerator: i64, denominator: i64) -> String {
    let a = gcd(numerator, denominator);
    let (n, d) = (numerator / a, denominator / a);
    if d == 1 {
        n.to_string()
    } else {
        format!("{n}/{d}")
    }
}

fn gcd(numerator: i64, denominator: i64) -> i64 {
    let (mut a, mut b) = (numerator.abs(), denominator);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
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
                roots: None,
                lower: -radius,
                upper: radius,
            };
            assert_eq!(integral.exact_fraction().0, 0);
        }
        let integral = Integral {
            terms: vec![(1, 2), (-4, 1), (3, 0)],
            roots: None,
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
                            roots: None,
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
    fn samples_stay_bounded_and_cover_polynomial_forms_and_interval_signs() {
        let mut rng = StdRng::seed_from_u64(31);
        let mut templates = [false; 3];
        let mut degrees = [false; 7];
        let mut signs = [false; 3];
        for _ in 0..2_000 {
            let integral = Integral::sample(&mut rng);
            assert!((-3..=2).contains(&integral.lower));
            assert!(integral.lower < integral.upper && integral.upper <= 3);
            assert!(integral.upper - integral.lower <= 3);
            assert!((1..=5).contains(&integral.terms.len()));
            let form = if integral.roots.is_some() {
                2
            } else {
                usize::from(integral.terms.len() > 1)
            };
            templates[form] = true;
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
            assert!((1..=420).contains(&denominator));
            assert!(numerator.abs() <= 29_511 * denominator);
            if let Some(roots) = &integral.roots {
                for x in [-2.5_f64, -0.5, 0.5, 2.5] {
                    let factored: f64 = roots.iter().map(|&root| x - root as f64).product();
                    let expanded: f64 = integral
                        .terms
                        .iter()
                        .map(|&(c, d)| c as f64 * x.powi(d as i32))
                        .sum();
                    assert!((factored - expanded).abs() < 1e-10);
                }
            }
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
                let (n, d) = if let Some((n, d)) = text.split_once('/') {
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
    fn rendered_answers_preserve_exact_integrals() {
        assert_eq!(fraction(-6, 9), "-2/3");
        assert_eq!(fraction(0, 4), "0");
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..80 {
            let integral = Integral::sample(&mut rng);
            let output = integral.render();
            let answer = output.split_once(" = ").unwrap().1;
            let (n, d) = answer.split_once('/').unwrap_or((answer, "1"));
            let (n, d) = (n.parse::<i64>().unwrap(), d.parse::<i64>().unwrap());
            let (expected_n, expected_d) = integral.exact_fraction();
            assert_eq!(n * expected_d, expected_n * d);
            assert!(!output.ends_with('\n'));
        }
    }
}
