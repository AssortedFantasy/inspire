use rand::{Rng, rngs::StdRng, seq::SliceRandom};

struct Instance {
    values: Vec<Vec<u32>>,
}

struct Allocation {
    owners: Vec<usize>,
    bundle_values: Vec<Vec<u32>>,
    total: u32,
    ef1_failures: usize,
    max_envy: u32,
    sum_envy: u32,
}

struct Solution {
    unrestricted: Allocation,
    ef1: Allocation,
    ef1_count: usize,
    envy_free_count: usize,
    allocation_count: usize,
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..7);
    let instance = sample(rng, regime);
    render(&instance)
}

fn render(instance: &Instance) -> String {
    let solution = solve(instance);
    let n = instance.values.len();
    let m = instance.values[0].len();
    let mut lines = vec!["Item values (additive, common point scale)".to_owned()];
    lines.push(format!(
        "Item  {}",
        (0..n)
            .map(|i| format!("{:>3}", agent(i)))
            .collect::<Vec<_>>()
            .join(" ")
    ));
    for g in 0..m {
        lines.push(format!(
            "{}     {}",
            item(g),
            instance
                .values
                .iter()
                .map(|row| format!("{:>3}", row[g]))
                .collect::<Vec<_>>()
                .join(" ")
        ));
    }
    let same = solution.unrestricted.owners == solution.ef1.owners;
    let heading = if same {
        "Maximum total value, also best under EF1"
    } else {
        "Maximum total value"
    };
    append_allocation(&mut lines, heading, instance, &solution.unrestricted);
    if !same {
        append_allocation(
            &mut lines,
            "Best total value under EF1",
            instance,
            &solution.ef1,
        );
        let cost = solution.unrestricted.total - solution.ef1.total;
        lines.push(format!(
            "EF1 costs {cost} {} of total value",
            if cost == 1 { "point" } else { "points" }
        ));
    }
    lines.push(format!(
        "\n{} of {} allocations satisfy EF1",
        solution.ef1_count, solution.allocation_count
    ));
    lines.push(if solution.envy_free_count == 0 {
        "No envy-free allocation exists".to_owned()
    } else {
        format!(
            "{} {} envy-free",
            solution.envy_free_count,
            if solution.envy_free_count == 1 {
                "allocation is"
            } else {
                "allocations are"
            }
        )
    });
    lines.join("\n")
}

fn sample(rng: &mut StdRng, regime: usize) -> Instance {
    let n = rng.random_range(2..=3);
    let values = match regime {
        // Two strongly preferred goods per agent.
        0 => (0..n)
            .map(|i| {
                (0..2 * n)
                    .map(|g| {
                        if g / 2 == i {
                            rng.random_range(7..=15)
                        } else {
                            rng.random_range(0..=2)
                        }
                    })
                    .collect()
            })
            .collect(),
        // One agent strictly leads every item: utilitarian monopoly.
        1 => {
            let m = rng.random_range(n + 1..=7);
            let mut values = (0..n)
                .map(|_| (0..m).map(|_| rng.random_range(2..=10)).collect())
                .collect::<Vec<Vec<u32>>>();
            for g in 0..m {
                values[0][g] = values.iter().skip(1).map(|row| row[g]).max().unwrap()
                    + rng.random_range(1..=4);
            }
            values
        }
        // The remainder of an equal division controls exact envy-freeness.
        2 => vec![vec![1; rng.random_range(3..=7)]; n],
        // A common trophy outweighs every accessory combined.
        3 => {
            let m = rng.random_range(3..=7);
            (0..n)
                .map(|_| {
                    let mut row = (0..m).map(|_| rng.random_range(0..=2)).collect::<Vec<_>>();
                    row[0] = rng.random_range(15..=28);
                    row
                })
                .collect()
        }
        // Each edge of a three-agent cycle has two disputed goods.
        4 => {
            let mut values = vec![vec![0; 6]; 3];
            for g in 0..6 {
                let first = g / 2;
                let second = (first + 1) % 3;
                for (i, row) in values.iter_mut().enumerate() {
                    row[g] = if i == first || i == second {
                        rng.random_range(8..=15)
                    } else {
                        rng.random_range(0..=2)
                    };
                }
            }
            values
        }
        5 => {
            let m = rng.random_range(3..=7);
            (0..n)
                .map(|_| {
                    let mut row = (0..m).map(|_| rng.random_range(0..=16)).collect::<Vec<_>>();
                    if row.iter().all(|&value| value == 0) {
                        row[0] = 1;
                    }
                    row
                })
                .collect()
        }
        // A private good lies one point below, on or above the EF1 boundary.
        _ => {
            let offset = rng.random_range(-1..=1);
            boundary(rng, offset)
        }
    };
    permute(rng, values)
}

