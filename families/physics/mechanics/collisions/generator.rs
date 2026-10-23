use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

include!("../../number.rs");

#[derive(Clone, Copy, Debug)]
struct V(f64, f64);

impl V {
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }

    fn scale(self, k: f64) -> Self {
        Self(k * self.0, k * self.1)
    }

    fn dot(self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1
    }

    fn text(self) -> String {
        format!("({}, {})", n(self.0), n(self.1))
    }
}

struct Impact {
    label: &'static str,
    mass: [f64; 2],
    before: [V; 2],
    after: [V; 2],
    normal: Option<V>,
    e: f64,
    impulse: V,
}

impl Impact {
    fn smooth(label: &'static str, mass: [f64; 2], before: [V; 2], normal: V, e: f64) -> Self {
        let closing = before[0].add(before[1].scale(-1.0)).dot(normal);
        let impulse = normal.scale((1.0 + e) * closing / (1.0 / mass[0] + 1.0 / mass[1]));
        Self {
            label,
            mass,
            before,
            after: [
                before[0].add(impulse.scale(-1.0 / mass[0])),
                before[1].add(impulse.scale(1.0 / mass[1])),
            ],
            normal: Some(normal),
            e,
            impulse,
        }
    }

    fn coalesce(mass: [f64; 2], before: [V; 2]) -> Self {
        let common = before[0]
            .scale(mass[0])
            .add(before[1].scale(mass[1]))
            .scale(1.0 / (mass[0] + mass[1]));
        Self {
            label: "Coalescing particles",
            mass,
            before,
            after: [common, common],
            normal: None,
            e: 0.0,
            impulse: common.add(before[1].scale(-1.0)).scale(mass[1]),
        }
    }

    fn momentum(&self, velocities: [V; 2]) -> V {
        velocities[0]
            .scale(self.mass[0])
            .add(velocities[1].scale(self.mass[1]))
    }

    fn energy(&self, velocities: [V; 2]) -> f64 {
        0.5 * (self.mass[0] * velocities[0].dot(velocities[0])
            + self.mass[1] * velocities[1].dot(velocities[1]))
    }
}

fn sample(rng: &mut StdRng) -> Impact {
    let regime = rng.random_range(0..8);
    sample_regime(rng, regime)
}

