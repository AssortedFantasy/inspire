use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

#[derive(Debug)]
struct Project {
    durations: Vec<u32>,
    predecessors: Vec<Vec<usize>>,
}

#[derive(Debug)]
struct Schedule {
    start: Vec<u32>,
    latest: Vec<u32>,
    finish: u32,
    paths: Vec<Vec<usize>>,
}

impl Project {
    fn successors(&self) -> Vec<Vec<usize>> {
        let mut next = vec![Vec::new(); self.durations.len()];
        for (to, previous) in self.predecessors.iter().enumerate() {
            for &from in previous {
                next[from].push(to);
            }
        }
        next
    }

    fn schedule(&self) -> Schedule {
        let count = self.durations.len();
        let mut start = vec![0; count];
        for i in 0..count {
            start[i] = self.predecessors[i]
                .iter()
                .map(|&j| start[j] + self.durations[j])
                .max()
                .unwrap_or(0);
        }
        let finish = (0..count)
            .map(|i| start[i] + self.durations[i])
            .max()
            .unwrap();
        let next = self.successors();
        let mut latest = vec![0; count];
        for i in (0..count).rev() {
            let end = next[i].iter().map(|&j| latest[j]).min().unwrap_or(finish);
            latest[i] = end - self.durations[i];
        }
        let mut paths = Vec::new();
        for i in 0..count {
            if self.predecessors[i].is_empty() && latest[i] == start[i] {
                self.critical_paths(i, &next, &start, &latest, &mut vec![], &mut paths);
            }
        }
        Schedule {
            start,
            latest,
            finish,
            paths,
        }
    }

    fn critical_paths(
        &self,
        i: usize,
        next: &[Vec<usize>],
        start: &[u32],
        latest: &[u32],
        path: &mut Vec<usize>,
        paths: &mut Vec<Vec<usize>>,
    ) {
        path.push(i);
        if next[i].is_empty() {
            paths.push(path.clone());
        }
        for &j in &next[i] {
            if start[j] == latest[j] && start[j] == start[i] + self.durations[i] {
                self.critical_paths(j, next, start, latest, path, paths);
            }
        }
        path.pop();
    }
}

fn make(rng: &mut StdRng, mode: usize) -> Project {
    match mode {
        0 => {
            let count = rng.random_range(5..=9);
            Project {
                durations: (0..count).map(|_| rng.random_range(2..=12)).collect(),
                predecessors: (0..count)
                    .map(|i| if i == 0 { vec![] } else { vec![i - 1] })
                    .collect(),
            }
        }
        1 => {
            let total = rng.random_range(8..=24);
            let gap = [0, 1, rng.random_range(2..=5)][rng.random_range(0..3)];
            let left = rng.random_range(2..total - 1);
            let right = rng.random_range(1..total - gap);
            Project {
                durations: vec![
                    rng.random_range(2..=8),
                    left,
                    total - left,
                    right,
                    total - gap - right,
                    rng.random_range(2..=8),
                ],
                predecessors: vec![vec![], vec![0], vec![1], vec![0], vec![3], vec![2, 4]],
            }
        }
        2 => {
            let total = rng.random_range(8..=24);
            let mut durations = vec![rng.random_range(2..=8)];
            for _ in 0..3 {
                let part = rng.random_range(1..total);
                durations.extend([part, total - part]);
            }
            durations.push(rng.random_range(2..=8));
            Project {
                durations,
                predecessors: vec![
                    vec![],
                    vec![0],
                    vec![1],
                    vec![0],
                    vec![3],
                    vec![0],
                    vec![5],
                    vec![2, 4, 6],
                ],
            }
        }
        3 => Project {
            durations: (0..10).map(|_| rng.random_range(2..=12)).collect(),
            predecessors: vec![
                vec![],
                vec![0],
                vec![0],
                vec![1],
                vec![1],
                vec![2],
                vec![3, 4, 5],
                vec![6],
                vec![6],
                vec![7, 8],
            ],
        },
        4 => Project {
            durations: (0..9).map(|_| rng.random_range(2..=12)).collect(),
            predecessors: vec![
                vec![],
                vec![],
                vec![],
                vec![0, 1],
                vec![1, 2],
                vec![3],
                vec![3, 4],
                vec![5, 6],
                vec![4, 6],
            ],
        },
        _ => {
            let count = rng.random_range(6..=11);
            let mut predecessors = vec![vec![]];
            for i in 1..count {
                let mut previous: Vec<_> = (0..i).filter(|_| rng.random_bool(0.3)).collect();
                if previous.is_empty() {
                    previous.push(rng.random_range(0..i));
                }
                predecessors.push(previous);
            }
            Project {
                durations: (0..count).map(|_| rng.random_range(1..=12)).collect(),
                predecessors,
            }
        }
    }
}

fn name(i: usize) -> char {
    (b'A' + i as u8) as char
}

