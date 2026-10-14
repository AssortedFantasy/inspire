use rand::{Rng, rngs::StdRng, seq::SliceRandom};

#[derive(Clone, Debug)]
struct Factor {
    u: i64,
    v: i64,
    multiplicity: usize,
}

#[derive(Debug)]
struct Polynomial {
    content: i64,
    factors: Vec<Factor>,
    coefficients: Vec<i64>,
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.abs()
}

fn expand(content: i64, factors: &[Factor]) -> Vec<i64> {
    let mut coefficients = vec![content];
    for f in factors {
        for _ in 0..f.multiplicity {
            let mut next = vec![0; coefficients.len() + 1];
            for (i, c) in coefficients.iter().enumerate() {
                next[i] -= f.v * c;
                next[i + 1] += f.u * c;
            }
            coefficients = next;
        }
    }
    coefficients
}

fn sample(rng: &mut StdRng) -> Polynomial {
    for _ in 0..64 {
        let degree: usize = rng.random_range(2..=6);
        let distinct = rng.random_range(2.max(degree.div_ceil(3))..=degree);
        let mut roots: Vec<_> = (1..=3)
            .flat_map(|u| (-5..=5).filter_map(move |v| (gcd(u, v) == 1).then_some((u, v))))
            .collect();
        roots.shuffle(rng);
        let mut factors: Vec<_> = roots[..distinct]
            .iter()
            .map(|&(u, v)| Factor {
                u,
                v,
                multiplicity: 1,
            })
            .collect();
        for _ in distinct..degree {
            let available: Vec<_> = (0..distinct)
                .filter(|&i| factors[i].multiplicity < 3)
                .collect();
            let i = available[rng.random_range(0..available.len())];
            factors[i].multiplicity += 1;
        }
        factors.sort_by(|a, b| (a.v * b.u).cmp(&(b.v * a.u)));
        let content = rng.random_range(1..=4) * if rng.random_bool(0.5) { 1 } else { -1 };
        let coefficients = expand(content, &factors);
        if coefficients.iter().all(|c| c.abs() <= 1500) {
            return Polynomial {
                content,
                factors,
                coefficients,
            };
        }
    }
    let factors = vec![
        Factor {
            u: 1,
            v: -1,
            multiplicity: 2,
        },
        Factor {
            u: 1,
            v: 2,
            multiplicity: 1,
        },
    ];
    Polynomial {
        content: 1,
        coefficients: expand(1, &factors),
        factors,
    }
}

fn polynomial_text(coefficients: &[i64]) -> String {
    let mut output = String::new();
    for (power, &coefficient) in coefficients.iter().enumerate().rev() {
        if coefficient == 0 {
            continue;
        }
        if output.is_empty() {
            if coefficient < 0 {
                output.push('-');
            }
        } else {
            output.push_str(if coefficient < 0 { " - " } else { " + " });
        }
        if power == 0 || coefficient.abs() != 1 {
            output.push_str(&coefficient.abs().to_string());
        }
        if power > 0 {
            output.push('x');
        }
        if power > 1 {
            output.push_str(&format!("^{power}"));
        }
    }
    output
}

fn render(p: &Polynomial) -> String {
    let mut factorization = match p.content {
        1 => String::new(),
        -1 => "-".into(),
        c => c.to_string(),
    };
    for f in &p.factors {
        let linear = polynomial_text(&[-f.v, f.u]);
        factorization.push_str(&format!("({linear})"));
        if f.multiplicity > 1 {
            factorization.push_str(&format!("^{}", f.multiplicity));
        }
    }
    let roots: Vec<_> = p
        .factors
        .iter()
        .map(|f| {
            let root = if f.u == 1 {
                f.v.to_string()
            } else {
                format!("{}/{}", f.v, f.u)
            };
            format!("{root} (multiplicity {})", f.multiplicity)
        })
        .collect();
    format!(
        "p(x) = {}\nComplete factorization over Q: p(x) = {factorization}\nRational roots: {}",
        polynomial_text(&p.coefficients),
        roots.join(", ")
    )
}

pub fn generate(rng: &mut StdRng) -> String {
    render(&sample(rng))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    // Evaluate u^degree p(v/u) using integer arithmetic.
    fn homogeneous(coefficients: &[i64], v: i64, u: i64) -> i64 {
        let degree = coefficients.len() - 1;
        coefficients
            .iter()
            .enumerate()
            .map(|(i, c)| c * v.pow(i as u32) * u.pow((degree - i) as u32))
            .sum()
    }

    #[test]
    fn exact_factorization_and_multiplicities() {
        for seed in 0..1000 {
            let p = sample(&mut StdRng::seed_from_u64(seed));
            let factors: Vec<_> = p
                .factors
                .iter()
                .flat_map(|f| std::iter::repeat_n((f.u, f.v), f.multiplicity))
                .collect();
            let degree = factors.len();
            assert!((2..=6).contains(&degree));
            assert!(p.factors.len() >= 2);
            assert!(p.coefficients.iter().all(|c| c.abs() <= 1500));
            assert_eq!(
                p.coefficients.iter().fold(0, |g, &c| gcd(g, c)),
                p.content.abs()
            );
            // Independently expand by choosing x or the constant from every factor.
            let mut expected = vec![0; degree + 1];
            for mask in 0_usize..(1 << degree) {
                let mut coefficient = p.content;
                for (i, &(u, v)) in factors.iter().enumerate() {
                    coefficient *= if mask & (1 << i) == 0 { -v } else { u };
                }
                expected[mask.count_ones() as usize] += coefficient;
            }
            assert_eq!(p.coefficients, expected);
            for f in &p.factors {
                assert!(f.u > 0 && gcd(f.u, f.v) == 1);
                assert!((1..=3).contains(&f.multiplicity));
                let mut derivative = p.coefficients.clone();
                for order in 0..=f.multiplicity {
                    let value = homogeneous(&derivative, f.v, f.u);
                    assert_eq!(
                        value == 0,
                        order < f.multiplicity,
                        "seed {seed}, derivative {order}"
                    );
                    derivative = derivative
                        .iter()
                        .enumerate()
                        .skip(1)
                        .map(|(i, &c)| c * i as i64)
                        .collect();
                }
            }
            for pair in p.factors.windows(2) {
                assert!(pair[0].v * pair[1].u < pair[1].v * pair[0].u);
            }
        }
    }

    #[test]
    fn deterministic_and_diverse() {
        let mut outputs = HashSet::new();
        let mut degrees = HashSet::new();
        let mut multiplicities = HashSet::new();
        let mut denominators = HashSet::new();
        let mut contents = HashSet::new();
        for seed in 0..500 {
            let p = sample(&mut StdRng::seed_from_u64(seed));
            let output = render(&p);
            assert_eq!(output, generate(&mut StdRng::seed_from_u64(seed)));
            assert!(!output.ends_with('\n'));
            outputs.insert(output);
            degrees.insert(p.coefficients.len() - 1);
            contents.insert(p.content);
            for f in p.factors {
                multiplicities.insert(f.multiplicity);
                denominators.insert(f.u);
            }
        }
        assert!(outputs.len() > 450);
        assert_eq!(degrees.len(), 5);
        assert_eq!(multiplicities.len(), 3);
        assert_eq!(denominators.len(), 3);
        assert_eq!(contents.len(), 8);
    }

    #[test]
    fn readable_signs() {
        assert_eq!(polynomial_text(&[-1, -1, 1]), "x^2 - x - 1");
        assert_eq!(polynomial_text(&[0, 1]), "x");
        assert_eq!(polynomial_text(&[2, 3]), "3x + 2");
    }
}
