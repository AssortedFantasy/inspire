use rand::{Rng, rngs::StdRng};

pub fn generate(rng: &mut StdRng) -> String {
    Problem::sample(rng).render()
}

struct Problem {
    kind: u8,
    a: i32,
    b: i32,
    c: i32,
    n: i32,
    lo: i32,
    hi: i32,
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
        let lo = rng.random_range(0..=3);
        Self {
            kind: rng.random_range(0..10),
            a: rng.random_range(1..=6),
            b: rng.random_range(-5..=5),
            c: rng.random_range(1..=9) * if rng.random() { 1 } else { -1 },
            n: rng.random_range(2..=6),
            lo,
            hi: lo + rng.random_range(1..=3),
        }
    }

    fn render(&self) -> String {
        let Self {
            kind,
            a,
            b,
            c,
            n,
            lo,
            hi,
        } = *self;
        let u = match b.cmp(&0) {
            std::cmp::Ordering::Greater => format!("({a}*x + {b})"),
            std::cmp::Ordering::Less => format!("({a}*x - {})", -b),
            std::cmp::Ordering::Equal => format!("({a}*x)"),
        };
        let scale = fraction(c, a);
        let (integrand, answer, domain) = match kind {
            0 => (
                format!("{c}*{u}^{n}"),
                format!("{}*{u}^{}", fraction(c, a * (n + 1)), n + 1),
                "all real x".into(),
            ),
            1 => (
                format!("{c}/{u}"),
                format!("{scale}*ln|{u}|"),
                format!("any interval with {u} != 0"),
            ),
            2 => (
                format!("{c}*exp{u}"),
                format!("{scale}*exp{u}"),
                "all real x".into(),
            ),
            3 => (
                format!("{c}*sin{u}"),
                format!("{}*cos{u}", fraction(-c, a)),
                "all real x".into(),
            ),
            4 => (
                format!("{c}*cos{u}"),
                format!("{scale}*sin{u}"),
                "all real x".into(),
            ),
            5 => (
                format!("{c}/(1 + {u}^2)"),
                format!("{scale}*atan{u}"),
                "all real x; atan is the principal real arctangent".into(),
            ),
            6 => (
                format!("{c}/sqrt(1 - {u}^2)"),
                format!("{scale}*asin{u}"),
                format!("-1 < {u} < 1; asin is the principal real arcsine"),
            ),
            7 => (
                format!("{c}*x*exp({a}*x)"),
                format!("{}*({a}*x - 1)*exp({a}*x)", fraction(c, a * a)),
                "all real x".into(),
            ),
            8 => (
                format!("{c}*ln{u}"),
                format!("{scale}*({u}*ln{u} - {u})"),
                format!("{u} > 0"),
            ),
            _ => {
                let numerator = c * (hi.pow((n + 1) as u32) - lo.pow((n + 1) as u32));
                return format!(
                    "Integral: integral from {lo} to {hi} of {c}*x^{n} dx\nAnswer: {}\nDomain: real x; a proper integral of a polynomial.",
                    fraction(numerator, n + 1)
                );
            }
        };
        format!(
            "Integral: integral of {integrand} dx\nAnswer: {answer} + C\nDomain: {domain}; C is constant on each such interval."
        )
    }

    #[cfg(test)]
    fn integrand(&self, x: f64) -> f64 {
        let (a, b, c) = (f64::from(self.a), f64::from(self.b), f64::from(self.c));
        let u = a * x + b;
        match self.kind {
            0 => c * u.powi(self.n),
            1 => c / u,
            2 => c * u.exp(),
            3 => c * u.sin(),
            4 => c * u.cos(),
            5 => c / (1.0 + u * u),
            6 => c / (1.0 - u * u).sqrt(),
            7 => c * x * (a * x).exp(),
            8 => c * u.ln(),
            _ => c * x.powi(self.n),
        }
    }

    #[cfg(test)]
    fn antiderivative(&self, x: f64) -> f64 {
        let (a, b, c) = (f64::from(self.a), f64::from(self.b), f64::from(self.c));
        let u = a * x + b;
        match self.kind {
            0 => c / (a * f64::from(self.n + 1)) * u.powi(self.n + 1),
            1 => c / a * u.abs().ln(),
            2 => c / a * u.exp(),
            3 => -c / a * u.cos(),
            4 => c / a * u.sin(),
            5 => c / a * u.atan(),
            6 => c / a * u.asin(),
            7 => c / (a * a) * (a * x - 1.0) * (a * x).exp(),
            8 => c / a * (u * u.ln() - u),
            _ => c / f64::from(self.n + 1) * x.powi(self.n + 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn differentiate_every_answer_across_random_parameters() {
        let mut rng = StdRng::seed_from_u64(861);
        for kind in 0..10 {
            for _ in 0..80 {
                let mut p = Problem::sample(&mut rng);
                p.kind = kind;
                for u in [0.25, 0.5, 0.75] {
                    let x = (u - f64::from(p.b)) / f64::from(p.a);
                    let h = 1e-6;
                    let derivative =
                        (p.antiderivative(x + h) - p.antiderivative(x - h)) / (2.0 * h);
                    let value = p.integrand(x);
                    assert!(
                        (derivative - value).abs() < 1e-5 * value.abs().max(1.0),
                        "kind {kind}: {}",
                        p.render()
                    );
                }
            }
        }
    }

    #[test]
    fn definite_answers_match_independent_simpson_quadrature() {
        let mut rng = StdRng::seed_from_u64(43);
        for _ in 0..100 {
            let mut p = Problem::sample(&mut rng);
            p.kind = 9;
            let (lo, hi) = (f64::from(p.lo), f64::from(p.hi));
            let h = (hi - lo) / 1000.0;
            let mut sum = p.integrand(lo) + p.integrand(hi);
            for i in 1..1000 {
                sum += if i % 2 == 0 { 2.0 } else { 4.0 } * p.integrand(lo + f64::from(i) * h);
            }
            let exact = p.antiderivative(hi) - p.antiderivative(lo);
            assert!((sum * h / 3.0 - exact).abs() < 1e-8 * exact.abs().max(1.0));
        }
    }

    #[test]
    fn fractions_reduce_and_domains_are_explicit() {
        assert_eq!(fraction(-6, 9), "(-2/3)");
        assert_eq!(fraction(0, 4), "0");
        let mut rng = StdRng::seed_from_u64(7);
        for kind in [1, 6, 8] {
            let mut p = Problem::sample(&mut rng);
            p.kind = kind;
            assert!(p.render().contains("Domain:"));
            assert!(p.render().contains(match kind {
                1 => "!= 0",
                6 => "-1 <",
                _ => "> 0",
            }));
        }
    }
}
