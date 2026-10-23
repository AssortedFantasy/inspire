use rand::{Rng, rngs::StdRng};
use std::f64::consts::PI;
use std::fmt::Write;

#[derive(Clone, Copy, Debug)]
enum Integrand {
    Cubic([f64; 4]),
    Quartic,
    Exponential(f64),
    Peak { center: f64, sharpness: f64 },
    Kink(f64),
    SquareRoot,
    Oscillation(u32),
}

impl Integrand {
    fn value(self, x: f64) -> f64 {
        match self {
            Self::Cubic(c) => ((c[3] * x + c[2]) * x + c[1]) * x + c[0],
            Self::Quartic => x.powi(4),
            Self::Exponential(k) => (k * x).exp(),
            Self::Peak { center, sharpness } => 1.0 / (1.0 + (sharpness * (x - center)).powi(2)),
            Self::Kink(center) => (x - center).abs(),
            Self::SquareRoot => x.sqrt(),
            Self::Oscillation(cycles) => 1.0 + (2.0 * PI * f64::from(cycles) * x).cos(),
        }
    }

    fn integral(self) -> f64 {
        // All generated intervals are [0, 1].
        match self {
            Self::Cubic(c) => c[0] + c[1] / 2.0 + c[2] / 3.0 + c[3] / 4.0,
            Self::Quartic => 0.2,
            Self::Exponential(k) => k.exp_m1() / k,
            Self::Peak { center, sharpness } => {
                ((sharpness * (1.0 - center)).atan() + (sharpness * center).atan()) / sharpness
            }
            Self::Kink(center) => (center.powi(2) + (1.0 - center).powi(2)) / 2.0,
            Self::SquareRoot => 2.0 / 3.0,
            Self::Oscillation(_) => 1.0,
        }
    }

    fn description(self) -> String {
        match self {
            Self::Cubic(c) => format!(
                "f(x) = {}x³ + ({})x² + ({})x + ({})",
                compact(c[3]),
                compact(c[2]),
                compact(c[1]),
                compact(c[0])
            ),
            Self::Quartic => "f(x) = x⁴".into(),
            Self::Exponential(k) => format!("f(x) = exp({}x)", compact(k)),
            Self::Peak { center, sharpness } => format!(
                "f(x) = 1 / (1 + ({}(x - {}))²)",
                compact(sharpness),
                compact(center)
            ),
            Self::Kink(center) => format!("f(x) = |x - {}|", compact(center)),
            Self::SquareRoot => "f(x) = √x".into(),
            Self::Oscillation(cycles) => format!("f(x) = 1 + cos(2π·{cycles}x)"),
        }
    }
}

