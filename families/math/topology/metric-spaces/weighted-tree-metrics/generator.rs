use rand::{Rng, rngs::StdRng};

type Edge = (usize, usize, i64);

pub fn generate(rng: &mut StdRng) -> String {
    let (n, edges) = sample(rng);
    let d = distances(n, &edges);
    let edge_text = edges
        .iter()
        .map(|(u, v, w)| format!("{u} -- {v} ({w})"))
        .collect::<Vec<_>>()
        .join("\n");
    let mut output = format!("Tree metric on V={{0,...,{}}}\n{edge_text}", n - 1);
    match rng.random_range(0..3) {
        0 => {
            let rows = d
                .iter()
                .map(|row| format!("  {row:?}"))
                .collect::<Vec<_>>()
                .join("\n");
            output.push_str(&format!("\nd=[\n{rows}\n]"));
        }
        1 => {
            let eccentricities = d
                .iter()
                .map(|row| *row.iter().max().unwrap())
                .collect::<Vec<_>>();
            let diameter = *eccentricities.iter().max().unwrap();
            let radius = *eccentricities.iter().min().unwrap();
            let centers = eccentricities
                .iter()
                .enumerate()
                .filter_map(|(i, &e)| (e == radius).then_some(i))
                .collect::<Vec<_>>();
            let (u, v) = (0..n)
                .flat_map(|u| (u + 1..n).map(move |v| (u, v)))
                .find(|&(u, v)| d[u][v] == diameter)
                .unwrap();
            output.push_str(&format!(
                "\ndiameter={diameter} between {u} and {v}\nradius={radius}\ncenters={centers:?}"
            ));
        }
        _ => {
            let center = rng.random_range(0..n);
            let radius = rng.random_range(1..=*d[center].iter().max().unwrap());
            let ball = |closed: bool| {
                d[center]
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &distance)| {
                        (distance < radius || (closed && distance == radius))
                            .then_some(i.to_string())
                    })
                    .collect::<Vec<_>>()
                    .join(",")
            };
            output.push_str(&format!(
                "\nB({center},{radius})={{{}}}\nB_closed({center},{radius})={{{}}}",
                ball(false),
                ball(true)
            ));
        }
    }
    output
}

fn sample(rng: &mut StdRng) -> (usize, Vec<Edge>) {
    let n = rng.random_range(4..=8);
    let code = (0..n - 2)
        .map(|_| rng.random_range(0..n))
        .collect::<Vec<_>>();
    let edges = decode_prufer(&code)
        .into_iter()
        .map(|(u, v)| (u, v, rng.random_range(1..=9)))
        .collect();
    (n, edges)
}

fn decode_prufer(code: &[usize]) -> Vec<(usize, usize)> {
    let n = code.len() + 2;
    let mut degree = vec![1; n];
    for &v in code {
        degree[v] += 1;
    }
    let mut edges = Vec::with_capacity(n - 1);
    for &v in code {
        let leaf = degree.iter().position(|&d| d == 1).unwrap();
        edges.push((leaf, v));
        degree[leaf] -= 1;
        degree[v] -= 1;
    }
    let leaves = degree
        .iter()
        .enumerate()
        .filter_map(|(v, &d)| (d == 1).then_some(v))
        .collect::<Vec<_>>();
    edges.push((leaves[0], leaves[1]));
    edges
}

fn distances(n: usize, edges: &[Edge]) -> Vec<Vec<i64>> {
    let mut adj = vec![Vec::new(); n];
    for &(u, v, w) in edges {
        adj[u].push((v, w));
        adj[v].push((u, w));
    }
    (0..n)
        .map(|source| {
            let mut row = vec![0; n];
            let mut stack = vec![(source, n, 0)];
            while let Some((v, parent, distance)) = stack.pop() {
                row[v] = distance;
                for &(u, w) in &adj[v] {
                    if u != parent {
                        stack.push((u, v, distance + w));
                    }
                }
            }
            row
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn prufer_codes_cover_every_labeled_four_vertex_tree_once() {
        let mut trees = std::collections::BTreeSet::new();
        for a in 0..4 {
            for b in 0..4 {
                let edges = decode_prufer(&[a, b]);
                let mut degrees = [0; 4];
                let mut normalized = edges
                    .iter()
                    .map(|&(u, v)| {
                        degrees[u] += 1;
                        degrees[v] += 1;
                        (u.min(v), u.max(v))
                    })
                    .collect::<Vec<_>>();
                for (v, &degree) in degrees.iter().enumerate() {
                    assert_eq!(degree, 1 + usize::from(a == v) + usize::from(b == v));
                }
                normalized.sort_unstable();
                assert!(trees.insert(normalized));
                let weighted = edges
                    .into_iter()
                    .map(|(u, v)| (u, v, 1))
                    .collect::<Vec<_>>();
                let d = distances(4, &weighted);
                assert!(
                    d.iter()
                        .enumerate()
                        .all(|(i, row)| row.iter().enumerate().all(|(j, &x)| i == j || x > 0))
                );
            }
        }
        assert_eq!(trees.len(), 16);
    }

    #[test]
    fn metric_axioms_four_point_condition_and_independent_floyd_warshall() {
        let mut rng = StdRng::seed_from_u64(536);
        for _ in 0..500 {
            let (n, edges) = sample(&mut rng);
            let d = distances(n, &edges);
            let mut independent = vec![vec![1_000; n]; n];
            for (i, row) in independent.iter_mut().enumerate() {
                row[i] = 0;
            }
            for &(u, v, w) in &edges {
                independent[u][v] = w;
                independent[v][u] = w;
            }
            for k in 0..n {
                for i in 0..n {
                    for j in 0..n {
                        independent[i][j] =
                            independent[i][j].min(independent[i][k] + independent[k][j]);
                    }
                }
            }
            assert_eq!(d, independent);
            for a in 0..n {
                for b in 0..n {
                    assert_eq!(d[a][b] == 0, a == b);
                    assert_eq!(d[a][b], d[b][a]);
                    for c in 0..n {
                        assert!(d[a][b] <= d[a][c] + d[c][b]);
                        for e in 0..n {
                            let mut sums =
                                [d[a][b] + d[c][e], d[a][c] + d[b][e], d[a][e] + d[b][c]];
                            sums.sort_unstable();
                            assert_eq!(sums[1], sums[2]);
                        }
                    }
                }
            }
        }
    }
}
