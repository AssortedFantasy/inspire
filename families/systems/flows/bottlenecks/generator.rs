use rand::{Rng, rngs::StdRng};
use std::{collections::VecDeque, fmt::Write};

#[derive(Clone, Debug)]
struct Network {
    capacity: Vec<Vec<i64>>,
}

#[derive(Debug)]
struct Solution {
    value: i64,
    flow: Vec<Vec<i64>>,
    reachable: Vec<bool>,
}

impl Network {
    fn new(count: usize, edges: &[(usize, usize, i64)]) -> Self {
        let mut capacity = vec![vec![0; count]; count];
        for &(from, to, cap) in edges {
            capacity[from][to] = cap;
        }
        Self { capacity }
    }

    fn solve(&self) -> Solution {
        let count = self.capacity.len();
        let sink = count - 1;
        let mut residual = self.capacity.clone();
        let mut value = 0;
        let reachable = loop {
            let mut parent = vec![None; count];
            parent[0] = Some(0);
            let mut queue = VecDeque::from([0]);
            while let Some(u) = queue.pop_front() {
                for v in 0..count {
                    if parent[v].is_none() && residual[u][v] > 0 {
                        parent[v] = Some(u);
                        queue.push_back(v);
                    }
                }
            }
            if parent[sink].is_none() {
                break parent.iter().map(Option::is_some).collect();
            }
            let mut amount = i64::MAX;
            let mut v = sink;
            while v != 0 {
                let u = parent[v].unwrap();
                amount = amount.min(residual[u][v]);
                v = u;
            }
            v = sink;
            while v != 0 {
                let u = parent[v].unwrap();
                residual[u][v] -= amount;
                residual[v][u] += amount;
                v = u;
            }
            value += amount;
        };
        let flow = self
            .capacity
            .iter()
            .enumerate()
            .map(|(u, row)| {
                row.iter()
                    .enumerate()
                    .map(|(v, cap)| cap - residual[u][v])
                    .collect()
            })
            .collect();
        Solution {
            value,
            flow,
            reachable,
        }
    }

    fn cuts(&self) -> Vec<(u32, i64)> {
        let count = self.capacity.len();
        (0..(1_u32 << count))
            .filter(|mask| mask & 1 != 0 && mask & (1 << (count - 1)) == 0)
            .map(|mask| {
                let capacity = self
                    .capacity
                    .iter()
                    .enumerate()
                    .filter(|(u, _)| mask & (1 << u) != 0)
                    .map(|(_, row)| {
                        row.iter()
                            .enumerate()
                            .filter(|(v, _)| mask & (1 << v) == 0)
                            .map(|(_, cap)| cap)
                            .sum::<i64>()
                    })
                    .sum();
                (mask, capacity)
            })
            .collect()
    }
}

fn make(rng: &mut StdRng, mode: usize) -> Network {
    let small = rng.random_range(3..=12);
    match mode {
        0 => {
            let big = small + rng.random_range(5..=20);
            Network::new(
                7,
                &[
                    (0, 1, big),
                    (0, 2, big),
                    (1, 3, big),
                    (2, 3, big),
                    (3, 4, small),
                    (4, 6, big),
                    (4, 5, big),
                    (5, 6, big),
                ],
            )
        }
        1 => {
            let count = rng.random_range(2..=5);
            let sink = count + 1;
            let mut edges = Vec::new();
            for i in 1..=count {
                edges.extend([
                    (0, i, rng.random_range(3..=18)),
                    (i, sink, rng.random_range(3..=18)),
                ]);
            }
            Network::new(sink + 1, &edges)
        }
        2 => {
            let mut edges = Vec::new();
            for (u, v) in [
                (0, 1),
                (0, 2),
                (1, 3),
                (1, 4),
                (2, 3),
                (2, 4),
                (3, 5),
                (4, 5),
            ] {
                edges.push((u, v, rng.random_range(2..=18)));
            }
            Network::new(6, &edges)
        }
        3 => {
            let gap = rng.random_range(2..=9);
            let bridge = gap + rng.random_range(-1..=1);
            Network::new(
                4,
                &[
                    (0, 1, small + gap),
                    (0, 2, small),
                    (1, 3, small),
                    (2, 3, small + gap),
                    (1, 2, bridge),
                ],
            )
        }
        4 => {
            let mut edges = Vec::new();
            for (u, v) in [
                (0, 1),
                (0, 2),
                (1, 3),
                (2, 3),
                (3, 4),
                (3, 5),
                (4, 6),
                (5, 6),
            ] {
                edges.push((u, v, rng.random_range(3..=18)));
            }
            Network::new(7, &edges)
        }
        _ => {
            let count = rng.random_range(5..=8);
            let mut edges = Vec::new();
            for u in 0..count - 1 {
                edges.push((u, u + 1, rng.random_range(2..=18)));
                for v in u + 2..count {
                    if rng.random_bool(0.35) {
                        edges.push((u, v, rng.random_range(1..=14)));
                    }
                }
            }
            Network::new(count, &edges)
        }
    }
}

