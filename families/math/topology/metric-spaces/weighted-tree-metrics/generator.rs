use rand::{Rng, rngs::StdRng};

type Edge = (usize, usize, i64);

pub fn generate(rng: &mut StdRng) -> String {
    let (n, edges) = sample(rng);
    let d = distances(n, &edges);
    let (mut diameter, mut ends) = (0, (0, 0));
    for (i, row) in d.iter().enumerate() {
        for (j, &distance) in row.iter().enumerate().skip(i + 1) {
            if distance > diameter {
                (diameter, ends) = (distance, (i, j));
            }
        }
    }
    let edge_text = edges
        .iter()
        .map(|(u, v, w)| format!("{u}-{v}: {w}"))
        .collect::<Vec<_>>()
        .join("; ");
    let rows = d
        .iter()
        .enumerate()
        .map(|(i, row)| format!("{i}: {row:?}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "Weighted tree metric on vertices 0..{}. Undirected edges and positive lengths: {edge_text}.\nDistance is the sum of lengths on the unique path. Distance matrix (rows and columns in vertex order):\n{rows}\nDiameter {diameter}, attained between {} and {}. This finite metric space is complete and compact; its topology is discrete. For any four vertices, the two largest of d(a,b)+d(c,d), d(a,c)+d(b,d), d(a,d)+d(b,c) are equal.",
        n - 1,
        ends.0,
        ends.1,
    )
}

fn sample(rng: &mut StdRng) -> (usize, Vec<Edge>) {
    let n = rng.random_range(4..=7);
    let edges = (1..n)
        .map(|v| (rng.random_range(0..v), v, rng.random_range(1..=9)))
        .collect();
    (n, edges)
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
    fn rendered_edges_matrix_and_diameter_agree() {
        for seed in 0..500 {
            let output = generate(&mut StdRng::seed_from_u64(seed));
            assert!(!output.ends_with('\n'));
            let lines = output.lines().collect::<Vec<_>>();
            let (header, edge_text) = lines[0]
                .split_once(". Undirected edges and positive lengths: ")
                .unwrap();
            let n = header
                .strip_prefix("Weighted tree metric on vertices 0..")
                .unwrap()
                .parse::<usize>()
                .unwrap()
                + 1;
            assert!((4..=7).contains(&n));
            assert_eq!(lines.len(), n + 3);
            let mut independent = vec![vec![1_000; n]; n];
            for (i, row) in independent.iter_mut().enumerate() {
                row[i] = 0;
            }
            let edges = edge_text.strip_suffix('.').unwrap().split("; ");
            let mut edge_count = 0;
            for (index, edge) in edges.enumerate() {
                let (pair, length) = edge.split_once(": ").unwrap();
                let (parent, child) = pair.split_once('-').unwrap();
                let (parent, child, length) = (
                    parent.parse::<usize>().unwrap(),
                    child.parse::<usize>().unwrap(),
                    length.parse::<i64>().unwrap(),
                );
                assert_eq!(child, index + 1);
                assert!(parent < child && child < n);
                assert!((1..=9).contains(&length));
                independent[parent][child] = length;
                independent[child][parent] = length;
                edge_count += 1;
            }
            assert_eq!(edge_count, n - 1);
            for k in 0..n {
                for i in 0..n {
                    for j in 0..n {
                        independent[i][j] =
                            independent[i][j].min(independent[i][k] + independent[k][j]);
                    }
                }
            }
            for (i, row) in independent.iter().enumerate() {
                let (label, values) = lines[i + 2].split_once(": ").unwrap();
                assert_eq!(label.parse::<usize>().unwrap(), i);
                let rendered = values
                    .strip_prefix('[')
                    .unwrap()
                    .strip_suffix(']')
                    .unwrap()
                    .split(", ")
                    .map(|value| value.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
                assert_eq!(&rendered, row, "seed {seed}");
            }
            let (diameter, witness) = lines[n + 2]
                .strip_prefix("Diameter ")
                .unwrap()
                .split_once(", attained between ")
                .unwrap();
            let (first, remaining) = witness.split_once(" and ").unwrap();
            let (second, claims) = remaining.split_once(". ").unwrap();
            let diameter = diameter.parse::<i64>().unwrap();
            let first = first.parse::<usize>().unwrap();
            let second = second.parse::<usize>().unwrap();
            assert!(first < second && second < n);
            assert_eq!(diameter, independent[first][second]);
            assert_eq!(diameter, *independent.iter().flatten().max().unwrap());
            assert!(claims.starts_with(
                "This finite metric space is complete and compact; its topology is discrete."
            ));
        }
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
