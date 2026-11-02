use rand::{Rng, rngs::StdRng, seq::SliceRandom};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Expr {
    Atom(usize),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Imp(Box<Expr>, Box<Expr>),
    Box(Box<Expr>),
    Diamond(Box<Expr>),
}

fn boxed(a: Expr) -> Expr {
    Expr::Box(Box::new(a))
}

fn diamond(a: Expr) -> Expr {
    Expr::Diamond(Box::new(a))
}

fn implies(a: Expr, b: Expr) -> Expr {
    Expr::Imp(Box::new(a), Box::new(b))
}

#[derive(Debug)]
struct Frame {
    edges: Vec<u8>,
}

impl Frame {
    fn universe(&self) -> u8 {
        (1 << self.edges.len()) - 1
    }

    fn truth_set(&self, expr: &Expr, valuation: [u8; 2]) -> u8 {
        let all = self.universe();
        match expr {
            Expr::Atom(i) => valuation[*i],
            Expr::Not(a) => all ^ self.truth_set(a, valuation),
            Expr::And(a, b) => self.truth_set(a, valuation) & self.truth_set(b, valuation),
            Expr::Or(a, b) => self.truth_set(a, valuation) | self.truth_set(b, valuation),
            Expr::Imp(a, b) => (all ^ self.truth_set(a, valuation)) | self.truth_set(b, valuation),
            Expr::Box(a) => {
                let inner = self.truth_set(a, valuation);
                self.edges
                    .iter()
                    .enumerate()
                    .fold(0, |set, (w, &row)| set | (u8::from(row & !inner == 0) << w))
            }
            Expr::Diamond(a) => {
                let inner = self.truth_set(a, valuation);
                self.edges
                    .iter()
                    .enumerate()
                    .fold(0, |set, (w, &row)| set | (u8::from(row & inner != 0) << w))
            }
        }
    }

    // Serial, reflexive, symmetric, transitive, right Euclidean.
    fn properties(&self) -> [bool; 5] {
        let n = self.edges.len();
        let r = |a: usize, b: usize| self.edges[a] & (1u8 << b) != 0;
        [
            self.edges.iter().all(|&row| row != 0),
            (0..n).all(|a| r(a, a)),
            (0..n).all(|a| (0..n).all(|b| !r(a, b) || r(b, a))),
            (0..n).all(|a| (0..n).all(|b| (0..n).all(|c| !r(a, b) || !r(b, c) || r(a, c)))),
            (0..n).all(|a| (0..n).all(|b| (0..n).all(|c| !r(a, b) || !r(a, c) || r(b, c)))),
        ]
    }
}

impl Expr {
    fn swap_atoms(&mut self) {
        match self {
            Self::Atom(i) => *i ^= 1,
            Self::Not(a) | Self::Box(a) | Self::Diamond(a) => a.swap_atoms(),
            Self::And(a, b) | Self::Or(a, b) | Self::Imp(a, b) => {
                a.swap_atoms();
                b.swap_atoms();
            }
        }
    }

    fn variables(&self) -> u8 {
        match self {
            Self::Atom(i) => 1 << i,
            Self::Not(a) | Self::Box(a) | Self::Diamond(a) => a.variables(),
            Self::And(a, b) | Self::Or(a, b) | Self::Imp(a, b) => a.variables() | b.variables(),
        }
    }

    fn text(&self) -> String {
        match self {
            Self::Atom(i) => ["p", "q"][*i].into(),
            Self::Not(a) => format!("¬{}", a.text()),
            Self::Box(a) => format!("□{}", a.text()),
            Self::Diamond(a) => format!("◇{}", a.text()),
            Self::And(a, b) => format!("({} ∧ {})", a.text(), b.text()),
            Self::Or(a, b) => format!("({} ∨ {})", a.text(), b.text()),
            Self::Imp(a, b) => format!("({} → {})", a.text(), b.text()),
        }
    }
}

