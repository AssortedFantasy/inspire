use rand::{Rng, rngs::StdRng};

pub fn generate(rng: &mut StdRng) -> String {
    match rng.random_range(0..6) {
        0 => exponential(rng.random_range(0..=8), rng.random_range(1..=6), 0),
        1 => {
            let shifts = [-3, -2, -1, 1, 2, 3];
            exponential(
                rng.random_range(0..=5),
                rng.random_range(1..=4),
                shifts[rng.random_range(0..shifts.len())],
            )
        }
        2 => generalized(
            rng.random_range(0..=5),
            rng.random_range(1..=4),
            rng.random_range(2..=3),
        ),
        3 => gaussian(rng.random_range(0..=5), rng.random_range(1..=6)),
        4 => laplace(rng.random_range(0..=4), rng.random_range(1..=6)),
        _ => logarithmic(rng.random_range(1..=7), rng.random_range(1..=6)),
    }
}

// Euler's integral, https://dlmf.nist.gov/5.2.E1; Gamma(n+1) = n!
// follows from integration by parts. All sampled rates are positive.
fn exponential(n: u32, a: i64, b: i64) -> String {
    let base = match b.cmp(&0) {
        std::cmp::Ordering::Less => format!("(x+{})", -b),
        std::cmp::Ordering::Equal => "x".to_owned(),
        std::cmp::Ordering::Greater => format!("(x-{b})"),
    };
    let decay = scaled(&base, a);
    let integrand = format!("{}exp(-{decay})", power_factor(&base, n));
    format!(
        "integral from {b} to infinity of {integrand} dx = {}",
        fraction(factorial(n), a.pow(n + 1)),
    )
}

// https://dlmf.nist.gov/5.9.E1 with mu=p and nu=p*(n+1).
fn generalized(n: u32, a: i64, p: u32) -> String {
    let m = p * (n + 1) - 1;
    let decay = scaled(&format!("x^{p}"), a);
    let integrand = format!("{}exp(-{decay})", power_factor("x", m));
    format!(
        "integral from 0 to infinity of {integrand} dx = {}",
        fraction(factorial(n), i64::from(p) * a.pow(n + 1)),
    )
}

fn gaussian(n: u32, a: i64) -> String {
    let coefficient = fraction(factorial(2 * n), 4_i64.pow(n) * factorial(n) * a.pow(n));
    let root = if a == 1 {
        "sqrt(pi)".to_owned()
    } else {
        format!("sqrt(pi/{a})")
    };
    let value = if coefficient == "1" {
        root
    } else {
        format!("{coefficient}*{root}")
    };
    let decay = scaled("x^2", a);
    format!(
        "integral from -infinity to infinity of {}exp(-{decay}) dx = {value}",
        power_factor("x", 2 * n)
    )
}

fn laplace(n: u32, a: i64) -> String {
    let decay = scaled("abs(x)", a);
    format!(
        "integral from -infinity to infinity of {}exp(-{decay}) dx = {}",
        power_factor("x", 2 * n),
        fraction(2 * factorial(2 * n), a.pow(2 * n + 1))
    )
}

fn logarithmic(a: i64, n: u32) -> String {
    let logarithm = if n == 1 {
        "(-ln(x))".to_owned()
    } else {
        format!("(-ln(x))^{n}")
    };
    format!(
        "integral from 0 to 1 of {}{logarithm} dx = {}",
        power_factor("x", a as u32 - 1),
        fraction(factorial(n), a.pow(n + 1))
    )
}

fn scaled(base: &str, coefficient: i64) -> String {
    if coefficient == 1 {
        base.to_owned()
    } else {
        format!("{coefficient}*{base}")
    }
}

fn power_factor(base: &str, n: u32) -> String {
    match n {
        0 => String::new(),
        1 => format!("{base}*"),
        _ => format!("{base}^{n}*"),
    }
}

fn factorial(n: u32) -> i64 {
    (1..=i64::from(n)).product()
}

