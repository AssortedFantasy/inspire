use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

#[derive(Clone, Copy, Debug)]
struct Point {
    east: f64,
    north: f64,
}

impl Point {
    fn along(bearing: f64, distance: f64) -> Self {
        let a = bearing.to_radians();
        Self {
            east: distance * a.sin(),
            north: distance * a.cos(),
        }
    }

    fn plus(self, rhs: Self) -> Self {
        Self {
            east: self.east + rhs.east,
            north: self.north + rhs.north,
        }
    }

    fn minus(self, rhs: Self) -> Self {
        Self {
            east: self.east - rhs.east,
            north: self.north - rhs.north,
        }
    }

    fn dot(self, rhs: Self) -> f64 {
        self.east * rhs.east + self.north * rhs.north
    }

    fn text(self) -> String {
        format!("({}, {})", number(self.east), number(self.north))
    }
}

fn number(x: f64) -> String {
    let rounded = (x * 10.0).round() / 10.0;
    let s = format!("{:.1}", if rounded == 0.0 { 0.0 } else { rounded });
    s.trim_end_matches('0').trim_end_matches('.').to_owned()
}

#[derive(Clone, Copy, Debug)]
enum Region {
    A,
    B,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Side {
    Port,
    Starboard,
}

impl Side {
    fn opposite(self) -> Self {
        match self {
            Self::Port => Self::Starboard,
            Self::Starboard => Self::Port,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Port => "port",
            Self::Starboard => "starboard",
        }
    }

    fn sign(self) -> f64 {
        match self {
            Self::Port => -1.0,
            Self::Starboard => 1.0,
        }
    }

    fn topmark(self) -> &'static str {
        match self {
            Self::Port => "cylinder",
            Self::Starboard => "cone pointing up",
        }
    }
}

fn colour(region: Region, side: Side) -> &'static str {
    match (region, side) {
        (Region::A, Side::Port) | (Region::B, Side::Starboard) => "red",
        _ => "green",
    }
}

fn region(rng: &mut StdRng) -> Region {
    if rng.random_bool(0.5) {
        Region::A
    } else {
        Region::B
    }
}

fn local(bearing: f64, along: f64, right: f64) -> Point {
    Point::along(bearing, along).plus(Point::along(bearing + 90.0, right))
}

struct Channel {
    region: Region,
    bearing: f64,
    outbound: bool,
    half_widths: [f64; 3],
    lengths: [f64; 3],
    offset: f64,
    beam: f64,
}

impl Channel {
    fn new(rng: &mut StdRng) -> Self {
        let beam = f64::from(rng.random_range(4..=12));
        let margin = [1.0, 2.0, 10.0, 18.0][rng.random_range(0..4)];
        let offset = f64::from(rng.random_range(-8..=8));
        let pinch = beam / 2.0 + margin + offset.abs();
        let widths = if rng.random_bool(0.65) {
            [
                pinch + f64::from(rng.random_range(12..=30)),
                pinch,
                pinch + f64::from(rng.random_range(6..=25)),
            ]
        } else {
            [pinch; 3]
        };
        Self {
            region: region(rng),
            bearing: f64::from(rng.random_range(0..24) * 15),
            outbound: rng.random_bool(0.5),
            half_widths: widths,
            lengths: [0.0, f64::from(rng.random_range(150..=300)), 600.0],
            offset,
            beam,
        }
    }

    fn clearance(&self) -> f64 {
        self.half_widths
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min)
            - self.offset.abs()
            - self.beam / 2.0
    }

    fn text(&self) -> String {
        let course = (self.bearing + if self.outbound { 180.0 } else { 0.0 }) % 360.0;
        let mut out = format!(
            "Schematic channel, IALA Region {:?}\nCoordinates (east, north) in m, true bearings\nCharted buoyage direction {}°T\nVessel course {}°T, beam {} m, centreline {} m right of buoyage axis\nBanks join successive paired marks with straight segments",
            self.region,
            number(self.bearing),
            number(course),
            number(self.beam),
            number(self.offset)
        );
        for side in [Side::Port, Side::Starboard] {
            let c = colour(self.region, side);
            let leave = if self.outbound { side.opposite() } else { side };
            write!(
                out,
                "\n{} marks: {c} pillars, {c} {} topmarks, unlit. Leave to {}",
                side.name(),
                side.topmark(),
                leave.name()
            )
            .unwrap();
        }
        out.push_str("\nPair    Port mark             Starboard mark");
        for i in 0..3 {
            let port = local(self.bearing, self.lengths[i], -self.half_widths[i]);
            let starboard = local(self.bearing, self.lengths[i], self.half_widths[i]);
            write!(
                out,
                "\n{}       {:<22}{}",
                i + 1,
                port.text(),
                starboard.text()
            )
            .unwrap();
        }
        let (start, end) = if self.outbound {
            (600.0, 0.0)
        } else {
            (0.0, 600.0)
        };
        write!(
            out,
            "\nVessel track {} → {}\nMinimum transverse bank clearance {} m",
            local(self.bearing, start, self.offset).text(),
            local(self.bearing, end, self.offset).text(),
            number(self.clearance())
        )
        .unwrap();
        out
    }
}

