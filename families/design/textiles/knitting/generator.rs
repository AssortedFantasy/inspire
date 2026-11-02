use rand::{Rng, rngs::StdRng};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Op {
    Knit(usize),
    Purl(usize),
    Yo,
    Kfb,
    Ssk,
    K2tog,
    Cable(usize, bool), // half-width, held in front
}

impl Op {
    fn text(&self) -> String {
        match self {
            Self::Knit(n) => format!("k{n}"),
            Self::Purl(n) => format!("p{n}"),
            Self::Yo => "yo".to_owned(),
            Self::Kfb => "kfb".to_owned(),
            Self::Ssk => "ssk".to_owned(),
            Self::K2tog => "k2tog".to_owned(),
            Self::Cable(n, front) => format!("C{}{}", 2 * n, if *front { 'F' } else { 'B' }),
        }
    }

    fn change(&self) -> isize {
        match self {
            Self::Yo | Self::Kfb => 1,
            Self::Ssk | Self::K2tog => -1,
            _ => 0,
        }
    }
}

struct Pattern {
    name: &'static str,
    width: usize,
    rounds: Vec<Vec<Op>>,
}

fn knit(row: &mut Vec<Op>, n: usize) {
    if n > 0 {
        row.push(Op::Knit(n));
    }
}

fn texture_row(cells: impl Iterator<Item = bool>) -> Vec<Op> {
    let mut runs: Vec<(bool, usize)> = Vec::new();
    for cell in cells {
        if let Some(last) = runs.last_mut().filter(|last| last.0 == cell) {
            last.1 += 1;
        } else {
            runs.push((cell, 1));
        }
    }
    runs.into_iter()
        .map(|(k, n)| if k { Op::Knit(n) } else { Op::Purl(n) })
        .collect()
}

fn pattern(rng: &mut StdRng) -> Pattern {
    match rng.random_range(0..4) {
        0 => {
            let depth = rng.random_range(2..=5);
            let width = 2 * depth + 5;
            let reverse = rng.random_bool(0.5);
            let mut rounds = Vec::new();
            for step in 0..=depth {
                let i = if reverse { depth - step } else { step };
                let mut row = Vec::new();
                knit(&mut row, depth - i);
                row.extend([Op::Yo, Op::Ssk]);
                knit(&mut row, 2 * i + 1);
                row.extend([Op::K2tog, Op::Yo]);
                knit(&mut row, depth - i);
                rounds.push(row);
                rounds.push(vec![Op::Knit(width)]);
            }
            Pattern {
                name: if reverse {
                    "Converging eyelets"
                } else {
                    "Diverging eyelets"
                },
                width,
                rounds,
            }
        }
        1 => {
            let half = rng.random_range(2..=3);
            let gap = rng.random_range(1..=3);
            let braid = rng.random_bool(0.5);
            let stagger = rng.random_bool(0.5);
            let height = if half == 2 { 8 } else { 12 };
            let width = if braid {
                3 * half + 2 * gap
            } else {
                4 * half + 3 * gap
            };
            let front = rng.random_bool(0.5);
            let mut rounds = Vec::new();
            for r in 0..height {
                let first = r == 0;
                let second = r == if stagger || braid { height / 2 } else { 0 };
                let mut row = vec![Op::Purl(gap)];
                if braid {
                    if first {
                        row.extend([Op::Cable(half, front), Op::Knit(half)]);
                    } else if second {
                        row.extend([Op::Knit(half), Op::Cable(half, !front)]);
                    } else {
                        row.push(Op::Knit(3 * half));
                    }
                } else {
                    row.push(if first {
                        Op::Cable(half, front)
                    } else {
                        Op::Knit(2 * half)
                    });
                    row.push(Op::Purl(gap));
                    row.push(if second {
                        Op::Cable(half, !front)
                    } else {
                        Op::Knit(2 * half)
                    });
                }
                row.push(Op::Purl(gap));
                rounds.push(row);
            }
            Pattern {
                name: if braid {
                    "Three-strand braid"
                } else {
                    "Opposed cables"
                },
                width,
                rounds,
            }
        }
        2 => {
            let block = rng.random_range(1..=4);
            let height = rng.random_range(1..=4);
            let diagonal = rng.random_bool(0.5);
            let width = if diagonal {
                rng.random_range(5..=10)
            } else {
                2 * block
            };
            let count = if diagonal { width } else { 2 * height };
            let phase = rng.random_range(0..width);
            let reverse = rng.random_bool(0.5);
            let rounds = (0..count)
                .map(|r| {
                    texture_row((0..width).map(|c| {
                        if diagonal {
                            (c + phase + if reverse { width - r } else { r }) % width < width / 2
                        } else {
                            (c / block + r / height) % 2 == 0
                        }
                    }))
                })
                .collect();
            Pattern {
                name: if diagonal {
                    "Travelling knit-purl ribs"
                } else {
                    "Knit-purl blocks"
                },
                width,
                rounds,
            }
        }
        _ => {
            let edge = rng.random_range(1..=3);
            let middle = rng.random_range(2..=5);
            let steps = rng.random_range(1..=3);
            let width = 2 * edge + middle + 2;
            let mut live = width;
            let mut rounds = Vec::new();
            for _ in 0..steps {
                rounds.push(vec![
                    Op::Knit(edge),
                    Op::Kfb,
                    Op::Knit(live - 2 * edge - 2),
                    Op::Kfb,
                    Op::Knit(edge),
                ]);
                live += 2;
                rounds.push(vec![Op::Knit(live)]);
            }
            for _ in 0..steps {
                rounds.push(
                    vec![
                        Op::Knit(edge),
                        Op::Ssk,
                        Op::Knit(live - 2 * edge - 4),
                        Op::K2tog,
                        Op::Knit(edge),
                    ]
                    .into_iter()
                    .filter(|op| !matches!(op, Op::Knit(0)))
                    .collect(),
                );
                live -= 2;
                rounds.push(vec![Op::Knit(live)]);
            }
            Pattern {
                name: "Expanding and contracting panels",
                width,
                rounds,
            }
        }
    }
}

