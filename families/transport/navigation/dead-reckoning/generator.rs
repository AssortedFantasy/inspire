use rand::{Rng, rngs::StdRng};
use std::f64::consts::{FRAC_PI_2, PI};
use std::fmt::Write;

// A nautical sphere: one angular degree is exactly 60 nautical miles.
const RADIUS: f64 = 10_800.0 / PI;

fn n(x: f64, decimals: usize) -> String {
    let scale = 10.0_f64.powi(decimals as i32);
    let x = (x * scale).round() / scale;
    let s = format!("{:.decimals$}", if x == 0.0 { 0.0 } else { x });
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_owned()
    } else {
        s
    }
}

#[derive(Clone, Copy, Debug)]
struct Vector {
    east: f64,
    north: f64,
}

impl Vector {
    fn bearing(degrees: f64, magnitude: f64) -> Self {
        let a = degrees.to_radians();
        Self {
            east: magnitude * a.sin(),
            north: magnitude * a.cos(),
        }
    }

    fn plus(self, rhs: Self) -> Self {
        Self {
            east: self.east + rhs.east,
            north: self.north + rhs.north,
        }
    }

    fn scale(self, factor: f64) -> Self {
        Self {
            east: self.east * factor,
            north: self.north * factor,
        }
    }

    fn speed(self) -> f64 {
        self.east.hypot(self.north)
    }

    fn course(self) -> f64 {
        self.east.atan2(self.north).to_degrees().rem_euclid(360.0)
    }

    fn text(self) -> String {
        format!("({}, {})", n(self.east, 2), n(self.north, 2))
    }
}

struct Leg {
    course: f64,
    water_speed: f64,
    current: Vector,
    minutes: i32,
}

fn reckoning(legs: &[Leg]) -> (Vector, Vector) {
    let mut dr = Vector {
        east: 0.0,
        north: 0.0,
    };
    let mut estimate = dr;
    for leg in legs {
        let water = Vector::bearing(leg.course, leg.water_speed);
        let hours = f64::from(leg.minutes) / 60.0;
        dr = dr.plus(water.scale(hours));
        estimate = estimate.plus(water.plus(leg.current).scale(hours));
    }
    (dr, estimate)
}

fn dead_reckoning(rng: &mut StdRng) -> String {
    let course = f64::from(rng.random_range(0..72) * 5);
    let speed = f64::from(rng.random_range(5..=14));
    let kind = rng.random_range(0..5);
    let current = match kind {
        0 => Vector::bearing(0.0, 0.0),
        1 => Vector::bearing(course + 90.0, speed * 0.4),
        2 => Vector::bearing(course + 180.0, speed - 0.2),
        3 => Vector::bearing(course + 180.0, speed + 1.0),
        _ => Vector::bearing(course + 70.0, speed * 0.35),
    };
    let mut legs = vec![Leg {
        course,
        water_speed: speed,
        current,
        minutes: [20, 30, 45, 60, 90][rng.random_range(0..5)],
    }];
    if kind == 4 {
        legs.push(Leg {
            course: (course + 90.0).rem_euclid(360.0),
            water_speed: speed - 1.0,
            current: current.scale(-1.0),
            minutes: [30, 45, 60][rng.random_range(0..3)],
        });
    }
    let mut out = String::from(
        "Local plane, true bearings, origin E 0 N 0 nmi\nConstant velocity within each leg, no leeway\nLeg  Minutes  Through-water course/speed  Current set/speed",
    );
    for (i, leg) in legs.iter().enumerate() {
        let current = if leg.current.speed() < 1e-10 {
            "calm".to_owned()
        } else {
            format!(
                "{}°T / {} kn",
                n(leg.current.course(), 1),
                n(leg.current.speed(), 1)
            )
        };
        write!(
            out,
            "\n{}    {:<7}  {}°T / {} kn            {current}",
            i + 1,
            leg.minutes,
            n(leg.course, 1),
            n(leg.water_speed, 1)
        )
        .unwrap();
    }
    let (dr, estimated) = reckoning(&legs);
    write!(
        out,
        "\nDR endpoint (E, N) {} nmi\nCurrent-adjusted endpoint {} nmi\nCurrent displacement {} nmi",
        dr.text(), estimated.text(), estimated.plus(dr.scale(-1.0)).text()
    ).unwrap();
    if legs.len() == 1 {
        let ground = Vector::bearing(legs[0].course, legs[0].water_speed).plus(current);
        write!(
            out,
            "\nGround track {}°T at {} kn",
            n(ground.course(), 1),
            n(ground.speed(), 2)
        )
        .unwrap();
    }
    out
}

