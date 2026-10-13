use rand::{Rng, rngs::StdRng};
use std::fmt;

/// Required family entry point. Use only the supplied RNG for reproducibility.
pub fn generate(rng: &mut StdRng) -> String {
    let (left, right) = identity(rng);
    format!("{left} = {right}")
}

#[derive(Clone)]
enum Expr {
    Integer(i32),
    Binary(char, Box<Expr>, Box<Expr>),
}

fn binary(op: char, left: Expr, right: Expr) -> Expr {
    Expr::Binary(op, Box::new(left), Box::new(right))
}

fn integer(rng: &mut StdRng) -> Expr {
    // Exclude zero and units so that examples do not trivially collapse.
    let magnitude = rng.random_range(2..=9);
    Expr::Integer(if rng.random() { magnitude } else { -magnitude })
}

fn identity(rng: &mut StdRng) -> (Expr, Expr) {
    let (a, b, c) = (integer(rng), integer(rng), integer(rng));
    match rng.random_range(0..4_u32) {
        // a * (b + c) = a * b + a * c
        0 => (
            binary('*', a.clone(), binary('+', b.clone(), c.clone())),
            binary('+', binary('*', a.clone(), b), binary('*', a, c)),
        ),
        // a * (b - c) = a * b - a * c
        1 => (
            binary('*', a.clone(), binary('-', b.clone(), c.clone())),
            binary('-', binary('*', a.clone(), b), binary('*', a, c)),
        ),
        // (a + b) * (a - b) = a * a - b * b
        2 => (
            binary(
                '*',
                binary('+', a.clone(), b.clone()),
                binary('-', a.clone(), b.clone()),
            ),
            binary('-', binary('*', a.clone(), a), binary('*', b.clone(), b)),
        ),
        // a - (b - c) = (a - b) + c
        _ => (
            binary('-', a.clone(), binary('-', b.clone(), c.clone())),
            binary('+', binary('-', a, b), c),
        ),
    }
}

impl Expr {
    fn precedence(&self) -> u8 {
        match self {
            Self::Integer(_) => 3,
            Self::Binary('*', _, _) => 2,
            Self::Binary(_, _, _) => 1,
        }
    }

    fn fmt_operand(&self, f: &mut fmt::Formatter<'_>, op: char, right: bool) -> fmt::Result {
        let precedence = if op == '*' { 2 } else { 1 };
        let parentheses = self.precedence() < precedence
            || (right && op == '-' && self.precedence() == precedence)
            || ((right || op == '*') && matches!(self, Self::Integer(n) if *n < 0));
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
            Self::Binary(op, left, right) => {
                left.fmt_operand(f, *op, false)?;
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
        ] {
            assert_eq!(expr.to_string(), expected);
        }
    }

    fn evaluate(expr: &Expr) -> i64 {
        match expr {
            Expr::Integer(n) => i64::from(*n),
            Expr::Binary(op, left, right) => {
                let (left, right) = (evaluate(left), evaluate(right));
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
            assert_eq!(evaluate(&left), evaluate(&right), "{left} = {right}");
        }
    }
}
