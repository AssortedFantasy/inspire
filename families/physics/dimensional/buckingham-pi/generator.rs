use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

include!("../../number.rs");

type Dimension = [i64; 5];

struct Quantity {
    symbol: &'static str,
    name: &'static str,
    dimension: Dimension,
}

fn q(symbol: &'static str, name: &'static str, dimension: Dimension) -> Quantity {
    Quantity {
        symbol,
        name,
        dimension,
    }
}

struct Problem {
    title: &'static str,
    quantities: Vec<Quantity>,
    repeating: Vec<Vec<usize>>,
}

fn problem(kind: usize) -> Problem {
    let force = [1, 1, -2, 0, 0];
    let density = [1, -3, 0, 0, 0];
    let speed = [0, 1, -1, 0, 0];
    let length = [0, 1, 0, 0, 0];
    let viscosity = [1, -1, -1, 0, 0];
    let acceleration = [0, 1, -2, 0, 0];
    let frequency = [0, 0, -1, 0, 0];
    let time = [0, 0, 1, 0, 0];
    let temperature = [0, 0, 0, 1, 0];
    let scalar = [0; 5];
    let (title, quantities, repeating) = match kind {
        0..=2 => {
            let mut vars = vec![
                q("F", "drag magnitude", force),
                q("rho", "fluid density", density),
                q("U", "relative speed", speed),
                q("D", "diameter", length),
                q("mu", "dynamic viscosity", viscosity),
            ];
            let title = match kind {
                1 => {
                    vars.push(q("a", "sound speed", speed));
                    "Compressible flow past a sphere"
                }
                2 => {
                    vars.push(q("sigma", "surface tension", [1, 0, -2, 0, 0]));
                    "Deformable drop in an incompressible fluid"
                }
                _ => "Rigid sphere in an incompressible fluid",
            };
            (
                title,
                vars,
                vec![vec![1, 2, 3], vec![1, 4, 3], vec![4, 2, 3]],
            )
        }
        3 => (
            "Finite-amplitude simple pendulum",
            vec![
                q("tau", "period", time),
                q("ell", "length", length),
                q("g", "gravity", acceleration),
                q("theta", "angular amplitude", scalar),
            ],
            vec![vec![1, 2]],
        ),
        4 => (
            "Harmonically forced damped oscillator",
            vec![
                q("A", "steady displacement amplitude", length),
                q("m", "mass", [1, 0, 0, 0, 0]),
                q("omega", "forcing angular frequency", frequency),
                q("F", "force amplitude", force),
                q("k", "spring stiffness", [1, 0, -2, 0, 0]),
                q("c", "viscous damping", [1, 0, -1, 0, 0]),
            ],
            vec![vec![1, 2, 3], vec![1, 4, 3]],
        ),
        5 => (
            "Pressure drop in a rough circular pipe",
            vec![
                q("dp", "pressure-drop magnitude", [1, -1, -2, 0, 0]),
                q("rho", "density", density),
                q("U", "mean speed", speed),
                q("D", "diameter", length),
                q("mu", "dynamic viscosity", viscosity),
                q("ell", "pipe length", length),
                q("e", "roughness height", length),
            ],
            vec![vec![1, 2, 3], vec![1, 4, 3]],
        ),
        6 => (
            "Small-amplitude capillary-gravity waves at finite depth",
            vec![
                q("omega", "angular frequency", frequency),
                q("rho", "liquid density", density),
                q("g", "gravity", acceleration),
                q("k", "wavenumber", [0, -1, 0, 0, 0]),
                q("h", "water depth", length),
                q("sigma", "surface tension", [1, 0, -2, 0, 0]),
            ],
            vec![vec![1, 2, 3], vec![1, 5, 3]],
        ),
        7 => (
            "Small-amplitude deep-water gravity waves",
            vec![
                q("omega", "angular frequency", frequency),
                q("g", "gravity", acceleration),
                q("k", "wavenumber", [0, -1, 0, 0, 0]),
            ],
            vec![vec![1, 2]],
        ),
        8 => (
            "Cantilever deflection under a point load",
            vec![
                q("delta", "tip-deflection magnitude", length),
                q("F", "load magnitude", force),
                q("L", "beam length", length),
                q("E", "Young modulus", [1, -1, -2, 0, 0]),
                q("I", "second moment of area", [0, 4, 0, 0, 0]),
                q("a", "load distance from clamp", length),
            ],
            vec![vec![1, 2], vec![3, 2]],
        ),
        9 => (
            "One-dimensional slab cooling",
            vec![
                q("dT", "local temperature excess", temperature),
                q("dT0", "initial temperature excess", temperature),
                q("alpha", "thermal diffusivity", [0, 2, -1, 0, 0]),
                q("t", "elapsed time", time),
                q("x", "depth", length),
                q("L", "slab thickness", length),
            ],
            vec![vec![1, 2, 5]],
        ),
        10 => (
            "Forced convection over a fixed-shape body",
            vec![
                q("h", "heat-transfer coefficient", [1, 0, -3, -1, 0]),
                q("k", "thermal conductivity", [1, 1, -3, -1, 0]),
                q("L", "body length", length),
                q("rho", "fluid density", density),
                q("U", "flow speed", speed),
                q("mu", "dynamic viscosity", viscosity),
                q("cp", "specific heat capacity", [0, 2, -2, -1, 0]),
            ],
            vec![vec![1, 2, 3, 4], vec![1, 2, 3, 5]],
        ),
        11 => (
            "Gray-surface radiation to an isothermal enclosure",
            vec![
                q("q", "net outward heat-flux magnitude", [1, 0, -3, 0, 0]),
                q("sigma", "Stefan-Boltzmann constant", [1, 0, -3, -4, 0]),
                q("T1", "surface absolute temperature", temperature),
                q("T2", "enclosure absolute temperature", temperature),
                q("eps", "emissivity", scalar),
            ],
            vec![vec![1, 2]],
        ),
        12 => (
            "RC discharge",
            vec![
                q("V", "voltage magnitude", [1, 2, -3, 0, -1]),
                q("V0", "initial voltage magnitude", [1, 2, -3, 0, -1]),
                q("R", "resistance", [1, 2, -3, 0, -2]),
                q("C", "capacitance", [-1, -2, 4, 0, 2]),
                q("t", "elapsed time", time),
            ],
            vec![vec![1, 2, 3], vec![1, 2, 4]],
        ),
        13 => (
            "Propeller thrust near a free surface, fixed pitch and shape",
            vec![
                q("F", "thrust magnitude", force),
                q("rho", "fluid density", density),
                q("n", "rotation frequency", frequency),
                q("D", "propeller diameter", length),
                q("U", "advance speed", speed),
                q("mu", "dynamic viscosity", viscosity),
                q("g", "gravity", acceleration),
                q("h", "shaft depth", length),
            ],
            vec![vec![1, 2, 3], vec![1, 4, 3]],
        ),
        14 => (
            "Strong spherical blast in a uniform medium",
            vec![
                q("r", "shock radius", length),
                q("E", "released energy", [1, 2, -2, 0, 0]),
                q("rho", "ambient density", density),
                q("t", "elapsed time", time),
            ],
            vec![vec![1, 2, 3]],
        ),
        15 => (
            "Capillary rise in a circular wetting tube",
            vec![
                q("h", "rise height", length),
                q("rho", "liquid density", density),
                q("g", "gravity", acceleration),
                q("a", "tube radius", length),
                q("sigma", "surface tension", [1, 0, -2, 0, 0]),
                q("theta", "contact angle", scalar),
            ],
            vec![vec![1, 2, 3], vec![1, 4, 3]],
        ),
        16 => (
            "Oscillatory viscous boundary layer",
            vec![
                q("delta", "penetration depth", length),
                q("nu", "kinematic viscosity", [0, 2, -1, 0, 0]),
                q("omega", "angular frequency", frequency),
            ],
            vec![vec![1, 2]],
        ),
        17 => (
            "Airfoil lift per span at fixed section shape",
            vec![
                q("Lp", "lift magnitude per span", [1, 0, -2, 0, 0]),
                q("rho", "freestream density", density),
                q("U", "freestream speed", speed),
                q("c", "chord", length),
                q("mu", "dynamic viscosity", viscosity),
                q("a", "sound speed", speed),
                q("alpha", "angle of attack", scalar),
            ],
            vec![vec![1, 2, 3], vec![1, 5, 3]],
        ),
        18 => (
            "Longitudinal bulk-wave transit through a homogeneous isotropic solid",
            vec![
                q("t", "transit time", time),
                q("L", "path length", length),
                q("rho", "density", density),
                q("K", "bulk modulus", [1, -1, -2, 0, 0]),
                q("G", "shear modulus", [1, -1, -2, 0, 0]),
            ],
            vec![vec![1, 2, 3]],
        ),
        _ => (
            "Diffusion with a first-order bulk reaction",
            vec![
                q("c", "local concentration", [1, -3, 0, 0, 0]),
                q("c0", "initial concentration", [1, -3, 0, 0, 0]),
                q("D", "diffusivity", [0, 2, -1, 0, 0]),
                q("t", "elapsed time", time),
                q("L", "domain length", length),
                q("k", "reaction-rate constant", frequency),
            ],
            vec![vec![1, 2, 4]],
        ),
    };
    Problem {
        title,
        quantities,
        repeating,
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.abs()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Fraction(i64, i64);

impl Fraction {
    fn new(a: i64, b: i64) -> Self {
        assert_ne!(b, 0);
        let divisor = gcd(a, b) * b.signum();
        Self(a / divisor, b / divisor)
    }

    fn sub(self, other: Self) -> Self {
        Self::new(self.0 * other.1 - other.0 * self.1, self.1 * other.1)
    }

    fn mul(self, other: Self) -> Self {
        Self::new(self.0 * other.0, self.1 * other.1)
    }

    fn div(self, other: Self) -> Self {
        Self::new(self.0 * other.1, self.1 * other.0)
    }
}

fn rref(mut matrix: Vec<Vec<Fraction>>) -> (Vec<Vec<Fraction>>, Vec<usize>) {
    let mut pivots = Vec::new();
    for column in 0..matrix[0].len() {
        let row = pivots.len();
        let Some(pivot) = (row..matrix.len()).find(|&i| matrix[i][column].0 != 0) else {
            continue;
        };
        matrix.swap(row, pivot);
        let divisor = matrix[row][column];
        for entry in &mut matrix[row] {
            *entry = entry.div(divisor);
        }
        for i in 0..matrix.len() {
            if i == row {
                continue;
            }
            let scale = matrix[i][column];
            for j in 0..matrix[i].len() {
                matrix[i][j] = matrix[i][j].sub(scale.mul(matrix[row][j]));
            }
        }
        pivots.push(column);
        if pivots.len() == matrix.len() {
            break;
        }
    }
    (matrix, pivots)
}

fn nullspace(quantities: &[Quantity], repeating: &[usize]) -> (usize, Vec<Vec<i64>>) {
    let mut order = repeating.to_vec();
    order.extend((0..quantities.len()).filter(|i| !repeating.contains(i)));
    let matrix = (0..5)
        .map(|d| {
            order
                .iter()
                .map(|&i| Fraction(quantities[i].dimension[d], 1))
                .collect()
        })
        .collect();
    let (reduced, pivots) = rref(matrix);
    let mut basis = Vec::new();
    for free in 0..order.len() {
        if pivots.contains(&free) {
            continue;
        }
        let mut vector = vec![Fraction(0, 1); order.len()];
        vector[free] = Fraction(1, 1);
        for (row, &pivot) in pivots.iter().enumerate() {
            vector[pivot] = Fraction(-reduced[row][free].0, reduced[row][free].1);
        }
        let multiple = vector.iter().fold(1, |m, f| m / gcd(m, f.1) * f.1);
        let mut integers = vec![0; order.len()];
        for (position, value) in vector.iter().enumerate() {
            integers[order[position]] = value.0 * (multiple / value.1);
        }
        let divisor = integers.iter().fold(0, |g, &v| gcd(g, v));
        for integer in &mut integers {
            *integer /= divisor;
        }
        basis.push(integers);
    }
    (pivots.len(), basis)
}

fn product(quantities: &[Quantity], exponents: &[i64]) -> String {
    let factors = |sign: i64| {
        quantities
            .iter()
            .zip(exponents)
            .filter_map(|(q, &e)| {
                if e.signum() != sign {
                    None
                } else if e.abs() == 1 {
                    Some(q.symbol.to_owned())
                } else {
                    Some(format!("{}^{}", q.symbol, e.abs()))
                }
            })
            .collect::<Vec<_>>()
    };
    let numerator = factors(1);
    let denominator = factors(-1);
    let top = if numerator.is_empty() {
        "1".to_owned()
    } else {
        numerator.join(" ")
    };
    match denominator.len() {
        0 => top,
        1 => format!("{top}/{}", denominator[0]),
        _ => format!("{top}/({})", denominator.join(" ")),
    }
}

fn unit_multiplier(dimension: Dimension, scales: [f64; 5]) -> String {
    let exponent = -scales
        .iter()
        .zip(dimension)
        .map(|(&s, d)| s.log2() as i32 * d as i32)
        .sum::<i32>();
    if exponent >= 0 {
        (1_u64 << exponent).to_string()
    } else {
        format!("1/{}", 1_u64 << -exponent)
    }
}

fn render(problem: &Problem, repeating: &[usize], scales: Option<[f64; 5]>) -> String {
    let (rank, basis) = nullspace(&problem.quantities, repeating);
    let labels = ["M", "L", "T", "Theta", "I"];
    let active = (0..5)
        .filter(|&d| problem.quantities.iter().any(|q| q.dimension[d] != 0))
        .collect::<Vec<_>>();
    let mut output = format!(
        "{}\n\nQuantity | symbol | {}",
        problem.title,
        active
            .iter()
            .map(|&d| labels[d])
            .collect::<Vec<_>>()
            .join(" ")
    );
    if scales.is_some() {
        output.push_str(" | new value/old value");
    }
    for q in &problem.quantities {
        write!(
            output,
            "\n{} | {} | {}",
            q.name,
            q.symbol,
            active
                .iter()
                .map(|&d| q.dimension[d].to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
        .unwrap();
        if let Some(scales) = scales {
            write!(output, " | {}", unit_multiplier(q.dimension, scales)).unwrap();
        }
    }
    write!(
        output,
        "\n\nRank {rank}, {} independent {}\nRepeating quantities: {}",
        basis.len(),
        if basis.len() == 1 { "group" } else { "groups" },
        repeating
            .iter()
            .map(|&i| problem.quantities[i].symbol)
            .collect::<Vec<_>>()
            .join(", ")
    )
    .unwrap();
    for (i, group) in basis.iter().enumerate() {
        write!(
            output,
            "\nPi{} = {}",
            i + 1,
            product(&problem.quantities, group)
        )
        .unwrap();
    }
    write!(
        output,
        "\nPi1 = Phi({})",
        (2..=basis.len())
            .map(|i| format!("Pi{i}"))
            .collect::<Vec<_>>()
            .join(", ")
    )
    .unwrap();
    if basis.len() == 1 {
        output.truncate(output.rfind('\n').unwrap());
        output.push_str("\nPi1 is a dimensionless constant");
    }
    if let Some(scales) = scales {
        write!(
            output,
            "\n\nNew base units / old base units: {}\nEvery Pi group is unchanged",
            active
                .iter()
                .map(|&d| format!("{} ×{}", labels[d], n(scales[d])))
                .collect::<Vec<_>>()
                .join(", ")
        )
        .unwrap();
    }
    output
}

pub fn generate(rng: &mut StdRng) -> String {
    let problem = problem(rng.random_range(0..20));
    let repeating = &problem.repeating[rng.random_range(0..problem.repeating.len())];
    let scales = if rng.random_ratio(1, 2) {
        let choices = [0.25, 0.5, 1.0, 2.0, 4.0];
        Some(std::array::from_fn(|_| {
            choices[rng.random_range(0..choices.len())]
        }))
    } else {
        None
    };
    render(&problem, repeating, scales)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_basis_is_complete_independent_and_exactly_dimensionless() {
        for kind in 0..20 {
            let p = problem(kind);
            for repeating in &p.repeating {
                let (rank, basis) = nullspace(&p.quantities, repeating);
                assert_eq!(rank, repeating.len(), "kind {kind}");
                assert_eq!(basis.len() + rank, p.quantities.len());
                for group in &basis {
                    for d in 0..5 {
                        assert_eq!(
                            p.quantities
                                .iter()
                                .zip(group)
                                .map(|(q, &e)| q.dimension[d] * e)
                                .sum::<i64>(),
                            0,
                            "kind {kind}"
                        );
                    }
                    assert_eq!(group.iter().fold(0, |g, &e| gcd(g, e)), 1);
                }
                let matrix = basis
                    .iter()
                    .map(|row| row.iter().map(|&e| Fraction(e, 1)).collect())
                    .collect();
                assert_eq!(rref(matrix).1.len(), basis.len());
            }
        }
    }

    #[test]
    fn changing_all_five_base_units_preserves_numeric_groups() {
        let scales = [2.5_f64, 0.3, 4.0, 1.7, 0.2];
        for kind in 0..20 {
            let p = problem(kind);
            for repeating in &p.repeating {
                for group in nullspace(&p.quantities, repeating).1 {
                    let mut before = 0.0;
                    let mut after = 0.0;
                    for (i, (q, &e)) in p.quantities.iter().zip(&group).enumerate() {
                        let value = 1.3 + i as f64 * 0.7;
                        let scale = scales
                            .iter()
                            .zip(q.dimension)
                            .map(|(&s, d)| s.powi(-d as i32))
                            .product::<f64>();
                        before += e as f64 * value.ln();
                        after += e as f64 * (value * scale).ln();
                    }
                    assert!((before - after).abs() < 1e-11, "kind {kind}");
                }
            }
        }
    }

    #[test]
    fn rank_deficiency_and_familiar_similarity_laws() {
        let pendulum = problem(3);
        assert_eq!(nullspace(&pendulum.quantities, &[1, 2]).1[0], [2, -1, 1, 0]);
        let beam = problem(8);
        assert_eq!(nullspace(&beam.quantities, &[1, 2]).0, 2);
        let blast = problem(14);
        assert_eq!(
            nullspace(&blast.quantities, &[1, 2, 3]).1[0],
            [5, -1, 1, -2]
        );
        let layer = problem(16);
        assert_eq!(nullspace(&layer.quantities, &[1, 2]).1[0], [2, -1, 1]);
    }
}
