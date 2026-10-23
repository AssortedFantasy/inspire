use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

struct Point {
    station: &'static str,
    arrival: i32,
    departure: i32,
    dwell: i32,
}

struct Train {
    id: String,
    points: Vec<Point>,
}

struct Block {
    from: &'static str,
    to: &'static str,
    minutes: i32,
    directional: bool,
    fast: Option<i32>,
}

struct Schedule {
    description: String,
    release: i32,
    blocks: Vec<Block>,
    trains: Vec<Train>,
    connections: Vec<(usize, usize, &'static str, i32)>,
    stock: Vec<(usize, usize, i32)>,
}

fn clock(minutes: i32) -> String {
    let day = minutes.div_euclid(1440);
    let time = minutes.rem_euclid(1440);
    let suffix = if day == 0 {
        String::new()
    } else {
        format!("+{day}d")
    };
    format!("{:02}:{:02}{suffix}", time / 60, time % 60)
}

fn point(station: &'static str, arrival: i32, departure: i32, dwell: i32) -> Point {
    Point {
        station,
        arrival,
        departure,
        dwell,
    }
}

fn train(id: String, stations: [&'static str; 3], times: [i32; 4], dwell: i32) -> Train {
    Train {
        id,
        points: vec![
            point(stations[0], times[0], times[0], 0),
            point(stations[1], times[1], times[2], dwell),
            point(stations[2], times[3], times[3], 0),
        ],
    }
}

fn block(from: &'static str, to: &'static str, minutes: i32) -> Block {
    Block {
        from,
        to,
        minutes,
        directional: false,
        fast: None,
    }
}

impl Schedule {
    fn text(&self) -> String {
        let mut out = format!(
            "Hypothetical railway\n{}\nBlock release {} min after arrival",
            self.description, self.release
        );
        for b in &self.blocks {
            write!(out, "\n{}–{} running {} min", b.from, b.to, b.minutes).unwrap();
            if let Some(fast) = b.fast {
                write!(out, " freight / {fast} min express").unwrap();
            }
            if b.directional {
                out.push_str(" on each directional track");
            }
        }
        if !self.connections.is_empty() {
            let margin = self
                .connections
                .iter()
                .map(|&(incoming, outgoing, station, minimum)| {
                    let a = self.trains[incoming]
                        .points
                        .iter()
                        .find(|p| p.station == station)
                        .unwrap();
                    let b = self.trains[outgoing]
                        .points
                        .iter()
                        .find(|p| p.station == station)
                        .unwrap();
                    b.departure - a.arrival - minimum
                })
                .min()
                .unwrap();
            write!(out, "\nTransfer margin {margin} min above minimum").unwrap();
        }
        if !self.stock.is_empty() {
            let margin = self
                .stock
                .iter()
                .map(|&(incoming, outgoing, minimum)| {
                    self.trains[outgoing].points.first().unwrap().departure
                        - self.trains[incoming].points.last().unwrap().arrival
                        - minimum
                })
                .min()
                .unwrap();
            write!(out, "\nTrainset turn margin {margin} min above minimum").unwrap();
        }
        out.push_str("\nTrain   Depart       Intermediate arr/dep          Arrive");
        for t in &self.trains {
            let first = t.points.first().unwrap();
            let last = t.points.last().unwrap();
            write!(
                out,
                "\n{:<7} {} {}",
                t.id,
                first.station,
                clock(first.departure)
            )
            .unwrap();
            for p in &t.points[1..t.points.len() - 1] {
                debug_assert!(p.departure - p.arrival >= p.dwell);
                write!(
                    out,
                    "    {} {}/{}",
                    p.station,
                    clock(p.arrival),
                    clock(p.departure)
                )
                .unwrap();
            }
            write!(out, "    {} {}", last.station, clock(last.arrival)).unwrap();
        }
        out
    }
}

fn passing(rng: &mut StdRng, base: i32) -> Schedule {
    let left = rng.random_range(7..=17);
    let right = rng.random_range(7..=18);
    let release = rng.random_range(1..=3);
    let dwell = rng.random_range(1..=3);
    let skew: i32 = [-5, -1, 0, 1, 5][rng.random_range(0..5)];
    let period = 2 * left.max(right) + dwell.max(release) + skew.abs() + 5;
    let mut trains = Vec::new();
    for cycle in 0..2 {
        let meet = base + 25 + cycle * period;
        let east_arrival = meet;
        let west_arrival = meet + skew;
        let east_departure = (east_arrival + dwell).max(west_arrival + release);
        let west_departure = (west_arrival + dwell).max(east_arrival + release);
        trains.push(train(
            format!("E{}", 11 + cycle * 2),
            ["A", "P", "B"],
            [
                meet - left,
                east_arrival,
                east_departure,
                east_departure + right,
            ],
            dwell,
        ));
        trains.push(train(
            format!("W{}", 12 + cycle * 2),
            ["B", "P", "A"],
            [
                west_arrival - right,
                west_arrival,
                west_departure,
                west_departure + left,
            ],
            dwell,
        ));
    }
    Schedule {
        description: format!(
            "A — P — B, single track with a two-road loop at P\nOpposing meets, minimum stop at P {dwell} min"
        ),
        release,
        blocks: vec![block("A", "P", left), block("P", "B", right)],
        trains,
        connections: vec![],
        stock: vec![],
    }
}

fn clockface(rng: &mut StdRng, base: i32) -> Schedule {
    let period = if rng.random_bool(0.5) { 30 } else { 60 };
    let left = rng.random_range(3..=6);
    let right = if period == 30 {
        rng.random_range(3..=4)
    } else {
        rng.random_range(8..=14)
    };
    let dwell = 1;
    let turn = rng.random_range(3..=4);
    let run = left + right + dwell;
    let b_turn = turn;
    let a_turn = period - 2 * run - b_turn;
    let release = 1;
    let mut trains = Vec::new();
    for cycle in 0..3 {
        let departure = base + cycle * period;
        let b_arrival = departure + run;
        let b_departure = b_arrival + b_turn;
        trains.push(train(
            format!("S{}", cycle * 2 + 1),
            ["A", "H", "B"],
            [
                departure,
                departure + left,
                departure + left + dwell,
                b_arrival,
            ],
            dwell,
        ));
        trains.push(train(
            format!("S{}", cycle * 2 + 2),
            ["B", "H", "A"],
            [
                b_departure,
                b_departure + right,
                b_departure + right + dwell,
                b_departure + run,
            ],
            dwell,
        ));
    }
    Schedule {
        description: format!(
            "A — H — B, single-track shuttle, one trainset\nEvery {period} min from A, H stop {dwell} min\nMinimum terminal turn 3 min. Scheduled turns A {a_turn} min, B {b_turn} min"
        ),
        release,
        blocks: vec![block("A", "H", left), block("H", "B", right)],
        trains,
        connections: vec![],
        stock: (0..5).map(|i| (i, i + 1, 3)).collect(),
    }
}

fn transfers(rng: &mut StdRng, base: i32) -> Schedule {
    let west = rng.random_range(8..=19);
    let east = rng.random_range(8..=19);
    let branch = rng.random_range(6..=17);
    let minimum = rng.random_range(4..=6);
    let slack = [0, 0, 1, 4][rng.random_range(0..4)];
    let connection = minimum + slack;
    let period = if 2 * branch + connection + 4 <= 30 {
        30
    } else {
        60
    };
    let mut trains = Vec::new();
    let mut connections = Vec::new();
    let mut stock = Vec::new();
    for cycle in 0..2 {
        let pulse = base + 25 + cycle * period;
        let arrival = pulse - 3;
        let departure = arrival + connection;
        let offset = trains.len();
        trains.push(train(
            format!("M{}", 1 + cycle * 2),
            ["W", "J", "E"],
            [arrival - west, arrival, departure, departure + east],
            1,
        ));
        trains.push(train(
            format!("M{}", 2 + cycle * 2),
            ["E", "J", "W"],
            [arrival - east, arrival, departure, departure + west],
            1,
        ));
        trains.push(Train {
            id: format!("B{}", 1 + cycle * 2),
            points: vec![
                point("B", arrival - branch, arrival - branch, 0),
                point("J", arrival, arrival, 0),
            ],
        });
        trains.push(Train {
            id: format!("B{}", 2 + cycle * 2),
            points: vec![
                point("J", departure, departure, 0),
                point("B", departure + branch, departure + branch, 0),
            ],
        });
        connections.extend([
            (offset, offset + 1, "J", minimum),
            (offset + 1, offset, "J", minimum),
            (offset, offset + 3, "J", minimum),
            (offset + 1, offset + 3, "J", minimum),
            (offset + 2, offset, "J", minimum),
            (offset + 2, offset + 1, "J", minimum),
        ]);
        stock.push((offset + 2, offset + 3, 3));
        if cycle > 0 {
            stock.push((offset - 1, offset + 2, 4));
        }
    }
    let mut w = block("W", "J", west);
    let mut e = block("J", "E", east);
    w.directional = true;
    e.directional = true;
    Schedule {
        description: format!(
            "W — J — E, separate directional tracks and platforms\nJ — B, single-track branch with one trainset and a separate bay at J\nInterchange every {period} min at J, all transfers {connection} min (minimum {minimum})\nBranch minimum turns J 3 min, B 4 min"
        ),
        release: 1,
        blocks: vec![w, e, block("J", "B", branch)],
        trains,
        connections,
        stock,
    }
}

fn overtaking(rng: &mut StdRng, base: i32) -> Schedule {
    let slow_left = rng.random_range(14..=24);
    let slow_right = rng.random_range(14..=24);
    let fast_left = rng.random_range(7..=11);
    let fast_right = rng.random_range(7..=11);
    let release = rng.random_range(1..=3);
    let express_stop = rng.random_range(0..=1);
    let freight_arrival = base + slow_left;
    let express_start = freight_arrival + release + rng.random_range(0..=3);
    let express_arrival = express_start + fast_left;
    let express_departure = express_arrival + express_stop;
    let freight_departure = express_departure + fast_right + release;
    let mut left = block("A", "P", slow_left);
    let mut right = block("P", "B", slow_right);
    left.fast = Some(fast_left);
    right.fast = Some(fast_right);
    Schedule {
        description: format!(
            "A — P — B, single track with a freight refuge at P\nE2 overtakes F1 at P. Freight minimum stop 2 min, express stop {express_stop} min\nFreight hold at P {} min",
            freight_departure - freight_arrival
        ),
        release,
        blocks: vec![left, right],
        trains: vec![
            train(
                "F1".into(),
                ["A", "P", "B"],
                [
                    base,
                    freight_arrival,
                    freight_departure,
                    freight_departure + slow_right,
                ],
                2,
            ),
            train(
                "E2".into(),
                ["A", "P", "B"],
                [
                    express_start,
                    express_arrival,
                    express_departure,
                    express_departure + fast_right,
                ],
                express_stop,
            ),
        ],
        connections: vec![],
        stock: vec![],
    }
}

fn scenario(rng: &mut StdRng, kind: usize) -> Schedule {
    let base = rng.random_range(6..=22) * 60 + rng.random_range(0..=11) * 5;
    match kind {
        0 => passing(rng, base),
        1 => clockface(rng, base),
        2 => transfers(rng, base),
        _ => overtaking(rng, base),
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let kind = rng.random_range(0..4);
    scenario(rng, kind).text()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn validate(s: &Schedule) {
        let mut uses = Vec::new();
        for t in &s.trains {
            for p in &t.points {
                assert!(p.departure - p.arrival >= p.dwell);
            }
            for leg in t.points.windows(2) {
                let a = &leg[0];
                let b = &leg[1];
                let (i, block) = s
                    .blocks
                    .iter()
                    .enumerate()
                    .find(|(_, block)| {
                        (block.from == a.station && block.to == b.station)
                            || (block.to == a.station && block.from == b.station)
                    })
                    .unwrap();
                let expected = if t.id == "E2" {
                    block.fast.unwrap_or(block.minutes)
                } else {
                    block.minutes
                };
                assert_eq!(b.arrival - a.departure, expected);
                let direction = block.directional && a.station == block.from;
                uses.push((i, direction, a.departure, b.arrival + s.release));
            }
        }
        for (i, a) in uses.iter().enumerate() {
            for b in &uses[i + 1..] {
                if (a.0, a.1) == (b.0, b.1) {
                    assert!(
                        a.3 <= b.2 || b.3 <= a.2,
                        "conflicting occupancies {a:?} {b:?}"
                    );
                }
            }
        }
        for &(incoming, outgoing, station, minimum) in &s.connections {
            let a = s.trains[incoming]
                .points
                .iter()
                .find(|p| p.station == station)
                .unwrap();
            let b = s.trains[outgoing]
                .points
                .iter()
                .find(|p| p.station == station)
                .unwrap();
            assert!(b.departure - a.arrival >= minimum);
        }
        for &(incoming, outgoing, minimum) in &s.stock {
            let a = s.trains[incoming].points.last().unwrap();
            let b = s.trains[outgoing].points.first().unwrap();
            assert_eq!(a.station, b.station);
            assert!(b.departure - a.arrival >= minimum);
        }
    }

    #[test]
    fn every_structure_obeys_running_release_transfer_and_stock_constraints() {
        for kind in 0..4 {
            for seed in 0..512 {
                let mut rng = StdRng::seed_from_u64(seed);
                validate(&scenario(&mut rng, kind));
            }
        }
    }

    #[test]
    fn freight_changes_order_only_inside_refuge() {
        for seed in 0..128 {
            let s = overtaking(&mut StdRng::seed_from_u64(seed), 480);
            let f = &s.trains[0].points;
            let e = &s.trains[1].points;
            assert!(f[0].departure < e[0].departure);
            assert!(f[1].arrival < e[1].arrival);
            assert!(e[1].departure < f[1].departure);
            assert!(e[2].arrival < f[2].arrival);
        }
    }

    #[test]
    fn midnight_carries_the_day() {
        assert_eq!(clock(1461), "00:21+1d");
    }
}
