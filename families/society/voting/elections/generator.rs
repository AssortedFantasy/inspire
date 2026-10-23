use rand::{Rng, rngs::StdRng, seq::SliceRandom};

struct Election {
    n: usize,
    ballots: Vec<(u32, Vec<usize>)>,
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..5);
    let election = sample(rng, regime);
    let n = election.n;
    let (plurality, borda, pairwise) = count(&election);
    let (runoff, ties) = irv(&election, (1 << n) - 1);
    let condorcet: Vec<_> = (0..n)
        .filter(|&a| (0..n).all(|b| a == b || pairwise[a][b] > pairwise[b][a]))
        .collect();
    let mut lines = vec![format!(
        "{} ballots",
        election.ballots.iter().map(|b| b.0).sum::<u32>()
    )];
    for (votes, ranking) in &election.ballots {
        lines.push(format!(
            "{votes:>3}  {}",
            ranking
                .iter()
                .map(|&a| name(a))
                .collect::<Vec<_>>()
                .join(" > ")
        ));
    }
    lines.push(format!(
        "\nPlurality {} → {}",
        scores(&plurality),
        names(&winners(&plurality))
    ));
    lines.push(format!(
        "Borda ({}) {} → {}",
        (0..n)
            .rev()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("–"),
        scores(&borda),
        names(&winners(&borda))
    ));
    lines.push(format!(
        "IRV{} {}{}",
        if runoff.count_ones() > 1 {
            " possible winners"
        } else {
            ""
        },
        names(
            &(0..n)
                .filter(|a| runoff & (1 << a) != 0)
                .collect::<Vec<_>>()
        ),
        if ties {
            " (all tied eliminations explored)"
        } else {
            ""
        }
    ));
    lines.push(format!("Condorcet {}", names(&condorcet)));
    lines.push("\nHead-to-head".into());
    for (a, row) in pairwise.iter().enumerate() {
        for (b, votes) in row.iter().enumerate().skip(a + 1) {
            lines.push(format!(
                "{}–{}  {}–{}",
                name(a),
                name(b),
                votes,
                pairwise[b][a]
            ));
        }
    }
    lines.join("\n")
}

fn sample(rng: &mut StdRng, regime: usize) -> Election {
    let n = if regime == 0 || regime == 4 {
        rng.random_range(3..=4)
    } else {
        3
    };
    let mut ballots = match regime {
        0 => {
            let groups = rng.random_range(2..=5);
            let mut rest = Vec::new();
            for _ in 0..groups {
                let mut ranking: Vec<_> = (0..n).collect();
                ranking.shuffle(rng);
                rest.push((rng.random_range(1..=12), ranking));
            }
            let other_votes: u32 = rest.iter().map(|b| b.0).sum();
            let mut ranking: Vec<_> = (1..n).collect();
            ranking.shuffle(rng);
            ranking.insert(0, 0);
            rest.push((4 * other_votes + rng.random_range(1..=7), ranking));
            rest
        }
        1 => {
            let k = rng.random_range(10..=40);
            vec![
                (k + rng.random_range(0..=5), vec![0, 1, 2]),
                (k + rng.random_range(0..=5), vec![1, 2, 0]),
                (k + rng.random_range(0..=5), vec![2, 0, 1]),
            ]
        }
        2 => {
            let k = rng.random_range(8..=30);
            vec![
                (k + rng.random_range(0..=k / 3), vec![0, 1, 2]),
                (k + rng.random_range(0..=k / 3), vec![2, 1, 0]),
                (
                    k - 1,
                    if rng.random_bool(0.5) {
                        vec![1, 0, 2]
                    } else {
                        vec![1, 2, 0]
                    },
                ),
            ]
        }
        3 => {
            let minority = rng.random_range(5..=45);
            let majority = rng.random_range(minority + 1..2 * minority);
            vec![(majority, vec![0, 1, 2]), (minority, vec![1, 2, 0])]
        }
        _ => (0..rng.random_range(4..=8))
            .map(|_| {
                let mut ranking: Vec<_> = (0..n).collect();
                ranking.shuffle(rng);
                (rng.random_range(1..=25), ranking)
            })
            .collect(),
    };
    let mut labels: Vec<_> = (0..n).collect();
    labels.shuffle(rng);
    for (_, ranking) in &mut ballots {
        for a in ranking {
            *a = labels[*a];
        }
    }
    ballots.sort_by(|a, b| a.1.cmp(&b.1));
    let mut combined: Vec<(u32, Vec<usize>)> = Vec::new();
    for (votes, ranking) in ballots {
        if let Some(last) = combined.last_mut().filter(|b| b.1 == ranking) {
            last.0 += votes;
        } else {
            combined.push((votes, ranking));
        }
    }
    Election {
        n,
        ballots: combined,
    }
}

