use rand::{Rng, rngs::StdRng};

#[derive(Clone, Copy, Debug)]
enum Kind {
    Reciprocal,
    LogarithmByParts,
    LogarithmicPower,
}

const KINDS: &[Kind] = &[
    Kind::Reciprocal,
    Kind::LogarithmByParts,
    Kind::LogarithmicPower,
];

pub fn generate(rng: &mut StdRng) -> String {
    Problem::sample(rng).render()
}

struct Problem {
    kind: Kind,
    a: i32,
    b: i32,
    c: i32,
    n: i32,
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
            kind: KINDS[rng.random_range(0..KINDS.len())],
            a: rng.random_range(1..=6),
            b: rng.random_range(-5..=5),
            c: rng.random_range(1..=9) * if rng.random() { 1 } else { -1 },
            n: [-1, -1, 1, 2, 3, 4][rng.random_range(0..6)],
        }
    }

    fn render(&self) -> String {
        let Self { kind, a, b, c, n } = *self;
        let term = product(a, 1, "x");
        let u = match b.cmp(&0) {
            std::cmp::Ordering::Greater => format!("({term} + {b})"),
            std::cmp::Ordering::Less => format!("({term} - {})", -b),
            std::cmp::Ordering::Equal => format!("({term})"),
        };
        let log = format!("ln|{}|", &u[1..u.len() - 1]);
        let (integrand, answer): (String, String) = match kind {
            Kind::Reciprocal => (format!("{c}/{u}"), product(c, a, &log)),
            Kind::LogarithmByParts => (
                product(c, 1, &log),
                product(c, a, &format!("({u}*{log} - {u})")),
            ),
            Kind::LogarithmicPower if n == -1 => (
                format!("{c}/({u}*{log})"),
                product(c, a, &format!("ln(|{log}|)")),
            ),
            Kind::LogarithmicPower => (
                product(c, 1, &format!("({log})^{n}/{u}")),
                product(c, a * (n + 1), &format!("({log})^{}", n + 1)),
            ),
        };
        let domain = if matches!(kind, Kind::LogarithmicPower) && n == -1 {
            format!("{u} not in {{-1, 0, 1}}")
        } else {
            format!("{u} != 0")
        };
        format!("integral ({integrand}) dx = {answer} + C\n{domain}")
    }

    #[cfg(test)]
    fn integrand(&self, x: f64) -> f64 {
        let (a, b, c) = (f64::from(self.a), f64::from(self.b), f64::from(self.c));
        let u = a * x + b;
        match self.kind {
            Kind::Reciprocal => c / u,
            Kind::LogarithmByParts => c * u.abs().ln(),
            Kind::LogarithmicPower => c * u.abs().ln().powi(self.n) / u,
        }
    }

    #[cfg(test)]
    fn antiderivative(&self, x: f64) -> f64 {
        let (a, b, c) = (f64::from(self.a), f64::from(self.b), f64::from(self.c));
        let u = a * x + b;
        match self.kind {
            Kind::Reciprocal => c / a * u.abs().ln(),
            Kind::LogarithmByParts => c / a * (u * u.abs().ln() - u),
            Kind::LogarithmicPower if self.n == -1 => c / a * u.abs().ln().abs().ln(),
            Kind::LogarithmicPower => {
                c / (a * f64::from(self.n + 1)) * u.abs().ln().powi(self.n + 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn differentiate_every_answer_on_both_sign_domains() {
        for &kind in KINDS {
            for a in 1..=6 {
                for b in -5..=5 {
                    for c in (-9..=9).filter(|&c| c != 0) {
                        for n in [-1, 1, 2, 3, 4] {
                            let p = Problem { kind, a, b, c, n };
                            for u in [-3.0_f64, -1.0, -0.75, -0.25, 0.25, 0.75, 1.0, 3.0] {
                                if matches!(kind, Kind::LogarithmicPower)
                                    && n == -1
                                    && u.abs() == 1.0
                                {
                                    continue;
                                }
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
            assert!(text.contains(" + C"));
            let nested = matches!(kind, Kind::LogarithmicPower) && p.n == -1;
            assert!(text.contains(if nested { "not in {-1, 0, 1}" } else { " != 0" }));
        }
    }
}