#[derive(Debug)]
struct Solution {
    heading: f64,
    ground_along: f64,
}

fn steering(track: f64, speed: f64, along: f64, across: f64) -> Vec<Solution> {
    if across.abs() > speed {
        return vec![];
    }
    let delta = (-across / speed).asin().to_degrees();
    let forward = (speed * speed - across * across).max(0.0).sqrt();
    let mut solutions = vec![Solution {
        heading: (track + delta).rem_euclid(360.0),
        ground_along: along + forward,
    }];
    if forward > 1e-10 {
        solutions.push(Solution {
            heading: (track + 180.0 - delta).rem_euclid(360.0),
            ground_along: along - forward,
        });
    }
    solutions
}

fn course_to_steer(rng: &mut StdRng) -> String {
    let track = f64::from(rng.random_range(0..72) * 5);
    let speed = f64::from(rng.random_range(5..=15));
    let (along, across) = match rng.random_range(0..7) {
        0 => (0.2 * speed, 0.4 * speed),
        1 => (0.3 * speed, speed),
        2 => (0.0, speed + 0.2),
        3 => (-speed, 0.0),
        4 => (-speed - 0.2, 0.0),
        5 => (1.5 * speed, 0.6 * speed),
        _ => (-0.2 * speed, -0.8 * speed),
    };
    let current = Vector::bearing(track, along).plus(Vector::bearing(track + 90.0, across));
    let distance = f64::from(rng.random_range(5..=35));
    let mut out = format!(
        "Local plane, steady current, no leeway\nTarget {} nmi along {}°T\nThrough-water speed {} kn\nCurrent set {}°T at {} kn\nCurrent components along track {} kn, to its right {} kn",
        n(distance, 1),
        n(track, 1),
        n(speed, 1),
        n(current.course(), 2),
        n(current.speed(), 2),
        n(along, 1),
        n(across, 1)
    );
    let solutions = steering(track, speed, along, across);
    if solutions.is_empty() {
        write!(
            out,
            "\nNo heading cancels cross-current\nMinimum residual drift {} kn to {} of target track",
            n(across.abs() - speed, 2),
            if across > 0.0 { "right" } else { "left" }
        ).unwrap();
    } else {
        out.push_str("\nHeadings with zero cross-track velocity");
        for solution in solutions {
            write!(out, "\n{}°T → ", n(solution.heading, 2)).unwrap();
            if solution.ground_along > 1e-10 {
                write!(
                    out,
                    "{} kn forward, arrival in {} min",
                    n(solution.ground_along, 2),
                    n(60.0 * distance / solution.ground_along, 1)
                )
                .unwrap();
            } else if solution.ground_along < -1e-10 {
                write!(
                    out,
                    "{} kn on reciprocal {}°T",
                    n(-solution.ground_along, 2),
                    n((track + 180.0).rem_euclid(360.0), 1)
                )
                .unwrap();
            } else {
                out.push_str("stationary over ground");
            }
        }
    }
    out
}

#[derive(Clone, Copy, Debug)]
struct Position {
    lat: f64,
    lon: f64,
}

fn wrap(angle: f64) -> f64 {
    (angle + PI).rem_euclid(2.0 * PI) - PI
}

impl Position {
    fn degrees(lat: f64, lon: f64) -> Self {
        Self {
            lat: lat.to_radians(),
            lon: lon.to_radians(),
        }
    }

    fn xyz(self) -> [f64; 3] {
        [
            self.lat.cos() * self.lon.cos(),
            self.lat.cos() * self.lon.sin(),
            self.lat.sin(),
        ]
    }

