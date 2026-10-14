use rand::{Rng, rngs::StdRng, seq::SliceRandom};
use std::fmt;

/// Required family entry point. Use only the supplied RNG for reproducibility.
pub fn generate(rng: &mut StdRng) -> String {
    let (left, right) = identity(rng);
    if rng.random_ratio(1, 3) {
        format!("{right} = {left}")
    } else {
        format!("{left} = {right}")
    }
}

enum Expr {
    Integer(i32),
    Variable(char, u8),
    Negated(Box<Expr>),
    Binary(char, Box<Expr>, Box<Expr>),
}

fn binary(op: char, left: Expr, right: Expr) -> Expr {
    Expr::Binary(op, Box::new(left), Box::new(right))
}

fn expression(
    rng: &mut StdRng,
    operations: usize,
    numbers: &mut Vec<i32>,
    two_variables: bool,
) -> Expr {
    if operations == 0 {
        return if rng.random_ratio(1, 2) {
            Expr::Integer(numbers.pop().unwrap())
        } else {
            Expr::Variable(
                if two_variables && rng.random() {
                    'y'
                } else {
                    'x'
                },
                1,
            )
        };
    }
    let left_operations = rng.random_range(0..operations);
    let op = ['+', '-', '*', '*'][rng.random_range(0..4)];
    binary(
        op,
        expression(rng, left_operations, numbers, two_variables),
        expression(
            rng,
            operations - 1 - left_operations,
            numbers,
            two_variables,
        ),
    )
}

fn identity(rng: &mut StdRng) -> (Expr, Expr) {
    // Sample tree shapes and operators, then collect terms exactly. Bound retries
    // to keep runtime predictable even when candidates cancel or grow too large.
    for _ in 0..64 {
        let mut numbers: Vec<_> = (2..=12).collect();
        numbers.shuffle(rng);
        let operations = rng.random_range(3..=5);
        let two_variables = rng.random_ratio(1, 3);
        let left = expression(rng, operations, &mut numbers, two_variables);
        if numbers.len() > 9 {
            continue;
        }
        let Some(polynomial) = Polynomial::from_expr(&left) else {
            continue;
        };
        if polynomial.0.iter().filter(|&&n| n != 0).count() < 2
            || (polynomial.0[1..3].iter().all(|n| n.abs() <= 1)
                && polynomial.0[3..].iter().all(|&n| n == 0))
        {
            continue;
        }
        let right = polynomial.expression();
        let (left_text, right_text) = (left.to_string(), right.to_string());
        if left_text != right_text && left_text.len() + right_text.len() <= 100 {
            return (left, right);
        }
    }
    // A readable fallback with distinct numeric inputs and no cancellation.
    let mut numbers: Vec<_> = (2..=12).collect();
    numbers.shuffle(rng);
    let left = binary(
        '*',
        Expr::Integer(numbers[0]),
        binary('+', Expr::Variable('x', 1), Expr::Integer(numbers[1])),
    );
    let right = Polynomial::from_expr(&left).unwrap().expression();
    (left, right)
}

// Coefficients of 1, x, y, x^2, xy, y^2. Restrict degree and coefficient size
// while constructing candidates, so both computation and output stay small.
const POWERS: [(u8, u8); 6] = [(0, 0), (1, 0), (0, 1), (2, 0), (1, 1), (0, 2)];

struct Polynomial([i32; 6]);

impl Polynomial {
    fn from_expr(expr: &Expr) -> Option<Self> {
        let mut coefficients = [0; 6];
        match expr {
            Expr::Integer(n) => coefficients[0] = *n,
            Expr::Variable(variable, power) => {
                let powers = if *variable == 'x' {
                    (*power, 0)
                } else {
                    (0, *power)
                };
                coefficients[POWERS.iter().position(|&p| p == powers)?] = 1;
            }
            Expr::Negated(expr) => {
                coefficients = Self::from_expr(expr)?.0.map(|n| -n);
            }
            Expr::Binary(op, left, right) => {
                let (left, right) = (Self::from_expr(left)?, Self::from_expr(right)?);
                if *op == '*' {
                    for (i, a) in left.0.into_iter().enumerate().filter(|&(_, n)| n != 0) {
                        for (j, b) in right.0.into_iter().enumerate().filter(|&(_, n)| n != 0) {
                            let powers = (POWERS[i].0 + POWERS[j].0, POWERS[i].1 + POWERS[j].1);
                            let index = POWERS.iter().position(|&p| p == powers)?;
                            coefficients[index] += a * b;
                        }
                    }
                } else {
                    for (i, coefficient) in coefficients.iter_mut().enumerate() {
                        *coefficient =
                            left.0[i] + if *op == '+' { right.0[i] } else { -right.0[i] };
                    }
                }
            }
        }
        if coefficients.iter().any(|n| n.abs() > 200)
            || coefficients.iter().all(|&n| n == 0)
            || (coefficients[1..].iter().all(|&n| n == 0) && coefficients[0].abs() == 1)
        {
            return None;
        }
        Some(Self(coefficients))
    }