fn count(election: &Election) -> (Vec<u32>, Vec<u32>, Vec<Vec<u32>>) {
    let n = election.n;
    let mut plurality = vec![0; n];
    let mut borda = vec![0; n];
    let mut pairwise = vec![vec![0; n]; n];
    for (votes, ranking) in &election.ballots {
        plurality[ranking[0]] += votes;
        for (rank, &a) in ranking.iter().enumerate() {
            borda[a] += votes * (n - rank - 1) as u32;
            for &b in &ranking[rank + 1..] {
                pairwise[a][b] += votes;
            }
        }
    }
    (plurality, borda, pairwise)
}

fn irv(election: &Election, active: u8) -> (u8, bool) {
    let mut tallies = vec![0; election.n];
    for (votes, ranking) in &election.ballots {
        tallies[*ranking.iter().find(|&&a| active & (1 << a) != 0).unwrap()] += votes;
    }
    let total: u32 = tallies.iter().sum();
    if let Some(a) = (0..election.n).find(|&a| tallies[a] * 2 > total) {
        return (1 << a, false);
    }
    let minimum = (0..election.n)
        .filter(|a| active & (1 << a) != 0)
        .map(|a| tallies[a])
        .min()
        .unwrap();
    let eliminated: Vec<_> = (0..election.n)
        .filter(|a| active & (1 << a) != 0 && tallies[*a] == minimum)
        .collect();
    let mut winners = 0;
    let mut ties = eliminated.len() > 1;
    for a in eliminated {
        let (possible, child_ties) = irv(election, active & !(1 << a));
        winners |= possible;
        ties |= child_ties;
    }
    (winners, ties)
}

fn winners(scores: &[u32]) -> Vec<usize> {
    let maximum = scores.iter().max().unwrap();
    (0..scores.len())
        .filter(|&a| scores[a] == *maximum)
        .collect()
}

fn name(a: usize) -> String {
    char::from(b'A' + a as u8).to_string()
}

fn names(candidates: &[usize]) -> String {
    if candidates.is_empty() {
        "none".into()
    } else {
        candidates
            .iter()
            .map(|&a| name(a))
            .collect::<Vec<_>>()
            .join(" / ")
    }
}

fn scores(tallies: &[u32]) -> String {
    tallies
        .iter()
        .enumerate()
        .map(|(a, v)| format!("{} {v}", name(a)))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn constructed_elections_preserve_their_regimes_and_exact_vote_totals() {
        let mut rng = StdRng::seed_from_u64(1937);
        for regime in 0..5 {
            for _ in 0..300 {
                let e = sample(&mut rng, regime);
                let total: u32 = e.ballots.iter().map(|b| b.0).sum();
                let (plurality, borda, pairwise) = count(&e);
                assert_eq!(plurality.iter().sum::<u32>(), total);
                assert_eq!(
                    borda.iter().sum::<u32>(),
                    total * (e.n * (e.n - 1) / 2) as u32
                );
                for a in 0..e.n {
                    for b in a + 1..e.n {
                        assert_eq!(pairwise[a][b] + pairwise[b][a], total);
                        let direct: u32 = e
                            .ballots
                            .iter()
                            .filter(|(_, r)| {
                                r.iter().position(|&x| x == a) < r.iter().position(|&x| x == b)
                            })
                            .map(|b| b.0)
                            .sum();
                        assert_eq!(pairwise[a][b], direct);
                    }
                }
                let cw: Vec<_> = (0..e.n)
                    .filter(|&a| (0..e.n).all(|b| a == b || pairwise[a][b] > pairwise[b][a]))
                    .collect();
                let (irv_winners, _) = irv(&e, (1 << e.n) - 1);
                match regime {
                    0 => {
                        let a = winners(&plurality)[0];
                        assert!(2 * plurality[a] > total);
                        assert_eq!(cw, vec![a]);
                        assert_eq!(winners(&borda), vec![a]);
                        assert_eq!(irv_winners, 1 << a);
                    }
                    1 => {
                        assert!(cw.is_empty());
                        assert!((0..3).all(|a| {
                            (0..3).filter(|&b| pairwise[a][b] > pairwise[b][a]).count() == 1
                        }));
                    }
                    2 => {
                        assert_eq!(cw.len(), 1);
                        assert_eq!(irv_winners & (1 << cw[0]), 0);
                        assert_eq!(plurality[cw[0]], *plurality.iter().min().unwrap());
                    }
                    3 => {
                        let a = winners(&plurality)[0];
                        assert!(plurality[a] * 2 > total);
                        assert_ne!(winners(&borda), vec![a]);
                        assert_eq!(irv_winners, 1 << a);
                        assert_eq!(cw, vec![a]);
                    }
                    _ => assert_ne!(irv_winners, 0),
                }
            }
        }
    }

    #[test]
    fn irv_explores_ties_without_a_hidden_candidate_order() {
        let e = Election {
            n: 3,
            ballots: vec![(1, vec![0, 1, 2]), (1, vec![1, 2, 0]), (1, vec![2, 0, 1])],
        };
        assert_eq!(irv(&e, 7), (7, true));
        let e = Election {
            n: 4,
            ballots: vec![
                (4, vec![0, 1, 2, 3]),
                (3, vec![1, 2, 3, 0]),
                (3, vec![2, 1, 0, 3]),
            ],
        };
        assert_eq!(irv(&e, 15), (6, true));
    }
}
