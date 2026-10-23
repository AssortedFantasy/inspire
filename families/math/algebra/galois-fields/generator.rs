use rand::{Rng, rngs::StdRng};

#[derive(Clone, Copy, Debug)]
struct Field {
    p: usize,
    modulus: &'static [usize],
}

const FIELDS: [Field; 5] = [
    Field {
        p: 2,
        modulus: &[1, 1, 0, 1],
    },
    Field {
        p: 2,
        modulus: &[1, 1, 0, 0, 1],
    },
    Field {
        p: 2,
        modulus: &[1, 1, 0, 0, 0, 0, 1],
    },
    Field {
        p: 3,
        modulus: &[1, 2, 0, 1],
    },
    Field {
        p: 5,
        modulus: &[1, 1, 0, 1],
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
                let c = x % self.p;
                x /= self.p;
                c
            })
            .collect()
    }

    fn encode(self, coefficients: &[usize]) -> usize {
        coefficients.iter().rev().fold(0, |x, &c| x * self.p + c)
    }

    fn add(self, x: usize, y: usize) -> usize {
        let coefficients: Vec<_> = self
            .digits(x)
            .iter()
            .zip(self.digits(y))
            .map(|(&a, b)| (a + b) % self.p)
            .collect();
        self.encode(&coefficients)
    }

    fn neg(self, x: usize) -> usize {
        self.encode(
            &self
                .digits(x)
                .iter()
                .map(|&a| (self.p - a) % self.p)
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
            let leading = product[k];
            for j in 0..n {
                product[k - n + j] =
                    (product[k - n + j] + self.p - leading * self.modulus[j] % self.p) % self.p;
            }
        }
        self.encode(&product[..n])
    }

    fn pow(self, mut x: usize, mut exponent: usize) -> usize {
        let mut result = 1;
        while exponent > 0 {
            if exponent % 2 == 1 {
                result = self.mul(result, x);
            }
            x = self.mul(x, x);
            exponent /= 2;
        }
        result
    }

    fn orbit(self, x: usize) -> Vec<usize> {
        let mut orbit = vec![x];
        let mut next = self.pow(x, self.p);
        for _ in 1..self.degree() {
            if next == x {
                break;
            }
            orbit.push(next);
            next = self.pow(next, self.p);
        }
        orbit
    }

    fn order(self, x: usize) -> usize {
        let mut product = 1;
        for exponent in 1..self.size() {
            product = self.mul(product, x);
            if product == 1 {
                return exponent;
            }
        }
        unreachable!("nonzero field element")
    }

    fn trace_norm(self, x: usize) -> (usize, usize) {
        let (mut trace, mut norm, mut conjugate) = (0, 1, x);
        for _ in 0..self.degree() {
            trace = self.add(trace, conjugate);
            norm = self.mul(norm, conjugate);
            conjugate = self.pow(conjugate, self.p);
        }
        (trace, norm)
    }

    fn minimal(self, x: usize) -> Vec<usize> {
        let mut polynomial = vec![1];
        for conjugate in self.orbit(x) {
            let mut next = vec![0; polynomial.len() + 1];
            for (i, &coefficient) in polynomial.iter().enumerate() {
                next[i] = self.add(next[i], self.mul(self.neg(conjugate), coefficient));
                next[i + 1] = self.add(next[i + 1], coefficient);
            }
            polynomial = next;
        }
        polynomial
    }

    fn element(self, x: usize) -> String {
        polynomial(&self.digits(x), "α")
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
        terms.join(" + ")
    }
}

#[derive(Clone, Copy, Debug)]
enum Regime {
    Primitive,
    Nonprimitive,
    Subfield,
    TraceZero,
    NormOne,
}

const REGIMES: [Regime; 5] = [
    Regime::Primitive,
    Regime::Nonprimitive,
    Regime::Subfield,
    Regime::TraceZero,
    Regime::NormOne,
];

