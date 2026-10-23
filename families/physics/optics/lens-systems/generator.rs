use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

include!("../../number.rs");

#[derive(Clone, Copy, Debug)]
struct Matrix {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl Matrix {
    fn times(self, rhs: Self) -> Self {
        Self {
            a: self.a * rhs.a + self.b * rhs.c,
            b: self.a * rhs.b + self.b * rhs.d,
            c: self.c * rhs.a + self.d * rhs.c,
            d: self.c * rhs.b + self.d * rhs.d,
        }
    }

    fn space(distance: f64) -> Self {
        Self {
            a: 1.0,
            b: distance,
            c: 0.0,
            d: 1.0,
        }
    }

    fn lens(focal: f64) -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: -1.0 / focal,
            d: 1.0,
        }
    }
}

struct Bench {
    label: &'static str,
    f1: f64,
    second: Option<(f64, f64)>,
}

impl Bench {
    fn matrix(&self) -> Matrix {
        let first = Matrix::lens(self.f1);
        match self.second {
            Some((gap, f2)) => Matrix::lens(f2).times(Matrix::space(gap)).times(first),
            None => first,
        }
    }

    fn last_z(&self) -> f64 {
        self.second.map_or(0.0, |(gap, _)| gap)
    }

    fn conjugate(&self, object_distance: f64) -> Option<(f64, f64)> {
        let from_object = self.matrix().times(Matrix::space(object_distance));
        if from_object.d.abs() < 1e-12 {
            None
        } else {
            let q = -from_object.b / from_object.d;
            Some((q, from_object.a + q * from_object.c))
        }
    }

    fn text(&self) -> String {
        let mut out = format!(
            "{}\nParaxial thin lenses in air\nL1 at z = 0 mm, f1 = {} mm",
            self.label,
            n(self.f1)
        );
        if let Some((gap, f2)) = self.second {
            write!(out, "\nL2 at z = {} mm, f2 = {} mm", n(gap), n(f2)).unwrap();
        }
        out
    }
}

fn finite_image(bench: Bench, s: f64, rng: &mut StdRng) -> String {
    let mut out = bench.text();
    write!(out, "\nObject at z = -{} mm", n(s)).unwrap();
    match bench.conjugate(s) {
        Some((q, magnification)) => {
            write!(
                out,
                "\n\n{} image at z = {} mm\nM = {} ({})",
                if q > 0.0 { "Real" } else { "Virtual" },
                n(bench.last_z() + q),
                n(magnification),
                if magnification > 0.0 {
                    "upright"
                } else {
                    "inverted"
                }
            )
            .unwrap();
            if rng.random_bool(0.5) {
                let height = rng.random_range(2..=12) as f64 / 10.0;
                write!(
                    out,
                    "\nObject height {} mm → image height {} mm",
                    n(height),
                    n(height * magnification)
                )
                .unwrap();
            } else {
                write!(out, "\nImage distance from final lens = {} mm", n(q)).unwrap();
            }
        }
        None => {
            let height = rng.random_range(2..=12) as f64 / 10.0;
            let from_object = bench.matrix().times(Matrix::space(s));
            write!(
                out,
                "\n\nImage at infinity\nObject point y = {} mm → collimated bundle at θ = {} mrad",
                n(height),
                n(1000.0 * from_object.c * height)
            )
            .unwrap();
        }
    }
    out
}

enum Scene {
    Finite(Bench, f64),
    Telephoto(Bench),
    Afocal(Bench),
}

