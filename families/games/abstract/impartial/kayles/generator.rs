use rand::{Rng, rngs::StdRng};

fn grundies(max: usize) -> Vec<usize> {
    let mut g = vec![0; max + 1];
    for n in 1..=max {
        let mut seen = vec![false; 2 * n + 1];
        for removed in 1..=2.min(n) {
            for left in 0..=n - removed {
                let value = g[left] ^ g[n - removed - left];
                if value < seen.len() {
                    seen[value] = true;
                }
            }
        }
        g[n] = seen.iter().position(|&v| !v).unwrap();
    }
    g
}

fn value(rows: &[usize], g: &[usize]) -> usize {
    rows.iter().fold(0, |v, &n| v ^ g[n])
}

fn winning_moves(rows: &[usize], g: &[usize]) -> Vec<(usize, usize, usize)> {
    let total = value(rows, g);
    let mut moves = Vec::new();
    for (r, &n) in rows.iter().enumerate() {
        for removed in 1..=2.min(n) {
            for left in 0..=n - removed {
                if total ^ g[n] ^ g[left] ^ g[n - removed - left] == 0 {
                    moves.push((r, left, removed));
                }
            }
        }
    }
    moves
}

fn position(rng: &mut StdRng, g: &[usize]) -> Vec<usize> {
    match rng.random_range(0..4) {
        0 => {
            let mut rows = Vec::new();
            for _ in 0..rng.random_range(1..=2) {
                let n = rng.random_range(1..=18);
                rows.extend([n, n]);
            }
            rows
        }
        1 => vec![rng.random_range(1..=24)],
        _ => {
            let mut rows: Vec<_> = (0..rng.random_range(2..=5))
                .map(|_| rng.random_range(1..=16))
                .collect();
            if value(&rows, g) == 0 {
                rows.push(1);
            }
            rows
        }
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let g = grundies(24);
    let rows = position(rng, &g);
    let total = value(&rows, &g);
    let mut text = String::from("Kayles (normal play)");
    for (r, &n) in rows.iter().enumerate() {
        text.push_str(&format!("\nR{}  {}  ({n})", r + 1, "|".repeat(n)));
    }
    text.push_str(&format!(
        "\nNim-value: {total}\n{}-position",
        if total == 0 { "P" } else { "N" }
    ));
    let moves = winning_moves(&rows, &g);
    if moves.is_empty() {
        text.push_str("\nWinning removals: none");
    } else {
        text.push_str(&format!(
            "\nWinning removals: {}",
            moves
                .iter()
                .map(|&(r, left, count)| {
                    if count == 1 {
                        format!("R{}[{}]", r + 1, left + 1)
                    } else {
                        format!("R{}[{}–{}]", r + 1, left + 1, left + 2)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn split_recurrence_matches_independent_bitmask_game_and_all_winning_removals() {
        let g = grundies(24);
        assert_eq!(&g[..9], &[0, 1, 2, 3, 1, 4, 3, 2, 1]);
        // Full positions are solved directly as twelve fixed pin sites. This
        // evaluator does not use splitting, row XOR, or the Kayles recurrence.
        let mut exact = vec![0usize; 1 << 12];
        for mask in 1usize..1 << 12 {
            let mut children = Vec::new();
            for i in 0..12 {
                if mask & (1 << i) == 0 {
                    continue;
                }
                children.push(mask ^ (1 << i));
                if i < 11 && mask & (1 << (i + 1)) != 0 {
                    children.push(mask ^ (3 << i));
                }
            }
            let mut mex = 0;
            while children.iter().any(|&child| exact[child] == mex) {
                mex += 1;
            }
            exact[mask] = mex;
            let mut runs: Vec<Vec<usize>> = Vec::new();
            for i in 0..12 {
                if mask & (1 << i) != 0 {
                    if i == 0 || mask & (1 << (i - 1)) == 0 {
                        runs.push(Vec::new());
                    }
                    runs.last_mut().unwrap().push(i);
                }
            }
            let rows: Vec<_> = runs.iter().map(Vec::len).collect();
            assert_eq!(value(&rows, &g), exact[mask]);
            let reported = winning_moves(&rows, &g);
            let mut winning_children = Vec::new();
            for (r, left, count) in reported {
                let mut child = mask;
                for &site in &runs[r][left..left + count] {
                    child ^= 1 << site;
                }
                assert_eq!(exact[child], 0);
                winning_children.push(child);
            }
            let expected: Vec<_> = children
                .into_iter()
                .filter(|&child| exact[child] == 0)
                .collect();
            winning_children.sort_unstable();
            let mut expected = expected;
            expected.sort_unstable();
            assert_eq!(winning_children, expected);
        }
    }

    #[test]
    fn sampling_contains_paired_and_asymmetric_positions() {
        let g = grundies(24);
        let mut rng = StdRng::seed_from_u64(12760);
        let mut losing = 0;
        let mut splits = 0;
        for _ in 0..1000 {
            let rows = position(&mut rng, &g);
            assert!(rows.iter().all(|&n| (1..=24).contains(&n)));
            assert!((1..=6).contains(&rows.len()));
            losing += usize::from(value(&rows, &g) == 0);
            splits += winning_moves(&rows, &g)
                .iter()
                .filter(|&&(r, left, removed)| left > 0 && left + removed < rows[r])
                .count();
        }
        assert!((180..330).contains(&losing));
        assert!(splits > 300);
    }
}
