use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

include!("../../number.rs");

#[derive(Clone, Copy, Debug)]
struct Body {
    name: &'static str,
    g: f64,
}

const BODIES: [Body; 5] = [
    Body {
        name: "Earth",
        g: 9.81,
    },
    Body {
        name: "Moon",
        g: 1.62,
    },
    Body {
        name: "Mars",
        g: 3.73,
    },
    Body {
        name: "Titan",
        g: 1.35,
    },
    Body {
        name: "Ceres",
        g: 0.28,
    },
];

#[derive(Clone, Copy, Debug)]
enum Model {
    Vacuum,
    Linear { tau: f64, wind: f64 },
}

#[derive(Clone, Copy, Debug)]
struct State {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

#[derive(Clone, Copy, Debug)]
struct Flight {
    label: &'static str,
    body: Body,
    speed: f64,
    angle: f64,
    height: f64,
    slope: f64,
    model: Model,
}

impl Flight {
    fn state(self, t: f64) -> State {
        let vx = self.speed * self.angle.cos();
        let vy = self.speed * self.angle.sin();
        let g = self.body.g;
        match self.model {
            Model::Vacuum => State {
                x: vx * t,
                y: self.height + vy * t - 0.5 * g * t * t,
                vx,
                vy: vy - g * t,
            },
            Model::Linear { tau, wind } => {
                let decay = (-t / tau).exp();
                let integral = -tau * (-t / tau).exp_m1();
                State {
                    x: wind * t + (vx - wind) * integral,
                    y: self.height + (vy + g * tau) * integral - g * tau * t,
                    vx: wind + (vx - wind) * decay,
                    vy: (vy + g * tau) * decay - g * tau,
                }
            }
        }
    }

    fn landing_time(self) -> f64 {
        let vy = self.speed * self.angle.sin();
        let g = self.body.g;
        match self.model {
            Model::Vacuum => {
                let relative_v = vy - self.slope * self.speed * self.angle.cos();
                let root = (relative_v * relative_v + 2.0 * g * self.height).sqrt();
                if relative_v < 0.0 {
                    2.0 * self.height / (root - relative_v)
                } else {
                    (relative_v + root) / g
                }
            }
            Model::Linear { tau, .. } => {
                // y(t) <= h + tau(vy + g*tau) - g*tau*t for vy >= 0.
                let mut lo = self.apex_time();
                let mut hi = (self.height + tau * (vy + g * tau)) / (g * tau) + tau;
                for _ in 0..80 {
                    let mid = (lo + hi) / 2.0;
                    if self.state(mid).y > 0.0 {
                        lo = mid;
                    } else {
                        hi = mid;
                    }
                }
                (lo + hi) / 2.0
            }
        }
    }

    fn apex_time(self) -> f64 {
        let vy = self.speed * self.angle.sin();
        if vy <= 0.0 {
            return 0.0;
        }
        match self.model {
            Model::Vacuum => vy / self.body.g,
            Model::Linear { tau, .. } => tau * (vy / (self.body.g * tau)).ln_1p(),
        }
    }

    fn horizontal_turn(self) -> Option<f64> {
        let Model::Linear { tau, wind } = self.model else {
            return None;
        };
        (wind < 0.0).then(|| tau * ((self.speed * self.angle.cos() - wind) / -wind).ln())
    }
}

#[derive(Debug)]
struct Target {
    body: Body,
    speed: f64,
    x: f64,
    y: f64,
}

impl Target {
    fn envelope_height(&self) -> f64 {
        let v2 = self.speed * self.speed;
        v2 / (2.0 * self.body.g) - self.body.g * self.x * self.x / (2.0 * v2)
    }