fn integrate(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> (f64, f64) {
    assert!(n > 0 && n.is_multiple_of(2));
    let h = (b - a) / n as f64;
    let endpoints = f(a) + f(b);
    let mut odd = 0.0;
    let mut even = 0.0;
    for i in 1..n {
        let value = f(a + i as f64 * h);
        if i.is_multiple_of(2) {
            even += value;
        } else {
            odd += value;
        }
    }
    (
        h * (endpoints / 2.0 + odd + even),
        h * (endpoints + 4.0 * odd + 2.0 * even) / 3.0,
    )
}

fn instance(rng: &mut StdRng, regime: usize) -> Integrand {
    match regime {
        0 => Integrand::Cubic([
            rng.random_range(-3..=3) as f64,
            rng.random_range(-3..=3) as f64,
            rng.random_range(-3..=3) as f64,
            rng.random_range(1..=4) as f64,
        ]),
        1 => Integrand::Quartic,
        2 => Integrand::Exponential([0.5, 1.0, 2.0, 3.0][rng.random_range(0..4)]),
        3 => Integrand::Peak {
            center: [0.25, 0.375, 0.5, 0.625, 0.75][rng.random_range(0..5)],
            sharpness: [20.0, 40.0, 80.0][rng.random_range(0..3)],
        },
        4 => Integrand::Kink(rng.random_range(2..=8) as f64 / 10.0),
        5 => Integrand::SquareRoot,
        _ => Integrand::Oscillation([4, 8, 16, 32][rng.random_range(0..4)]),
    }
}

fn compact(value: f64) -> String {
    if value == 0.0 {
        return "0".into();
    }
    if value.abs() < 1e-5 || value.abs() >= 1e6 {
        let text = format!("{value:.4e}");
        let (mantissa, exponent) = text.split_once('e').unwrap();
        format!(
            "{}e{}",
            mantissa.trim_end_matches('0').trim_end_matches('.'),
            exponent.parse::<i32>().unwrap()
        )
    } else {
        format!("{value:.9}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .into()
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..7);
    let f = instance(rng, regime);
    let reference = f.integral();
    let mut text = format!(
        "{}\nIntegral on [0, 1]\nReference I ≈ {}\n\nn   Simpson S_n      |S_n - I|      E_prev/E_n  Trapezoid T_n    |T_n - I|",
        f.description(),
        compact(reference)
    );
    let mut previous: Option<f64> = None;
    for n in [2, 4, 8, 16, 32, 64] {
        let (trapezoid, simpson) = integrate(|x| f.value(x), 0.0, 1.0, n);
        let error = (simpson - reference).abs();
        let ratio = previous
            .filter(|&old| old > 1e-12 && error > 1e-12)
            .map(|old| {
                format!("{:.3}", old / error)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_owned()
            })
            .unwrap_or_else(|| "—".into());
        write!(
            text,
            "\n{n:<2}  {:<15}  {:<13}  {:<10}  {:<15}  {}",
            compact(simpson),
            compact(error),
            ratio,
            compact(trapezoid),
            compact((trapezoid - reference).abs())
        )
        .unwrap();
        previous = Some(error);
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_polynomial_moments_and_known_quartic_error() {
        for (a, b) in [(-2.0_f64, 3.0_f64), (0.0, 1.0), (0.25, 1.75)] {
            for n in [2, 4, 8, 16, 32] {
                for degree in 0..=3 {
                    let exact = (b.powi(degree + 1) - a.powi(degree + 1)) / f64::from(degree + 1);
                    let (trapezoid, simpson) = integrate(|x| x.powi(degree), a, b, n);
                    assert!((simpson - exact).abs() < 1e-10);
                    if degree <= 1 {
                        assert!((trapezoid - exact).abs() < 1e-10);
                    }
                }
                let exact = (b.powi(5) - a.powi(5)) / 5.0;
                let (_, simpson) = integrate(|x| x.powi(4), a, b, n);
                let expected_error = 2.0 * (b - a) * ((b - a) / n as f64).powi(4) / 15.0;
                assert!((simpson - exact - expected_error).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn analytic_references_and_different_convergence_regimes() {
        for f in [
            Integrand::Exponential(2.0),
            Integrand::Peak {
                center: 0.375,
                sharpness: 40.0,
            },
            Integrand::Kink(0.3),
            Integrand::SquareRoot,
        ] {
            // Independent midpoint sum, fine enough to resolve the narrowest peak.
            let n = 100_000;
            let midpoint: f64 = (0..n)
                .map(|i| f.value((i as f64 + 0.5) / n as f64))
                .sum::<f64>()
                / n as f64;
            assert!((midpoint - f.integral()).abs() < 1e-8, "{f:?}");
        }
        let error =
            |f: Integrand, n| (integrate(|x| f.value(x), 0.0, 1.0, n).1 - f.integral()).abs();
        assert!((error(Integrand::Quartic, 8) / error(Integrand::Quartic, 16) - 16.0).abs() < 1e-7);
        let endpoint_ratio = error(Integrand::SquareRoot, 32) / error(Integrand::SquareRoot, 64);
        assert!((2.8..2.9).contains(&endpoint_ratio));
        let kink_ratio = error(Integrand::Kink(0.3), 32) / error(Integrand::Kink(0.3), 64);
        assert!(kink_ratio < 16.0);
        // The first three refinements agree exactly while missing a unit of area.
        for n in [2, 4, 8] {
            assert!((error(Integrand::Oscillation(8), n) - 1.0).abs() < 1e-12);
        }
        assert!(error(Integrand::Oscillation(8), 32) < 1e-12);
    }
}
