use rand::{Rng, rngs::StdRng, seq::SliceRandom};
use std::fmt::Write;

type Edge = (usize, usize, i32);
fn representative(parent: &mut [usize], mut v: usize) -> usize {
    while parent[v] != v {
        parent[v] = parent[parent[v]];
        v = parent[v];
    }
    v
}
fn execute(n: usize, edges: &[Edge]) -> (Vec<Edge>, Vec<(Edge, bool)>) {
    let mut sorted = edges.to_vec();
    sorted.sort_by_key(|&(a, b, w)| (w, a, b));
    let mut parent: Vec<_> = (0..n).collect();
    let mut tree = Vec::new();
    let mut trace = Vec::new();
    for edge @ (a, b, _) in sorted {
        let root_a = representative(&mut parent, a);
        let root_b = representative(&mut parent, b);
        let accepted = root_a != root_b;
        if accepted {
            parent[root_a] = root_b;
            tree.push(edge);
        }
        trace.push((edge, accepted));
    }
    (tree, trace)
}
fn instance(rng: &mut StdRng) -> (usize, Vec<Edge>) {
    let n = rng.random_range(4..=6);
    let mut weights: Vec<i32> = (-9..=40).collect();
    weights.shuffle(rng);
    let mut edges = Vec::new();
    for b in 1..n {
        let a = rng.random_range(0..b);
        edges.push((a, b, weights.pop().unwrap()));
    }
    for a in 0..n {
        for b in a + 1..n {
            if edges.len() < 9
                && !edges.iter().any(|&(x, y, _)| x == a && y == b)
                && rng.random_bool(0.3)
            {
                edges.push((a, b, weights.pop().unwrap()));
            }
        }
    }
    if edges.len() == n - 1 {
        let missing: Vec<_> = (0..n)
            .flat_map(|a| (a + 1..n).map(move |b| (a, b)))
            .filter(|&(a, b)| !edges.iter().any(|&(x, y, _)| x == a && y == b))
            .collect();
        let (a, b) = missing[rng.random_range(0..missing.len())];
        edges.push((a, b, weights.pop().unwrap()));
    }
    (n, edges)
}
pub fn generate(rng: &mut StdRng) -> String {
    let (n, edges) = instance(rng);
    let (tree, trace) = execute(n, &edges);
    let mut text = String::from("Kruskal's minimum spanning tree\nEdge   Weight   Decision");
    for (edge, accepted) in trace {
        let (a, b, weight) = edge;
        let decision = if accepted { "keep" } else { "cycle" };
        write!(text, "\n{a}—{b}    {weight:>3}      {decision}").unwrap();
    }
    let total: i32 = tree.iter().map(|e| e.2).sum();
    write!(text, "\nTotal weight {total}").unwrap();
    text
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn matches_exhaustive_spanning_tree_minimum() {
        let mut rng = StdRng::seed_from_u64(287);
        for _ in 0..300 {
            let (n, edges) = instance(&mut rng);
            let (tree, _) = execute(n, &edges);
            assert_eq!(tree.len(), n - 1);
            let mut best = i32::MAX;
            for mask in 0usize..1 << edges.len() {
                if mask.count_ones() as usize != n - 1 {
                    continue;
                }
                // Independent connectivity by repeated reachability, not union-find.
                let mut reached = vec![false; n];
                reached[0] = true;
                for _ in 0..n {
                    for (i, &(a, b, _)) in edges.iter().enumerate() {
                        if mask & (1 << i) != 0 && (reached[a] || reached[b]) {
                            reached[a] = true;
                            reached[b] = true;
                        }
                    }
                }
                if reached.iter().all(|&x| x) {
                    let weight = edges
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| mask & (1 << i) != 0)
                        .map(|(_, e)| e.2)
                        .sum();
                    best = best.min(weight);
                }
            }
            assert_eq!(tree.iter().map(|e| e.2).sum::<i32>(), best);
        }
    }
}