fn name(i: usize, count: usize) -> char {
    if i == 0 {
        's'
    } else if i == count - 1 {
        't'
    } else {
        (b'a' + i as u8 - 1) as char
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let mode = rng.random_range(0..6);
    let mut graph = make(rng, mode);
    let s = graph.solve();
    let count = graph.capacity.len();
    let mut out = String::from("Directed network s → t, units/h\nEdge  Capacity  Flow");
    let mut edges = Vec::new();
    let mut cut_edges = Vec::new();
    for u in 0..count {
        for v in 0..count {
            if graph.capacity[u][v] > 0 {
                edges.push((u, v));
                write!(
                    out,
                    "\n{}→{}   {:>7}  {:>4}",
                    name(u, count),
                    name(v, count),
                    graph.capacity[u][v],
                    s.flow[u][v]
                )
                .unwrap();
                if s.reachable[u] && !s.reachable[v] {
                    cut_edges.push(format!(
                        "{}→{} ({})",
                        name(u, count),
                        name(v, count),
                        graph.capacity[u][v]
                    ));
                }
            }
        }
    }
    let source_side = (0..count)
        .filter(|&i| s.reachable[i])
        .map(|i| name(i, count).to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let cuts = graph.cuts();
    let minimum = cuts.iter().filter(|(_, cap)| *cap == s.value).count();
    write!(out, "\n\nMaximum flow = {}\nMinimum cut S = {{{source_side}}}\n{} = {}\nMinimum cuts = {minimum}", s.value, cut_edges.join(" + "), s.value).unwrap();
    // Re-solve every single-edge upgrade; choose both useful and ineffective edges when available.
    let increase = rng.random_range(1..=5);
    let mut outcomes = Vec::new();
    for &(u, v) in &edges {
        graph.capacity[u][v] += increase;
        outcomes.push((u, v, graph.solve().value));
        graph.capacity[u][v] -= increase;
    }
    let want_gain = rng.random_bool(0.5);
    let preferred: Vec<_> = outcomes
        .iter()
        .filter(|(_, _, value)| (*value > s.value) == want_gain)
        .collect();
    let chosen = if preferred.is_empty() {
        &outcomes[rng.random_range(0..outcomes.len())]
    } else {
        preferred[rng.random_range(0..preferred.len())]
    };
    write!(
        out,
        "\n\nAdd {increase} capacity to {}→{} → maximum flow {}",
        name(chosen.0, count),
        name(chosen.1, count),
        chosen.2
    )
    .unwrap();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn all_cuts_certify_optimality_and_every_flow_is_feasible() {
        for seed in 0..320 {
            let mut rng = StdRng::seed_from_u64(seed);
            for mode in 0..6 {
                let mut graph = make(&mut rng, mode);
                let s = graph.solve();
                let count = graph.capacity.len();
                assert_eq!(
                    s.value,
                    graph.cuts().iter().map(|(_, cap)| *cap).min().unwrap()
                );
                assert!(s.reachable[0] && !s.reachable[count - 1]);
                let mut cut = 0;
                for u in 0..count {
                    let balance: i64 = s.flow[u].iter().sum();
                    assert_eq!(
                        balance,
                        if u == 0 {
                            s.value
                        } else if u == count - 1 {
                            -s.value
                        } else {
                            0
                        }
                    );
                    for v in 0..count {
                        assert_eq!(s.flow[u][v], -s.flow[v][u]);
                        assert!(s.flow[u][v] <= graph.capacity[u][v]);
                        if s.reachable[u] && !s.reachable[v] {
                            cut += graph.capacity[u][v];
                        }
                        if graph.capacity[u][v] > 0 {
                            assert!(s.flow[u][v] >= 0);
                            graph.capacity[u][v] += 1;
                            let updated = graph.solve().value;
                            assert_eq!(
                                updated,
                                graph.cuts().iter().map(|(_, cap)| *cap).min().unwrap()
                            );
                            assert!((s.value..=s.value + 1).contains(&updated));
                            graph.capacity[u][v] -= 1;
                        }
                    }
                }
                assert_eq!(cut, s.value);
                if mode == 0 {
                    assert_eq!(s.value, graph.capacity[3][4]);
                }
                if mode == 3 {
                    assert_eq!(
                        s.value,
                        (graph.capacity[0][1] + graph.capacity[0][2])
                            .min(2 * graph.capacity[0][2] + graph.capacity[1][2])
                    );
                }
            }
        }
    }

    #[test]
    fn rerouting_uses_reverse_residual_arcs() {
        let g = Network::new(
            6,
            &[
                (0, 1, 1),
                (0, 2, 1),
                (1, 3, 1),
                (1, 4, 1),
                (2, 3, 1),
                (3, 5, 1),
                (4, 5, 1),
            ],
        );
        let s = g.solve();
        assert_eq!(s.value, 2);
        assert_eq!(s.flow[1][3], 0);
        assert_eq!(s.flow[1][4], 1);
        assert_eq!(s.flow[2][3], 1);
    }
}