fn boundary(rng: &mut StdRng, offset: i32) -> Vec<Vec<u32>> {
    let low = rng.random_range(4..=12);
    let high = low + rng.random_range(0..=6);
    vec![
        vec![
            low + rng.random_range(1..=4),
            high + rng.random_range(1..=4),
            0,
        ],
        vec![low, high, (low as i32 + offset) as u32],
    ]
}

fn permute(rng: &mut StdRng, values: Vec<Vec<u32>>) -> Instance {
    let mut agents = (0..values.len()).collect::<Vec<_>>();
    let mut goods = (0..values[0].len()).collect::<Vec<_>>();
    agents.shuffle(rng);
    goods.shuffle(rng);
    Instance {
        values: agents
            .iter()
            .map(|&i| goods.iter().map(|&g| values[i][g]).collect())
            .collect(),
    }
}

fn evaluate(instance: &Instance, owners: Vec<usize>) -> Allocation {
    let n = instance.values.len();
    let mut bundle_values = vec![vec![0_u32; n]; n];
    let mut largest = vec![vec![0_u32; n]; n];
    for (g, &owner) in owners.iter().enumerate() {
        for (i, values) in instance.values.iter().enumerate() {
            bundle_values[i][owner] += values[g];
            largest[i][owner] = largest[i][owner].max(values[g]);
        }
    }
    let mut total = 0;
    let mut ef1_failures = 0;
    let mut max_envy = 0;
    let mut sum_envy = 0;
    for (i, row) in bundle_values.iter().enumerate() {
        let own = row[i];
        total += own;
        for (j, &value) in row.iter().enumerate() {
            let envy = value.saturating_sub(own);
            max_envy = max_envy.max(envy);
            sum_envy += envy;
            if value - largest[i][j] > own {
                ef1_failures += 1;
            }
        }
    }
    Allocation {
        owners,
        bundle_values,
        total,
        ef1_failures,
        max_envy,
        sum_envy,
    }
}

fn better(candidate: &Allocation, incumbent: &Allocation) -> bool {
    candidate.total > incumbent.total
        || (candidate.total == incumbent.total
            && (
                candidate.ef1_failures,
                candidate.max_envy,
                candidate.sum_envy,
                &candidate.owners,
            ) < (
                incumbent.ef1_failures,
                incumbent.max_envy,
                incumbent.sum_envy,
                &incumbent.owners,
            ))
}

fn solve(instance: &Instance) -> Solution {
    let n = instance.values.len();
    let m = instance.values[0].len();
    let allocation_count = n.pow(m as u32);
    let mut unrestricted = None;
    let mut ef1 = None;
    let mut ef1_count = 0;
    let mut envy_free_count = 0;
    for mut code in 0..allocation_count {
        let owners = (0..m)
            .map(|_| {
                let owner = code % n;
                code /= n;
                owner
            })
            .collect();
        let allocation = evaluate(instance, owners);
        if allocation.ef1_failures == 0 {
            ef1_count += 1;
            if allocation.max_envy == 0 {
                envy_free_count += 1;
            }
            if ef1
                .as_ref()
                .is_none_or(|incumbent| better(&allocation, incumbent))
            {
                ef1 = Some(evaluate(instance, allocation.owners.clone()));
            }
        }
        if unrestricted
            .as_ref()
            .is_none_or(|incumbent| better(&allocation, incumbent))
        {
            unrestricted = Some(allocation);
        }
    }
    Solution {
        unrestricted: unrestricted.unwrap(),
        ef1: ef1.unwrap(),
        ef1_count,
        envy_free_count,
        allocation_count,
    }
}

