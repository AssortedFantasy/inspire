use rand::{Rng, rngs::StdRng, seq::SliceRandom};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    And,
    Or,
    Imp,
    Iff,
    Xor,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Expr {
    Atom(usize),
    Not(Box<Expr>),
    Binary(Op, Box<Expr>, Box<Expr>),
}

fn not(a: Expr) -> Expr {
    if let Expr::Not(inner) = a {
        *inner
    } else {
        Expr::Not(Box::new(a))
    }
}

fn binary(op: Op, a: Expr, b: Expr) -> Expr {
    Expr::Binary(op, Box::new(a), Box::new(b))
}

impl Expr {
    fn rotate_atoms(&mut self, n: usize) {
        match self {
            Self::Atom(i) => *i = (*i + 1) % n,
            Self::Not(a) => a.rotate_atoms(n),
            Self::Binary(_, a, b) => {
                a.rotate_atoms(n);
                b.rotate_atoms(n);
            }
        }
    }

    fn eval(&self, valuation: u8) -> bool {
        match self {
            Self::Atom(i) => valuation & (1 << i) != 0,
            Self::Not(a) => !a.eval(valuation),
            Self::Binary(op, a, b) => {
                let (a, b) = (a.eval(valuation), b.eval(valuation));
                match op {
                    Op::And => a && b,
                    Op::Or => a || b,
                    Op::Imp => !a || b,
                    Op::Iff => a == b,
                    Op::Xor => a != b,
                }
            }
        }
    }

    fn variables(&self) -> u8 {
        match self {
            Self::Atom(i) => 1 << i,
            Self::Not(a) => a.variables(),
            Self::Binary(_, a, b) => a.variables() | b.variables(),
        }
    }

    fn text(&self) -> String {
        match self {
            Self::Atom(i) => ["p", "q", "r", "s"][*i].into(),
            Self::Not(a) => format!("¬{}", a.text()),
            Self::Binary(op, a, b) => format!(
                "({} {} {})",
                a.text(),
                match op {
                    Op::And => "∧",
                    Op::Or => "∨",
                    Op::Imp => "→",
                    Op::Iff => "↔",
                    Op::Xor => "⊕",
                },
                b.text()
            ),
        }
    }
}

fn random_expr(rng: &mut StdRng, depth: usize, n: usize) -> Expr {
    if depth == 0 || rng.random_ratio(1, 5) {
        return Expr::Atom(rng.random_range(0..n));
    }
    if rng.random_ratio(1, 5) {
        return not(random_expr(rng, depth - 1, n));
    }
    let op = [Op::And, Op::Or, Op::Imp, Op::Iff, Op::Xor][rng.random_range(0..5)];
    let (a, b) = random_pair(rng, depth - 1, n);
    binary(op, a, b)
}

fn random_pair(rng: &mut StdRng, depth: usize, n: usize) -> (Expr, Expr) {
    let a = random_expr(rng, depth, n);
    let mut b = random_expr(rng, depth, n);
    // Every expression contains an atom; this changes a colliding tree without
    // rejection or extra depth. Intentional identities use binary() directly.
    if a == b {
        b.rotate_atoms(n);
    }
    (a, b)
}

