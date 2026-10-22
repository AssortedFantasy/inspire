use rand::{Rng, rngs::StdRng, seq::SliceRandom};
use std::{collections::BTreeSet, fmt::Write};

struct Execution {
    order: Vec<usize>,
    ready: Vec<Vec<usize>>,
    blocked: Vec<usize>,
}

fn execute(graph: &[Vec<usize>]) -> Execution {
    let mut incoming = vec![0; graph.len()];
    for neighbors in graph {
        for &w in neighbors {
            incoming[w] += 1;
        }
    }
    let mut available: BTreeSet<_> = (0..graph.len()).filter(|&v| incoming[v] == 0).collect();
    let mut order = Vec::new();
    let mut ready = Vec::new();
    while !available.is_empty() {
        ready.push(available.iter().copied().collect());
        let v = available.pop_first().unwrap();
        order.push(v);
        for &w in &graph[v] {
            incoming[w] -= 1;
            if incoming[w] == 0 {
                available.insert(w);
            }
        }
    }
    let blocked = (0..graph.len()).filter(|&v| incoming[v] != 0).collect();
    Execution {
        order,
        ready,
        blocked,
    }
}

fn cycle(graph: &[Vec<usize>]) -> Option<Vec<usize>> {
    fn visit(
        graph: &[Vec<usize>],
        v: usize,
        state: &mut [u8],
        stack: &mut Vec<usize>,
    ) -> Option<Vec<usize>> {
        state[v] = 1;
        stack.push(v);
        for &w in &graph[v] {
            if state[w] == 0 {
                if let Some(cycle) = visit(graph, w, state, stack) {
                    return Some(cycle);
                }
            } else if state[w] == 1 {
                let start = stack.iter().position(|&x| x == w).unwrap();
                let mut cycle = stack[start..].to_vec();
                cycle.push(w);
                return Some(cycle);
            }
        }
        stack.pop();
        state[v] = 2;
        None
    }
    let mut state = vec![0; graph.len()];
    for v in 0..graph.len() {
        if state[v] == 0
            && let Some(cycle) = visit(graph, v, &mut state, &mut Vec::new())
        {
            return Some(cycle);
        }
    }
    None
}

fn instance(rng: &mut StdRng) -> Vec<Vec<usize>> {
    let n = rng.random_range(6..=9);
    let mut hidden_order: Vec<_> = (0..n).collect();
    hidden_order.shuffle(rng);
    let mut graph = vec![Vec::new(); n];
    for i in 0..n {
        for j in i + 1..n {
            if rng.random_bool(0.27) {
                graph[hidden_order[i]].push(hidden_order[j]);
            }
        }
    }
    if rng.random_bool(0.35) {
        hidden_order.shuffle(rng);
        let size = rng.random_range(2..=4);
        for i in 0..size {
            graph[hidden_order[i]].push(hidden_order[(i + 1) % size]);
        }
    }
    for neighbors in &mut graph {
        neighbors.sort_unstable();
        neighbors.dedup();
    }
    graph
}

fn names(vertices: &[usize], separator: &str) -> String {
    vertices
        .iter()
        .map(|&v| ((b'A' + v as u8) as char).to_string())
        .collect::<Vec<_>>()
        .join(separator)
}

pub fn generate(rng: &mut StdRng) -> String {
    let graph = instance(rng);
    let result = execute(&graph);
    let mut text = String::from("Directed graph");
    for (v, neighbors) in graph.iter().enumerate() {
        let label = (b'A' + v as u8) as char;
        let neighbors = names(neighbors, ", ");
        write!(
            text,
            "\n{label} → {}",
            if neighbors.is_empty() {
                "∅"
            } else {
                &neighbors
            }
        )
        .unwrap();
    }
    text.push_str("\n\nKahn's topological sort\nReady          Take");
    for (ready, &v) in result.ready.iter().zip(&result.order) {
        let label = (b'A' + v as u8) as char;
        write!(text, "\n{:<15}{label}", names(ready, " ")).unwrap();
    }
    if result.blocked.is_empty() {
        write!(text, "\nOrder: {}", names(&result.order, " → ")).unwrap();
    } else {
        if result.order.is_empty() {
            text.push_str("\n∅              —");
        }
        write!(
            text,
            "\nBlocked: {}\nCycle: {}",
            names(&result.blocked, ", "),
            names(&cycle(&graph).unwrap(), " → ")
        )
        .unwrap();
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_four_vertex_graph_has_a_valid_order_or_cycle_witness() {
        let edges: Vec<_> = (0..4)
            .flat_map(|v| (0..4).filter(move |&w| w != v).map(move |w| (v, w)))
            .collect();
        for mask in 0usize..1 << edges.len() {
            let mut graph = vec![Vec::new(); 4];
            let mut reachable = [[false; 4]; 4];
            for (i, &(v, w)) in edges.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    graph[v].push(w);
                    reachable[v][w] = true;
                }
            }
            for k in 0..4 {
                for v in 0..4 {
                    for w in 0..4 {
                        reachable[v][w] |= reachable[v][k] && reachable[k][w];
                    }
                }
            }
            let cyclic = (0..4).any(|v| reachable[v][v]);
            let result = execute(&graph);
            assert_eq!(!result.blocked.is_empty(), cyclic);
            assert_eq!(cycle(&graph).is_some(), cyclic);
            if let Some(witness) = cycle(&graph) {
                assert_eq!(witness.first(), witness.last());
                assert!(
                    witness
                        .windows(2)
                        .all(|pair| graph[pair[0]].contains(&pair[1]))
                );
            } else {
                let mut position = [usize::MAX; 4];
                for (i, &v) in result.order.iter().enumerate() {
                    assert_eq!(position[v], usize::MAX);
                    position[v] = i;
                }
                assert!(position.iter().all(|&i| i != usize::MAX));
                for (v, neighbors) in graph.iter().enumerate() {
                    assert!(neighbors.iter().all(|&w| position[v] < position[w]));
                }
            }
            let mut processed = [false; 4];
            for (ready, &chosen) in result.ready.iter().zip(&result.order) {
                let expected: Vec<_> = (0..4)
                    .filter(|&v| {
                        !processed[v] && (0..4).all(|u| processed[u] || !graph[u].contains(&v))
                    })
                    .collect();
                assert_eq!(*ready, expected);
                assert_eq!(chosen, ready[0]);
                processed[chosen] = true;
            }
        }
    }
}
