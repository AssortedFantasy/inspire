use rand::{Rng, rngs::StdRng, seq::SliceRandom};
use std::fmt::Write;

struct Search<'a> {
    graph: &'a [Vec<usize>],
    index: Vec<Option<usize>>,
    low: Vec<usize>,
    stack: Vec<usize>,
    active: Vec<bool>,
    order: Vec<usize>,
    components: Vec<Vec<usize>>,
}

impl Search<'_> {
    fn visit(&mut self, v: usize) {
        let index = self.order.len();
        self.index[v] = Some(index);
        self.low[v] = index;
        self.order.push(v);
        self.stack.push(v);
        self.active[v] = true;
        for &w in &self.graph[v] {
            if self.index[w].is_none() {
                self.visit(w);
                self.low[v] = self.low[v].min(self.low[w]);
            } else if self.active[w] {
                self.low[v] = self.low[v].min(self.index[w].unwrap());
            }
        }
        if self.low[v] == index {
            let mut component = Vec::new();
            loop {
                let w = self.stack.pop().unwrap();
                self.active[w] = false;
                component.push(w);
                if w == v {
                    break;
                }
            }
            self.components.push(component);
        }
    }
}

fn execute(graph: &[Vec<usize>]) -> Search<'_> {
    let n = graph.len();
    let mut search = Search {
        graph,
        index: vec![None; n],
        low: vec![0; n],
        stack: Vec::new(),
        active: vec![false; n],
        order: Vec::new(),
        components: Vec::new(),
    };
    for v in 0..n {
        if search.index[v].is_none() {
            search.visit(v);
        }
    }
    search
}

fn instance(rng: &mut StdRng) -> Vec<Vec<usize>> {
    let n = rng.random_range(7..=11);
    let mut vertices: Vec<_> = (0..n).collect();
    vertices.shuffle(rng);
    let mut blocks = Vec::new();
    let mut remaining = vertices.as_slice();
    while !remaining.is_empty() {
        let size = if blocks.is_empty() {
            rng.random_range(2..=4)
        } else {
            rng.random_range(1..=4.min(remaining.len()))
        };
        let (block, rest) = remaining.split_at(size);
        blocks.push(block);
        remaining = rest;
    }
    let mut graph = vec![Vec::new(); n];
    for (i, block) in blocks.iter().enumerate() {
        if block.len() > 1 {
            for j in 0..block.len() {
                graph[block[j]].push(block[(j + 1) % block.len()]);
            }
        }
        for &a in *block {
            for &b in *block {
                if a != b && rng.random_bool(0.2) {
                    graph[a].push(b);
                }
            }
            for later in &blocks[i + 1..] {
                for &b in *later {
                    if rng.random_bool(0.2) {
                        graph[a].push(b);
                    }
                }
            }
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
    let search = execute(&graph);
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
    write!(
        text,
        "\n\nTarjan discovery: {}\nVertex  Index  Low",
        names(&search.order, " ")
    )
    .unwrap();
    for v in 0..graph.len() {
        let label = (b'A' + v as u8) as char;
        write!(
            text,
            "\n{label}        {:>2}   {:>2}",
            search.index[v].unwrap(),
            search.low[v]
        )
        .unwrap();
    }
    text.push_str("\n\nComponents, in stack-pop order");
    for component in &search.components {
        write!(text, "\n{{{}}}", names(component, ", ")).unwrap();
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn components_equal_mutual_reachability() {
        let mut rng = StdRng::seed_from_u64(1831);
        for _ in 0..500 {
            let graph = instance(&mut rng);
            let n = graph.len();
            let result = execute(&graph);
            let mut reachable = vec![vec![false; n]; n];
            for (v, row) in reachable.iter_mut().enumerate() {
                row[v] = true;
                for &w in &graph[v] {
                    row[w] = true;
                }
            }
            for k in 0..n {
                for v in 0..n {
                    for w in 0..n {
                        reachable[v][w] |= reachable[v][k] && reachable[k][w];
                    }
                }
            }
            let mut membership = vec![usize::MAX; n];
            for (i, component) in result.components.iter().enumerate() {
                for &v in component {
                    assert_eq!(membership[v], usize::MAX);
                    membership[v] = i;
                }
            }
            assert!(membership.iter().all(|&i| i != usize::MAX));
            for v in 0..n {
                for w in 0..n {
                    assert_eq!(
                        membership[v] == membership[w],
                        reachable[v][w] && reachable[w][v]
                    );
                    if graph[v].contains(&w) && membership[v] != membership[w] {
                        assert!(membership[v] > membership[w]);
                    }
                }
            }
            assert!(result.components.len() >= 2);
            assert!(
                result
                    .components
                    .iter()
                    .any(|component| component.len() > 1)
            );
        }
    }
}
