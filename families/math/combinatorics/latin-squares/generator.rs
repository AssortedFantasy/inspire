use rand::{Rng, rngs::StdRng, seq::SliceRandom};

type Square = Vec<Vec<usize>>;

#[derive(Clone, Copy, Debug)]
struct Field {
    p: usize,
    modulus: &'static [usize],
}

const FIELDS: [Field; 6] = [
    Field {
        p: 3,
        modulus: &[0, 1],
    },
    Field {
        p: 2,
        modulus: &[1, 1, 1],
    },
    Field {
        p: 5,
        modulus: &[0, 1],
    },
    Field {
        p: 7,
        modulus: &[0, 1],
    },
    Field {
        p: 2,
        modulus: &[1, 1, 0, 1],
    },
    Field {
        p: 3,
        modulus: &[1, 0, 1],
    },
];

impl Field {
    fn degree(self) -> usize {
        self.modulus.len() - 1
    }
    fn size(self) -> usize {
        self.p.pow(self.degree() as u32)
    }

    fn digits(self, mut x: usize) -> Vec<usize> {
        (0..self.degree())
            .map(|_| {
                let digit = x % self.p;
                x /= self.p;
                digit
            })
            .collect()
    }

    fn encode(self, digits: &[usize]) -> usize {
        digits.iter().rev().fold(0, |x, &digit| x * self.p + digit)
    }

    fn add(self, x: usize, y: usize) -> usize {
        self.encode(
            &self
                .digits(x)
                .iter()
                .zip(self.digits(y))
                .map(|(&a, b)| (a + b) % self.p)
                .collect::<Vec<_>>(),
        )
    }

    fn mul(self, x: usize, y: usize) -> usize {
        let n = self.degree();
        let mut product = vec![0; 2 * n - 1];
        for (i, a) in self.digits(x).into_iter().enumerate() {
            for (j, b) in self.digits(y).into_iter().enumerate() {
                product[i + j] = (product[i + j] + a * b) % self.p;
            }
        }
        for k in (n..product.len()).rev() {
            let c = product[k];
            for j in 0..n {
                product[k - n + j] =
                    (product[k - n + j] + self.p - c * self.modulus[j] % self.p) % self.p;
            }
        }
        self.encode(&product[..n])
    }

    fn element(self, x: usize) -> String {
        polynomial(&self.digits(x), "α")
    }

    fn model(self) -> String {
        if self.degree() == 1 {
            format!("F_{}", self.p)
        } else {
            format!(
                "K = F_{}[t]/({}), α = [t]",
                self.p,
                polynomial(self.modulus, "t")
            )
        }
    }
}

fn polynomial(coefficients: &[usize], variable: &str) -> String {
    let terms: Vec<_> = coefficients
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(degree, &c)| {
            if c == 0 {
                return None;
            }
            let power = match degree {
                0 => return Some(c.to_string()),
                1 => variable.to_owned(),
                _ => format!("{variable}^{degree}"),
            };
            Some(if c == 1 { power } else { format!("{c}{power}") })
        })
        .collect();
    if terms.is_empty() {
        "0".to_owned()
    } else {
        terms.join("+")
    }
}

fn field_square(field: Field, slope: usize) -> Square {
    (0..field.size())
        .map(|r| {
            (0..field.size())
                .map(|c| field.add(r, field.mul(slope, c)))
                .collect()
        })
        .collect()
}

#[derive(Clone, Copy, Debug)]
enum Group {
    Cyclic(usize),
    Product(&'static [usize]),
    Dihedral,
}

const GROUPS: [Group; 12] = [
    Group::Cyclic(3),
    Group::Cyclic(4),
    Group::Cyclic(5),
    Group::Cyclic(6),
    Group::Cyclic(7),
    Group::Cyclic(8),
    Group::Cyclic(9),
    Group::Product(&[2, 2]),
    Group::Product(&[2, 4]),
    Group::Product(&[2, 2, 2]),
    Group::Product(&[3, 3]),
    Group::Dihedral,
];

impl Group {
    fn size(self) -> usize {
        match self {
            Self::Cyclic(n) => n,
            Self::Product(factors) => factors.iter().product(),
            Self::Dihedral => 8,
        }
    }

    fn product(self, mut x: usize, mut y: usize) -> usize {
        match self {
            Self::Cyclic(n) => (x + y) % n,
            Self::Product(factors) => {
                let (mut result, mut place) = (0, 1);
                for &n in factors {
                    result += ((x % n + y % n) % n) * place;
                    x /= n;
                    y /= n;
                    place *= n;
                }
                result
            }
            Self::Dihedral => {
                let (i, j, k, l) = (x % 4, x / 4, y % 4, y / 4);
                (i + if j == 0 { k } else { (4 - k) % 4 }) % 4 + 4 * ((j + l) % 2)
            }
        }
    }