fn append_allocation(
    lines: &mut Vec<String>,
    heading: &str,
    instance: &Instance,
    allocation: &Allocation,
) {
    lines.push(format!("\n{heading} ({})", allocation.total));
    for (i, row) in allocation.bundle_values.iter().enumerate() {
        let goods = allocation
            .owners
            .iter()
            .enumerate()
            .filter(|&(_, &owner)| owner == i)
            .map(|(g, _)| item(g).to_string())
            .collect::<Vec<_>>()
            .join(", ");
        lines.push(format!("{} gets {{{goods}}}, value {}", agent(i), row[i]));
    }
    if allocation.max_envy == 0 {
        lines.push("Envy-free".to_owned());
        return;
    }
    lines.push(if allocation.ef1_failures == 0 {
        "EF1 holds".to_owned()
    } else {
        "EF1 fails".to_owned()
    });
    for (i, row) in allocation.bundle_values.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value <= row[i] {
                continue;
            }
            let g = allocation
                .owners
                .iter()
                .enumerate()
                .filter(|&(_, &owner)| owner == j)
                .max_by_key(|&(g, _)| instance.values[i][g])
                .map(|(g, _)| g)
                .unwrap();
            let residual = value - instance.values[i][g];
            lines.push(format!(
                "{} envies {} by {}. Without {}: {} {} own {}.",
                agent(i),
                agent(j),
                value - row[i],
                item(g),
                residual,
                if residual <= row[i] { "≤" } else { ">" },
                row[i]
            ));
        }
    }
}

fn agent(i: usize) -> char {
    (b'A' + i as u8) as char
}

