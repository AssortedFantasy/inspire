use rand::{Rng, rngs::StdRng};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind {
    Sine,
    Cosine,
    SineSquared,
    CosineSquared,
    SecantSquared,
}

const KINDS: &[Kind] = &[
    Kind::Sine,
    Kind::Cosine,
    Kind::SineSquared,
    Kind::CosineSquared,
    Kind::SecantSquared,
];

pub fn generate(rng: &mut StdRng) -> String {
    Problem::sample(rng).render()
}

struct Problem {
    kind: Kind,
    a: i32,
    b: i32,
    c: i32,
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

impl Problem {
    fn sample(rng: &mut StdRng) -> Self {
        Self {
            kind: KINDS[rng.random_range(0..KINDS.len())],
            a: rng.random_range(1..=6),
            b: rng.random_range(-5..=5),
            c: rng.random_range(1..=9) * if rng.random() { 1 } else { -1 },
        }
    }

    fn render(&self) -> String {
        let Self { kind, a, b, c } = *self;
        let u = match b.cmp(&0) {
            std::cmp::Ordering::Greater => format!("({a}*x + {b})"),
            std::cmp::Ordering::Less => format!("({a}*x - {})", -b),
            std::cmp::Ordering::Equal => format!("({a}*x)"),
        };
        let scale = fraction(c, a);
        let (integrand, answer, domain): (String, String, String) = match kind {
            Kind::Sine => (
                format!("{c}*sin{u}"),
                format!("{}*cos{u}", fraction(-c, a)),
                "all real x".into(),
            ),
            Kind::Cosine => (
                format!("{c}*cos{u}"),
                format!("{scale}*sin{u}"),
                "all real x".into(),
            ),
            Kind::SineSquared | Kind::CosineSquared => {
                let function = if kind == Kind::SineSquared {
                    "sin"
                } else {
                    "cos"
                };
                let sign = if (kind == Kind::SineSquared) == (c > 0) {
                    "-"
                } else {
                    "+"
                };
                (
                    format!("{c}*({function}{u})^2"),
                    format!(
                        "{}*x {sign} {}*sin(2*{u})",
                        fraction(c, 2),
                        fraction(c.abs(), 4 * a)
                    ),
                    "all real x".into(),
                )
            }
            Kind::SecantSquared => (
                format!("{c}*(sec{u})^2"),
                format!("{scale}*tan{u}"),
                format!("real x with {u} != pi/2 + k*pi for every integer k"),
            ),
        };
        format!(
            "Integral: integral of {integrand} dx\nAnswer: {answer} + C\nDomain: {domain}. Angles are in radians; C is arbitrary and independent on each connected interval."
        )
    }

    #[cfg(test)]
    fn integrand(&self, x: f64) -> f64 {
        let (a, b, c) = (f64::from(self.a), f64::from(self.b), f64::from(self.c));
        let u = a * x + b;
        match self.kind {
            Kind::Sine => c * u.sin(),
            Kind::Cosine => c * u.cos(),
            Kind::SineSquared => c * u.sin().powi(2),
            Kind::CosineSquared => c * u.cos().powi(2),
            Kind::SecantSquared => c / u.cos().powi(2),
        }
    }

    #[cfg(test)]
    fn antiderivative(&self, x: f64) -> f64 {
        let (a, b, c) = (f64::from(self.a), f64::from(self.b), f64::from(self.c));
        let u = a * x + b;
        match self.kind {
            Kind::Sine => -c / a * u.cos(),
            Kind::Cosine => c / a * u.sin(),
            Kind::SineSquared => c / 2.0 * x - c / (4.0 * a) * (2.0 * u).sin(),
            Kind::CosineSquared => c / 2.0 * x + c / (4.0 * a) * (2.0 * u).sin(),
            Kind::SecantSquared => c / a * u.tan(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn differentiate_every_answer_across_all_parameters_and_domain_components() {
        for &kind in KINDS {
            for a in 1..=6 {
                for b in -5..=5 {
                    for c in (-9..=9).filter(|&c| c != 0) {
                        let p = Problem { kind, a, b, c };
                        // Five distinct secant-domain components, away from their poles.
                        for component in -2..=2 {
                            for offset in [-0.8, -0.3, 0.3, 0.8] {
                                let u = f64::from(component) * std::f64::consts::PI + offset;
                                let x = (u - f64::from(b)) / f64::from(a);
                                let h = 1e-6;
                                let derivative =
                                    (p.antiderivative(x + h) - p.antiderivative(x - h)) / (2.0 * h);
                                let value = p.integrand(x);
                                assert!(
                                    (derivative - value).abs() < 1e-5 * value.abs().max(1.0),
                                    "{} at x={x}",
                                    p.render()
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn fractions_reduce_and_every_template_renders_its_answer() {
        assert_eq!(fraction(-6, 9), "(-2/3)");
        assert_eq!(fraction(0, 4), "0");
        for (kind, expected) in [
            (Kind::Sine, "Answer: -2*cos(3*x - 2) + C"),
            (Kind::Cosine, "Answer: 2*sin(3*x - 2) + C"),
            (
                Kind::SineSquared,
                "Answer: 3*x - (1/2)*sin(2*(3*x - 2)) + C",
            ),
            (
                Kind::CosineSquared,
                "Answer: 3*x + (1/2)*sin(2*(3*x - 2)) + C",
            ),
            (Kind::SecantSquared, "Answer: 2*tan(3*x - 2) + C"),
        ] {
            assert!(
                Problem {
                    kind,
                    a: 3,
                    b: -2,
                    c: 6
                }
                .render()
                .contains(expected)
            );
        }
        for (kind, sign) in [(Kind::SineSquared, "+"), (Kind::CosineSquared, "-")] {
            assert!(
                Problem {
                    kind,
                    a: 3,
                    b: 0,
                    c: -6
                }
                .render()
                .contains(&format!("Answer: -3*x {sign} (1/2)*sin(2*(3*x)) + C"))
            );
        }
    }

    #[test]
    fn samples_respect_bounds_and_declare_domains_and_independent_constants() {
        let mut rng = StdRng::seed_from_u64(7);
        let mut seen = [false; 5];
        for _ in 0..500 {
            let p = Problem::sample(&mut rng);
            assert!((1..=6).contains(&p.a));
            assert!((-5..=5).contains(&p.b));
            assert!((1..=9).contains(&p.c.abs()));
            seen[KINDS.iter().position(|&kind| kind == p.kind).unwrap()] = true;
            let text = p.render();
            if p.kind == Kind::SecantSquared {
                assert!(text.contains("!= pi/2 + k*pi for every integer k"));
            } else {
                assert!(text.contains("Domain: all real x."));
            }
            assert!(text.contains(" + C"));
            assert!(text.contains("Angles are in radians"));
            assert!(text.contains("C is arbitrary and independent on each connected interval."));
            assert!(!text.ends_with('\n'));
        }
        assert!(seen.into_iter().all(|seen| seen));
    }
}