fn sample_regime(rng: &mut StdRng, regime: u32) -> Impact {
    let m = rng.random_range(1..=12) as f64 / 10.0;
    let speed = rng.random_range(2..=18) as f64 / 2.0;
    let drift = V(
        rng.random_range(-8..=8) as f64 / 2.0,
        rng.random_range(-6..=6) as f64 / 2.0,
    );
    match regime {
        0 => Impact::smooth(
            "Equal-mass elastic impact",
            [m, m],
            [drift.add(V(speed, 0.0)), drift],
            V(1.0, 0.0),
            1.0,
        ),
        1 => Impact::smooth(
            "Heavy projectile, light target",
            [m * rng.random_range(8..=30) as f64, m],
            [V(speed, 0.0), V(0.0, 0.0)],
            V(1.0, 0.0),
            1.0,
        ),
        2 => Impact::smooth(
            "Light projectile, heavy target",
            [m, m * rng.random_range(8..=30) as f64],
            [V(speed, 0.0), V(0.0, 0.0)],
            V(1.0, 0.0),
            1.0,
        ),
        3 => Impact::coalesce(
            [m, rng.random_range(2..=25) as f64 / 10.0],
            [
                drift.add(V(speed, speed / 2.0)),
                drift.add(V(-speed / 3.0, speed)),
            ],
        ),
        _ => {
            let angle = (rng.random_range(-11..=11) as f64 * 7.5).to_radians();
            let normal = V(angle.cos(), angle.sin());
            let tangent = V(-normal.1, normal.0);
            let closing = if regime == 6 { speed / 30.0 } else { speed };
            let slip = speed * rng.random_range(3..=12) as f64 / 4.0;
            let e = match regime {
                5 => rng.random_range(1..=9) as f64 / 10.0,
                7 => 0.0,
                _ => 1.0,
            };
            Impact::smooth(
                if regime == 6 {
                    "Near-grazing smooth impact"
                } else {
                    "Smooth oblique impact"
                },
                [m, rng.random_range(1..=25) as f64 / 10.0],
                [
                    drift.add(normal.scale(closing)).add(tangent.scale(slip)),
                    drift,
                ],
                normal,
                e,
            )
        }
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let impact = sample(rng);
    let centre = impact
        .momentum(impact.before)
        .scale(1.0 / (impact.mass[0] + impact.mass[1]));
    let com_frame = rng.random_bool(0.4);
    let origin = if com_frame { centre } else { V(0.0, 0.0) };
    let before = impact.before.map(|v| v.add(origin.scale(-1.0)));
    let after = impact.after.map(|v| v.add(origin.scale(-1.0)));
    let mut out = impact.label.to_owned();
    if let Some(normal) = impact.normal {
        write!(
            out,
            "\nn from A to B = {}\ne = {}",
            normal.text(),
            n(impact.e)
        )
        .unwrap();
    }
    if com_frame {
        write!(out, "\nCenter-of-mass frame, V_cm = {} m/s", centre.text()).unwrap();
    } else {
        out.push_str("\nLab frame");
    }
    out.push_str("\n\nBody | mass (kg) | before (vx, vy), m/s | after (vx, vy), m/s");
    for i in 0..2 {
        write!(
            out,
            "\n{} | {} | {} | {}",
            ['A', 'B'][i],
            n(impact.mass[i]),
            before[i].text(),
            after[i].text()
        )
        .unwrap();
    }
    write!(
        out,
        "\n\nK = {} → {} J\nK lost = {} J\nImpulse on B = {} N·s",
        n(impact.energy(before)),
        n(impact.energy(after)),
        n(impact.energy(before) - impact.energy(after)),
        impact.impulse.text()
    )
    .unwrap();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn near(a: f64, b: f64) {
        assert!(
            (a - b).abs() < 1e-9 * (1.0 + a.abs() + b.abs()),
            "{a} != {b}"
        );
    }

    #[test]
    fn impulses_conserve_momentum_and_dissipate_the_right_energy() {
        for (regime, seed) in (0..8).flat_map(|regime| (0..128).map(move |seed| (regime, seed))) {
            let impact = sample_regime(&mut StdRng::seed_from_u64(seed), regime);
            let initial = impact.momentum(impact.before);
            let final_p = impact.momentum(impact.after);
            near(initial.0, final_p.0);
            near(initial.1, final_p.1);
            let relative = impact.before[0].add(impact.before[1].scale(-1.0));
            let reduced_mass = impact.mass[0] * impact.mass[1] / (impact.mass[0] + impact.mass[1]);
            let expected_loss = if let Some(normal) = impact.normal {
                let closing = relative.dot(normal);
                assert!(closing > 0.0);
                let final_relative = impact.after[0].add(impact.after[1].scale(-1.0));
                near(final_relative.dot(normal), -impact.e * closing);
                let tangent = V(-normal.1, normal.0);
                for body in 0..2 {
                    near(
                        impact.before[body].dot(tangent),
                        impact.after[body].dot(tangent),
                    );
                }
                0.5 * reduced_mass * (1.0 - impact.e * impact.e) * closing * closing
            } else {
                near(impact.after[0].0, impact.after[1].0);
                near(impact.after[0].1, impact.after[1].1);
                0.5 * reduced_mass * relative.dot(relative)
            };
            near(
                impact.energy(impact.before) - impact.energy(impact.after),
                expected_loss,
            );
            let delta_p = impact.after[1]
                .add(impact.before[1].scale(-1.0))
                .scale(impact.mass[1]);
            near(delta_p.0, impact.impulse.0);
            near(delta_p.1, impact.impulse.1);
            let centre = initial.scale(1.0 / (impact.mass[0] + impact.mass[1]));
            let shifted_before = impact.before.map(|v| v.add(centre.scale(-1.0)));
            let shifted_after = impact.after.map(|v| v.add(centre.scale(-1.0)));
            near(
                impact.energy(shifted_before) - impact.energy(shifted_after),
                expected_loss,
            );
        }
    }

    #[test]
    fn equal_mass_head_on_elastic_collision_swaps_velocities() {
        let before = [V(4.0, -2.0), V(-3.0, -2.0)];
        let impact = Impact::smooth("", [0.3, 0.3], before, V(1.0, 0.0), 1.0);
        near(impact.after[0].0, before[1].0);
        near(impact.after[1].0, before[0].0);
    }
}
