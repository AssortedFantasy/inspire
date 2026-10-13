use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

pub struct Node {
    pub path: &'static str,
    pub weight: u32,
    pub source: Source,
}

pub enum Source {
    Branch {
        children: &'static [usize],
        total: u64,
    },
    Corpus {
        bytes: &'static [u8],
        count: u64,
        decoded_len: usize,
    },
    Generator(fn(&mut StdRng) -> String),
}

fn child_for_ticket(nodes: &[Node], children: &[usize], mut ticket: u64) -> usize {
    for &index in children {
        let weight = u64::from(nodes[index].weight);
        if ticket < weight {
            return index;
        }
        ticket -= weight;
    }
    unreachable!("ticket must be below the branch's positive total")
}

pub fn sample_leaf<'a>(nodes: &'a [Node], mut index: usize, rng: &mut StdRng) -> &'a Node {
    while let Source::Branch { children, total } = nodes[index].source {
        index = child_for_ticket(nodes, children, rng.random_range(0..total));
    }
    &nodes[index]
}

pub fn listing(nodes: &[Node]) -> String {
    fn visit(nodes: &[Node], index: usize, depth: usize, probability: f64, output: &mut String) {
        let node = &nodes[index];
        if index != 0 {
            let kind = match node.source {
                Source::Branch { .. } => "branch",
                Source::Corpus { .. } => "corpus",
                Source::Generator(_) => "generator",
            };
            writeln!(
                output,
                "{}{} [{kind}]\tweight={}\teffective={:.4}%",
                "  ".repeat(depth - 1),
                node.path,
                node.weight,
                probability * 100.0
            )
            .unwrap();
        }
        if let Source::Branch { children, total } = node.source {
            for &child in children {
                visit(
                    nodes,
                    child,
                    depth + 1,
                    probability * f64::from(nodes[child].weight) / total as f64,
                    output,
                );
            }
        }
    }
    let mut output = String::new();
    visit(nodes, 0, 0, 1.0, &mut output);
    output.pop();
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn dummy(_: &mut StdRng) -> String {
        "example".into()
    }

    // Sibling branches receive 1:2 weight regardless of their leaf counts.
    static FIXTURE: &[Node] = &[
        Node {
            path: "",
            weight: 1,
            source: Source::Branch {
                children: &[1, 2, 5],
                total: 3,
            },
        },
        Node {
            path: "poems",
            weight: 1,
            source: Source::Generator(dummy),
        },
        Node {
            path: "math",
            weight: 2,
            source: Source::Branch {
                children: &[3, 4],
                total: 4,
            },
        },
        Node {
            path: "math/algebra",
            weight: 1,
            source: Source::Generator(dummy),
        },
        Node {
            path: "math/geometry",
            weight: 3,
            source: Source::Generator(dummy),
        },
        Node {
            path: "disabled",
            weight: 0,
            source: Source::Branch {
                children: &[6],
                total: 1,
            },
        },
        Node {
            path: "disabled/leaf",
            weight: 1,
            source: Source::Generator(dummy),
        },
    ];

    #[test]
    fn sibling_weights_define_exact_intervals() {
        assert_eq!(child_for_ticket(FIXTURE, &[1, 2, 5], 0), 1);
        assert_eq!(child_for_ticket(FIXTURE, &[1, 2, 5], 1), 2);
        assert_eq!(child_for_ticket(FIXTURE, &[1, 2, 5], 2), 2);
        for ticket in 0..4 {
            assert_eq!(
                child_for_ticket(FIXTURE, &[3, 4], ticket),
                if ticket == 0 { 3 } else { 4 }
            );
        }
    }

    #[test]
    fn subtree_sampling_stays_inside_and_explicit_selection_bypasses_incoming_weight() {
        let mut rng = StdRng::seed_from_u64(42);
        for _ in 0..100 {
            assert!(sample_leaf(FIXTURE, 2, &mut rng).path.starts_with("math/"));
            assert!(
                !sample_leaf(FIXTURE, 0, &mut rng)
                    .path
                    .starts_with("disabled/")
            );
        }
        assert_eq!(sample_leaf(FIXTURE, 5, &mut rng).path, "disabled/leaf");
        assert_eq!(sample_leaf(FIXTURE, 6, &mut rng).path, "disabled/leaf");
    }

    #[test]
    fn listing_multiplies_probabilities_along_the_path() {
        let output = listing(FIXTURE);
        assert!(output.contains("math [branch]\tweight=2\teffective=66.6667%"));
        assert!(output.contains("math/algebra [generator]\tweight=1\teffective=16.6667%"));
        assert!(output.contains("math/geometry [generator]\tweight=3\teffective=50.0000%"));
        assert!(output.contains("disabled/leaf [generator]\tweight=1\teffective=0.0000%"));
    }
}
