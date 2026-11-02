use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

#[derive(Clone, Copy, Debug)]
enum Problem {
    Square(f64),
    Multiple { root: f64, multiplicity: i32 },
    Cosine,
    Cycle,
    Stationary(f64),
    CubeRoot,
}

impl Problem {
    fn value(self, x: f64) -> f64 {
        match self {
            Self::Square(a) => x * x - a,
            Self::Multiple { root, multiplicity } => (x - root).powi(multiplicity),
            Self::Cosine => x.cos() - x,
            Self::Cycle => x.powi(3) - 2.0 * x + 2.0,
            Self::Stationary(a) => x.powi(3) - a,
            Self::CubeRoot => x.cbrt(),
        }
    }

    fn derivative(self, x: f64) -> f64 {
        match self {
            Self::Square(_) => 2.0 * x,
            Self::Multiple { root, multiplicity } => {
                f64::from(multiplicity) * (x - root).powi(multiplicity - 1)
            }
            Self::Cosine => -x.sin() - 1.0,
            Self::Cycle => 3.0 * x * x - 2.0,
            Self::Stationary(_) => 3.0 * x * x,
            Self::CubeRoot => 1.0 / (3.0 * x.cbrt().powi(2)),
        }
    }

    fn root(self) -> f64 {
        match self {
            Self::Square(a) => a.sqrt(),
            Self::Multiple { root, .. } => root,
            Self::Cosine => 0.739_085_133_215_160_7,
            Self::Cycle => -1.769_292_354_238_631_4,
            Self::Stationary(a) => a.cbrt(),
            Self::CubeRoot => 0.0,
        }
    }