struct Cardinal {
    name: &'static str,
    bearing: f64,
    bands: &'static str,
    cones: &'static str,
    quick: &'static str,
    very_quick: &'static str,
}

const CARDINALS: [Cardinal; 4] = [
    Cardinal {
        name: "North",
        bearing: 0.0,
        bands: "black above yellow",
        cones: "both pointing up",
        quick: "Q",
        very_quick: "VQ",
    },
    Cardinal {
        name: "East",
        bearing: 90.0,
        bands: "black-yellow-black horizontal bands",
        cones: "base to base",
        quick: "Q(3) 10s",
        very_quick: "VQ(3) 5s",
    },
    Cardinal {
        name: "South",
        bearing: 180.0,
        bands: "yellow above black",
        cones: "both pointing down",
        quick: "Q(6)+LFl 15s",
        very_quick: "VQ(6)+LFl 10s",
    },
    Cardinal {
        name: "West",
        bearing: 270.0,
        bands: "yellow-black-yellow horizontal bands",
        cones: "point to point",
        quick: "Q(9) 15s",
        very_quick: "VQ(9) 10s",
    },
];

fn segment_distance(point: Point, a: Point, b: Point) -> f64 {
    let ab = b.minus(a);
    let ap = point.minus(a);
    let t = (ap.dot(ab) / ab.dot(ab)).clamp(0.0, 1.0);
    let nearest = Point {
        east: a.east + t * ab.east,
        north: a.north + t * ab.north,
    };
    let displacement = point.minus(nearest);
    displacement.east.hypot(displacement.north)
}

struct Shoal {
    cardinal: usize,
    centre: Point,
    radius: f64,
    mark: Point,
    route: [Point; 2],
    beam: f64,
    very_quick: bool,
}

impl Shoal {
    fn new(rng: &mut StdRng) -> Self {
        let cardinal = rng.random_range(0..4);
        let direction = CARDINALS[cardinal].bearing;
        let centre = Point {
            east: f64::from(rng.random_range(-100..=100)),
            north: f64::from(rng.random_range(-100..=100)),
        };
        let radius = f64::from(rng.random_range(30..=100));
        let gap = f64::from(rng.random_range(5..=20));
        let mark = centre.plus(Point::along(direction, radius + gap));
        let beam = f64::from(rng.random_range(4..=12));
        let outward = beam / 2.0 + [2.0, 5.0, 20.0][rng.random_range(0..3)];
        let crossing = mark.plus(Point::along(direction, outward));
        let span = radius + 80.0;
        let mut route = [
            crossing.plus(Point::along(direction + 90.0, -span)),
            crossing.plus(Point::along(direction + 90.0, span)),
        ];
        if rng.random_bool(0.5) {
            route.swap(0, 1);
        }
        Self {
            cardinal,
            centre,
            radius,
            mark,
            route,
            beam,
            very_quick: rng.random_bool(0.5),
        }
    }

    fn clearance(&self) -> f64 {
        segment_distance(self.centre, self.route[0], self.route[1]) - self.radius - self.beam / 2.0
    }

    fn text(&self) -> String {
        let c = &CARDINALS[self.cardinal];
        format!(
            "Schematic shoal, IALA cardinal buoyage\nCoordinates (east, north) in m\nCircular shoal centre {}, radius {} m\n{} cardinal pillar at {}\n{}\nTwo black cones, {}\nWhite {}\nTrack {} → {}, beam {} m\nPass on the {} side of the mark\nMinimum hull clearance to shoal {} m",
            self.centre.text(),
            number(self.radius),
            c.name,
            self.mark.text(),
            c.bands,
            c.cones,
            if self.very_quick {
                c.very_quick
            } else {
                c.quick
            },
            self.route[0].text(),
            self.route[1].text(),
            number(self.beam),
            c.name.to_lowercase(),
            number(self.clearance())
        )
    }
}

struct Fork {
    region: Region,
    bearing: f64,
    preferred: Side,
    route: [Point; 3],
    other_exit: Point,
    beam: f64,
}

impl Fork {
    fn new(rng: &mut StdRng) -> Self {
        let bearing = f64::from(rng.random_range(0..24) * 15);
        let preferred = if rng.random_bool(0.5) {
            Side::Port
        } else {
            Side::Starboard
        };
        let beam = f64::from(rng.random_range(4..=12));
        let pass = beam / 2.0 + f64::from(rng.random_range(8..=25));
        let length = f64::from(rng.random_range(120..=280));
        let divergence = f64::from(rng.random_range(25..=50)).to_radians().tan() * length;
        Self {
            region: region(rng),
            bearing,
            preferred,
            route: [
                local(bearing, -length, 0.0),
                local(bearing, 0.0, preferred.sign() * pass),
                local(bearing, length, preferred.sign() * (pass + divergence)),
            ],
            other_exit: local(bearing, length, -preferred.sign() * divergence),
            beam,
        }
    }