fn axiom(index: usize) -> Expr {
    let p = Expr::Atom(0);
    let q = Expr::Atom(1);
    match index {
        0 => implies(
            boxed(implies(p.clone(), q.clone())),
            implies(boxed(p), boxed(q)),
        ),
        1 => implies(boxed(p.clone()), diamond(p)),
        2 => implies(boxed(p.clone()), p),
        3 => implies(p.clone(), boxed(diamond(p))),
        4 => implies(boxed(p.clone()), boxed(boxed(p))),
        5 => implies(diamond(p.clone()), boxed(diamond(p))),
        6 => Expr::And(Box::new(axiom(2)), Box::new(axiom(4))),
        7 => Expr::And(Box::new(axiom(2)), Box::new(axiom(5))),
        _ => unreachable!(),
    }
}

fn counterexample(frame: &Frame, formula: &Expr, two_atoms: bool) -> Option<([u8; 2], usize)> {
    for p in 0..=frame.universe() {
        for q in 0..=if two_atoms { frame.universe() } else { 0 } {
            let truth = frame.truth_set(formula, [p, q]);
            if truth != frame.universe() {
                let world = (0..frame.edges.len())
                    .find(|w| truth & (1 << w) == 0)
                    .unwrap();
                return Some(([p, q], world));
            }
        }
    }
    None
}

fn transitive_closure(edges: &mut [u8]) {
    for k in 0..edges.len() {
        for i in 0..edges.len() {
            if edges[i] & (1 << k) != 0 {
                edges[i] |= edges[k];
            }
        }
    }
}

fn sample_frame(rng: &mut StdRng) -> Frame {
    let regime = rng.random_range(0..10);
    let n = if [6, 7, 9].contains(&regime) {
        rng.random_range(3..=4)
    } else {
        rng.random_range(2..=4)
    };
    let mut edges = vec![0; n];
    match regime {
        0 => {
            for row in &mut edges {
                *row = rng.random_range(0..1 << n);
            }
        }
        1 => (), // All dead ends.
        2 => {
            for (i, row) in edges.iter_mut().enumerate() {
                *row = 1 << ((i + 1) % n);
            }
        }
        3 => {
            let classes = (0..n).map(|_| rng.random_range(0..n)).collect::<Vec<_>>();
            for i in 0..n {
                for j in 0..n {
                    edges[i] |= u8::from(classes[i] == classes[j]) << j;
                }
            }
        }
        4 | 8 => {
            for (i, row) in edges.iter_mut().enumerate() {
                if regime == 4 {
                    *row |= 1 << i;
                }
                for j in i + 1..n {
                    *row |= u8::from(rng.random_bool(0.5)) << j;
                }
            }
            transitive_closure(&mut edges);
        }
        5 => {
            for (i, row) in edges.iter_mut().enumerate() {
                *row = 1 << ((i + 1) % n);
                for j in 0..n {
                    if i != j && rng.random_bool(0.4) {
                        *row |= 1 << j;
                    }
                }
            }
        }
        6 => {
            for (i, row) in edges.iter_mut().enumerate() {
                *row = 1 << i;
                if i + 1 < n {
                    *row |= 1 << (i + 1);
                }
            }
        }
        7 => {
            edges[0] = ((1 << n) - 1) ^ 1;
            for row in &mut edges[1..] {
                *row = 1;
            }
        }
        _ => {
            let clique = ((1 << n) - 1) ^ 1;
            edges.fill(clique);
        }
    }
    let mut permutation = (0..n).collect::<Vec<_>>();
    permutation.shuffle(rng);
    let mut relabeled = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if edges[i] & (1 << j) != 0 {
                relabeled[permutation[i]] |= 1 << permutation[j];
            }
        }
    }
    Frame { edges: relabeled }
}

