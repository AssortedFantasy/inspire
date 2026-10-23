use rand::{Rng, rngs::StdRng};
use std::f64::consts::PI;
use std::fmt::Write;

include!("../../number.rs");

const SPEED: f64 = 343.0;
const AXES: [&str; 3] = ["x", "y", "z"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    Axial,
    Tangential,
    Oblique,
}

impl Family {
    fn name(self) -> &'static str {
        match self {
            Self::Axial => "axial",
            Self::Tangential => "tangential",
            Self::Oblique => "oblique",
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Mode {
    orders: [u32; 3],
    squared: f64,
}

impl Mode {
    fn family(self) -> Family {
        match self.orders.iter().filter(|&&order| order != 0).count() {
            1 => Family::Axial,
            2 => Family::Tangential,
            3 => Family::Oblique,
            _ => unreachable!("the static pressure field is not a resonance"),
        }
    }

    fn frequency(self) -> f64 {
        0.5 * SPEED * self.squared.sqrt()
    }

    fn label(self) -> String {
        format!("({},{},{})", self.orders[0], self.orders[1], self.orders[2])
    }
}

struct Room {
    lengths: [f64; 3],
    description: &'static str,
    pair: Option<[[u32; 3]; 2]>,
    crossing_height: Option<f64>,
    exact_crossing: bool,
}

impl Room {
    fn mode(&self, orders: [u32; 3]) -> Mode {
        let mut terms = std::array::from_fn::<_, 3, _>(|axis| {
            (orders[axis] as f64 / self.lengths[axis]).powi(2)
        });
        // Permuting equal room edges must not change a sum's rounding order.
        terms.sort_by(f64::total_cmp);
        Mode {
            orders,
            squared: terms.into_iter().sum(),
        }
    }

    fn pressure(&self, mode: Mode, point: [f64; 3]) -> f64 {
        (0..3)
            .map(|axis| (PI * mode.orders[axis] as f64 * point[axis] / self.lengths[axis]).cos())
            .product()
    }

    fn lowest(&self, count: u32, family: Option<Family>) -> Vec<Mode> {
        let mut modes = Vec::new();
        // An index > count has count strictly lower modes obtained by replacing
        // that index with 1..=count, even with a fixed family. No omitted mode
        // can reach the cutoff, regardless of the room's aspect ratio.
        for x in 0..=count {
            for y in 0..=count {
                for z in 0..=count {
                    if x + y + z == 0 {
                        continue;
                    }
                    let mode = self.mode([x, y, z]);
                    if family.is_none_or(|family| family == mode.family()) {
                        modes.push(mode);
                    }
                }
            }
        }
        modes.sort_by(|a, b| {
            a.squared
                .total_cmp(&b.squared)
                .then_with(|| a.orders.cmp(&b.orders))
        });
        let cutoff = modes[count as usize - 1].squared;
        modes.retain(|mode| mode.squared <= cutoff || coincident(mode.squared, cutoff));
        modes
    }
}

fn coincident(a: f64, b: f64) -> bool {
    (a - b).abs() <= 64.0 * f64::EPSILON * a.max(b)
}

fn sample_regime(rng: &mut StdRng, regime: u32) -> Room {
    let mut room = Room {
        lengths: [0.0; 3],
        description: "Rectangular room with rigid walls",
        pair: None,
        crossing_height: None,
        exact_crossing: false,
    };
    match regime {
        0 => {
            room.lengths = [
                rng.random_range(41..=89) as f64 / 10.0,
                rng.random_range(28..=53) as f64 / 10.0,
                rng.random_range(23..=35) as f64 / 10.0,
            ];
        }
        1 => {
            let edge = rng.random_range(28..=62) as f64 / 10.0;
            room.lengths = [edge; 3];
            room.description = "Rigid-walled cube";
            room.pair = Some([[1, 0, 0], [0, 1, 0]]);
        }
        2 | 3 => {
            let edge = rng.random_range(36..=72) as f64 / 10.0;
            let offset = if regime == 3 {
                rng.random_range(2..=18) as f64 / 1000.0
            } else {
                0.0
            };
            room.lengths = [
                edge,
                edge * (1.0 + offset),
                rng.random_range(22..=34) as f64 / 10.0,
            ];
            room.description = if regime == 2 {
                "Rigid-walled room with a square floor"
            } else {
                "Rigid-walled room with a nearly square floor"
            };
            room.pair = Some([[1, 0, 0], [0, 1, 0]]);
        }
        4 => {
            let height = rng.random_range(22..=36) as f64 / 10.0;
            room.lengths = [3.0 * height, 2.0 * height, height];
            room.description = "Rigid-walled room with edges in the ratio 3:2:1";
            room.pair = Some([[3, 0, 0], [0, 2, 0]]);
        }
        5 => {
            room.lengths = [
                rng.random_range(120..=260) as f64 / 10.0,
                rng.random_range(14..=22) as f64 / 10.0,
                rng.random_range(23..=31) as f64 / 10.0,
            ];
            room.description = "Rigid-walled corridor";
        }
        6 => {
            room.lengths = [
                rng.random_range(90..=150) as f64 / 10.0,
                rng.random_range(50..=80) as f64 / 10.0,
                rng.random_range(21..=27) as f64 / 10.0,
            ];
            room.description = "Rigid-walled room with a low ceiling";
        }
        7 | 8 => {
            let width = rng.random_range(28..=42) as f64 / 10.0;
            let length = width * rng.random_range(140..=190) as f64 / 100.0;
            let crossing = 1.0 / (1.0 / length.powi(2) + 1.0 / width.powi(2)).sqrt();
            let offset = if regime == 8 {
                let sign = if rng.random_bool(0.5) { -1.0 } else { 1.0 };
                sign * rng.random_range(3..=25) as f64 / 1000.0
            } else {
                0.0
            };
            room.lengths = [length, width, crossing * (1.0 + offset)];
            room.description = if regime == 7 {
                "Rigid-walled room at an axial–tangential coincidence"
            } else {
                "Rigid-walled room near an axial–tangential crossing"
            };
            room.pair = Some([[0, 0, 1], [1, 1, 0]]);
            room.crossing_height = Some(crossing);
            room.exact_crossing = regime == 7;
        }
        _ => unreachable!(),
    }
    room
}

fn nodes(order: u32, length: f64) -> Vec<f64> {
    (0..order)
        .map(|j| (2 * j + 1) as f64 * length / (2 * order) as f64)
        .collect()
}

fn antinodes(order: u32, length: f64) -> Vec<f64> {
    (0..=order)
        .map(|j| j as f64 * length / order as f64)
        .collect()
}

fn positions(values: &[f64]) -> String {
    values
        .iter()
        .map(|value| n(*value))
        .collect::<Vec<_>>()
        .join(", ")
}

fn shape(mode: Mode) -> String {
    mode.orders
        .iter()
        .zip(AXES)
        .filter(|(order, _)| **order != 0)
        .map(|(&order, axis)| {
            if order == 1 {
                format!("cos(π{axis}/L{axis})")
            } else {
                format!("cos({order}π{axis}/L{axis})")
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn spectrum(out: &mut String, modes: &[Mode]) {
    out.push_str("\nf (Hz) | pressure modes");
    let mut start = 0;
    while start < modes.len() {
        let first = modes[start];
        let end = (start + 1..modes.len())
            .find(|&index| !coincident(first.squared, modes[index].squared))
            .unwrap_or(modes.len());
        let labels = modes[start..end]
            .iter()
            .map(|mode| format!("{} {}", mode.label(), mode.family().name()))
            .collect::<Vec<_>>()
            .join(", ");
        write!(out, "\n{} | {labels}", n(first.frequency())).unwrap();
        start = end;
    }
}

fn geometry(out: &mut String, room: &Room, mode: Mode) {
    write!(
        out,
        "\n\n{} {} mode at {} Hz\np/p₀ = {}",
        mode.label(),
        mode.family().name(),
        n(mode.frequency()),
        shape(mode)
    )
    .unwrap();
    out.push_str("\nPressure-node planes");
    for (axis, &order) in mode.orders.iter().enumerate() {
        if order != 0 {
            write!(
                out,
                "\n{} = [{}] m",
                AXES[axis],
                positions(&nodes(order, room.lengths[axis]))
            )
            .unwrap();
        }
    }
    out.push_str("\nPressure antinodes at every combination of");
    for (axis, &order) in mode.orders.iter().enumerate() {
        if order == 0 {
            write!(out, "\n{} anywhere in the room", AXES[axis]).unwrap();
        } else {
            write!(
                out,
                "\n{} = [{}] m",
                AXES[axis],
                positions(&antinodes(order, room.lengths[axis]))
            )
            .unwrap();
        }
    }
    let center = room.lengths.map(|length| 0.5 * length);
    write!(out, "\np(center)/p₀ = {}", n(room.pressure(mode, center))).unwrap();
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..9);
    let room = sample_regime(rng, regime);
    let mut out = format!(
        "{}\nLx = {} m, Ly = {} m, Lz = {} m\nc = 343 m/s",
        room.description,
        n(room.lengths[0]),
        n(room.lengths[1]),
        n(room.lengths[2])
    );
    if let Some(height) = room.crossing_height {
        write!(
            out,
            "\n{} = 1/√(Lx⁻² + Ly⁻²) ≈ {} m",
            if room.exact_crossing {
                "Lz"
            } else {
                "Crossing height"
            },
            n(height)
        )
        .unwrap();
    }
    match rng.random_range(0..5) {
        0 | 1 => {
            out.push_str("\n\nLowest 12 modes, including ties at the cutoff");
            spectrum(&mut out, &room.lowest(12, None));
        }
        2 => {
            out.push_str("\n\nLowest 3 modes of each family, including cutoff ties");
            let mut modes = Vec::new();
            for family in [Family::Axial, Family::Tangential, Family::Oblique] {
                modes.extend(room.lowest(3, Some(family)));
            }
            modes.sort_by(|a, b| {
                a.squared
                    .total_cmp(&b.squared)
                    .then_with(|| a.orders.cmp(&b.orders))
            });
            spectrum(&mut out, &modes);
        }
        3 if room.pair.is_some() => {
            let modes = room.pair.unwrap().map(|orders| room.mode(orders));
            out.push_str("\n\nMode pair");
            let mut sorted = modes;
            sorted.sort_by(|a, b| a.squared.total_cmp(&b.squared));
            spectrum(&mut out, &sorted);
            let gap = (modes[0].frequency() - modes[1].frequency()).abs();
            write!(
                out,
                "\nFrequency separation = {} Hz\n{} pressure shape = {}\n{} pressure shape = {}",
                n(if coincident(modes[0].squared, modes[1].squared) {
                    0.0
                } else {
                    gap
                }),
                modes[0].label(),
                shape(modes[0]),
                modes[1].label(),
                shape(modes[1])
            )
            .unwrap();
        }
        _ => {
            let family =
                [Family::Axial, Family::Tangential, Family::Oblique][rng.random_range(0..3)];
            let modes = room.lowest(4, Some(family));
            let mode = modes[rng.random_range(0..modes.len())];
            geometry(&mut out, &room, mode);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::BTreeSet;

    fn room(lengths: [f64; 3]) -> Room {
        Room {
            lengths,
            description: "",
            pair: None,
            crossing_height: None,
            exact_crossing: false,
        }
    }

    fn close(a: f64, b: f64) {
        assert!(
            (a - b).abs() <= 1e-10 * a.abs().max(b.abs()).max(1.0),
            "{a} != {b}"
        );
    }

    #[test]
    fn cubic_shells_and_filtered_degeneracies_are_complete() {
        let room = room([5.0; 3]);
        let f0 = SPEED / 10.0;
        for (family, count, shells) in [
            (None, 4, vec![(1, 3), (2, 3)]),
            (None, 8, vec![(1, 3), (2, 3), (3, 1), (4, 3)]),
            (None, 12, vec![(1, 3), (2, 3), (3, 1), (4, 3), (5, 6)]),
            (Some(Family::Axial), 4, vec![(1, 3), (4, 3)]),
            (Some(Family::Tangential), 4, vec![(2, 3), (5, 6)]),
            (Some(Family::Oblique), 5, vec![(3, 1), (6, 3), (9, 3)]),
        ] {
            let modes = room.lowest(count, family);
            assert_eq!(
                modes.len(),
                shells.iter().map(|(_, count)| count).sum::<usize>()
            );
            for (shell, multiplicity) in shells {
                assert_eq!(
                    modes
                        .iter()
                        .filter(|mode| {
                            ((mode.frequency() / f0).powi(2) - shell as f64).abs() < 1e-10
                        })
                        .count(),
                    multiplicity
                );
            }
        }
    }

    #[test]
    fn corridor_lowest_modes_are_not_limited_by_a_small_fixed_order() {
        let room = room([100.0, 1.0, 1.0]);
        let modes = room.lowest(12, None);
        assert_eq!(modes.len(), 12);
        for (index, mode) in modes.iter().enumerate() {
            assert_eq!(mode.orders, [index as u32 + 1, 0, 0]);
        }
        let oblique = room.lowest(4, Some(Family::Oblique));
        for (index, mode) in oblique.iter().enumerate() {
            assert_eq!(mode.orders, [index as u32 + 1, 1, 1]);
        }
        let tangential = room.lowest(3, Some(Family::Tangential));
        assert_eq!(tangential.len(), 4);
        assert_eq!(
            tangential
                .iter()
                .map(|mode| mode.orders)
                .collect::<BTreeSet<_>>(),
            BTreeSet::from([[1, 1, 0], [1, 0, 1], [2, 1, 0], [2, 0, 1]])
        );
    }

    #[test]
    fn reference_room_order_distinguishes_frequencies_with_the_same_display_rounding() {
        // COMSOL's empty 5 × 4 × 2.6 m room, c = 343 m/s.
        let room = room([5.0, 4.0, 2.6]);
        let expected = [
            [1, 0, 0],
            [0, 1, 0],
            [1, 1, 0],
            [0, 0, 1],
            [2, 0, 0],
            [1, 0, 1],
            [0, 1, 1],
            [2, 1, 0],
            [0, 2, 0],
            [1, 1, 1],
            [1, 2, 0],
            [2, 0, 1],
        ];
        assert_eq!(
            room.lowest(12, None)
                .iter()
                .map(|mode| mode.orders)
                .collect::<Vec<_>>(),
            expected
        );
        close(room.mode([1, 0, 0]).frequency(), 34.3);
        close(room.mode([0, 2, 0]).frequency(), 85.75);
        assert!(!coincident(
            room.mode([0, 2, 0]).squared,
            room.mode([1, 1, 1]).squared
        ));
    }

    #[test]
    fn exact_and_near_coincidences_remain_distinct() {
        let near = room([5.0, 5.0, 5.001]);
        let modes = near.lowest(1, None);
        assert_eq!(modes.len(), 1);
        assert_eq!(modes[0].orders, [0, 0, 1]);
        for seed in 0..128 {
            let exact = sample_regime(&mut StdRng::seed_from_u64(seed), 7);
            let pair = exact.pair.unwrap().map(|orders| exact.mode(orders));
            assert!(coincident(pair[0].squared, pair[1].squared));
            let crossed = sample_regime(&mut StdRng::seed_from_u64(seed), 8);
            let pair = crossed.pair.unwrap().map(|orders| crossed.mode(orders));
            assert!(!coincident(pair[0].squared, pair[1].squared));
            assert_eq!(
                pair[0].frequency() < pair[1].frequency(),
                crossed.lengths[2] > crossed.crossing_height.unwrap()
            );
            let integer = sample_regime(&mut StdRng::seed_from_u64(seed), 4);
            close(
                integer.mode([3, 0, 0]).frequency(),
                integer.mode([0, 2, 0]).frequency(),
            );
            close(
                integer.mode([3, 0, 0]).frequency(),
                integer.mode([0, 0, 1]).frequency(),
            );
        }
    }

    #[test]
    fn ordering_and_cutoff_agree_with_a_larger_independent_lattice() {
        for regime in 0..9 {
            for seed in 0..8 {
                let room = sample_regime(&mut StdRng::seed_from_u64(seed), regime);
                for family in [
                    None,
                    Some(Family::Axial),
                    Some(Family::Tangential),
                    Some(Family::Oblique),
                ] {
                    let count = if family.is_none() { 12 } else { 4 };
                    let modes = room.lowest(count, family);
                    let cutoff = modes[count as usize - 1].squared;
                    assert!(
                        modes
                            .windows(2)
                            .all(|pair| pair[0].squared <= pair[1].squared)
                    );
                    let actual = modes
                        .iter()
                        .map(|mode| mode.orders)
                        .collect::<BTreeSet<_>>();
                    assert_eq!(actual.len(), modes.len());
                    let mut expected = BTreeSet::new();
                    for x in 0..=20 {
                        for y in 0..=20 {
                            for z in 0..=20 {
                                let orders = [x, y, z];
                                let active = orders.iter().filter(|&&order| order != 0).count();
                                if active == 0
                                    || family.is_some_and(|family| {
                                        active
                                            != match family {
                                                Family::Axial => 1,
                                                Family::Tangential => 2,
                                                Family::Oblique => 3,
                                            }
                                    })
                                {
                                    continue;
                                }
                                let squared = (x as f64).powi(2) / room.lengths[0].powi(2)
                                    + (y as f64).powi(2) / room.lengths[1].powi(2)
                                    + (z as f64).powi(2) / room.lengths[2].powi(2);
                                if squared < cutoff || coincident(squared, cutoff) {
                                    expected.insert(orders);
                                }
                            }
                        }
                    }
                    assert_eq!(
                        actual, expected,
                        "regime {regime}, seed {seed}, family {family:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn pressure_geometry_satisfies_helmholtz_and_rigid_boundaries() {
        for regime in 0..9 {
            let room = sample_regime(&mut StdRng::seed_from_u64(regime as u64), regime);
            for family in [Family::Axial, Family::Tangential, Family::Oblique] {
                for mode in room.lowest(4, Some(family)) {
                    let point = std::array::from_fn(|axis| {
                        room.lengths[axis] * [0.213, 0.379, 0.631][axis]
                    });
                    let pressure = room.pressure(mode, point);
                    let mut laplacian = 0.0;
                    for axis in 0..3 {
                        let step = room.lengths[axis] * 1e-4;
                        let mut plus = point;
                        let mut minus = point;
                        plus[axis] += step;
                        minus[axis] -= step;
                        laplacian += (room.pressure(mode, plus) - 2.0 * pressure
                            + room.pressure(mode, minus))
                            / step.powi(2);
                        for boundary in [0.0, room.lengths[axis]] {
                            plus[axis] = boundary + step;
                            minus[axis] = boundary - step;
                            close(room.pressure(mode, plus), room.pressure(mode, minus));
                        }
                        for node in nodes(mode.orders[axis], room.lengths[axis]) {
                            let mut at_node = point;
                            at_node[axis] = node;
                            assert!(room.pressure(mode, at_node).abs() < 1e-12);
                        }
                        if mode.orders[axis] != 0 {
                            for peak in antinodes(mode.orders[axis], room.lengths[axis]) {
                                let mut at_peak = [0.0; 3];
                                at_peak[axis] = peak;
                                close(room.pressure(mode, at_peak).abs(), 1.0);
                            }
                        }
                    }
                    let k_squared = (2.0 * PI * mode.frequency() / SPEED).powi(2);
                    assert!((laplacian + k_squared * pressure).abs() < 1e-6 * k_squared.max(1.0));
                    for corner in 0..8 {
                        let point = std::array::from_fn(|axis| {
                            if corner & (1 << axis) == 0 {
                                0.0
                            } else {
                                room.lengths[axis]
                            }
                        });
                        close(room.pressure(mode, point).abs(), 1.0);
                    }
                }
            }
        }
    }

    #[test]
    fn degenerate_modes_are_orthogonal_and_have_the_expected_norm() {
        let room = room([4.8, 4.8, 3.2]);
        let modes =
            [[1, 0, 0], [0, 1, 0], [1, 1, 0], [1, 0, 1], [2, 1, 1]].map(|orders| room.mode(orders));
        for (a_index, a) in modes.iter().enumerate() {
            for (b_index, b) in modes.iter().enumerate() {
                let mut mean = 0.0;
                for x in 0..8 {
                    for y in 0..8 {
                        for z in 0..8 {
                            let indices = [x, y, z];
                            let point = std::array::from_fn(|axis| {
                                room.lengths[axis] * (indices[axis] as f64 + 0.5) / 8.0
                            });
                            mean += room.pressure(*a, point) * room.pressure(*b, point) / 512.0;
                        }
                    }
                }
                let expected = if a_index == b_index {
                    0.5_f64.powi(a.orders.iter().filter(|&&order| order != 0).count() as i32)
                } else {
                    0.0
                };
                close(mean, expected);
            }
        }
    }
}