fn candidates(field: Field, regime: Regime) -> Vec<usize> {
    (1..field.size())
        .filter(|&x| {
            let degree = field.orbit(x).len();
            match regime {
                Regime::Primitive => degree == field.degree() && field.order(x) == field.size() - 1,
                Regime::Nonprimitive => {
                    degree == field.degree() && field.order(x) < field.size() - 1
                }
                Regime::Subfield => degree > 1 && degree < field.degree(),
                Regime::TraceZero => degree == field.degree() && field.trace_norm(x).0 == 0,
                Regime::NormOne => {
                    field.p > 2 && degree == field.degree() && field.trace_norm(x).1 == 1
                }
            }
        })
        .collect()
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = REGIMES[rng.random_range(0..REGIMES.len())];
    let models: Vec<_> = FIELDS
        .into_iter()
        .filter_map(|field| {
            let elements = candidates(field, regime);
            (!elements.is_empty()).then_some((field, elements))
        })
        .collect();
    let (field, elements) = &models[rng.random_range(0..models.len())];
    let field = *field;
    let x = elements[rng.random_range(0..elements.len())];
    let p = field.p;
    let n = field.degree();
    let q = field.size();
    let model = format!("K = F_{p}[t]/({}), α = [t]", polynomial(field.modulus, "t"));
    let element = field.element(x);
    let minimal = polynomial(&field.minimal(x), "t");
    let order = field.order(x);
    let (trace, norm) = field.trace_norm(x);
    if matches!(regime, Regime::Subfield) {
        let degree = field.orbit(x).len();
        let subfield_size = p.pow(degree as u32);
        return format!(
            "{model}\nβ = {element}\nF_{subfield_size} = {{z ∈ K : z^{subfield_size} = z}}\n[F_{p}(β) : F_{p}] = {degree} < {n}\nmin_{{F_{p}}}(β) = {minimal}\nTr_{{K/F_{p}}}(β) = {trace}\nN_{{K/F_{p}}}(β) = {norm}"
        );
    }
    match rng.random_range(0..3) {
        0 => {
            let kind = if order == q - 1 {
                "primitive"
            } else {
                "nonprimitive"
            };
            let conjugates = field
                .orbit(x)
                .into_iter()
                .map(|z| field.element(z))
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "{model}\nβ = {element}\nmin_{{F_{p}}}(β) = {minimal} ({kind})\nFrobenius conjugates = {{{conjugates}}}\nord(β) = {order} in K^× of order {}",
                q - 1
            )
        }
        1 => format!(
            "{model}\nβ = {element}\nTr_{{K/F_{p}}}(β) = {trace}\nN_{{K/F_{p}}}(β) = {norm}\nmin_{{F_{p}}}(β) = {minimal}"
        ),
        _ => {
            let mut orbit = field
                .orbit(x)
                .into_iter()
                .map(|z| field.element(z))
                .collect::<Vec<_>>();
            orbit.push(element.clone());
            format!(
                "{model}\nβ = {element}\nFrob_{p}: {}\nmin_{{F_{p}}}(β) = {minimal}\nord(β) = {order}",
                orbit.join(" → ")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn remainder(mut dividend: Vec<usize>, divisor: &[usize], p: usize) -> Vec<usize> {
        for k in (divisor.len() - 1..dividend.len()).rev() {
            let c = dividend[k];
            for j in 0..divisor.len() {
                let index = k + j + 1 - divisor.len();
                dividend[index] = (dividend[index] + p - c * divisor[j] % p) % p;
            }
        }
        dividend.truncate(divisor.len() - 1);
        dividend
    }

    fn irreducible(polynomial: &[usize], p: usize) -> bool {
        for degree in 1..=(polynomial.len() - 1) / 2 {
            for mut code in 0..p.pow(degree as u32) {
                let mut divisor = vec![];
                for _ in 0..degree {
                    divisor.push(code % p);
                    code /= p;
                }
                divisor.push(1);
                if remainder(polynomial.to_vec(), &divisor, p)
                    .iter()
                    .all(|&x| x == 0)
                {
                    return false;
                }
            }
        }
        true
    }

    fn determinant(mut matrix: Vec<Vec<usize>>, p: usize) -> usize {
        let mut result = 1;
        for column in 0..matrix.len() {
            let Some(pivot) = (column..matrix.len()).find(|&row| matrix[row][column] != 0) else {
                return 0;
            };
            if pivot != column {
                matrix.swap(pivot, column);
                result = (p - result) % p;
            }
            let value = matrix[column][column];
            result = result * value % p;
            let inverse = (1..p).find(|&a| a * value % p == 1).unwrap();
            let pivot_row = matrix[column].clone();
            for values in matrix.iter_mut().skip(column + 1) {
                let scale = values[column] * inverse % p;
                for (value, &pivot) in values.iter_mut().zip(&pivot_row).skip(column) {
                    *value = (*value + p - scale * pivot % p) % p;
                }
            }
        }
        result
    }

    #[test]
    fn defining_polynomials_and_minimal_polynomials_are_irreducible() {
        for field in FIELDS {
            assert!(irreducible(field.modulus, field.p), "{field:?}");
            for x in 0..field.size() {
                let minimal = field.minimal(x);
                assert!(minimal.iter().all(|&c| c < field.p));
                assert!(irreducible(&minimal, field.p));
                let value = minimal
                    .iter()
                    .rev()
                    .fold(0, |value, &c| field.add(field.mul(value, x), c));
                assert_eq!(value, 0);
                assert_eq!(field.degree() % (minimal.len() - 1), 0);
                assert_eq!(field.pow(x, field.size()), x);
                if x > 0 {
                    let order = field.order(x);
                    assert_eq!((field.size() - 1) % order, 0);
                    assert_eq!(field.pow(x, order), 1);
                    for divisor in 1..order {
                        if order % divisor == 0 {
                            assert_ne!(field.pow(x, divisor), 1);
                        }
                    }
                    assert_eq!(
                        field.pow(x, field.size() - 2),
                        (1..field.size()).find(|&y| field.mul(x, y) == 1).unwrap()
                    );
                }
            }
        }
    }

    #[test]
    fn trace_and_norm_match_multiplication_matrix() {
        for field in FIELDS {
            for x in 0..field.size() {
                let n = field.degree();
                let columns = (0..n)
                    .map(|column| field.digits(field.mul(x, field.p.pow(column as u32))))
                    .collect::<Vec<_>>();
                let matrix = (0..n)
                    .map(|row| columns.iter().map(|column| column[row]).collect::<Vec<_>>())
                    .collect::<Vec<_>>();
                let trace = (0..n).map(|i| matrix[i][i]).sum::<usize>() % field.p;
                assert_eq!(field.trace_norm(x), (trace, determinant(matrix, field.p)));
            }
        }
    }

    #[test]
    fn field_operations_and_frobenius_are_exact() {
        for field in FIELDS {
            for x in 0..field.size() {
                for y in 0..field.size() {
                    assert_eq!(field.add(x, y), field.add(y, x));
                    assert_eq!(field.mul(x, y), field.mul(y, x));
                    assert_eq!(
                        field.pow(field.add(x, y), field.p),
                        field.add(field.pow(x, field.p), field.pow(y, field.p))
                    );
                    for z in [0, 1, field.p, field.size() - 1] {
                        assert_eq!(
                            field.mul(x, field.add(y, z)),
                            field.add(field.mul(x, y), field.mul(x, z))
                        );
                        assert_eq!(field.mul(field.mul(x, y), z), field.mul(x, field.mul(y, z)));
                    }
                }
            }
        }
    }

    #[test]
    fn all_structural_regimes_are_populated() {
        for regime in REGIMES {
            assert!(
                FIELDS
                    .into_iter()
                    .any(|field| !candidates(field, regime).is_empty()),
                "{regime:?}"
            );
        }
        for field in FIELDS {
            for divisor in 1..=field.degree() {
                if field.degree() % divisor == 0 {
                    let size = field.p.pow(divisor as u32);
                    assert_eq!(
                        (0..field.size())
                            .filter(|&x| field.pow(x, size) == x)
                            .count(),
                        size
                    );
                }
            }
        }
        for (field, primitive, nonprimitive) in [
            (FIELDS[0], 6, 0),
            (FIELDS[1], 8, 4),
            (FIELDS[2], 36, 18),
            (FIELDS[3], 12, 12),
            (FIELDS[4], 60, 60),
        ] {
            assert_eq!(candidates(field, Regime::Primitive).len(), primitive);
            assert_eq!(candidates(field, Regime::Nonprimitive).len(), nonprimitive);
        }
        for field in FIELDS {
            let elements = candidates(field, Regime::NormOne);
            if field.p == 2 {
                assert!(elements.is_empty());
            } else {
                assert!(!elements.is_empty());
                assert!(
                    elements.len()
                        < candidates(field, Regime::Nonprimitive).len()
                            + candidates(field, Regime::Primitive).len()
                );
            }
        }
        let mut a = StdRng::seed_from_u64(1023);
        let mut b = StdRng::seed_from_u64(1023);
        for _ in 0..80 {
            assert_eq!(generate(&mut a), generate(&mut b));
        }
    }
}
