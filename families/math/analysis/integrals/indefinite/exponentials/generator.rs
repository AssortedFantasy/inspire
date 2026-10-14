use rand::{Rng, rngs::StdRng};

#[derive(Clone, Copy)]
enum Template {
    Affine,
    LinearMoment,
    QuadraticMoment,
}

const TEMPLATES: [Template; 3] = [
    Template::Affine,
    Template::LinearMoment,
    Template::QuadraticMoment,
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
    match b.cmp(&0) {
        std::cmp::Ordering::Greater => format!("({a}*x + {b})"),
        std::cmp::Ordering::Less => format!("({a}*x - {})", -b),
        std::cmp::Ordering::Equal => format!("({a}*x)"),
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
            Template::Affine => (format!("{c}*exp{u}"), format!("{}*exp{u}", fraction(c, a))),
            Template::LinearMoment => (
                format!("{c}*x*exp{u}"),
                format!("{}*{}*exp{u}", fraction(c, a * a), affine(a, -1)),
            ),
            Template::QuadraticMoment => {
                let sign = if a > 0 { "-" } else { "+" };
                (
                    format!("{c}*x^2*exp{u}"),
                    format!(
                        "{}*({}*x^2 {sign} {}*x + 2)*exp{u}",
                        fraction(c, a * a * a),
                        a * a,
                        2 * a.abs()
                    ),
                )
            }
        };
        format!(
            "Integral: integral of {integrand} dx\nAnswer: {answer} + C\nDomain: all real x; C is an arbitrary real constant."
        )
    }

    #[cfg(test)]
    fn integrand(&self, x: f64) -> f64 {
        let (a, b, c) = (f64::from(self.a), f64::from(self.b), f64::from(self.c));
        let degree = match self.template {
            Template::Affine => 0,
            Template::LinearMoment => 1,
            Template::QuadraticMoment => 2,
        };
        c * x.powi(degree) * (a * x + b).exp()
    }

    #[cfg(test)]
    fn antiderivative(&self, x: f64) -> f64 {
        let (a, b, c) = (f64::from(self.a), f64::from(self.b), f64::from(self.c));
        let polynomial = match self.template {
            Template::Affine => c / a,
            Template::LinearMoment => c / (a * a) * (a * x - 1.0),
            Template::QuadraticMoment => c / a.powi(3) * (a * a * x * x - 2.0 * a * x + 2.0),
        };
        polynomial * (a * x + b).exp()
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
            "Integral: integral of 6*x*exp(-2*x + 3) dx\nAnswer: (3/2)*(-2*x - 1)*exp(-2*x + 3) + C\nDomain: all real x; C is an arbitrary real constant."
        );
        let p = Problem {
            template: Template::QuadraticMoment,
            a: -2,
            b: -3,
            c: 6,
        };
        assert!(
            p.render()
                .contains("Answer: (-3/4)*(4*x^2 + 4*x + 2)*exp(-2*x - 3) + C")
        );
        let p = Problem { a: 2, ..p };
        assert!(
            p.render()
                .contains("Answer: (3/4)*(4*x^2 - 4*x + 2)*exp(2*x - 3) + C")
        );
    }

    #[test]
    fn sampled_outputs_have_domain_and_constant() {
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..100 {
            let text = generate(&mut rng);
            assert!(text.contains("Domain: all real x"));
            assert!(text.contains(" + C"));
            assert!(text.contains("C is an arbitrary real constant."));
            assert!(!text.ends_with('\n'));
        }
    }
}