struct RoundRun<'a> {
    first: usize,
    last: usize,
    row: &'a [Op],
}

fn round_runs(rounds: &[Vec<Op>]) -> Vec<RoundRun<'_>> {
    let mut runs: Vec<RoundRun<'_>> = Vec::new();
    for (i, row) in rounds.iter().enumerate() {
        if let Some(last) = runs
            .last_mut()
            .filter(|last| last.row == row && row.iter().map(Op::change).sum::<isize>() == 0)
        {
            last.last = i + 1;
        } else {
            runs.push(RoundRun {
                first: i + 1,
                last: i + 1,
                row,
            });
        }
    }
    runs
}

pub fn generate(rng: &mut StdRng) -> String {
    let p = pattern(rng);
    let mut lines = vec![
        p.name.to_owned(),
        format!("{}-stitch repeat, in the round", p.width),
    ];
    let mut live = p.width as isize;
    let shaped = p
        .rounds
        .iter()
        .any(|row| row.iter().map(Op::change).sum::<isize>() != 0);
    for run in round_runs(&p.rounds) {
        live += run.row.iter().map(Op::change).sum::<isize>() * (run.last - run.first + 1) as isize;
        let label = if run.first == run.last {
            format!("Round {}", run.first)
        } else {
            format!("Rounds {}–{}", run.first, run.last)
        };
        let text = run.row.iter().map(Op::text).collect::<Vec<_>>().join(", ");
        lines.push(if shaped {
            format!("{label}  [{text}]  ({live} sts)")
        } else {
            format!("{label}  [{text}]")
        });
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    #[test]
    fn compressed_rounds_expand_to_the_same_working_instructions() {
        let mut compressed = HashSet::new();
        for seed in 0..512 {
            let p = pattern(&mut StdRng::seed_from_u64(seed));
            let runs = round_runs(&p.rounds);
            let mut expanded = Vec::new();
            for run in &runs {
                assert_eq!(run.first, expanded.len() + 1);
                assert!(run.last >= run.first);
                expanded.extend((run.first..=run.last).map(|_| run.row.to_vec()));
            }
            assert_eq!(expanded, p.rounds);
            assert!(runs.windows(2).all(|pair| pair[0].row != pair[1].row));
            if runs.len() < p.rounds.len() {
                compressed.insert(p.name);
            }
        }
        assert!(compressed.contains("Opposed cables"));
        assert!(compressed.contains("Three-strand braid"));
        assert!(compressed.contains("Knit-purl blocks"));

        // Neither the holding direction nor distinct live counts can disappear.
        let rounds = vec![
            vec![Op::Cable(2, true)],
            vec![Op::Cable(2, false)],
            vec![Op::Kfb],
            vec![Op::Kfb],
        ];
        assert_eq!(round_runs(&rounds).len(), rounds.len());
    }

    // Interpret each row against an actual needle, consuming every old loop.
    // The pairs carried by a decreased loop retain both source positions.
    fn work(row: &[Op], needle: Vec<Vec<usize>>, next_id: &mut usize) -> Vec<Vec<usize>> {
        let mut old = needle.into_iter();
        let mut new = Vec::new();
        for op in row {
            match *op {
                Op::Knit(n) | Op::Purl(n) => {
                    assert!(n > 0);
                    for _ in 0..n {
                        new.push(old.next().expect("available stitch"));
                    }
                }
                Op::Yo => {
                    new.push(vec![*next_id]);
                    *next_id += 1;
                }
                Op::Kfb => {
                    let original = old.next().expect("increase source");
                    new.push(original);
                    new.push(vec![*next_id]);
                    *next_id += 1;
                }
                Op::Ssk | Op::K2tog => {
                    let mut a = old.next().expect("decrease first");
                    a.extend(old.next().expect("decrease second"));
                    new.push(a);
                }
                Op::Cable(n, _) => {
                    let a: Vec<_> = (0..n).map(|_| old.next().expect("held stitch")).collect();
                    let b: Vec<_> = (0..n)
                        .map(|_| old.next().expect("crossing stitch"))
                        .collect();
                    new.extend(b);
                    new.extend(a);
                }
            }
        }
        assert!(old.next().is_none(), "all stitches worked");
        new
    }

    #[test]
    fn each_repeat_is_workable_and_returns_to_its_live_stitch_count() {
        let mut names = HashSet::new();
        let mut saw = [false; 4];
        for seed in 0..512 {
            let p = pattern(&mut StdRng::seed_from_u64(seed));
            names.insert(p.name);
            let mut needle: Vec<_> = (0..p.width).map(|i| vec![i]).collect();
            let mut next_id = p.width;
            // Work twice, including the boundary between vertical repeats.
            for row in p.rounds.iter().cycle().take(2 * p.rounds.len()) {
                let before = needle.len() as isize;
                needle = work(row, needle, &mut next_id);
                assert_eq!(
                    needle.len() as isize,
                    before + row.iter().map(Op::change).sum::<isize>()
                );
                let sources: Vec<_> = needle.iter().flatten().copied().collect();
                assert_eq!(sources.len(), sources.iter().collect::<HashSet<_>>().len());
                for op in row {
                    match op {
                        Op::Yo => saw[0] = true,
                        Op::Kfb => saw[1] = true,
                        Op::Ssk | Op::K2tog => saw[2] = true,
                        Op::Cable(_, _) => saw[3] = true,
                        _ => (),
                    }
                }
            }
            assert_eq!(needle.len(), p.width);
            assert!((1..=24).contains(&p.rounds.len()));
        }
        assert_eq!(names.len(), 7);
        assert!(saw.into_iter().all(|value| value));
    }

    #[test]
    fn cables_exchange_intact_strands_without_losing_loops() {
        for half in [2, 3] {
            for front in [false, true] {
                let needle: Vec<_> = (0..2 * half).map(|i| vec![i]).collect();
                let crossed = work(&[Op::Cable(half, front)], needle.clone(), &mut (2 * half));
                assert_ne!(crossed, needle);
                assert_eq!(
                    work(&[Op::Cable(half, front)], crossed, &mut (2 * half)),
                    needle
                );
            }
        }
    }
}