    fn text(self) -> String {
        format!(
            "{}°{} {}°{}",
            n(self.lat.to_degrees().abs(), 3),
            if self.lat < 0.0 { "S" } else { "N" },
            n(wrap(self.lon).to_degrees().abs(), 3),
            if wrap(self.lon) < 0.0 { "W" } else { "E" }
        )
    }
}

fn angle(a: Position, b: Position) -> f64 {
    let hav = ((b.lat - a.lat) / 2.0).sin().powi(2)
        + a.lat.cos() * b.lat.cos() * (wrap(b.lon - a.lon) / 2.0).sin().powi(2);
    2.0 * hav.clamp(0.0, 1.0).sqrt().asin()
}

fn initial_course(a: Position, b: Position) -> f64 {
    let lon = wrap(b.lon - a.lon);
    (lon.sin() * b.lat.cos())
        .atan2(a.lat.cos() * b.lat.sin() - a.lat.sin() * b.lat.cos() * lon.cos())
}

fn midpoint(a: Position, b: Position) -> Position {
    let a = a.xyz();
    let b = b.xyz();
    let x = a[0] + b[0];
    let y = a[1] + b[1];
    let z = a[2] + b[2];
    Position {
        lat: z.atan2(x.hypot(y)),
        lon: y.atan2(x),
    }
}

fn rhumb(a: Position, b: Position) -> (f64, f64) {
    let lat = b.lat - a.lat;
    let lon = wrap(b.lon - a.lon);
    let psi = ((FRAC_PI_2 / 2.0 + b.lat / 2.0).tan() / (FRAC_PI_2 / 2.0 + a.lat / 2.0).tan()).ln();
    let q = if psi.abs() > 1e-12 {
        lat / psi
    } else {
        a.lat.cos()
    };
    (RADIUS * lat.hypot(q * lon), lon.atan2(psi))
}

fn endpoints(rng: &mut StdRng, kind: usize) -> (Position, Position) {
    let sign = if rng.random_bool(0.5) { 1.0 } else { -1.0 };
    let longitude = f64::from(rng.random_range(-140..=140));
    let (lat1, lon1, lat2, lon2) = match kind {
        0 => (0.0, longitude, 0.0, longitude + 30.0 * sign),
        1 => (
            f64::from(rng.random_range(-50..=20)),
            longitude,
            55.0,
            longitude,
        ),
        2 => {
            let lat = sign * f64::from(rng.random_range(50..=70));
            (
                lat,
                longitude,
                lat,
                longitude + f64::from(rng.random_range(50..=120)),
            )
        }
        3 => (sign * 35.0, 170.0, sign * 50.0, -170.0),
        4 => {
            let lat = f64::from(rng.random_range(10..=35));
            (lat, longitude, -lat + 2.0, longitude + 178.0 * sign)
        }
        _ => (
            sign * f64::from(rng.random_range(15..=50)),
            longitude,
            -sign * f64::from(rng.random_range(10..=40)),
            longitude + 65.0 * sign,
        ),
    };
    (Position::degrees(lat1, lon1), Position::degrees(lat2, lon2))
}

fn spherical(rng: &mut StdRng) -> String {
    let kind = rng.random_range(0..6);
    let (a, b) = endpoints(rng, kind);
    let great_circle = angle(a, b) * RADIUS;
    let (rhumb_distance, rhumb_course) = rhumb(a, b);
    let initial = initial_course(a, b).to_degrees().rem_euclid(360.0);
    let final_course = (initial_course(b, a).to_degrees() + 180.0).rem_euclid(360.0);
    format!(
        "Sphere, 1° great-circle arc = 60 nmi\nA {}\nB {}\nShorter great-circle arc {} nmi\nInitial course {}°T, arrival course {}°T\nHalfway point {}\nShort-longitude rhumb {} nmi on constant {}°T\nRhumb excess {} nmi",
        a.text(),
        b.text(),
        n(great_circle, 1),
        n(initial, 2),
        n(final_course, 2),
        midpoint(a, b).text(),
        n(rhumb_distance, 1),
        n(rhumb_course.to_degrees().rem_euclid(360.0), 2),
        n(rhumb_distance - great_circle, 1)
    )
}