    fn angles(&self) -> Vec<f64> {
        let v2 = self.speed * self.speed;
        let d = 1.0 - (self.body.g * self.x / v2).powi(2) - 2.0 * self.body.g * self.y / v2;
        if d < -1e-12 {
            Vec::new()
        } else if d.abs() <= 1e-12 {
            vec![(v2 / (self.body.g * self.x)).atan()]
        } else {
            [-1.0, 1.0]
                .map(|sign| ((1.0 + sign * d.sqrt()) * v2 / (self.body.g * self.x)).atan())
                .to_vec()
        }
    }
}

enum Case {
    Flight(Flight),
    Pair(Flight, Flight),
    Target(Target),
}

fn sample_regime(rng: &mut StdRng, regime: u32) -> Case {
    let body = BODIES[rng.random_range(0..BODIES.len())];
    // Keep local arcs small even at Ceres gravity.
    let speed = rng.random_range(4..=24) as f64 * body.g.sqrt();
    let angle = (rng.random_range(20..=70) as f64).to_radians();
    let mut flight = Flight {
        label: "Level-ground vacuum arc",
        body,
        speed,
        angle,
        height: 0.0,
        slope: 0.0,
        model: Model::Vacuum,
    };
    match regime {
        0 => {}
        1 | 2 => {
            flight.label = "Vacuum launch from a cliff";
            flight.height = rng.random_range(5..=100) as f64;
            flight.angle = if regime == 1 {
                0.0
            } else {
                -(rng.random_range(10..=60) as f64).to_radians()
            };
        }
        3 | 4 => {
            flight.label = "Vacuum arc above an inclined plane";
            let incline = if regime == 3 {
                rng.random_range(-30..=30) as f64
            } else {
                rng.random_range(5..=35) as f64
            };
            let clearance = if regime == 4 {
                rng.random_range(1..=10) as f64 / 10.0
            } else {
                rng.random_range(10..=40) as f64
            };
            flight.angle = (incline + clearance).to_radians();
            flight.slope = incline.to_radians().tan();
        }
        5 => {
            flight.angle = (rng.random_range(15..=35) as f64).to_radians();
            let high = Flight {
                angle: std::f64::consts::FRAC_PI_2 - flight.angle,
                ..flight
            };
            return Case::Pair(flight, high);
        }
        6..=8 => {
            flight.body = BODIES[0];
            flight.speed = rng.random_range(4..=30) as f64;
            let strength = if regime == 6 {
                rng.random_range(5..=20) as f64 / 100.0
            } else {
                rng.random_range(15..=50) as f64 / 10.0
            };
            let tau = flight.speed / (flight.body.g * strength);
            flight.label = "Linear-drag arc";
            flight.model = Model::Linear { tau, wind: 0.0 };
            if regime == 8 {
                flight.label = "Headwind reverses a linear-drag arc";
                let t = flight.landing_time();
                let integral = -tau * (-t / tau).exp_m1();
                let return_wind = -flight.speed * flight.angle.cos() * integral / (t - integral);
                let factor = [0.8, 1.0, 1.2][rng.random_range(0..3)];
                flight.model = Model::Linear {
                    tau,
                    wind: factor * return_wind,
                };
            }
        }
        9..=11 => {
            let x = rng.random_range(3..=14) as f64 / 10.0 * speed * speed / body.g;
            let mut target = Target {
                body,
                speed,
                x,
                y: 0.0,
            };
            let offset = if regime == 10 {
                0.0
            } else {
                let sign = if regime == 9 { -1.0 } else { 1.0 };
                sign * rng.random_range(1..=8) as f64 / 100.0 * speed * speed / body.g
            };
            target.y = target.envelope_height() + offset;
            return Case::Target(target);
        }
        _ => unreachable!(),
    }
    Case::Flight(flight)
}

fn render_flight(flight: Flight) -> String {
    let t = flight.landing_time();
    let landing = flight.state(t);
    let peak_t = flight.apex_time().min(t);
    let peak = flight.state(peak_t);
    let mut out = format!(
        "{}\n{} gravity, g = {} m/s²\nLaunch {} m/s at {}° from (0, {}) m",
        flight.label,
        flight.body.name,
        n(flight.body.g),
        n(flight.speed),
        n(flight.angle.to_degrees()),
        n(flight.height)
    );
    if flight.slope != 0.0 {
        write!(out, "\nGround y = {}x", n(flight.slope)).unwrap();
    }
    if let Model::Linear { tau, wind } = flight.model {
        write!(
            out,
            "\nDrag a = −(v − u)/τ\nτ = {} s, u = ({}, 0) m/s",
            n(tau),
            n(wind)
        )
        .unwrap();
    }
    write!(
        out,
        "\n\nLanding at t = {} s, (x, y) = ({}, {}) m\nImpact (vx, vy) = ({}, {}) m/s",
        n(t),
        n(landing.x),
        n(landing.y),
        n(landing.vx),
        n(landing.vy)
    )
    .unwrap();
    if peak_t == 0.0 {
        write!(
            out,
            "\nHighest point is the launch, y = {} m",
            n(flight.height)
        )
        .unwrap();
    } else if peak_t == t {
        write!(out, "\nStill rising at impact, y = {} m", n(landing.y)).unwrap();
    } else {
        write!(
            out,
            "\nApex at t = {} s, (x, y) = ({}, {}) m",
            n(peak_t),
            n(peak.x),
            n(peak.y)
        )
        .unwrap();
    }
    if let Some(turn_t) = flight.horizontal_turn() {
        let turn = flight.state(turn_t);
        write!(
            out,
            "\nFarthest forward at t = {} s, x = {} m",
            n(turn_t),
            n(turn.x)
        )
        .unwrap();
    } else if let Model::Linear { tau, .. } = flight.model {
        let vacuum = Flight {
            model: Model::Vacuum,
            ..flight
        };
        let vacuum_range = vacuum.state(vacuum.landing_time()).x;
        write!(
            out,
            "\nRange is {}% of the vacuum arc\nStill-air terminal fall speed {} m/s",
            n(100.0 * landing.x / vacuum_range),
            n(flight.body.g * tau)
        )
        .unwrap();
    } else if flight.slope != 0.0 {
        write!(
            out,
            "\nDistance along slope {} m",
            n(landing.x * (1.0 + flight.slope * flight.slope).sqrt())
        )
        .unwrap();
    }
    out
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..12);
    match sample_regime(rng, regime) {
        Case::Flight(flight) => render_flight(flight),
        Case::Pair(low, high) => {
            let range = low.state(low.landing_time()).x;
            let mut out = format!(
                "Two vacuum arcs with the same range\n{} gravity, g = {} m/s²\nLaunch speed {} m/s\nLevel-ground range {} m\n\nAngle | flight time (s) | apex height (m)",
                low.body.name,
                n(low.body.g),
                n(low.speed),
                n(range)
            );
            for flight in [low, high] {
                write!(
                    out,
                    "\n{}° | {} | {}",
                    n(flight.angle.to_degrees()),
                    n(flight.landing_time()),
                    n(flight.state(flight.apex_time()).y)
                )
                .unwrap();
            }
            out
        }
        Case::Target(target) => {
            let mut out = format!(
                "Fixed-speed vacuum trajectories to a point\n{} gravity, g = {} m/s²\nLaunch at (0, 0), speed {} m/s\nTarget (x, y) = ({}, {}) m",
                target.body.name,
                n(target.body.g),
                n(target.speed),
                n(target.x),
                n(target.y)
            );
            let angles = target.angles();
            match angles.len() {
                0 => write!(out, "\n\nUnreachable by {} m above the trajectory envelope\nMinimum launch speed {} m/s", n(target.y - target.envelope_height()), n((target.body.g * (target.y + target.x.hypot(target.y))).sqrt())).unwrap(),
                1 => out.push_str("\n\nOne tangent trajectory at the reachability boundary"),
                _ => out.push_str("\n\nTwo trajectories"),
            }
            for angle in angles {
                let t = target.x / (target.speed * angle.cos());
                let vy = target.speed * angle.sin() - target.body.g * t;
                write!(
                    out,
                    "\n{}° reaches the point in {} s, {}",
                    n(angle.to_degrees()),
                    n(t),
                    if vy > 0.0 { "rising" } else { "falling" }
                )
                .unwrap();
            }
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn near(a: f64, b: f64) {
        assert!(
            (a - b).abs() <= 1e-8 * (1.0 + a.abs() + b.abs()),
            "{a} != {b}"
        );
    }

    #[test]
    fn every_flight_hits_the_first_ground_crossing_and_obeys_its_equations() {
        for regime in 0..9 {
            for seed in 0..128 {
                let case = sample_regime(&mut StdRng::seed_from_u64(seed), regime);
                let flights = match case {
                    Case::Flight(flight) => vec![flight],
                    Case::Pair(low, high) => {
                        near(
                            low.state(low.landing_time()).x,
                            high.state(high.landing_time()).x,
                        );
                        assert!(low.landing_time() < high.landing_time());
                        vec![low, high]
                    }
                    _ => unreachable!(),
                };
                for flight in flights {
                    let t = flight.landing_time();
                    assert!(t.is_finite() && t > 0.0);
                    let end = flight.state(t);
                    near(end.y, flight.slope * end.x);
                    assert!(end.vy - flight.slope * end.vx < 0.0);
                    for fraction in [0.01, 0.25, 0.5, 0.75, 0.99] {
                        let s = flight.state(t * fraction);
                        assert!(s.y > flight.slope * s.x);
                        let epsilon = t * 1e-5;
                        let left = flight.state(t * fraction - epsilon);
                        let right = flight.state(t * fraction + epsilon);
                        let vx_dt = (right.vx - left.vx) / (2.0 * epsilon);
                        let vy_dt = (right.vy - left.vy) / (2.0 * epsilon);
                        near((right.x - left.x) / (2.0 * epsilon), s.vx);
                        near((right.y - left.y) / (2.0 * epsilon), s.vy);
                        match flight.model {
                            Model::Vacuum => {
                                near(vx_dt, 0.0);
                                near(vy_dt, -flight.body.g);
                                near(
                                    s.vx * s.vx + s.vy * s.vy + 2.0 * flight.body.g * s.y,
                                    flight.speed * flight.speed
                                        + 2.0 * flight.body.g * flight.height,
                                );
                            }
                            Model::Linear { tau, wind } => {
                                near(vx_dt, -(s.vx - wind) / tau);
                                near(vy_dt, -flight.body.g - s.vy / tau);
                            }
                        }
                    }
                    if let Some(turn_t) = flight.horizontal_turn() {
                        assert!(turn_t > 0.0 && turn_t < t);
                        near(flight.state(turn_t).vx, 0.0);
                        assert!(flight.state(turn_t).x > end.x);
                    }
                    let apex_t = flight.apex_time();
                    if apex_t > 0.0 {
                        near(flight.state(apex_t).vy, 0.0);
                    }
                }
            }
        }
    }

    #[test]
    fn reachability_changes_from_two_arcs_through_tangency_to_none() {
        for regime in 9..12 {
            for seed in 0..256 {
                let Case::Target(target) = sample_regime(&mut StdRng::seed_from_u64(seed), regime)
                else {
                    unreachable!()
                };
                let angles = target.angles();
                assert_eq!(angles.len(), 11 - regime as usize);
                for angle in angles {
                    let t = target.x / (target.speed * angle.cos());
                    let flight = Flight {
                        label: "",
                        body: target.body,
                        speed: target.speed,
                        angle,
                        height: 0.0,
                        slope: 0.0,
                        model: Model::Vacuum,
                    };
                    near(flight.state(t).x, target.x);
                    near(flight.state(t).y, target.y);
                }
                let minimum = (target.body.g * (target.y + target.x.hypot(target.y))).sqrt();
                match regime {
                    9 => assert!(minimum < target.speed),
                    10 => near(minimum, target.speed),
                    11 => assert!(minimum > target.speed),
                    _ => unreachable!(),
                }
            }
        }
    }

    #[test]
    fn still_air_drag_reduces_range_and_has_the_vacuum_limit() {
        for seed in 0..128 {
            let Case::Flight(drag) = sample_regime(&mut StdRng::seed_from_u64(seed), 7) else {
                unreachable!()
            };
            let vacuum = Flight {
                model: Model::Vacuum,
                ..drag
            };
            assert!(drag.state(drag.landing_time()).x < vacuum.state(vacuum.landing_time()).x);
            assert!(drag.state(drag.apex_time()).y < vacuum.state(vacuum.apex_time()).y);
            let weak = Flight {
                model: Model::Linear {
                    tau: 1e5,
                    wind: 0.0,
                },
                ..drag
            };
            let t = vacuum.landing_time();
            assert!((weak.state(t).x - vacuum.state(t).x).abs() < 1e-3 * (1.0 + vacuum.state(t).x));
            let Model::Linear { tau, .. } = drag.model else {
                unreachable!()
            };
            near(drag.state(40.0 * tau).vy, -drag.body.g * tau);
        }
    }
}