fn random_formula(rng: &mut StdRng, depth: usize) -> Expr {
    if depth == 0 {
        return Expr::Atom(rng.random_range(0..2));
    }
    match rng.random_range(0..6) {
        0 => Expr::Not(Box::new(random_formula(rng, depth - 1))),
        1 => boxed(random_formula(rng, depth - 1)),
        2 => diamond(random_formula(rng, depth - 1)),
        kind => {
            let a = Box::new(random_formula(rng, depth - 1));
            let mut b = Box::new(random_formula(rng, depth - 1));
            // Preserve the sampled shape, but avoid accidental A ∧ A, A ∨ A,
            // or A → A. Swapping both atoms always changes a colliding tree.
            if a == b {
                b.swap_atoms();
            }
            match kind {
                3 => Expr::And(a, b),
                4 => Expr::Or(a, b),
                _ => Expr::Imp(a, b),
            }
        }
    }
}

fn set(mask: u8, n: usize) -> String {
    let worlds = (0..n)
        .filter(|w| mask & (1 << w) != 0)
        .map(|w| format!("w{w}"))
        .collect::<Vec<_>>();
    if worlds.is_empty() {
        "∅".into()
    } else {
        format!("{{{}}}", worlds.join(", "))
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let frame = sample_frame(rng);
    let n = frame.edges.len();
    let mut lines = frame
        .edges
        .iter()
        .enumerate()
        .map(|(w, &successors)| format!("R(w{w}) = {}", set(successors, n)))
        .collect::<Vec<_>>();
    if rng.random_bool(0.5) {
        let valuation = [
            rng.random_range(0..=frame.universe()),
            rng.random_range(0..=frame.universe()),
        ];
        let depth = rng.random_range(1..=2);
        let inner = random_formula(rng, depth);
        let formula = if rng.random_bool(0.5) {
            boxed(inner)
        } else {
            diamond(inner)
        };
        let truth = frame.truth_set(&formula, valuation);
        let world = rng.random_range(0..n);
        for i in 0..2 {
            if formula.variables() & (1 << i) != 0 {
                lines.push(format!("{} = {}", ["p", "q"][i], set(valuation[i], n)));
            }
        }
        lines.push(format!("φ = {}", formula.text()));
        lines.push(format!("[[φ]] = {}", set(truth, n)));
        lines.push(format!(
            "w{world} {} φ",
            if truth & (1 << world) != 0 {
                "⊨"
            } else {
                "⊭"
            }
        ));
    } else {
        let properties = frame.properties();
        let conditions = [
            true,
            properties[0],
            properties[1],
            properties[2],
            properties[3],
            properties[4],
            properties[1] && properties[3],
            properties[1] && properties[4],
        ];
        let failures = (1..8).filter(|&i| !conditions[i]).collect::<Vec<_>>();
        let index = if !failures.is_empty() && rng.random_bool(0.6) {
            failures[rng.random_range(0..failures.len())]
        } else {
            rng.random_range(0..8)
        };
        let formula = axiom(index);
        let names = [
            "serial",
            "reflexive",
            "symmetric",
            "transitive",
            "Euclidean",
        ];
        let relevant = match index {
            0 => vec![],
            6 => vec![1, 3],
            7 => vec![1, 4],
            _ => vec![index - 1],
        };
        for property in relevant {
            lines.push(format!("{}: {}", names[property], properties[property]));
        }
        lines.push(format!(
            "{}: φ = {}",
            ["K", "D", "T", "B", "4", "5", "S4 (T ∧ 4)", "S5 (T ∧ 5)"][index],
            formula.text()
        ));
        if let Some((valuation, world)) = counterexample(&frame, &formula, index == 0) {
            lines.push("Not valid on this frame.".into());
            lines.push(format!("Countervaluation p = {}", set(valuation[0], n)));
            if index == 0 {
                lines.push(format!("q = {}", set(valuation[1], n)));
            }
            lines.push(format!("w{world} ⊭ φ"));
        } else {
            let valuations = 1usize << (if index == 0 { 2 * n } else { n });
            lines.push(format!(
                "Valid on this frame (all {valuations} valuations, all {n} worlds)."
            ));
        }
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn distinct_operands(expr: &Expr) -> bool {
        match expr {
            Expr::Atom(_) => true,
            Expr::Not(a) | Expr::Box(a) | Expr::Diamond(a) => distinct_operands(a),
            Expr::And(a, b) | Expr::Or(a, b) | Expr::Imp(a, b) => {
                a != b && distinct_operands(a) && distinct_operands(b)
            }
        }
    }

    #[test]
    fn random_modal_trees_avoid_accidental_identical_operands() {
        let mut rng = StdRng::seed_from_u64(9_173_021);
        for depth in 0..=4 {
            for _ in 0..1_000 {
                let expr = random_formula(&mut rng, depth);
                assert!(distinct_operands(&expr), "{expr:?}");
            }
        }
    }

    fn at(frame: &Frame, expr: &Expr, valuation: [u8; 2], w: usize) -> bool {
        match expr {
            Expr::Atom(i) => valuation[*i] & (1 << w) != 0,
            Expr::Not(a) => !at(frame, a, valuation, w),
            Expr::And(a, b) => at(frame, a, valuation, w) && at(frame, b, valuation, w),
            Expr::Or(a, b) => at(frame, a, valuation, w) || at(frame, b, valuation, w),
            Expr::Imp(a, b) => !at(frame, a, valuation, w) || at(frame, b, valuation, w),
            Expr::Box(a) => (0..frame.edges.len())
                .all(|v| frame.edges[w] & (1 << v) == 0 || at(frame, a, valuation, v)),
            Expr::Diamond(a) => (0..frame.edges.len())
                .any(|v| frame.edges[w] & (1 << v) != 0 && at(frame, a, valuation, v)),
        }
    }

    fn frame_from_bits(n: usize, bits: usize) -> Frame {
        Frame {
            edges: (0..n)
                .map(|i| ((bits >> (i * n)) & ((1 << n) - 1)) as u8)
                .collect(),
        }
    }

    #[test]
    fn modal_correspondence_holds_on_every_frame_with_up_to_three_worlds() {
        for n in 1..=3 {
            let mut counts = [0; 5];
            for bits in 0..1 << (n * n) {
                let frame = frame_from_bits(n, bits);
                let properties = frame.properties();
                for index in 0..8 {
                    let formula = axiom(index);
                    let expected = match index {
                        0 => true,
                        6 => properties[1] && properties[3],
                        7 => properties[1] && properties[4],
                        _ => properties[index - 1],
                    };
                    let mut valid = true;
                    for p in 0..=frame.universe() {
                        for q in 0..=if index == 0 { frame.universe() } else { 0 } {
                            for w in 0..n {
                                let scalar = at(&frame, &formula, [p, q], w);
                                assert_eq!(
                                    scalar,
                                    frame.truth_set(&formula, [p, q]) & (1 << w) != 0
                                );
                                valid &= scalar;
                            }
                        }
                    }
                    assert_eq!(valid, expected, "n={n}, R={bits}, axiom={index}");
                    assert_eq!(
                        valid,
                        counterexample(&frame, &formula, index == 0).is_none()
                    );
                }
                for i in 0..5 {
                    counts[i] += usize::from(properties[i]);
                }
            }
            if n == 3 {
                assert_eq!(counts, [343, 64, 64, 171, 39]);
            }
        }
    }

    #[test]
    fn arbitrary_nested_formulas_match_world_by_world_semantics() {
        let mut rng = StdRng::seed_from_u64(811_273);
        for _ in 0..300 {
            let frame = sample_frame(&mut rng);
            let formula = random_formula(&mut rng, 3);
            for p in 0..=frame.universe() {
                for q in 0..=frame.universe() {
                    for w in 0..frame.edges.len() {
                        assert_eq!(
                            at(&frame, &formula, [p, q], w),
                            frame.truth_set(&formula, [p, q]) & (1 << w) != 0
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn dead_ends_and_nonreflexive_euclidean_frames_are_preserved() {
        let empty = Frame { edges: vec![0, 0] };
        assert_eq!(empty.truth_set(&boxed(Expr::Atom(0)), [0, 0]), 3);
        assert_eq!(empty.truth_set(&diamond(Expr::Atom(0)), [3, 0]), 0);
        let successor_clique = Frame {
            edges: vec![6, 6, 6],
        };
        assert_eq!(
            successor_clique.properties(),
            [true, false, false, true, true]
        );
    }
}