    fn clearance(&self) -> f64 {
        let origin = Point {
            east: 0.0,
            north: 0.0,
        };
        self.route
            .windows(2)
            .map(|leg| segment_distance(origin, leg[0], leg[1]))
            .fold(f64::INFINITY, f64::min)
            - self.beam / 2.0
    }

    fn text(&self) -> String {
        let mark_side = self.preferred.opposite();
        let body = colour(self.region, mark_side);
        let band = colour(self.region, self.preferred);
        format!(
            "Schematic bifurcation, IALA Region {:?}\nCoordinates (east, north) in m, true bearings\nCharted buoyage direction {}°T\nPreferred channel to {}\nModified lateral pillar at (0, 0)\n{body} with one broad horizontal {band} band\n{body} {} topmark, {body} Fl(2+1) light\nPreferred track {} → {} → {}\nSecondary channel exit {}\nBeam {} m, minimum hull distance from point mark {} m\nLeave mark to {} on preferred track",
            self.region,
            number(self.bearing),
            self.preferred.name(),
            mark_side.topmark(),
            self.route[0].text(),
            self.route[1].text(),
            self.route[2].text(),
            self.other_exit.text(),
            number(self.beam),
            number(self.clearance()),
            mark_side.name()
        )
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    match rng.random_range(0..3) {
        0 => Channel::new(rng).text(),
        1 => Shoal::new(rng).text(),
        _ => Fork::new(rng).text(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn lateral_colours_change_by_region_but_shapes_do_not() {
        assert_eq!(colour(Region::A, Side::Port), "red");
        assert_eq!(colour(Region::A, Side::Starboard), "green");
        assert_eq!(colour(Region::B, Side::Port), "green");
        assert_eq!(colour(Region::B, Side::Starboard), "red");
        assert_eq!(Side::Port.topmark(), "cylinder");
        assert_eq!(Side::Starboard.topmark(), "cone pointing up");
    }

    #[test]
    fn generated_channel_tracks_fit_and_marks_have_correct_signed_sides() {
        for seed in 0..512 {
            let c = Channel::new(&mut StdRng::seed_from_u64(seed));
            assert!(c.clearance() >= 1.0);
            let right = Point::along(c.bearing + 90.0, 1.0);
            for i in 0..3 {
                for side in [Side::Port, Side::Starboard] {
                    let p = local(c.bearing, c.lengths[i], side.sign() * c.half_widths[i]);
                    assert!((p.dot(right) - side.sign() * c.half_widths[i]).abs() < 1e-9);
                    let vessel_right = if c.outbound {
                        Point::along(c.bearing - 90.0, 1.0)
                    } else {
                        right
                    };
                    let relative = p.minus(local(c.bearing, c.lengths[i], c.offset));
                    let leave = if c.outbound { side.opposite() } else { side };
                    assert!(relative.dot(vessel_right) * leave.sign() > 0.0);
                }
            }
        }
    }

    #[test]
    fn shoal_clearance_and_named_side_are_geometric() {
        for seed in 0..512 {
            let s = Shoal::new(&mut StdRng::seed_from_u64(seed));
            let c = &CARDINALS[s.cardinal];
            let outward = Point::along(c.bearing, 1.0);
            assert!(s.mark.minus(s.centre).dot(outward) > s.radius);
            for p in s.route {
                assert!(p.minus(s.mark).dot(outward) > s.beam / 2.0);
            }
            assert!(s.clearance() >= 7.0 - 1e-9);
            let closest = segment_distance(s.centre, s.route[0], s.route[1]);
            let midpoint = Point {
                east: (s.route[0].east + s.route[1].east) / 2.0,
                north: (s.route[0].north + s.route[1].north) / 2.0,
            };
            let v = midpoint.minus(s.centre);
            assert!((closest - v.east.hypot(v.north)).abs() < 1e-9);
        }
    }

    #[test]
    fn preferred_fork_passes_correct_side_without_crossing_the_mark() {
        for seed in 0..512 {
            let f = Fork::new(&mut StdRng::seed_from_u64(seed));
            let right = Point::along(f.bearing + 90.0, 1.0);
            assert!(f.route[1].dot(right) * f.preferred.sign() > 0.0);
            assert!(f.route[2].dot(right) * f.preferred.sign() > 0.0);
            assert!(f.other_exit.dot(right) * f.preferred.sign() < 0.0);
            assert!(f.clearance() > 7.0);
            // Cross product of the first route leg and vector to the mark gives its passing side.
            let velocity = f.route[1].minus(f.route[0]);
            let to_mark = Point {
                east: -f.route[0].east,
                north: -f.route[0].north,
            };
            let starboard_component = velocity.north * to_mark.east - velocity.east * to_mark.north;
            assert!(starboard_component * f.preferred.opposite().sign() > 0.0);
        }
    }
}
