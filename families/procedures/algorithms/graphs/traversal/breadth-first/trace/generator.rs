use rand::{Rng, rngs::StdRng, seq::SliceRandom};
use std::{collections::VecDeque, fmt::Write};

fn bfs(graph: &[Vec<usize>], source: usize) -> (Vec<Option<usize>>, Vec<usize>) {
    let mut distances = vec![None; graph.len()];
    let mut queue = VecDeque::from([source]);
    let mut order = Vec::new();
    distances[source] = Some(0);
    while let Some(v) = queue.pop_front() {
        order.push(v);
        for &w in &graph[v] {
            if distances[w].is_none() {
                distances[w] = Some(distances[v].unwrap() + 1);
                queue.push_back(w);
            }
        }
    }
    (distances, order)
}
fn instance(rng: &mut StdRng) -> Vec<Vec<usize>> {
    let n = rng.random_range(5..=8);
    let mut graph = vec![Vec::new(); n];
    for a in 0..n {
        for b in a + 1..n {
            if rng.random_bool(0.28) {
                graph[a].push(b);
                graph[b].push(a);
            }
        }
    }
    let mut chain: Vec<_> = (0..n).collect();
    chain.shuffle(rng);
    for pair in chain[..4].windows(2) {
        graph[pair[0]].push(pair[1]);
        graph[pair[1]].push(pair[0]);
    }
    for neighbors in &mut graph {
        neighbors.sort_unstable();
        neighbors.dedup();
    }
    graph
}
pub fn generate(rng: &mut StdRng) -> String {
    let graph = instance(rng);
    let candidates: Vec<_> = (0..graph.len())
        .filter(|&v| bfs(&graph, v).1.len() >= 4)
        .collect();
    let source = candidates[rng.random_range(0..candidates.len())];
    let (distances, order) = bfs(&graph, source);
    let mut text = format!("Breadth-first search from {source}\n");
    for (v, neighbors) in graph.iter().enumerate() {
        writeln!(text, "{v} — {neighbors:?}").unwrap();
    }
    write!(text, "\nVisit {order:?}").unwrap();
    for depth in 0..=distances.iter().flatten().copied().max().unwrap() {
        let layer: Vec<_> = (0..graph.len())
            .filter(|&v| distances[v] == Some(depth))
            .collect();
        write!(text, "\nDistance {depth}: {layer:?}").unwrap();
    }
    let unreachable: Vec<_> = (0..graph.len())
        .filter(|&v| distances[v].is_none())
        .collect();
    if !unreachable.is_empty() {
        write!(text, "\nUnreachable: {unreachable:?}").unwrap();
    }
    text
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn distances_equal_independent_all_pairs_dynamic_program() {
        let mut rng = StdRng::seed_from_u64(285);
        for _ in 0..1000 {
            let graph = instance(&mut rng);
            let n = graph.len();
            let mut reference = vec![vec![1000; n]; n];
            for v in 0..n {
                reference[v][v] = 0;
                for &w in &graph[v] {
                    reference[v][w] = 1;
                }
            }
            for k in 0..n {
                for v in 0..n {
                    for w in 0..n {
                        reference[v][w] = reference[v][w].min(reference[v][k] + reference[k][w]);
                    }
                }
            }
            for (source, reference_distances) in reference.iter().enumerate() {
                let (distances, order) = bfs(&graph, source);
                for (v, &d) in distances.iter().enumerate() {
                    assert_eq!(
                        d,
                        (reference_distances[v] < 1000).then_some(reference_distances[v])
                    );
                }
                assert!(
                    order
                        .windows(2)
                        .all(|pair| distances[pair[0]] <= distances[pair[1]])
                );
                let mut unique = order;
                unique.sort();
                unique.dedup();
                assert_eq!(unique.len(), distances.iter().flatten().count());
            }
        }
    }
}