    fn name(self) -> String {
        match self {
            Self::Cyclic(n) => format!("C_{n} addition"),
            Self::Product(factors) => format!(
                "{} addition",
                factors
                    .iter()
                    .map(|n| format!("C_{n}"))
                    .collect::<Vec<_>>()
                    .join(" × ")
            ),
            Self::Dihedral => "D_8 = ⟨r,s | r^4 = s^2 = 1, srs = r^(-1)⟩".to_owned(),
        }
    }

    fn element(self, mut x: usize) -> String {
        match self {
            Self::Cyclic(_) => x.to_string(),
            Self::Product(factors) => {
                let coordinates: Vec<_> = factors
                    .iter()
                    .map(|&n| {
                        let c = x % n;
                        x /= n;
                        c.to_string()
                    })
                    .collect();
                format!("({})", coordinates.join(","))
            }
            Self::Dihedral => {
                let rotation = match x % 4 {
                    0 => "",
                    1 => "r",
                    2 => "r^2",
                    _ => "r^3",
                };
                if x >= 4 {
                    format!("{rotation}s")
                } else if x == 0 {
                    "1".to_owned()
                } else {
                    rotation.to_owned()
                }
            }
        }
    }

    fn square(self) -> Square {
        (0..self.size())
            .map(|r| (0..self.size()).map(|c| self.product(r, c)).collect())
            .collect()
    }
}

#[derive(Clone, Copy, Debug)]
struct Trade {
    rows: [usize; 2],
    columns: [usize; 2],
}

fn intercalates(square: &Square) -> Vec<Trade> {
    let n = square.len();
    let mut trades = vec![];
    for a in 0..n {
        for b in a + 1..n {
            for c in 0..n {
                for d in c + 1..n {
                    if square[a][c] == square[b][d] && square[a][d] == square[b][c] {
                        trades.push(Trade {
                            rows: [a, b],
                            columns: [c, d],
                        });
                    }
                }
            }
        }
    }
    trades
}

fn trade(square: &mut Square, selected: Trade) {
    for row in selected.rows {
        square[row].swap(selected.columns[0], selected.columns[1]);
    }
}

fn grid(square: &Square, labels: &[String], rows: &[usize], columns: &[usize]) -> String {
    let width = labels
        .iter()
        .map(|label| label.chars().count())
        .max()
        .unwrap();
    let mut lines = vec![format!(
        "{:>width$} | {}",
        "",
        columns
            .iter()
            .map(|&c| format!("{:>width$}", labels[c]))
            .collect::<Vec<_>>()
            .join(" ")
    )];
    for &r in rows {
        lines.push(format!(
            "{:>width$} | {}",
            labels[r],
            columns
                .iter()
                .map(|&c| format!("{:>width$}", labels[square[r][c]]))
                .collect::<Vec<_>>()
                .join(" ")
        ));
    }
    lines.join("\n")
}

fn indices(rng: &mut StdRng, n: usize) -> (Vec<usize>, Vec<usize>) {
    let mut rows: Vec<_> = (0..n).collect();
    let mut columns = rows.clone();
    rows.shuffle(rng);
    columns.shuffle(rng);
    (rows, columns)
}

pub fn generate(rng: &mut StdRng) -> String {
    match rng.random_range(0..3) {
        0 => {
            let group = GROUPS[rng.random_range(7..GROUPS.len())];
            let labels: Vec<_> = (0..group.size()).map(|x| group.element(x)).collect();
            let (rows, columns) = indices(rng, group.size());
            format!(
                "{}\n{}",
                group.name(),
                grid(&group.square(), &labels, &rows, &columns)
            )
        }
        1 => {
            let field = FIELDS[rng.random_range(0..FIELDS.len())];
            let a = rng.random_range(1..field.size());
            let mut b = rng.random_range(1..field.size() - 1);
            if b >= a {
                b += 1;
            }
            let labels: Vec<_> = (0..field.size()).map(|x| field.element(x)).collect();
            let (rows, columns) = indices(rng, field.size());
            let formula = |slope| {
                if slope == 1 {
                    "r+c".to_owned()
                } else {
                    format!("r+({})c", field.element(slope))
                }
            };
            format!(
                "{}\nL(r,c) = {}\n{}\nM(r,c) = {}\n{}\nL ⟂ M",
                field.model(),
                formula(a),
                grid(&field_square(field, a), &labels, &rows, &columns),
                formula(b),
                grid(&field_square(field, b), &labels, &rows, &columns)
            )
        }
        _ => {
            let groups: Vec<_> = GROUPS
                .into_iter()
                .filter_map(|group| {
                    let trades = intercalates(&group.square());
                    (!trades.is_empty()).then_some((group, trades))
                })
                .collect();
            let (group, trades) = &groups[rng.random_range(0..groups.len())];
            let selected = trades[rng.random_range(0..trades.len())];
            let mut square = group.square();
            let labels: Vec<_> = (0..group.size()).map(|x| group.element(x)).collect();
            let [r, s] = selected.rows;
            let [c, d] = selected.columns;
            let (a, b) = (&labels[square[r][c]], &labels[square[r][d]]);
            let context = format!(
                "{} with a Latin trade\nRows {}, {} and columns {}, {}\n[{a} {b} / {b} {a}] → [{b} {a} / {a} {b}]",
                group.name(),
                labels[r],
                labels[s],
                labels[c],
                labels[d]
            );
            trade(&mut square, selected);
            let (rows, columns) = indices(rng, group.size());
            format!("{context}\n{}", grid(&square, &labels, &rows, &columns))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    fn latin(square: &Square) -> bool {
        let n = square.len();
        (0..n).all(|i| {
            (0..n).map(|j| square[i][j]).collect::<HashSet<_>>().len() == n
                && (0..n).map(|j| square[j][i]).collect::<HashSet<_>>().len() == n
                && square[i].iter().all(|&x| x < n)
        })
    }

    #[test]
    fn all_six_field_models_satisfy_field_axioms() {
        for field in FIELDS {
            let q = field.size();
            if field.degree() > 1 {
                for x in 0..field.p {
                    assert_ne!(
                        field
                            .modulus
                            .iter()
                            .rev()
                            .fold(0, |value, &c| (value * x + c) % field.p),
                        0
                    );
                }
            }
            for x in 0..q {
                assert_eq!(field.add(x, 0), x);
                assert_eq!(field.mul(x, 1), x);
                assert!((0..q).any(|y| field.add(x, y) == 0));
                if x > 0 {
                    assert!((1..q).any(|y| field.mul(x, y) == 1));
                }
                for y in 0..q {
                    assert_eq!(field.add(x, y), field.add(y, x));
                    assert_eq!(field.mul(x, y), field.mul(y, x));
                    for z in 0..q {
                        assert_eq!(field.add(field.add(x, y), z), field.add(x, field.add(y, z)));
                        assert_eq!(field.mul(field.mul(x, y), z), field.mul(x, field.mul(y, z)));
                        assert_eq!(
                            field.mul(x, field.add(y, z)),
                            field.add(field.mul(x, y), field.mul(x, z))
                        );
                    }
                }
            }
        }
        assert_eq!(FIELDS[1].mul(2, 2), 3);
        assert_eq!(FIELDS[4].mul(4, 2), 3);
        assert_eq!(FIELDS[5].mul(3, 3), 2);
    }

    #[test]
    fn every_distinct_nonzero_slope_pair_is_orthogonal() {
        for field in FIELDS {
            let q = field.size();
            for a in 1..q {
                let left = field_square(field, a);
                assert!(latin(&left));
                for b in a + 1..q {
                    let right = field_square(field, b);
                    let pairs: HashSet<_> = (0..q)
                        .flat_map(|r| (0..q).map(move |c| (r, c)))
                        .map(|(r, c)| (left[r][c], right[r][c]))
                        .collect();
                    assert_eq!(pairs.len(), q * q, "{field:?}, slopes {a}, {b}");
                }
            }
        }
    }

    #[test]
    fn group_tables_and_every_intercalate_trade_are_latin() {
        for group in GROUPS {
            let square = group.square();
            assert!(latin(&square), "{group:?}");
            for x in 0..group.size() {
                for y in 0..group.size() {
                    for z in 0..group.size() {
                        assert_eq!(
                            group.product(group.product(x, y), z),
                            group.product(x, group.product(y, z))
                        );
                    }
                }
            }
            for selected in intercalates(&square) {
                let mut changed = square.clone();
                trade(&mut changed, selected);
                assert!(latin(&changed));
                assert_ne!(changed, square);
                trade(&mut changed, selected);
                assert_eq!(changed, square);
            }
        }
        assert_eq!(intercalates(&Group::Cyclic(4).square()).len(), 4);
        assert_eq!(intercalates(&Group::Product(&[2, 2]).square()).len(), 12);
        assert_eq!(intercalates(&Group::Cyclic(8).square()).len(), 16);
        assert_eq!(
            intercalates(&Group::Product(&[2, 2, 2]).square()).len(),
            112
        );
        assert!(intercalates(&Group::Cyclic(9).square()).is_empty());
        assert_ne!(Group::Dihedral.product(1, 4), Group::Dihedral.product(4, 1));
    }

    #[test]
    fn seeded_output_is_reproducible_and_varied() {
        let mut first = StdRng::seed_from_u64(20261023);
        let mut second = StdRng::seed_from_u64(20261023);
        let mut outputs = HashSet::new();
        for _ in 0..100 {
            let text = generate(&mut first);
            assert_eq!(text, generate(&mut second));
            assert!(!text.ends_with('\n'));
            outputs.insert(text);
        }
        assert!(outputs.len() > 95);
    }
}
