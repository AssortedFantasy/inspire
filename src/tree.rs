use crate::layout::{Name, Node, Source};
use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

pub struct Tree<'a> {
    pub nodes: &'a [Node],
    pub children: &'a [u32],
    pub names: &'a [Name],
    pub name_bytes: &'a [u8],
}

fn child_for_ticket(nodes: &[Node], children: &[u32], mut ticket: u64) -> usize {
    for &index in children {
        let weight = u64::from(nodes[index as usize].weight);
        if ticket < weight {
            return index as usize;
        }
        ticket -= weight;
    }
    unreachable!("ticket must be below the branch's positive total")
}

impl Tree<'_> {
    fn child_range(&self, first: u32, count: u32) -> &[u32] {
        &self.children[first as usize..first as usize + count as usize]
    }

    pub fn name(&self, index: usize) -> &str {
        let Name { offset, len } = self.names[index];
        std::str::from_utf8(&self.name_bytes[offset as usize..offset as usize + len as usize])
            .expect("build validates node names")
    }

    pub fn resolve(&self, path: &str) -> Option<usize> {
        let mut index = 0;
        for segment in path.split('/') {
            let Source::Branch {
                first_child,
                child_count,
                ..
            } = self.nodes[index].source
            else {
                return None;
            };
            let children = self.child_range(first_child, child_count);
            let position = children
                .binary_search_by(|&child| self.name(child as usize).cmp(segment))
                .ok()?;
            index = children[position] as usize;
        }
        Some(index)
    }

    pub fn sample_leaf(&self, mut index: usize, rng: &mut StdRng) -> usize {
        while let Source::Branch {
            first_child,
            child_count,
            total_weight,
        } = self.nodes[index].source
        {
            index = child_for_ticket(
                self.nodes,
                self.child_range(first_child, child_count),
                rng.random_range(0..total_weight),
            );
        }
        index
    }

    pub fn listing(&self) -> String {
        let mut output = String::new();
        self.visit(0, 0, 1.0, &mut String::new(), &mut output);
        output.pop();
        output
    }

    fn visit(
        &self,
        index: usize,
        depth: usize,
        probability: f64,
        path: &mut String,
        output: &mut String,
    ) {
        let saved_len = path.len();
        let node = &self.nodes[index];
        if index != 0 {
            if !path.is_empty() {
                path.push('/');
            }
            path.push_str(self.name(index));
            let kind = match node.source {
                Source::Branch { .. } => "branch",
                Source::Corpus(_) => "corpus",
                Source::Generator(_) => "generator",
            };
            writeln!(
                output,
                "{:indent$}{path} [{kind}]\tweight={}\teffective={:.4}%",
                "",
                node.weight,
                probability * 100.0,
                indent = (depth - 1) * 2
            )
            .unwrap();
        }
        if let Source::Branch {
            first_child,
            child_count,
            total_weight,
        } = node.source
        {
            for &child in self.child_range(first_child, child_count) {
                self.visit(
                    child as usize,
                    depth + 1,
                    probability * f64::from(self.nodes[child as usize].weight)
                        / total_weight as f64,
                    path,
                    output,
                );
            }
        }
        path.truncate(saved_len);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    // Sorted siblings; branch weights are independent of their leaf counts.
    static FIXTURE: Tree<'static> = Tree {
        nodes: &[
            Node {
                weight: 1,
                source: Source::Branch {
                    first_child: 0,
                    child_count: 3,
                    total_weight: 3,
                },
            },
            Node {
                weight: 0,
                source: Source::Branch {
                    first_child: 3,
                    child_count: 1,
                    total_weight: 1,
                },
            },
            Node {
                weight: 1,
                source: Source::Generator(0),
            },
            Node {
                weight: 2,
                source: Source::Branch {
                    first_child: 4,
                    child_count: 2,
                    total_weight: 4,
                },
            },
            Node {
                weight: 1,
                source: Source::Generator(0),
            },
            Node {
                weight: 3,
                source: Source::Generator(0),
            },
            Node {
                weight: 1,
                source: Source::Generator(0),
            },
        ],
        children: &[1, 3, 6, 2, 4, 5],
        names: &[
            Name { offset: 0, len: 0 },
            Name { offset: 0, len: 8 },
            Name { offset: 8, len: 4 },
            Name { offset: 12, len: 4 },
            Name { offset: 16, len: 7 },
            Name { offset: 23, len: 8 },
            Name { offset: 31, len: 5 },
        ],
        name_bytes: b"disabledleafmathalgebrageometrypoems",
    };

    #[test]
    fn sibling_weights_define_exact_intervals() {
        assert_eq!(child_for_ticket(FIXTURE.nodes, &[1, 3, 6], 0), 3);
        assert_eq!(child_for_ticket(FIXTURE.nodes, &[1, 3, 6], 1), 3);
        assert_eq!(child_for_ticket(FIXTURE.nodes, &[1, 3, 6], 2), 6);
        for ticket in 0..4 {
            assert_eq!(
                child_for_ticket(FIXTURE.nodes, &[4, 5], ticket),
                if ticket == 0 { 4 } else { 5 }
            );
        }
    }

    #[test]
    fn subtree_sampling_stays_inside_and_explicit_selection_bypasses_incoming_weight() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..100 {
            assert!([4, 5].contains(&FIXTURE.sample_leaf(3, &mut rng)));
            assert_ne!(FIXTURE.sample_leaf(0, &mut rng), 2);
        }
        assert_eq!(FIXTURE.sample_leaf(1, &mut rng), 2);
        assert_eq!(FIXTURE.sample_leaf(2, &mut rng), 2);
    }

    #[test]
    fn paths_resolve_segments_and_reject_partial_or_malformed_paths() {
        for (path, index) in [
            ("math", 3),
            ("math/algebra", 4),
            ("poems", 6),
            ("disabled/leaf", 2),
        ] {
            assert_eq!(FIXTURE.resolve(path), Some(index));
        }
        for path in [
            "",
            "/math",
            "math/",
            "math//algebra",
            "math/algebra/extra",
            "Math",
            "math\\algebra",
            "algebra",
            "mat",
        ] {
            assert_eq!(FIXTURE.resolve(path), None, "{path}");
        }
    }

    #[test]
    fn listing_multiplies_probabilities_and_restores_paths_between_siblings() {
        let output = FIXTURE.listing();
        assert!(output.contains("math [branch]\tweight=2\teffective=66.6667%"));
        assert!(output.contains("math/algebra [generator]\tweight=1\teffective=16.6667%"));
        assert!(output.contains("math/geometry [generator]\tweight=3\teffective=50.0000%"));
        assert!(output.contains("disabled/leaf [generator]\tweight=1\teffective=0.0000%"));
        assert!(output.ends_with("poems [generator]\tweight=1\teffective=33.3333%"));
    }
}