fn law(rng: &mut StdRng, n: usize) -> Expr {
    let (mut a, mut b) = random_pair(rng, 1, n);
    // Otherwise contraposition can collapse to the literal identity A ↔ A.
    if b == not(a.clone()) {
        b.rotate_atoms(n);
    }
    let mut c = random_expr(rng, 1, n);
    if c == a || c == b {
        // Two exclusions cannot exhaust the at least four available literals.
        let alternatives = (0..n)
            .flat_map(|i| [Expr::Atom(i), not(Expr::Atom(i))])
            .filter(|expr| expr != &a && expr != &b)
            .collect::<Vec<_>>();
        c = alternatives[rng.random_range(0..alternatives.len())].clone();
    }
    match rng.random_range(0..7) {
        0 => binary(Op::Or, a.clone(), not(a)),
        1 => binary(
            Op::Imp,
            binary(Op::Imp, binary(Op::Imp, a.clone(), b), a.clone()),
            a,
        ),
        2 => binary(
            Op::Iff,
            binary(Op::Imp, a.clone(), b.clone()),
            binary(Op::Imp, not(b), not(a)),
        ),
        3 => binary(
            Op::Iff,
            not(binary(Op::And, a.clone(), b.clone())),
            binary(Op::Or, not(a), not(b)),
        ),
        4 => {
            // With depth-one substitutions, this collision requires b and c
            // to be atoms, so rotating a cannot collide with either of them.
            if a == binary(Op::Or, b.clone(), c.clone()) {
                a.rotate_atoms(n);
            }
            binary(
                Op::Iff,
                binary(Op::And, a.clone(), binary(Op::Or, b.clone(), c.clone())),
                binary(Op::Or, binary(Op::And, a.clone(), b), binary(Op::And, a, c)),
            )
        }
        5 => binary(
            Op::Iff,
            binary(Op::Iff, a.clone(), b.clone()),
            not(binary(Op::Xor, a, b)),
        ),
        _ => binary(
            Op::Imp,
            binary(
                Op::And,
                binary(Op::Imp, a.clone(), b.clone()),
                binary(Op::Imp, b, c.clone()),
            ),
            binary(Op::Imp, a, c),
        ),
    }
}

// Equivalent conjunctions vary the AST without changing the unique model.
fn conjunction(rng: &mut StdRng, a: Expr, b: Expr) -> Expr {
    match rng.random_range(0..3) {
        0 => binary(Op::And, a, b),
        1 => not(binary(Op::Or, not(a), not(b))),
        _ => not(binary(Op::Imp, a, not(b))),
    }
}

fn unique_model(rng: &mut StdRng, n: usize) -> Expr {
    let mut variables = (0..n).collect::<Vec<_>>();
    variables.shuffle(rng);
    let mut pieces = variables
        .into_iter()
        .map(|i| {
            let atom = Expr::Atom(i);
            if rng.random_bool(0.5) {
                atom
            } else {
                not(atom)
            }
        })
        .collect::<Vec<_>>();
    while pieces.len() > 1 {
        let a = pieces.remove(rng.random_range(0..pieces.len()));
        let b = pieces.remove(rng.random_range(0..pieces.len()));
        pieces.push(conjunction(rng, a, b));
    }
    pieces.pop().unwrap()
}

fn sample(rng: &mut StdRng, regime: usize) -> Expr {
    let n = rng.random_range(2..=4);
    match regime {
        0 => law(rng, n),
        1 => not(law(rng, n)),
        2 => unique_model(rng, n),
        3 => not(unique_model(rng, n)),
        _ => {
            // A single polarity switch can turn the surrounding arbitrary
            // subformula into an implication, equivalence, or parity boundary.
            let (a, b) = random_pair(rng, 2, n);
            let op = [Op::Imp, Op::Iff, Op::Xor][rng.random_range(0..3)];
            binary(op, a, b)
        }
    }
}

