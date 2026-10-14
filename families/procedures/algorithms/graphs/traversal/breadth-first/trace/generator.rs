use rand::{Rng, rngs::StdRng};
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
    if graph.iter().all(Vec::is_empty) {
        graph[0].push(1);
        graph[1].push(0);
    }
    graph
}
pub fn generate(rng: &mut StdRng) -> String {
    let graph = instance(rng);
    let candidates: Vec<_> = (0..graph.len()).filter(|&v| !graph[v].is_empty()).collect();
    let source = candidates[rng.random_range(0..candidates.len())];
    let (distances, order) = bfs(&graph, source);
    let mut text = format!(
        "Breadth-first search execution\nUndirected unweighted graph; vertices 0..{}; neighbors visited in ascending order.\nSource: {source}",
        graph.len() - 1
    );
    for (v, neighbors) in graph.iter().enumerate() {
        write!(text, "\nAdjacency {v}: {neighbors:?}").unwrap();
    }
    write!(
        text,
        "\nFIFO dequeue order: {order:?}\nShortest edge distances:"
    )
    .unwrap();
    for (v, distance) in distances.iter().enumerate() {
        let label = distance.map_or_else(|| "unreachable".to_string(), |d| d.to_string());
        write!(text, "\n{v}: {label}").unwrap();
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
