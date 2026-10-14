use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

type Edge = (usize, usize, u32);
type Execution = (Vec<Option<u32>>, Vec<Option<usize>>, Vec<usize>);
fn execute(n: usize, edges: &[Edge], source: usize) -> Execution {
    let mut distances = vec![None; n];
    let mut predecessors = vec![None; n];
    let mut settled = vec![false; n];
    let mut order = Vec::new();
    distances[source] = Some(0);
    while let Some(v) = (0..n)
        .filter(|&v| !settled[v] && distances[v].is_some())
        .min_by_key(|&v| (distances[v].unwrap(), v))
    {
        settled[v] = true;
        order.push(v);
        for &(a, b, weight) in edges {
            if a != v || settled[b] {
                continue;
            }
            let proposed = distances[v].unwrap() + weight;
            if distances[b].is_none_or(|old| proposed < old) {
                distances[b] = Some(proposed);
                predecessors[b] = Some(v);
            }
        }
    }
    (distances, predecessors, order)
}
fn instance(rng: &mut StdRng) -> (usize, Vec<Edge>, usize) {
    let n = rng.random_range(5..=7);
    let mut edges = Vec::new();
    for a in 0..n {
        for b in 0..n {
            if a != b && rng.random_bool(0.28) {
                edges.push((a, b, rng.random_range(0..=12)));
            }
        }
    }
    if edges.is_empty() {
        edges.push((0, 1, rng.random_range(1..=12)));
    }
    let candidates: Vec<_> = (0..n).filter(|&v| edges.iter().any(|e| e.0 == v)).collect();
    let source = candidates[rng.random_range(0..candidates.len())];
    (n, edges, source)
}
pub fn generate(rng: &mut StdRng) -> String {
    let (n, edges, source) = instance(rng);
    let (distances, predecessors, order) = execute(n, &edges, source);
    let mut text = format!(
        "Dijkstra execution\nDirected graph; vertices 0..{}; edges (from, to, nonnegative weight): {edges:?}\nSource: {source}; distance ties settle the smaller vertex; equal-length alternatives do not change predecessors.\nSettlement order: {order:?}",
        n - 1
    );
    for (v, distance) in distances.iter().enumerate() {
        if let Some(distance) = distance {
            let mut path = vec![v];
            let mut current = v;
            while let Some(previous) = predecessors[current] {
                path.push(previous);
                current = previous;
            }
            path.reverse();
            write!(text, "\n{v}: distance {distance}, path {path:?}").unwrap();
        } else {
            write!(text, "\n{v}: unreachable").unwrap();
        }
    }
    text
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn distances_and_reported_paths_match_floyd_warshall() {
        let mut rng = StdRng::seed_from_u64(286);
        for _ in 0..1000 {
            let (n, edges, source) = instance(&mut rng);
            let mut reference = vec![vec![10000; n]; n];
            for (v, row) in reference.iter_mut().enumerate() {
                row[v] = 0;
            }
            for &(a, b, w) in &edges {
                reference[a][b] = w;
            }
            for k in 0..n {
                for a in 0..n {
                    for b in 0..n {
                        reference[a][b] = reference[a][b].min(reference[a][k] + reference[k][b]);
                    }
                }
            }
            let (distances, predecessors, order) = execute(n, &edges, source);
            assert!(
                order
                    .windows(2)
                    .all(|pair| distances[pair[0]] <= distances[pair[1]])
            );
            for v in 0..n {
                assert_eq!(
                    distances[v],
                    (reference[source][v] < 10000).then_some(reference[source][v])
                );
                if let Some(distance) = distances[v] {
                    let (mut current, mut weight, mut steps) = (v, 0, 0);
                    while let Some(previous) = predecessors[current] {
                        weight += edges
                            .iter()
                            .find(|&&(a, b, _)| a == previous && b == current)
                            .unwrap()
                            .2;
                        current = previous;
                        steps += 1;
                        assert!(steps < n);
                    }
                    assert_eq!(current, source);
                    assert_eq!(weight, distance);
                }
            }
        }
    }
}