pub fn generate(rng: &mut StdRng) -> String {
    let mode = rng.random_range(0..6);
    let mut project = make(rng, mode);
    let s = project.schedule();
    let mut out = if s.paths.len() == 1 && s.paths[0].len() == project.durations.len() {
        let chain = project
            .durations
            .iter()
            .enumerate()
            .map(|(i, duration)| format!("{} ({duration})", name(i)))
            .collect::<Vec<_>>()
            .join(" → ");
        format!(
            "Serial project, durations in days\n{chain}\nCompletion = day {}",
            s.finish
        )
    } else {
        let mut table =
            String::from("Project DAG, durations in days\nTask  Days  After       ES  LS  Slack");
        for i in 0..project.durations.len() {
            let previous = if project.predecessors[i].is_empty() {
                "—".into()
            } else {
                project.predecessors[i]
                    .iter()
                    .map(|&j| name(j).to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            };
            let duration = project.durations[i];
            write!(
                table,
                "\n{:<4}  {:>4}  {:<10}  {:>2}  {:>2}  {:>5}",
                name(i),
                duration,
                previous,
                s.start[i],
                s.latest[i],
                s.latest[i] - s.start[i]
            )
            .unwrap();
        }
        // Retain tied alternatives without listing every critical path.
        write!(
            table,
            "\n\nCompletion = day {}\n{} critical {}\n{}",
            s.finish,
            s.paths.len(),
            if s.paths.len() == 1 {
                "path"
            } else {
                "paths, one shown"
            },
            s.paths[0]
                .iter()
                .map(|&i| name(i).to_string())
                .collect::<Vec<_>>()
                .join(" → ")
        )
        .unwrap();
        table
    };
    let candidates: Vec<_> = (0..project.durations.len())
        .filter(|&i| project.durations[i] > 1)
        .collect();
    let chosen = candidates[rng.random_range(0..candidates.len())];
    project.durations[chosen] -= 1;
    let shorter = project.schedule();
    write!(
        out,
        "\n\nReduce {} by one day → completion day {}",
        name(chosen),
        shorter.finish
    )
    .unwrap();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn enumerate(
        next: &[Vec<usize>],
        i: usize,
        path: &mut Vec<usize>,
        paths: &mut Vec<Vec<usize>>,
    ) {
        path.push(i);
        if next[i].is_empty() {
            paths.push(path.clone());
        }
        for &j in &next[i] {
            enumerate(next, j, path, paths);
        }
        path.pop();
    }

    #[test]
    fn all_paths_independently_certify_schedule_and_slack() {
        for seed in 0..160 {
            let mut rng = StdRng::seed_from_u64(seed);
            for mode in 0..6 {
                let mut p = make(&mut rng, mode);
                let s = p.schedule();
                let next = p.successors();
                let mut paths = Vec::new();
                for i in 0..p.durations.len() {
                    if p.predecessors[i].is_empty() {
                        enumerate(&next, i, &mut vec![], &mut paths);
                    }
                }
                let length = |path: &Vec<usize>| path.iter().map(|&i| p.durations[i]).sum::<u32>();
                assert_eq!(paths.iter().map(length).max().unwrap(), s.finish);
                let mut critical: Vec<_> = paths
                    .iter()
                    .filter(|path| length(path) == s.finish)
                    .cloned()
                    .collect();
                critical.sort();
                let mut reported = s.paths.clone();
                reported.sort();
                assert_eq!(reported, critical);
                for i in 0..p.durations.len() {
                    let through = paths
                        .iter()
                        .filter(|path| path.contains(&i))
                        .map(length)
                        .max()
                        .unwrap();
                    assert_eq!(s.latest[i] - s.start[i], s.finish - through);
                    for &j in &p.predecessors[i] {
                        assert!(s.start[i] >= s.start[j] + p.durations[j]);
                        assert!(s.latest[i] >= s.latest[j] + p.durations[j]);
                    }
                }
                for i in 0..p.durations.len() {
                    p.durations[i] += 1;
                    let delayed = p.schedule();
                    assert_eq!(
                        delayed.finish - s.finish,
                        u32::from(s.latest[i] == s.start[i])
                    );
                    p.durations[i] -= 1;
                    if p.durations[i] > 1 {
                        p.durations[i] -= 1;
                        let shortened = p.schedule();
                        assert_eq!(
                            s.finish - shortened.finish,
                            u32::from(critical.iter().all(|path| path.contains(&i)))
                        );
                        p.durations[i] += 1;
                    }
                }
            }
        }
    }

    #[test]
    fn serial_and_cocritical_regimes_have_different_shortening_effects() {
        for seed in 0..100 {
            let mut rng = StdRng::seed_from_u64(seed);
            let mut serial = make(&mut rng, 0);
            let s = serial.schedule();
            assert_eq!(s.paths.len(), 1);
            assert_eq!(s.finish, serial.durations.iter().sum::<u32>());
            assert_eq!(s.start, s.latest);
            serial.durations[2] -= 1;
            assert_eq!(serial.schedule().finish, s.finish - 1);

            let mut tied = make(&mut rng, 2);
            let s = tied.schedule();
            assert_eq!(s.paths.len(), 3);
            assert_eq!(s.start, s.latest);
            let branch = if tied.durations[1] > 1 { 1 } else { 2 };
            tied.durations[branch] -= 1;
            assert_eq!(tied.schedule().finish, s.finish);
            tied.durations[0] -= 1;
            assert_eq!(tied.schedule().finish, s.finish - 1);

            let near = make(&mut rng, 1);
            let s = near.schedule();
            let gap = near.durations[1] + near.durations[2] - near.durations[3] - near.durations[4];
            assert_eq!(s.latest[3] - s.start[3], gap);
            assert_eq!(s.latest[4] - s.start[4], gap);
            assert_eq!(s.paths.len(), if gap == 0 { 2 } else { 1 });
        }
    }
}