fn item(g: usize) -> char {
    (b'a' + g as u8) as char
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    // Independent predicate: explicitly try deleting every good, rather than
    // using the production implementation's largest-good shortcut.
    fn direct_check(values: &[Vec<u32>], owners: &[usize]) -> (u32, bool, bool, (usize, u32, u32)) {
        let mut total = 0;
        let mut envy_free = true;
        let mut ef1_failures = 0;
        let mut max_envy = 0;
        let mut sum_envy = 0;
        for (i, row) in values.iter().enumerate() {
            let own = row
                .iter()
                .zip(owners)
                .filter(|&(_, &owner)| owner == i)
                .map(|(&value, _)| value)
                .sum::<u32>();
            total += own;
            for j in 0..values.len() {
                let bundle = owners
                    .iter()
                    .enumerate()
                    .filter(|&(_, &owner)| owner == j)
                    .map(|(g, _)| g)
                    .collect::<Vec<_>>();
                let other = bundle.iter().map(|&g| row[g]).sum::<u32>();
                if own < other {
                    envy_free = false;
                    max_envy = max_envy.max(other - own);
                    sum_envy += other - own;
                    if !bundle.iter().any(|&removed| {
                        bundle
                            .iter()
                            .filter(|&&g| g != removed)
                            .map(|&g| row[g])
                            .sum::<u32>()
                            <= own
                    }) {
                        ef1_failures += 1;
                    }
                }
            }
        }
        (
            total,
            ef1_failures == 0,
            envy_free,
            (ef1_failures, max_envy, sum_envy),
        )
    }

    fn enumerate(n: usize, m: usize, owners: &mut Vec<usize>, output: &mut Vec<Vec<usize>>) {
        if owners.len() == m {
            output.push(owners.clone());
        } else {
            for owner in 0..n {
                owners.push(owner);
                enumerate(n, m, owners, output);
                owners.pop();
            }
        }
    }

    fn check_solution(instance: &Instance) -> Solution {
        let solution = solve(instance);
        let mut assignments = Vec::new();
        enumerate(
            instance.values.len(),
            instance.values[0].len(),
            &mut Vec::new(),
            &mut assignments,
        );
        let checked = assignments
            .iter()
            .map(|owners| direct_check(&instance.values, owners))
            .collect::<Vec<_>>();
        assert_eq!(solution.allocation_count, assignments.len());
        assert_eq!(
            solution.unrestricted.total,
            checked.iter().map(|row| row.0).max().unwrap()
        );
        assert_eq!(
            solution.ef1.total,
            checked
                .iter()
                .filter(|row| row.1)
                .map(|row| row.0)
                .max()
                .unwrap()
        );
        assert_eq!(
            solution.ef1_count,
            checked.iter().filter(|row| row.1).count()
        );
        assert_eq!(
            solution.envy_free_count,
            checked.iter().filter(|row| row.2).count()
        );
        let best = assignments
            .iter()
            .zip(&checked)
            .min_by(|(left_owners, left), (right_owners, right)| {
                right
                    .0
                    .cmp(&left.0)
                    .then_with(|| left.3.cmp(&right.3))
                    .then_with(|| left_owners.cmp(right_owners))
            })
            .unwrap();
        assert_eq!(&solution.unrestricted.owners, best.0);
        let best_fair = assignments
            .iter()
            .zip(&checked)
            .filter(|(_, checked)| checked.1)
            .min_by(|(left_owners, left), (right_owners, right)| {
                right
                    .0
                    .cmp(&left.0)
                    .then_with(|| left.3.cmp(&right.3))
                    .then_with(|| left_owners.cmp(right_owners))
            })
            .unwrap();
        assert_eq!(&solution.ef1.owners, best_fair.0);
        assert!(direct_check(&instance.values, &solution.ef1.owners).1);
        for owners in &assignments {
            let evaluated = evaluate(instance, owners.clone());
            let direct = direct_check(&instance.values, owners);
            assert_eq!(evaluated.total, direct.0);
            assert_eq!(evaluated.ef1_failures == 0, direct.1);
            assert_eq!(evaluated.max_envy == 0, direct.2);
        }
        if solution.unrestricted.total == solution.ef1.total {
            assert_eq!(solution.unrestricted.owners, solution.ef1.owners);
        }
        solution
    }

    #[test]
    fn exhausts_small_valuations_including_zeroes_and_ties() {
        for mut code in 0..3_usize.pow(6) {
            let mut values = vec![vec![0; 3]; 2];
            for row in &mut values {
                for value in row {
                    *value = (code % 3) as u32;
                    code /= 3;
                }
            }
            check_solution(&Instance { values });
        }
    }

    #[test]
    fn sampled_regimes_have_their_promised_structure() {
        let mut rng = StdRng::seed_from_u64(0xefa1);
        for regime in 0..7 {
            for _ in 0..12 {
                let instance = sample(&mut rng, regime);
                let solution = check_solution(&instance);
                let itemwise_maximum = (0..instance.values[0].len())
                    .map(|g| instance.values.iter().map(|row| row[g]).max().unwrap())
                    .sum::<u32>();
                assert_eq!(solution.unrestricted.total, itemwise_maximum);
                match regime {
                    0 => assert_eq!(solution.unrestricted.max_envy, 0),
                    1 => {
                        assert!(solution.unrestricted.ef1_failures > 0);
                        assert!(solution.unrestricted.total > solution.ef1.total);
                    }
                    2 => {
                        let n = instance.values.len();
                        let m = instance.values[0].len();
                        assert_eq!(solution.envy_free_count > 0, m.is_multiple_of(n));
                        assert_eq!(solution.unrestricted.total, solution.ef1.total);
                        assert_eq!(solution.ef1.max_envy, u32::from(!m.is_multiple_of(n)));
                    }
                    3 => {
                        assert_eq!(solution.envy_free_count, 0);
                        assert!(solution.ef1.max_envy > 0);
                    }
                    4 => {
                        for g in 0..6 {
                            assert_eq!(instance.values.iter().filter(|row| row[g] >= 8).count(), 2);
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    #[test]
    fn private_good_crosses_the_exact_ef1_boundary() {
        let mut rng = StdRng::seed_from_u64(0xb0a1);
        for offset in -1..=1 {
            for _ in 0..40 {
                let values = boundary(&mut rng, offset);
                let expected_loss = (values[0][0] - values[1][0]).min(values[0][1] - values[1][1]);
                let instance = permute(&mut rng, values);
                let solution = check_solution(&instance);
                assert_eq!(solution.unrestricted.ef1_failures == 0, offset >= 0);
                assert_eq!(
                    solution.unrestricted.total - solution.ef1.total,
                    if offset < 0 { expected_loss } else { 0 }
                );
            }
        }
    }
}