    fn description(self) -> String {
        match self {
            Self::Square(a) => format!("f(x) = x² - {}\nSimple positive root", compact(a)),
            Self::Multiple { root, multiplicity } => format!(
                "f(x) = (x - ({}))^{multiplicity}\nMultiplicity {multiplicity}, linear convergence factor {}",
                compact(root),
                compact(f64::from(multiplicity - 1) / f64::from(multiplicity))
            ),
            Self::Cosine => "f(x) = cos(x) - x\nSimple root".into(),
            Self::Cycle => "f(x) = x³ - 2x + 2\nStarting orbit 0 ↔ 1".into(),
            Self::Stationary(a) => format!("f(x) = x³ - {}\nStationary starting point", compact(a)),
            Self::CubeRoot => "f(x) = cbrt(x)\nDivergent orbit x[n+1] = -2 x[n]".into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stop {
    Converged,
    ZeroDerivative,
    TwoCycle,
    Limit,
}

fn iterate(problem: Problem, initial: f64, limit: usize) -> (Vec<f64>, Stop) {
    let mut values = vec![initial];
    for _ in 0..limit {
        let x = *values.last().unwrap();
        let f = problem.value(x);
        if f == 0.0 {
            return (values, Stop::Converged);
        }
        let derivative = problem.derivative(x);
        if derivative == 0.0 {
            return (values, Stop::ZeroDerivative);
        }
        let next = x - f / derivative;
        values.push(next);
        if problem.value(next).abs() <= 1e-10 && (next - x).abs() <= 1e-10 * (1.0 + next.abs()) {
            return (values, Stop::Converged);
        }
        if values.len() >= 3 && next == values[values.len() - 3] && next != x {
            return (values, Stop::TwoCycle);
        }
    }
    (values, Stop::Limit)
}

fn instance(rng: &mut StdRng, regime: usize) -> (Problem, f64, usize) {
    match regime {
        0 => {
            let a = [2.0, 3.0, 5.0, 7.0, 11.0, 17.0, 23.0, 31.0][rng.random_range(0..8)];
            (
                Problem::Square(a),
                rng.random_range(1..=12) as f64 / 2.0,
                12,
            )
        }
        1 => {
            let root = rng.random_range(-3..=3) as f64;
            let multiplicity = rng.random_range(2..=5);
            let offset = rng.random_range(2..=8) as f64 / 2.0;
            (Problem::Multiple { root, multiplicity }, root + offset, 12)
        }
        2 => (Problem::Cosine, rng.random_range(0..=6) as f64 / 4.0, 12),
        3 => (
            Problem::Cycle,
            if rng.random_bool(0.5) { 1.0 } else { 0.0 },
            12,
        ),
        4 => (Problem::Stationary(rng.random_range(1..=9) as f64), 0.0, 12),
        _ => {
            let sign = if rng.random_bool(0.5) { 1.0 } else { -1.0 };
            (
                Problem::CubeRoot,
                sign * [0.25, 0.5, 1.0, 2.0][rng.random_range(0..4)],
                8,
            )
        }
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
    let regime = rng.random_range(0..6);
    let (problem, initial, limit) = instance(rng, regime);
    let (values, stop) = iterate(problem, initial, limit);
    let root = problem.root();
    let mut text = format!(
        "{}\nNewton iterates\nReference root ≈ {}\n\nn   x_n              f(x_n)",
        problem.description(),
        compact(root)
    );
    for (i, &x) in values.iter().enumerate() {
        write!(
            text,
            "\n{i:<2}  {:<15}  {}",
            compact(x),
            compact(problem.value(x))
        )
        .unwrap();
    }
    let result = match stop {
        Stop::Converged => "Residual and step tolerances reached".into(),
        Stop::ZeroDerivative => "Stopped at zero derivative with nonzero residual".into(),
        Stop::TwoCycle => "Stopped at exact two-cycle".into(),
        Stop::Limit => format!("Stopped after {limit} updates, tolerance not reached"),
    };
    write!(text, "\n{result}").unwrap();
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn derivatives_agree_with_symmetric_difference() {
        let problems = [
            Problem::Square(7.0),
            Problem::Multiple {
                root: -1.0,
                multiplicity: 4,
            },
            Problem::Cosine,
            Problem::Cycle,
            Problem::Stationary(3.0),
            Problem::CubeRoot,
        ];
        for problem in problems {
            assert!(problem.value(problem.root()).abs() < 1e-12);
            for x in [-2.5, -0.5, 0.25, 1.5, 3.0] {
                let h = 1e-5;
                let numerical = (problem.value(x + h) - problem.value(x - h)) / (2.0 * h);
                assert!((numerical - problem.derivative(x)).abs() < 1e-6 * (1.0 + numerical.abs()));
            }
        }
    }

    #[test]
    fn regime_outcomes_and_independent_recurrences() {
        let mut rng = StdRng::seed_from_u64(20261023);
        for regime in 0..6 {
            for _ in 0..100 {
                let (problem, initial, limit) = instance(&mut rng, regime);
                let (values, stop) = iterate(problem, initial, limit);
                assert!(values.len() <= limit + 1);
                assert!(values.iter().all(|x| x.is_finite()));
                match problem {
                    Problem::Square(a) => {
                        assert_eq!(stop, Stop::Converged);
                        for pair in values.windows(2) {
                            assert!((pair[1] - (pair[0] + a / pair[0]) / 2.0).abs() < 1e-12);
                        }
                        assert!((values.last().unwrap() - a.sqrt()).abs() < 1e-9);
                    }
                    Problem::Multiple { root, multiplicity } => {
                        assert_eq!(stop, Stop::Limit);
                        let ratio = f64::from(multiplicity - 1) / f64::from(multiplicity);
                        for (n, &x) in values.iter().enumerate() {
                            assert!(
                                (x - root - (initial - root) * ratio.powi(n as i32)).abs() < 1e-12
                            );
                        }
                        assert!((values.last().unwrap() - root).abs() > 1e-5);
                    }
                    Problem::Cosine => {
                        assert_eq!(stop, Stop::Converged);
                        assert!((values.last().unwrap() - problem.root()).abs() < 1e-9);
                    }
                    Problem::Cycle => {
                        assert_eq!(stop, Stop::TwoCycle);
                        assert_eq!(
                            values,
                            if initial == 0.0 {
                                vec![0.0, 1.0, 0.0]
                            } else {
                                vec![1.0, 0.0, 1.0]
                            }
                        );
                    }
                    Problem::Stationary(_) => {
                        assert_eq!(stop, Stop::ZeroDerivative);
                        assert_eq!(values, vec![0.0]);
                    }
                    Problem::CubeRoot => {
                        assert_eq!(stop, Stop::Limit);
                        for (n, &x) in values.iter().enumerate() {
                            assert!((x - initial * (-2.0_f64).powi(n as i32)).abs() < 1e-9);
                        }
                    }
                }
            }
        }
    }
}