pub fn generate(rng: &mut StdRng) -> String {
    match rng.random_range(0..3) {
        0 => dead_reckoning(rng),
        1 => course_to_steer(rng),
        _ => spherical(rng),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn near(a: f64, b: f64, tolerance: f64) {
        assert!((a - b).abs() < tolerance, "{a} != {b}");
    }

    #[test]
    fn currents_add_displacement_with_correct_east_north_signs() {
        let legs = [
            Leg {
                course: 90.0,
                water_speed: 10.0,
                current: Vector {
                    east: -2.0,
                    north: 3.0,
                },
                minutes: 30,
            },
            Leg {
                course: 180.0,
                water_speed: 6.0,
                current: Vector {
                    east: 1.0,
                    north: -2.0,
                },
                minutes: 60,
            },
        ];
        let (dr, estimate) = reckoning(&legs);
        near(dr.east, 5.0, 1e-12);
        near(dr.north, -6.0, 1e-12);
        near(estimate.east, 5.0, 1e-12);
        near(estimate.north, -6.5, 1e-12);
    }

    #[test]
    fn steering_roots_close_the_velocity_triangle() {
        for track in [0.0, 35.0, 90.0, 185.0, 275.0, 355.0] {
            for along in [-10.2, -10.0, -3.0, 0.0, 15.0] {
                for across in [-10.2, -10.0, -8.0, 0.0, 6.0, 10.0, 10.2] {
                    let solutions = steering(track, 10.0, along, across);
                    if f64::abs(across) > 10.0 {
                        assert!(solutions.is_empty());
                        continue;
                    }
                    assert_eq!(
                        solutions.len(),
                        if f64::abs(across) == 10.0 { 1 } else { 2 }
                    );
                    let current =
                        Vector::bearing(track, along).plus(Vector::bearing(track + 90.0, across));
                    for solution in solutions {
                        let ground = Vector::bearing(solution.heading, 10.0).plus(current);
                        let expected = Vector::bearing(track, solution.ground_along);
                        near(ground.east, expected.east, 1e-10);
                        near(ground.north, expected.north, 1e-10);
                    }
                }
            }
        }
        assert_eq!(steering(0.0, 10.0, -10.0, 0.0)[0].ground_along, 0.0);
        assert!(
            steering(0.0, 10.0, 15.0, 6.0)
                .iter()
                .all(|s| s.ground_along > 0.0)
        );
    }

    #[test]
    fn spherical_routes_close_and_great_circle_is_shortest() {
        for kind in 0..6 {
            for seed in 0..256 {
                let (a, b) = endpoints(&mut StdRng::seed_from_u64(seed), kind);
                let arc = angle(a, b);
                let middle = midpoint(a, b);
                near(angle(a, middle), arc / 2.0, 1e-11);
                near(angle(middle, b), arc / 2.0, 1e-11);
                // Independent Cartesian dot-product distance and forward geodesic endpoint.
                let av = a.xyz();
                let bv = b.xyz();
                let dot = av.iter().zip(bv).map(|(x, y)| x * y).sum::<f64>();
                near(dot.clamp(-1.0, 1.0).acos(), arc, 1e-11);
                let c = initial_course(a, b);
                let lat = (a.lat.sin() * arc.cos() + a.lat.cos() * arc.sin() * c.cos()).asin();
                let lon = a.lon
                    + (c.sin() * arc.sin() * a.lat.cos())
                        .atan2(arc.cos() - a.lat.sin() * lat.sin());
                near(lat, b.lat, 1e-11);
                near(wrap(lon - b.lon), 0.0, 1e-11);
                let (distance, _) = rhumb(a, b);
                assert!(distance + 1e-7 >= arc * RADIUS);
                near(rhumb(b, a).0, distance, 1e-8);
                if kind < 2 {
                    near(distance, arc * RADIUS, 1e-8);
                }
            }
        }
    }

    #[test]
    fn date_line_uses_short_longitude_difference() {
        let a = Position::degrees(0.0, 179.0);
        let b = Position::degrees(0.0, -179.0);
        near(rhumb(a, b).0, 120.0, 1e-9);
        near(angle(a, b) * RADIUS, 120.0, 1e-9);
        near(rhumb(a, b).1.to_degrees(), 90.0, 1e-9);
    }
}
