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

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(n) if *n < 0 => write!(f, "({n})"),
            Self::Integer(n) => write!(f, "{n}"),
            Self::Binary(op, left, right) => write!(f, "({left} {op} {right})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

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