fn fraction(n: i64, d: i64) -> String {
    let (mut a, mut b) = (n, d);
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

    fn integrate(f: impl Fn(f64) -> f64, start: f64, end: f64) -> f64 {
        let steps = 20_000;
        let h = (end - start) / f64::from(steps);
        let mut sum = f(start) + f(end);
        for i in 1..steps {
            sum += (if i % 2 == 0 { 2.0 } else { 4.0 }) * f(start + f64::from(i) * h);
        }
        sum * h / 3.0
    }

    // Read the answer actually rendered, independently of fraction().
    fn rendered_value(text: &str) -> f64 {
        let answer = text.lines().next().unwrap().split(" = ").nth(1).unwrap();
        let (rational, scale) = if let Some((coefficient, root)) = answer.split_once("*sqrt(pi") {
            (coefficient, root)
        } else if let Some(root) = answer.strip_prefix("sqrt(pi") {
            ("1", root)
        } else {
            (answer, "")
        };
        let scale = match scale {
            "" => 1.0,
            ")" => std::f64::consts::PI.sqrt(),
            root => (std::f64::consts::PI
                / root.trim_matches(&['/', ')'][..]).parse::<f64>().unwrap())
            .sqrt(),
        };
        let mut parts = rational.split('/');
        let numerator: f64 = parts.next().unwrap().parse().unwrap();
        let denominator: f64 = parts.next().unwrap_or("1").parse().unwrap();
        assert!(parts.next().is_none());
        numerator / denominator * scale
    }

    fn close(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-8 * expected.max(1e-5),
            "actual={actual}, expected={expected}"
        );
    }

    #[test]
    fn independent_quadrature_checks_every_parameter_pair() {
        for n in 0..=8 {
            // The relative tail beyond t=50 equals
            // exp(-50)*sum(50^k/k!), by repeated integration by parts.
            let mut term = 1.0;
            let mut sum = term;
            for k in 1..=n {
                term *= 50.0 / k as f64;
                sum += term;
            }
            assert!((-50.0_f64).exp() * sum < 1e-12);
            for a in 1_i64..=6 {
                let endpoint = 50.0 / a as f64;
                let integrand = |x: f64| x.powi(n as i32) * (-a as f64 * x).exp();
                assert_eq!(integrand(0.0), if n == 0 { 1.0 } else { 0.0 });
                close(
                    integrate(integrand, 0.0, endpoint),
                    rendered_value(&exponential(n, a, 0)),
                );
            }
        }
    }

    #[test]
    fn translated_moments_check_every_lower_endpoint() {
        for n in 0..=5 {
            for a in 1_i64..=4 {
                for b in [-3, -2, -1, 1, 2, 3] {
                    let start = b as f64;
                    let integrand =
                        |x: f64| (x - start).powi(n as i32) * (-a as f64 * (x - start)).exp();
                    close(
                        integrate(integrand, start, start + 50.0 / a as f64),
                        rendered_value(&exponential(n, a, b)),
                    );
                    assert_eq!(integrand(start), if n == 0 { 1.0 } else { 0.0 });
                }
            }
        }
    }

    #[test]
    fn generalized_moments_check_powers_jacobians_and_convergence() {
        for n in 0..=5 {
            for a in 1_i64..=4 {
                for p in 2..=3 {
                    let m = p * (n + 1) - 1;
                    let integrand =
                        |x: f64| x.powi(m as i32) * (-a as f64 * x.powi(p as i32)).exp();
                    let end = (50.0 / a as f64).powf(1.0 / p as f64);
                    close(
                        integrate(integrand, 0.0, end),
                        rendered_value(&generalized(n, a, p)),
                    );
                    assert_eq!(integrand(0.0), 0.0);
                    for x in [0.1_f64, 0.5, 1.0, 2.0] {
                        let t = a as f64 * x.powi(p as i32);
                        let derivative = a as f64 * p as f64 * x.powi(p as i32 - 1);
                        close(
                            x.powi(m as i32) / derivative,
                            t.powi(n as i32) / (p as f64 * (a as f64).powi(n as i32 + 1)),
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn exact_arithmetic_is_bounded_and_reduced() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(8), 40_320);
        assert_eq!(6_i64.pow(9), 10_077_696);
        assert_eq!(3 * 4_i64.pow(6), 12_288);
        for n in 0..=8 {
            for a in 1_i64..=6 {
                for p in 1_i64..=3 {
                    let numerator = factorial(n);
                    let denominator = p * a.pow(n + 1);
                    let rendered = fraction(numerator, denominator);
                    let parts: Vec<i64> = rendered.split('/').map(|s| s.parse().unwrap()).collect();
                    let reduced_denominator = *parts.get(1).unwrap_or(&1);
                    assert_eq!(parts[0] * denominator, numerator * reduced_denominator);
                    let (mut u, mut v) = (parts[0], reduced_denominator);
                    while v != 0 {
                        (u, v) = (v, u % v);
                    }
                    assert_eq!(u, 1);
                }
            }
        }
    }

    #[test]
    fn zero_moments_and_signed_shifts_preserve_the_integrand() {
        assert!(exponential(0, 1, 0).contains("exp(-x)"));
        assert!(exponential(0, 2, -3).contains("exp(-2*(x+3))"));
        assert!(exponential(1, 3, 2).contains("(x-2)*exp(-3*(x-2))"));
        assert!(generalized(0, 2, 2).contains("x*exp(-2*x^2)"));
        assert!(generalized(0, 2, 3).contains("x^2*exp(-2*x^3)"));
        for n in 0..=8 {
            assert!(!exponential(n, 1, 0).contains("^0"));
        }
    }

    #[test]
    fn whole_line_and_logarithmic_moments_match_independent_quadrature() {
        for n in 0..=5 {
            for a in 1_i64..=6 {
                let end = (50.0 / a as f64).sqrt();
                close(
                    integrate(
                        |x| x.powi(2 * n as i32) * (-a as f64 * x * x).exp(),
                        -end,
                        end,
                    ),
                    rendered_value(&gaussian(n, a)),
                );
                if n <= 4 {
                    let end = 50.0 / a as f64;
                    close(
                        integrate(
                            |x| 2.0 * x.powi(2 * n as i32) * (-a as f64 * x).exp(),
                            0.0,
                            end,
                        ),
                        rendered_value(&laplace(n, a)),
                    );
                }
            }
        }
        for a in 1_i64..=7 {
            for n in 1..=6 {
                close(
                    integrate(
                        |t| {
                            let x = (-t).exp();
                            x.powi(a as i32 - 1) * (-x.ln()).powi(n as i32) * x
                        },
                        0.0,
                        50.0 / a as f64,
                    ),
                    rendered_value(&logarithmic(a, n)),
                );
            }
        }
    }

    #[test]
    fn seeded_output_reaches_every_presentation() {
        let mut rng = StdRng::seed_from_u64(1042);
        let mut seen = [false; 6];
        for _ in 0..300 {
            let text = generate(&mut rng);
            assert!(!text.ends_with('\n'));
            assert!(rendered_value(&text).is_finite());
            assert!(!text.contains("^-"));
            let presentation = if text.contains("ln(x)") {
                5
            } else if text.contains("abs(x)") {
                4
            } else if text.contains("from -infinity") {
                3
            } else if text.contains("*(x") || text.contains("exp(-(x") {
                1
            } else if text.contains("exp(-x^") || text.contains("*x^") {
                2
            } else {
                0
            };
            seen[presentation] = true;
        }
        assert_eq!(seen, [true; 6]);
    }
}