    fn expression(&self) -> Expr {
        let mut result = None;
        for index in [3, 4, 5, 1, 2, 0] {
            let coefficient = self.0[index];
            if coefficient == 0 {
                continue;
            }
            let (x, y) = POWERS[index];
            let monomial = match (x, y) {
                (0, 0) => None,
                (x, 0) => Some(Expr::Variable('x', x)),
                (0, y) => Some(Expr::Variable('y', y)),
                (x, y) => Some(binary('*', Expr::Variable('x', x), Expr::Variable('y', y))),
            };
            let magnitude = coefficient.abs();
            let mut term = match monomial {
                None => Expr::Integer(magnitude),
                Some(expr) if magnitude == 1 => expr,
                Some(expr) => binary('*', Expr::Integer(magnitude), expr),
            };
            if result.is_none() && coefficient < 0 {
                term = Expr::Negated(Box::new(term));
            }
            result = Some(match result {
                None => term,
                Some(expr) => binary(if coefficient < 0 { '-' } else { '+' }, expr, term),
            });
        }
        result.unwrap()
    }
}

impl Expr {
    fn precedence(&self) -> u8 {
        match self {
            Self::Integer(_) | Self::Variable(_, _) => 3,
            Self::Negated(_) => 2,
            Self::Binary('*', _, _) => 2,
            Self::Binary(_, _, _) => 1,
        }
    }

