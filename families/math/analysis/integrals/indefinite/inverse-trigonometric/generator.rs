use rand::{Rng, rngs::StdRng};

#[derive(Clone, Copy)]
enum Kind {
    ReciprocalQuadratic,
    ReciprocalRoot,
    ArctangentByParts,
}

const KINDS: &[Kind] = &[
    Kind::ReciprocalQuadratic,
    Kind::ReciprocalRoot,
    Kind::ArctangentByParts,
];

pub fn generate(rng: &mut StdRng) -> String {
    Problem::sample(rng).render()
}

struct Problem {
    kind: Kind,
    a: i32,
    b: i32,
    c: i32,
    r: i32,
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
            r: rng.random_range(1..=5),
        }
    }

    fn render(&self) -> String {
        let Self { kind, a, b, c, r } = *self;
        let u = match b.cmp(&0) {
            std::cmp::Ordering::Greater => format!("({a}*x + {b})"),
            std::cmp::Ordering::Less => format!("({a}*x - {})", -b),
            std::cmp::Ordering::Equal => format!("({a}*x)"),
        };
        let argument = if r == 1 {
            u.clone()
        } else {
            format!("({u}/{r})")
        };
        let r2 = r * r;
        let scale = fraction(c, a);
        let (integrand, answer, domain): (String, String, String) = match kind {
            Kind::ReciprocalQuadratic => (
                format!("{c}/({r2} + {u}^2)"),
                format!("{}*atan{argument}", fraction(c, a * r)),
                "all real x; atan is the principal real arctangent".into(),
            ),
            Kind::ReciprocalRoot => (
                format!("{c}/sqrt({r2} - {u}^2)"),
                format!("{scale}*asin{argument}"),
                format!("-{r} < {u} < {r}; asin is the principal real arcsine"),
            ),
            Kind::ArctangentByParts => (
                format!("{c}*atan{argument}"),
                format!(
                    "{scale}*[{u}*atan{argument} - {}*ln({r2} + {u}^2)]",
                    fraction(r, 2)
                ),
                "all real x; atan is the principal real arctangent".into(),
            ),
        };
        format!(
            "Integral: integral of {integrand} dx\nAnswer: {answer} + C\nDomain: {domain}; C is an arbitrary real constant on each connected interval."
        )
    }

    #[cfg(test)]
    fn integrand(&self, x: f64) -> f64 {
        let (a, b, c) = (f64::from(self.a), f64::from(self.b), f64::from(self.c));
        let u = a * x + b;
        let r = f64::from(self.r);
        match self.kind {
            Kind::ReciprocalQuadratic => c / (r * r + u * u),
            Kind::ReciprocalRoot => c / (r * r - u * u).sqrt(),
            Kind::ArctangentByParts => c * (u / r).atan(),
        }
    }

    #[cfg(test)]
    fn antiderivative(&self, x: f64) -> f64 {
        let (a, b, c) = (f64::from(self.a), f64::from(self.b), f64::from(self.c));
        let u = a * x + b;
        let r = f64::from(self.r);
        match self.kind {
            Kind::ReciprocalQuadratic => c / (a * r) * (u / r).atan(),
            Kind::ReciprocalRoot => c / a * (u / r).asin(),
            Kind::ArctangentByParts => {
                c / a * (u * (u / r).atan() - r / 2.0 * (r * r + u * u).ln())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn differentiate_every_answer_across_all_parameters() {
        for &kind in KINDS {
            for a in 1..=6 {
                for b in -5..=5 {
                    for c in (-9..=9).filter(|&c| c != 0) {
                        for r in 1..=5 {
                            let p = Problem { kind, a, b, c, r };
                            let points: &[f64] = match kind {
                                Kind::ReciprocalRoot => &[-0.9, -0.25, 0.0, 0.25, 0.9],
                                _ => &[-8.0, -1.0, -0.25, 0.0, 0.25, 1.0, 8.0],
                            };
                            for &ratio in points {
                                let u = ratio * f64::from(r);
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
    fn fractions_reduce_and_output_has_domain_and_constant() {
        assert_eq!(fraction(-6, 9), "(-2/3)");
        assert_eq!(fraction(0, 4), "0");
        let mut rng = StdRng::seed_from_u64(7);
        for &kind in KINDS {
            let mut p = Problem::sample(&mut rng);
            p.kind = kind;
            let text = p.render();
            assert!(text.contains("Domain:"));
            assert!(text.contains(" + C"));
            assert!(text.contains("C is an arbitrary real constant on each connected interval."));
        }
    }

    #[test]
    fn rendered_formulas_include_radius_chain_factor_and_domain() {
        let mut p = Problem {
            kind: Kind::ReciprocalQuadratic,
            a: 6,
            b: -3,
            c: -9,
            r: 4,
        };
        let text = p.render();
        assert!(text.contains("-9/(16 + (6*x - 3)^2)"));
        assert!(text.contains("Answer: (-3/8)*atan((6*x - 3)/4) + C"));
        assert!(text.contains("all real x"));
        p.kind = Kind::ReciprocalRoot;
        let text = p.render();
        assert!(text.contains("-9/sqrt(16 - (6*x - 3)^2)"));
        assert!(text.contains("Answer: (-3/2)*asin((6*x - 3)/4) + C"));
        assert!(text.contains("-4 < (6*x - 3) < 4"));
        p.kind = Kind::ArctangentByParts;
        let text = p.render();
        assert!(text.contains("-9*atan((6*x - 3)/4)"));
        assert!(
            text.contains(
                "Answer: (-3/2)*[(6*x - 3)*atan((6*x - 3)/4) - 2*ln(16 + (6*x - 3)^2)] + C"
            )
        );
        assert!(text.contains("all real x"));
    }
}
