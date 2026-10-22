use rand::{Rng, rngs::StdRng, seq::SliceRandom};
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
    let n = rng.random_range(5..=8);
    let mut vertices: Vec<_> = (0..n).collect();
    vertices.shuffle(rng);
    let reachable = rng.random_range(4..=n);
    let mut edges = Vec::new();
    for b in 1..reachable {
        let a = rng.random_range(0..b);
        edges.push((vertices[a], vertices[b], rng.random_range(0..=12)));
    }
    for (i, &a) in vertices.iter().enumerate() {
        for (j, &b) in vertices.iter().enumerate() {
            if a != b
                && (i >= reachable || j < reachable)
                && !edges.iter().any(|&(x, y, _)| x == a && y == b)
                && rng.random_bool(0.28)
            {
                edges.push((a, b, rng.random_range(0..=12)));
            }
        }
    }
    edges.sort_unstable();
    (n, edges, vertices[0])
}
pub fn generate(rng: &mut StdRng) -> String {
    let (n, edges, source) = instance(rng);
    let (distances, predecessors, order) = execute(n, &edges, source);
    let mut text = format!("Dijkstra from {source}\n");
    for a in 0..n {
        let outgoing = edges
            .iter()
            .filter(|e| e.0 == a)
            .map(|&(_, b, w)| format!("{b} ({w})"))
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(
            text,
            "{a} → {}",
            if outgoing.is_empty() {
                "∅"
            } else {
                &outgoing
            }
        )
        .unwrap();
    }
    write!(text, "\nSettle {order:?}\nVertex  Distance  Path").unwrap();
    for (v, distance) in distances.iter().enumerate() {
        if let Some(distance) = distance {
            let mut path = vec![v];
            let mut current = v;
            while let Some(previous) = predecessors[current] {
                path.push(previous);
                current = previous;
            }
            path.reverse();
            let path = path
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(" → ");
            write!(text, "\n{v:>3}     {distance:>3}     {path}").unwrap();
        } else {
            write!(text, "\n{v:>3}      ∞      —").unwrap();
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