fn sample_regime(rng: &mut StdRng, regime: u32) -> Scene {
    let focal = rng.random_range(4..=24) as f64 * 10.0;
    match regime {
        0..=5 => {
            let (label, f, s) = match regime {
                0 => (
                    "Converging lens",
                    focal,
                    focal * rng.random_range(25..=70) as f64 / 10.0,
                ),
                1 => (
                    "Magnifying glass",
                    focal,
                    focal * rng.random_range(3..=8) as f64 / 10.0,
                ),
                2 => ("Symmetric 2f conjugates", focal, 2.0 * focal),
                3 => {
                    let offset = rng.random_range(1..=4) as f64 / 100.0;
                    let side = if rng.random_bool(0.5) { 1.0 } else { -1.0 };
                    (
                        "Near-focus conjugates",
                        focal,
                        focal * (1.0 + side * offset),
                    )
                }
                4 => ("Focal-plane object", focal, focal),
                _ => (
                    "Diverging lens",
                    -focal,
                    focal * rng.random_range(5..=40) as f64 / 10.0,
                ),
            };
            Scene::Finite(
                Bench {
                    label,
                    f1: f,
                    second: None,
                },
                s,
            )
        }
        6 => {
            let f2 = focal * rng.random_range(1..=4) as f64 / rng.random_range(1..=4) as f64;
            Scene::Finite(
                Bench {
                    label: "Two-lens focal-plane relay",
                    f1: focal,
                    second: Some((focal + f2, f2)),
                },
                focal,
            )
        }
        7 | 8 => {
            let negative_ratio = rng.random_range(4..=6) as f64 / 10.0;
            let gap = focal * (1.0 - negative_ratio + rng.random_range(8..=25) as f64 / 100.0);
            Scene::Telephoto(Bench {
                label: "Telephoto pair",
                f1: focal,
                second: Some((gap, -negative_ratio * focal)),
            })
        }
        _ => {
            let factor = rng.random_range(2..=6) as f64;
            let short_focal = rng.random_range(2..=6) as f64 * 10.0;
            let f1 = if regime == 9 {
                -short_focal
            } else {
                short_focal
            };
            let f2 = factor * short_focal;
            Scene::Afocal(Bench {
                label: if f1 < 0.0 {
                    "Galilean beam expander"
                } else {
                    "Keplerian beam expander"
                },
                f1,
                second: Some((f1 + f2, f2)),
            })
        }
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..11);
    match sample_regime(rng, regime) {
        Scene::Finite(bench, s) => finite_image(bench, s, rng),
        Scene::Telephoto(bench) => {
            let matrix = bench.matrix();
            let effective_focal = -1.0 / matrix.c;
            let back_focus = -matrix.a / matrix.c;
            let mut out = bench.text();
            write!(
                out,
                "\nObject at infinity\n\nReal image at z = {} mm\nEffective focal length = {} mm\nBack focal distance = {} mm\nTrack length / effective focal length = {}",
                n(bench.last_z() + back_focus),
                n(effective_focal),
                n(back_focus),
                n((bench.last_z() + back_focus) / effective_focal)
            )
            .unwrap();
            out
        }
        Scene::Afocal(bench) => {
            let matrix = bench.matrix();
            let mut out = bench.text();
            out.push_str("\n\nAfocal");
            if rng.random_bool(0.5) {
                let diameter = rng.random_range(4..=12) as f64 / 10.0;
                write!(
                    out,
                    "\nCollimated beam diameter {} mm → {} mm",
                    n(diameter),
                    n(diameter * matrix.a.abs())
                )
                .unwrap();
            } else {
                let divergence = rng.random_range(2..=15) as f64 / 10.0;
                write!(
                    out,
                    "\nParaxial angular spread {} mrad → {} mrad",
                    n(divergence),
                    n(divergence * matrix.d.abs())
                )
                .unwrap();
            }
            write!(
                out,
                "\nTransverse parity {}",
                if matrix.a > 0.0 {
                    "upright"
                } else {
                    "inverted"
                }
            )
            .unwrap();
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn every_sampled_architecture_satisfies_its_optical_constraints() {
        for (regime, seed) in (0..11).flat_map(|regime| (0..128).map(move |seed| (regime, seed))) {
            let scene = sample_regime(&mut StdRng::seed_from_u64(seed), regime);
            match scene {
                Scene::Finite(bench, s) => {
                    let matrix = bench.matrix();
                    near(matrix.a * matrix.d - matrix.b * matrix.c, 1.0);
                    if regime == 4 {
                        assert!(bench.conjugate(s).is_none());
                    } else {
                        let (q, magnification) = bench.conjugate(s).unwrap();
                        let total = Matrix::space(q).times(matrix).times(Matrix::space(s));
                        near(total.b, 0.0);
                        near(total.a, magnification);
                        near(total.d, 1.0 / magnification);
                        if regime == 1 || regime == 5 {
                            assert!(q < 0.0 && magnification > 0.0);
                        }
                        if regime == 6 {
                            let (_, f2) = bench.second.unwrap();
                            near(q, f2);
                            near(magnification, -f2 / bench.f1);
                        }
                    }
                }
                Scene::Telephoto(bench) => {
                    let matrix = bench.matrix();
                    let focal = -1.0 / matrix.c;
                    let q = -matrix.a / matrix.c;
                    assert!(q > 0.0);
                    assert!(bench.last_z() + q < focal);
                    near(Matrix::space(q).times(matrix).a, 0.0);
                }
                Scene::Afocal(bench) => {
                    let matrix = bench.matrix();
                    near(matrix.c, 0.0);
                    near(matrix.a * matrix.d, 1.0);
                    assert!(matrix.a.abs() >= 2.0);
                    assert_eq!(matrix.a > 0.0, regime == 9);
                }
            }
        }
    }

    fn near(a: f64, b: f64) {
        assert!(
            (a - b).abs() < 1e-9 * (1.0 + a.abs() + b.abs()),
            "{a} != {b}"
        );
    }

    #[test]
    fn conjugate_plane_is_independent_of_launch_angle() {
        for f1 in [-100.0, 40.0, 120.0] {
            for second in [None, Some((80.0, 150.0)), Some((35.0, -80.0))] {
                let bench = Bench {
                    label: "",
                    f1,
                    second,
                };
                let matrix = bench.matrix();
                near(matrix.a * matrix.d - matrix.b * matrix.c, 1.0);
                for s in [20.0, 39.6, 40.0, 40.4, 80.0, 300.0] {
                    if let Some((q, magnification)) = bench.conjugate(s) {
                        let total = Matrix::space(q).times(matrix).times(Matrix::space(s));
                        near(total.b, 0.0);
                        near(total.a, magnification);
                        for angle in [-0.02, 0.0, 0.03] {
                            near(total.a * 0.7 + total.b * angle, magnification * 0.7);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn single_lens_equation_and_focal_singularity() {
        for f in [-160.0, 40.0, 100.0] {
            let bench = Bench {
                label: "",
                f1: f,
                second: None,
            };
            for s in [20.0, 99.0, 101.0, 250.0] {
                if let Some((q, magnification)) = bench.conjugate(s) {
                    near(1.0 / s + 1.0 / q, 1.0 / f);
                    near(magnification, -q / s);
                }
            }
            if f > 0.0 {
                assert!(bench.conjugate(f).is_none());
                let (q, magnification) = bench.conjugate(2.0 * f).unwrap();
                near(q, 2.0 * f);
                near(magnification, -1.0);
            }
        }
    }

    #[test]
    fn telescope_and_relay_matrices_have_expected_parity_and_scale() {
        for f1 in [-60.0_f64, 60.0] {
            for ratio in [2.0, 3.0, 6.0] {
                let f2 = f1.abs() * ratio;
                let bench = Bench {
                    label: "",
                    f1,
                    second: Some((f1 + f2, f2)),
                };
                let matrix = bench.matrix();
                near(matrix.c, 0.0);
                near(matrix.a, -f2 / f1);
                near(matrix.d, -f1 / f2);
                if f1 > 0.0 {
                    let (q, magnification) = bench.conjugate(f1).unwrap();
                    near(q, f2);
                    near(magnification, -f2 / f1);
                }
            }
        }
    }

    #[test]
    fn telephoto_construction_has_positive_back_focus_and_short_track() {
        for focal in [40.0, 100.0, 240.0] {
            for ratio in [0.4, 0.5, 0.6] {
                for offset in [0.08, 0.16, 0.25] {
                    let gap = focal * (1.0 - ratio + offset);
                    let matrix = Bench {
                        label: "",
                        f1: focal,
                        second: Some((gap, -ratio * focal)),
                    }
                    .matrix();
                    let effective = -1.0 / matrix.c;
                    let back = -matrix.a / matrix.c;
                    assert!(back > 0.0);
                    assert!(gap + back < effective);
                    near(Matrix::space(back).times(matrix).a, 0.0);
                }
            }
        }
    }
}
