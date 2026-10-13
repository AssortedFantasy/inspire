use rand::{Rng, rngs::StdRng, seq::SliceRandom};

pub fn generate(rng: &mut StdRng) -> String {
    let group = match rng.random_range(0..7) {
        0 => Group::Cyclic(rng.random_range(2..=48)),
        1 => Group::Product(rng.random_range(2..=12), rng.random_range(2..=12)),
        2 => Group::Dihedral(rng.random_range(3..=16)),
        3 => Group::Quaternion,
        4 => Group::Symmetric(rng.random_range(3..=6)),
        5 => Group::Affine([3, 5, 7, 11][rng.random_range(0..4)]),
        _ => Group::Heisenberg([2, 3, 5, 7][rng.random_range(0..4)]),
    };
    let x = group.sample(rng);
    let y = group.sample(rng);
    let operator = if matches!(group, Group::Cyclic(_) | Group::Product(..)) {
        "+"
    } else {
        "*"
    };
    format!(
        "{}\nExample: {} {operator} {} = {}.",
        group.description(),
        group.display(&x),
        group.display(&y),
        group.display(&group.multiply(&x, &y))
    )
}

#[derive(Clone, Copy, Debug)]
enum Group {
    Cyclic(usize),
    Product(usize, usize),
    Dihedral(usize),
    Quaternion,
    Symmetric(usize),
    Affine(usize),
    Heisenberg(usize),
}

impl Group {
    fn order(self) -> usize {
        match self {
            Self::Cyclic(n) => n,
            Self::Product(m, n) => m * n,
            Self::Dihedral(n) => 2 * n,
            Self::Quaternion => 8,
            Self::Symmetric(n) => (1..=n).product(),
            Self::Affine(p) => p * (p - 1),
            Self::Heisenberg(p) => p * p * p,
        }
    }

    fn description(self) -> String {
        let construction = match self {
            Self::Cyclic(n) => format!("C_{n}: residues 0..{}, with addition modulo {n}. Identity: 0. Abelian.", n - 1),
            Self::Product(m, n) => format!("C_{m} x C_{n}: pairs (a,b), a mod {m}, b mod {n}; componentwise addition. Identity: (0,0). Abelian."),
            Self::Dihedral(n) => format!("D_{n}: symmetries of a regular {n}-gon. Elements (a,b) represent r^a s^b, a mod {n}, b mod 2. Product: (a,b)*(c,d)=(a+(-1)^b*c mod {n}, b+d mod 2). Identity: (0,0). Nonabelian; D_{n} has order {}, not {n}.", 2 * n),
            Self::Quaternion => "Q_8: {1,-1,i,-i,j,-j,k,-k}, under quaternion multiplication. i^2=j^2=k^2=ijk=-1; ij=k, jk=i, ki=j, reversing these products changes the sign. Identity: 1. Nonabelian.".into(),
            Self::Symmetric(n) => format!("S_{n}: all permutations of 1..={n}, under composition p*q=p after q (apply q first). [p(1),...,p({n})] is one-line notation. Identity: [1,...,{n}]. Nonabelian."),
            Self::Affine(p) => format!("AGL(1,{p}): maps t -> a*t+b on F_{p}, represented by (a,b), a in 1..{}, b in 0..{}. Product is composition: (a,b)*(c,d)=(a*c, a*d+b), modulo {p}. Identity: (1,0). Nonabelian.", p - 1, p - 1),
            Self::Heisenberg(p) => format!("H(F_{p}): upper unitriangular 3x3 matrices [[1,a,c],[0,1,b],[0,0,1]], represented by (a,b,c) in F_{p}^3. Matrix product: (a,b,c)*(u,v,w)=(a+u,b+v,c+w+a*v), modulo {p}. Identity: (0,0,0). Nonabelian."),
        };
        format!("{construction}\nOrder: {}.", self.order())
    }

    fn sample(self, rng: &mut StdRng) -> Vec<usize> {
        match self {
            Self::Cyclic(n) => vec![rng.random_range(0..n)],
            Self::Product(m, n) => vec![rng.random_range(0..m), rng.random_range(0..n)],
            Self::Dihedral(n) => vec![rng.random_range(0..n), rng.random_range(0..2)],
            Self::Quaternion => vec![rng.random_range(0..8)],
            Self::Symmetric(n) => {
                let mut p: Vec<_> = (0..n).collect();
                p.shuffle(rng);
                p
            }
            Self::Affine(p) => vec![rng.random_range(1..p), rng.random_range(0..p)],
            Self::Heisenberg(p) => (0..3).map(|_| rng.random_range(0..p)).collect(),
        }
    }

    fn multiply(self, x: &[usize], y: &[usize]) -> Vec<usize> {
        match self {
            Self::Cyclic(n) => vec![(x[0] + y[0]) % n],
            Self::Product(m, n) => vec![(x[0] + y[0]) % m, (x[1] + y[1]) % n],
            Self::Dihedral(n) => vec![
                (x[0] + if x[1] == 0 { y[0] } else { n - y[0] }) % n,
                (x[1] + y[1]) % 2,
            ],
            Self::Quaternion => {
                // Basis order 1,i,j,k. Signed entries encode sign and basis.
                const TABLE: [[(usize, usize); 4]; 4] = [
                    [(0, 0), (0, 1), (0, 2), (0, 3)],
                    [(0, 1), (1, 0), (0, 3), (1, 2)],
                    [(0, 2), (1, 3), (1, 0), (0, 1)],
                    [(0, 3), (0, 2), (1, 1), (1, 0)],
                ];
                let (sign, basis) = TABLE[x[0] % 4][y[0] % 4];
                vec![4 * ((x[0] / 4 + y[0] / 4 + sign) % 2) + basis]
            }
            Self::Symmetric(n) => (0..n).map(|i| x[y[i]]).collect(),
            Self::Affine(p) => vec![x[0] * y[0] % p, (x[0] * y[1] + x[1]) % p],
            Self::Heisenberg(p) => vec![
                (x[0] + y[0]) % p,
                (x[1] + y[1]) % p,
                (x[2] + y[2] + x[0] * y[1]) % p,
            ],
        }
    }