fn truth(value: bool) -> &'static str {
    if value { "T" } else { "F" }
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..5);
    let expr = sample(rng, regime);
    let used = expr.variables();
    let variables = (0..4).filter(|i| used & (1 << i) != 0).collect::<Vec<_>>();
    let valuations = (0..1u8 << variables.len())
        .map(|row| {
            variables.iter().enumerate().fold(0, |v, (j, i)| {
                v | (((row >> (variables.len() - 1 - j)) & 1) << i)
            })
        })
        .collect::<Vec<u8>>();
    let mut lines = vec![format!("φ = {}", expr.text())];
    lines.push(format!(
        "{} | φ",
        variables
            .iter()
            .map(|&i| ["p", "q", "r", "s"][i])
            .collect::<Vec<_>>()
            .join(" ")
    ));
    for &valuation in &valuations {
        let values = variables
            .iter()
            .map(|i| truth(valuation & (1 << i) != 0))
            .collect::<Vec<_>>()
            .join(" ");
        lines.push(format!("{values} | {}", truth(expr.eval(valuation))));
    }
    let satisfying = valuations.iter().filter(|&&v| expr.eval(v)).count();
    let classification = if satisfying == valuations.len() {
        "Tautology"
    } else if satisfying == 0 {
        "Contradiction"
    } else {
        "Contingent"
    };
    lines.push(format!(
        "{classification} ({}/{} rows true)",
        satisfying,
        valuations.len()
    ));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn distinct_operands(expr: &Expr) -> bool {
        match expr {
            Expr::Atom(_) => true,
            Expr::Not(a) => distinct_operands(a),
            Expr::Binary(_, a, b) => a != b && distinct_operands(a) && distinct_operands(b),
        }
    }

    #[test]
    fn sampled_trees_avoid_accidental_identical_operands() {
        let mut rng = StdRng::seed_from_u64(20_261_102);
        for n in 2..=4 {
            for depth in 0..=3 {
                for _ in 0..500 {
                    let expr = random_expr(&mut rng, depth, n);
                    assert!(distinct_operands(&expr), "{expr:?}");
                    assert_eq!(expr.variables() >> n, 0);
                }
            }
        }
        for regime in 0..5 {
            for _ in 0..500 {
                let expr = sample(&mut rng, regime);
                assert!(distinct_operands(&expr), "{expr:?}");
            }
        }
    }

    #[test]
    fn explicit_identities_remain_available() {
        let a = binary(Op::Imp, Expr::Atom(0), Expr::Atom(1));
        for valuation in 0..4 {
            assert!(binary(Op::Iff, a.clone(), a.clone()).eval(valuation));
            assert!(!binary(Op::Xor, a.clone(), a.clone()).eval(valuation));
            assert_eq!(
                binary(Op::And, a.clone(), a.clone()).eval(valuation),
                a.eval(valuation)
            );
        }
    }

    // A second evaluator represents the 16 valuations as bits and builds
    // implication and equivalence from Boolean algebra rather than booleans.
    fn truth_vector(expr: &Expr) -> u16 {
        match expr {
            Expr::Atom(i) => [0xaaaa, 0xcccc, 0xf0f0, 0xff00][*i],
            Expr::Not(a) => !truth_vector(a),
            Expr::Binary(op, a, b) => {
                let (a, b) = (truth_vector(a), truth_vector(b));
                match op {
                    Op::And => a & b,
                    Op::Or => a | b,
                    Op::Imp => !(a & !b),
                    Op::Iff => (a & b) | (!a & !b),
                    Op::Xor => (a & !b) | (!a & b),
                }
            }
        }
    }

    #[test]
    fn constructors_and_scalar_semantics_match_independent_truth_vectors() {
        let mut rng = StdRng::seed_from_u64(902_177);
        for regime in 0..5 {
            for _ in 0..400 {
                let expr = sample(&mut rng, regime);
                let vector = truth_vector(&expr);
                for valuation in 0..16 {
                    assert_eq!(expr.eval(valuation), vector & (1 << valuation) != 0);
                }
                let used = expr.variables().count_ones();
                let repeats = 1 << (4 - used);
                match regime {
                    0 => assert_eq!(vector, u16::MAX),
                    1 => assert_eq!(vector, 0),
                    2 => assert_eq!(vector.count_ones(), repeats),
                    3 => assert_eq!(vector.count_zeros(), repeats),
                    _ => (),
                }
            }
        }
    }

    #[test]
    fn every_binary_connective_has_its_classical_four_rows() {
        for (op, expected) in [
            (Op::And, [false, false, false, true]),
            (Op::Or, [false, true, true, true]),
            (Op::Imp, [true, false, true, true]),
            (Op::Iff, [true, false, false, true]),
            (Op::Xor, [false, true, true, false]),
        ] {
            let expr = binary(op, Expr::Atom(0), Expr::Atom(1));
            assert_eq!([0, 1, 2, 3].map(|v| expr.eval(v)), expected);
        }
    }
}