    fn fmt_operand(&self, f: &mut fmt::Formatter<'_>, op: char, right: bool) -> fmt::Result {
        let precedence = if op == '*' { 2 } else { 1 };
        let parentheses = self.precedence() < precedence
            || (right && op == '-' && self.precedence() == precedence)
            || ((right || op == '*') && matches!(self, Self::Integer(n) if *n < 0))
            || (right && matches!(self, Self::Negated(_)));
        if parentheses {
            write!(f, "({self})")
        } else {
            write!(f, "{self}")
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(n) => write!(f, "{n}"),
            Self::Variable(variable, 1) => write!(f, "{variable}"),
            Self::Variable(variable, power) => write!(f, "{variable}^{power}"),
            Self::Negated(expr) => {
                write!(f, "-")?;
                expr.fmt_operand(f, '*', true)
            }
            Self::Binary(op, left, right) => {
                left.fmt_operand(f, *op, false)?;
                if matches!(op, '+' | '-')
                    && let Self::Negated(expr) = right.as_ref()
                {
                    let sign = if *op == '+' { '-' } else { '+' };
                    write!(f, " {sign} ")?;
                    return expr.fmt_operand(f, sign, true);
                }
                // Render addition/subtraction of negative integers with a single sign.
                if matches!(op, '+' | '-')
                    && let Self::Integer(n) = right.as_ref()
                    && *n < 0
                {
                    let sign = if *op == '+' { '-' } else { '+' };
                    write!(f, " {sign} {}", -i64::from(*n))
                } else {
                    write!(f, " {op} ")?;
                    right.fmt_operand(f, *op, true)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn formatting_preserves_grouping_and_cleans_up_signs() {
        let n = Expr::Integer;
        for (expr, expected) in [
            (binary('*', n(8), binary('+', n(8), n(-3))), "8 * (8 - 3)"),
            (
                binary('+', binary('*', n(8), n(8)), binary('*', n(8), n(-3))),
                "8 * 8 + 8 * (-3)",
            ),
            (binary('-', n(3), binary('-', n(3), n(8))), "3 - (3 - 8)"),
            (binary('-', n(3), binary('+', n(4), n(5))), "3 - (4 + 5)"),
            (binary('+', binary('-', n(3), n(3)), n(8)), "3 - 3 + 8"),
            (
                binary('*', binary('+', n(8), n(6)), binary('-', n(8), n(6))),
                "(8 + 6) * (8 - 6)",
            ),
            (binary('-', n(-2), n(-9)), "-2 + 9"),
            (binary('*', n(-2), n(-9)), "(-2) * (-9)"),
            (
                binary('-', binary('*', n(-2), n(-2)), binary('*', n(-9), n(-9))),
                "(-2) * (-2) - (-9) * (-9)",
            ),
            (Expr::Negated(Box::new(Expr::Variable('x', 1))), "-x"),
            (
                Expr::Negated(Box::new(binary('*', n(12), Expr::Variable('x', 2)))),
                "-12 * x^2",
            ),
            (
                binary(
                    '-',
                    Expr::Variable('x', 1),
                    Expr::Negated(Box::new(binary('+', Expr::Variable('y', 1), n(2)))),
                ),
                "x + y + 2",
            ),
            (
                Expr::Negated(Box::new(binary('-', Expr::Variable('x', 1), n(2)))),
                "-(x - 2)",
            ),
        ] {
            assert_eq!(expr.to_string(), expected);
        }
    }

    fn evaluate(expr: &Expr, x: i64, y: i64) -> i64 {
        match expr {
            Expr::Integer(n) => i64::from(*n),
            Expr::Variable(variable, power) => {
                (if *variable == 'x' { x } else { y }).pow(u32::from(*power))
            }
            Expr::Negated(expr) => -evaluate(expr, x, y),
            Expr::Binary(op, left, right) => {
                let (left, right) = (evaluate(left, x, y), evaluate(right, x, y));
                match op {
                    '+' => left + right,
                    '-' => left - right,
                    '*' => left * right,
                    _ => panic!("unsupported operation"),
                }
            }
        }
    }

    #[test]
    fn generated_identities_are_true() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..10_000 {
            let (left, right) = identity(&mut rng);
            for x in -3..=3 {
                for y in -3..=3 {
                    assert_eq!(
                        evaluate(&left, x, y),
                        evaluate(&right, x, y),
                        "{left} = {right}; x={x}, y={y}"
                    );
                }
            }
        }
    }

    #[test]
    fn samples_use_distinct_inputs_and_cover_quadratics_and_two_variables() {
        fn numbers(expr: &Expr, seen: &mut Vec<i32>) {
            match expr {
                Expr::Integer(n) => {
                    assert!((2..=12).contains(n));
                    assert!(!seen.contains(n), "repeated input {n}");
                    seen.push(*n);
                }
                Expr::Binary(_, left, right) => {
                    numbers(left, seen);
                    numbers(right, seen);
                }
                Expr::Variable(_, _) => (),
                Expr::Negated(expr) => numbers(expr, seen),
            }
        }
        let mut rng = StdRng::seed_from_u64(7);
        let (mut quadratic, mut two_variables) = (false, false);
        for _ in 0..1_000 {
            let (left, right) = identity(&mut rng);
            let mut inputs = Vec::new();
            numbers(&left, &mut inputs);
            assert!(inputs.len() >= 2);
            assert_ne!(left.to_string(), right.to_string());
            assert!(left.to_string().len() + right.to_string().len() <= 100);
            let coefficients = Polynomial::from_expr(&left).unwrap().0;
            quadratic |= coefficients[3..].iter().any(|&n| n != 0);
            two_variables |= coefficients[4] != 0;
        }
        assert!(quadratic && two_variables);
    }

    #[test]
    fn rejects_cancellation_high_degree_and_large_coefficients() {
        let x = || Expr::Variable('x', 1);
        for expr in [
            binary('-', x(), x()),
            binary('*', x(), binary('*', x(), x())),
            binary(
                '*',
                Expr::Integer(12),
                binary('*', Expr::Integer(11), Expr::Integer(10)),
            ),
        ] {
            assert!(Polynomial::from_expr(&expr).is_none());
        }
    }

    #[test]
    fn collected_coefficients_preserve_all_monomials_and_signs() {
        for coefficients in [
            [7, -3, 2, -1, 4, -5],
            [-7, 1, -1, 2, -3, 4],
            [0, 0, 0, -1, 1, -1],
            [200, -200, 200, -200, 200, -200],
        ] {
            let polynomial = Polynomial(coefficients);
            let expr = polynomial.expression();
            assert_eq!(Polynomial::from_expr(&expr).unwrap().0, coefficients);
            for x in -3_i64..=3 {
                for y in -3_i64..=3 {
                    let expected: i64 = coefficients
                        .iter()
                        .zip(POWERS)
                        .map(|(&c, (a, b))| i64::from(c) * x.pow(a.into()) * y.pow(b.into()))
                        .sum();
                    assert_eq!(evaluate(&expr, x, y), expected, "{expr}");
                }
            }
        }
        assert_eq!(
            Polynomial([7, -3, 2, -1, 4, -5]).expression().to_string(),
            "-x^2 + 4 * x * y - 5 * y^2 - 3 * x + 2 * y + 7"
        );
    }
}
