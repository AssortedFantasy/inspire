use rand::{Rng, rngs::StdRng};

#[derive(Clone, Copy)]
enum Template {
    Affine,
    LinearMoment,
    QuadraticMoment,
    QuadraticPhase,
    OscillatingSine,
    OscillatingCosine,
}

const TEMPLATES: [Template; 6] = [
    Template::Affine,
    Template::LinearMoment,
    Template::QuadraticMoment,
    Template::QuadraticPhase,
    Template::OscillatingSine,
    Template::OscillatingCosine,
];

pub fn generate(rng: &mut StdRng) -> String {
    Problem::sample(rng).render()
}

struct Problem {
    template: Template,
    a: i32,
    b: i32,
    c: i32,
}

fn fraction(numerator: i32, denominator: i32) -> String {
    let mut a = numerator.abs();
    let mut b = denominator.abs();
    while b != 0 {
        (a, b) = (b, a % b);
    }
    let sign = denominator.signum();
    let (n, d) = (sign * numerator / a, denominator.abs() / a);
    if d == 1 {
        n.to_string()
    } else {
        format!("({n}/{d})")
    }
}

fn affine(a: i32, b: i32) -> String {
    let term = match a {
        1 => "x".to_string(),
        -1 => "-x".to_string(),
        _ => format!("{a}*x"),
    };
    match b.cmp(&0) {
        std::cmp::Ordering::Greater => format!("({term} + {b})"),
        std::cmp::Ordering::Less => format!("({term} - {})", -b),
        std::cmp::Ordering::Equal => format!("({term})"),
    }
}

fn product(numerator: i32, denominator: i32, factor: &str) -> String {
    if numerator == denominator {
        factor.into()
    } else if numerator == -denominator {
        format!("-{factor}")
    } else {
        format!("{}*{factor}", fraction(numerator, denominator))
    }
}

impl Problem {
    fn sample(rng: &mut StdRng) -> Self {
        Self {
            template: TEMPLATES[rng.random_range(0..TEMPLATES.len())],
            a: rng.random_range(1..=6) * if rng.random() { 1 } else { -1 },
            b: rng.random_range(-5..=5),
            c: rng.random_range(1..=9) * if rng.random() { 1 } else { -1 },
        }
    }

    fn render(&self) -> String {
        let Self { template, a, b, c } = *self;
        let u = affine(a, b);
        let (integrand, answer) = match template {
            Template::Affine => (
                product(c, 1, &format!("exp{u}")),
                product(c, a, &format!("exp{u}")),
            ),
            Template::LinearMoment => (
                product(c, 1, &format!("x*exp{u}")),
                product(c, a * a, &format!("{}*exp{u}", affine(a, -1))),
            ),
            Template::QuadraticMoment => {
                let sign = if a > 0 { "-" } else { "+" };
                (
                    product(c, 1, &format!("x^2*exp{u}")),
                    product(
                        c,
                        a * a * a,
                        &format!(
                            "({} {sign} {}*x + 2)*exp{u}",
                            product(a * a, 1, "x^2"),
                            2 * a.abs()
                        ),
                    ),
                )
            }
            Template::QuadraticPhase => {
                let phase = u.replace('x', "x^2");
                (
                    product(c, 1, &format!("x*exp{phase}")),
                    product(c, 2 * a, &format!("exp{phase}")),
                )
            }
            Template::OscillatingSine | Template::OscillatingCosine => {
                let (function, combination) = match template {
                    Template::OscillatingSine => ("sin", format!("(sin{u} - cos{u})")),
                    _ => ("cos", format!("(sin{u} + cos{u})")),
                };
                (
                    product(c, 1, &format!("exp{u}*{function}{u}")),
                    product(c, 2 * a, &format!("exp{u}*{combination}")),
                )
            }
        };
        format!("integral ({integrand}) dx = {answer} + C")
    }

    #[cfg(test)]
    fn integrand(&self, x: f64) -> f64 {
        let (a, b, c) = (f64::from(self.a), f64::from(self.b), f64::from(self.c));
        let u = a * x + b;
        match self.template {
            Template::Affine => c * u.exp(),
            Template::LinearMoment => c * x * u.exp(),
            Template::QuadraticMoment => c * x * x * u.exp(),
            Template::QuadraticPhase => c * x * (a * x * x + b).exp(),
            Template::OscillatingSine => c * u.exp() * u.sin(),
            Template::OscillatingCosine => c * u.exp() * u.cos(),
        }
    }

    #[cfg(test)]
    fn antiderivative(&self, x: f64) -> f64 {
        let (a, b, c) = (f64::from(self.a), f64::from(self.b), f64::from(self.c));
        let u = a * x + b;
        match self.template {
            Template::Affine => c / a * u.exp(),
            Template::LinearMoment => c / (a * a) * (a * x - 1.0) * u.exp(),
            Template::QuadraticMoment => {
                c / a.powi(3) * (a * a * x * x - 2.0 * a * x + 2.0) * u.exp()
            }
            Template::QuadraticPhase => c / (2.0 * a) * (a * x * x + b).exp(),
            Template::OscillatingSine => c / (2.0 * a) * u.exp() * (u.sin() - u.cos()),
            Template::OscillatingCosine => c / (2.0 * a) * u.exp() * (u.sin() + u.cos()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn differentiate_every_answer_across_all_parameters() {
        for template in TEMPLATES {
            for a in (-6..=6).filter(|&a| a != 0) {
                for b in -5..=5 {
                    for c in (-9..=9).filter(|&c| c != 0) {
                        let p = Problem { template, a, b, c };
                        for u in [-0.75_f64, -0.25, 0.0, 0.25, 0.75] {
                            let x = (u - f64::from(b)) / f64::from(a);
                            let h = 1e-5;
                            let derivative =
                                (p.antiderivative(x + h) - p.antiderivative(x - h)) / (2.0 * h);
                            let value = p.integrand(x);
                            assert!(
                                (derivative - value).abs() < 1e-6 * value.abs().max(1.0),
                                "{} at x={x}: derivative={derivative}, integrand={value}",
                                p.render()
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn fractions_reduce_and_normalize_signs() {
        assert_eq!(fraction(-6, 9), "(-2/3)");
        assert_eq!(fraction(6, -9), "(-2/3)");
        assert_eq!(fraction(-6, -9), "(2/3)");
        assert_eq!(fraction(0, -4), "0");
        assert_eq!(fraction(6, -3), "-2");
    }

    #[test]
    fn rendered_moments_use_phase_and_correct_slope_signs() {
        let p = Problem {
            template: Template::LinearMoment,
            a: -2,
            b: 3,
            c: 6,
        };
        assert_eq!(
            p.render(),
            "integral (6*x*exp(-2*x + 3)) dx = (3/2)*(-2*x - 1)*exp(-2*x + 3) + C"
        );
        let p = Problem {
            template: Template::QuadraticMoment,
            a: -2,
            b: -3,
            c: 6,
        };
        assert!(
            p.render()
                .contains("= (-3/4)*(4*x^2 + 4*x + 2)*exp(-2*x - 3) + C")
        );
        let p = Problem { a: 2, ..p };
        assert!(
            p.render()
                .contains("= (3/4)*(4*x^2 - 4*x + 2)*exp(2*x - 3) + C")
        );
    }

    #[test]
    fn sampled_outputs_are_complete() {
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..100 {
            let text = generate(&mut rng);
            assert!(text.contains(" + C"));
            assert!(!text.ends_with('\n'));
        }
    }
}