    fn display(self, x: &[usize]) -> String {
        match self {
            Self::Quaternion => ["1", "i", "j", "k", "-1", "-i", "-j", "-k"][x[0]].into(),
            Self::Cyclic(_) => x[0].to_string(),
            Self::Symmetric(_) => format!(
                "[{}]",
                x.iter()
                    .map(|i| (i + 1).to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            _ => format!(
                "({})",
                x.iter().map(usize::to_string).collect::<Vec<_>>().join(",")
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn elements(g: Group) -> Vec<Vec<usize>> {
        let (dimensions, base) = match g {
            Group::Cyclic(n) => (1, n),
            Group::Product(m, n) => {
                return (0..m)
                    .flat_map(|a| (0..n).map(move |b| vec![a, b]))
                    .collect();
            }
            Group::Dihedral(n) => {
                return (0..n)
                    .flat_map(|a| (0..2).map(move |b| vec![a, b]))
                    .collect();
            }
            Group::Quaternion => (1, 8),
            Group::Affine(p) => {
                return (1..p)
                    .flat_map(|a| (0..p).map(move |b| vec![a, b]))
                    .collect();
            }
            Group::Heisenberg(p) => (3, p),
            Group::Symmetric(n) => {
                fn permutations(prefix: Vec<usize>, n: usize, out: &mut Vec<Vec<usize>>) {
                    if prefix.len() == n {
                        out.push(prefix);
                        return;
                    }
                    for a in 0..n {
                        if !prefix.contains(&a) {
                            let mut next = prefix.clone();
                            next.push(a);
                            permutations(next, n, out);
                        }
                    }
                }
                let mut out = Vec::new();
                permutations(Vec::new(), n, &mut out);
                return out;
            }
        };
        (0..base.pow(dimensions))
            .map(|mut k| {
                (0..dimensions)
                    .map(|_| {
                        let a = k % base;
                        k /= base;
                        a
                    })
                    .collect()
            })
            .collect()
    }

    #[test]
    fn exhaustive_small_instances_satisfy_all_group_axioms() {
        for (g, identity, abelian) in [
            (Group::Cyclic(7), vec![0], true),
            (Group::Product(3, 4), vec![0, 0], true),
            (Group::Dihedral(5), vec![0, 0], false),
            (Group::Quaternion, vec![0], false),
            (Group::Symmetric(3), vec![0, 1, 2], false),
            (Group::Affine(5), vec![1, 0], false),
            (Group::Heisenberg(3), vec![0, 0, 0], false),
        ] {
            let all = elements(g);
            assert_eq!(all.len(), g.order());
            let mut commutes = true;
            for x in &all {
                assert_eq!(g.multiply(x, &identity), *x);
                assert_eq!(g.multiply(&identity, x), *x);
                assert!(
                    all.iter()
                        .any(|y| g.multiply(x, y) == identity && g.multiply(y, x) == identity)
                );
                for y in &all {
                    let xy = g.multiply(x, y);
                    assert!(all.contains(&xy), "closure: {g:?}");
                    commutes &= xy == g.multiply(y, x);
                    for z in &all {
                        assert_eq!(
                            g.multiply(&xy, z),
                            g.multiply(x, &g.multiply(y, z)),
                            "associativity: {g:?}"
                        );
                    }
                }
            }
            assert_eq!(commutes, abelian, "{g:?}");
        }
    }

    #[test]
    fn quaternion_and_composition_conventions_are_explicit() {
        let q = Group::Quaternion;
        for i in 1..=3 {
            assert_eq!(q.multiply(&[i], &[i]), vec![4]);
        }
        assert_eq!(q.multiply(&q.multiply(&[1], &[2]), &[3]), vec![4]);
        assert_eq!(
            Group::Symmetric(3).multiply(&[1, 0, 2], &[0, 2, 1]),
            vec![1, 2, 0]
        );
        assert_eq!(
            Group::Heisenberg(5).multiply(&[1, 0, 0], &[0, 1, 0]),
            vec![1, 1, 1]
        );
    }

    #[test]
    fn seeded_samples_are_reproducible_and_cover_constructions() {
        let mut a = StdRng::seed_from_u64(123);
        let mut b = StdRng::seed_from_u64(123);
        let mut seen = [false; 7];
        for _ in 0..300 {
            let output = generate(&mut a);
            assert_eq!(output, generate(&mut b));
            assert!(!output.ends_with('\n'));
            for (i, prefix) in ["C_", "C_", "D_", "Q_", "S_", "AGL", "H("]
                .iter()
                .enumerate()
            {
                if output.starts_with(prefix)
                    && (i != 0 || !output.lines().next().unwrap().contains(" x "))
                    && (i != 1 || output.lines().next().unwrap().contains(" x "))
                {
                    seen[i] = true;
                }
            }
        }
        assert!(seen.into_iter().all(|x| x));
    }
}
