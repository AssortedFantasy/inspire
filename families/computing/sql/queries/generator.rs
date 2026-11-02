use rand::{Rng, rngs::StdRng};
use std::{collections::BTreeSet, fmt::Write};

type Cell = Option<i32>;
type Row = Vec<Cell>;

struct Sample {
    setup: String,
    query: String,
    headers: Vec<&'static str>,
    rows: Vec<Row>,
}

fn literal(cell: Cell) -> String {
    cell.map_or_else(|| "NULL".to_owned(), |n| n.to_string())
}

fn table(name: &str, columns: &[&str], rows: &[Row]) -> String {
    let mut sql = format!(
        "CREATE TABLE {name} ({});",
        columns
            .iter()
            .map(|c| format!("{c} INTEGER"))
            .collect::<Vec<_>>()
            .join(", ")
    );
    if !rows.is_empty() {
        write!(
            sql,
            "\nINSERT INTO {name} VALUES\n  {};",
            rows.iter()
                .map(|row| format!(
                    "({})",
                    row.iter()
                        .copied()
                        .map(literal)
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
                .collect::<Vec<_>>()
                .join(",\n  ")
        )
        .unwrap();
    }
    sql
}

fn render(sample: Sample) -> String {
    let mut text = format!(
        "{}\n\n{}\n\nResult\n{}",
        sample.setup,
        sample.query,
        sample.headers.join(" | ")
    );
    if sample.rows.is_empty() {
        text.push_str("\n(no rows)");
    } else {
        for row in sample.rows {
            write!(
                text,
                "\n{}",
                row.into_iter().map(literal).collect::<Vec<_>>().join(" | ")
            )
            .unwrap();
        }
    }
    text
}

fn left_rows(customers: &[i32], orders: &[(i32, i32, Cell)], cut: i32, mode: usize) -> Vec<Row> {
    let mut rows = Vec::new();
    for &customer in customers {
        let matches: Vec<_> = orders
            .iter()
            .filter(|(_, owner, amount)| {
                *owner == customer && (mode != 0 || amount.is_some_and(|a| a >= cut))
            })
            .collect();
        if matches.is_empty() {
            if mode != 1 {
                rows.push(vec![Some(customer), None, None]);
            }
        } else {
            for &&(id, _, amount) in &matches {
                if mode == 0 || amount.is_some_and(|a| a >= cut) {
                    rows.push(vec![Some(customer), Some(id), amount]);
                }
            }
        }
    }
    rows.sort();
    rows
}

fn outer_join(rng: &mut StdRng) -> Sample {
    let count = rng.random_range(3..=6);
    let customers: Vec<i32> = (1..=count).collect();
    let cut = rng.random_range(10..=40);
    let regime = rng.random_range(0..4);
    let mut orders = vec![
        (1, 1, Some(cut - 1)),
        (2, 1, Some(cut)),
        (3, 2, Some(cut - 1)),
    ];
    if regime != 0 {
        orders.push((4, 2, None));
    }
    for id in 5..rng.random_range(6..=11) {
        orders.push((
            id,
            rng.random_range(1..count),
            if rng.random_bool(0.2) {
                None
            } else {
                Some(cut + rng.random_range(-2..=2))
            },
        ));
    }
    if regime == 2 {
        orders.retain(|(_, _, amount)| amount.is_some_and(|a| a < cut));
    } else if regime == 3 {
        orders.retain(|(_, owner, _)| *owner == 1);
    }
    let mode = rng.random_range(0..3);
    let tail = match mode {
        0 => format!(" AND o.amount >= {cut}"),
        1 => format!("\nWHERE o.amount >= {cut}"),
        _ => format!("\nWHERE o.amount >= {cut} OR o.id IS NULL"),
    };
    Sample {
        setup: format!(
            "{}\n{}",
            table(
                "customers",
                &["id"],
                &customers.iter().map(|&c| vec![Some(c)]).collect::<Vec<_>>()
            ),
            table(
                "orders",
                &["id", "customer_id", "amount"],
                &orders
                    .iter()
                    .map(|&(id, owner, amount)| vec![Some(id), Some(owner), amount])
                    .collect::<Vec<_>>()
            )
        ),
        query: format!(
            "SELECT c.id AS customer, o.id AS order_id, o.amount\nFROM customers c\nLEFT JOIN orders o ON o.customer_id = c.id{tail}\nORDER BY c.id, o.id;"
        ),
        headers: vec!["customer", "order_id", "amount"],
        rows: left_rows(&customers, &orders, cut, mode),
    }
}

fn sql_not_in(value: Cell, banned: &[Cell]) -> Option<bool> {
    if banned.is_empty() {
        Some(true)
    } else if value.is_some() && banned.contains(&value) {
        Some(false)
    } else if value.is_none() || banned.contains(&None) {
        None
    } else {
        Some(true)
    }
}

fn anti_rows(probes: &[Cell], banned: &[Cell], mode: usize) -> Vec<Row> {
    let nonnull: Vec<_> = banned.iter().copied().flatten().map(Some).collect();
    probes
        .iter()
        .enumerate()
        .filter_map(|(i, &value)| {
            let keep = match mode {
                0 => sql_not_in(value, banned) == Some(true),
                1 => !banned.iter().any(|&b| value.is_some() && value == b),
                _ => sql_not_in(value, &nonnull) == Some(true),
            };
            keep.then(|| vec![Some(i as i32 + 1), value])
        })
        .collect()
}

fn anti_join(rng: &mut StdRng) -> Sample {
    let base = rng.random_range(-3..=9);
    let mut probes = vec![Some(base), Some(base + 1), Some(base + 2)];
    if rng.random_bool(0.7) {
        probes.push(None);
    }
    if rng.random_bool(0.5) {
        probes.push(Some(base));
    }
    let banned = match rng.random_range(0..5) {
        0 => vec![],
        1 => vec![None],
        2 => vec![Some(base), None, Some(base)],
        3 => vec![Some(base), Some(base + 1)],
        _ => vec![Some(base + 4), Some(base + 4)],
    };
    let mode = rng.random_range(0..3);
    let predicate = match mode {
        0 => "p.val NOT IN (SELECT val FROM banned)",
        1 => "NOT EXISTS (SELECT 1 FROM banned b WHERE b.val = p.val)",
        _ => "p.val NOT IN (SELECT val FROM banned WHERE val IS NOT NULL)",
    };
    Sample {
        setup: format!(
            "{}\n{}",
            table(
                "probes",
                &["id", "val"],
                &probes
                    .iter()
                    .enumerate()
                    .map(|(i, &v)| vec![Some(i as i32 + 1), v])
                    .collect::<Vec<_>>()
            ),
            table(
                "banned",
                &["val"],
                &banned.iter().map(|&v| vec![v]).collect::<Vec<_>>()
            )
        ),
        query: format!("SELECT p.id, p.val\nFROM probes p\nWHERE {predicate}\nORDER BY p.id;"),
        headers: vec!["id", "val"],
        rows: anti_rows(&probes, &banned, mode),
    }
}

fn aggregate(values: &[Cell]) -> Row {
    let nonnull: Vec<i32> = values.iter().copied().flatten().collect();
    let distinct: BTreeSet<i32> = nonnull.iter().copied().collect();
    let sum = (!nonnull.is_empty()).then(|| nonnull.iter().sum());
    vec![
        Some(values.len() as i32),
        Some(nonnull.len() as i32),
        Some(distinct.len() as i32),
        sum,
    ]
}

fn group_rows(data: &[(Cell, Cell)], mode: usize, cut: i32) -> Vec<Row> {
    if mode == 4 {
        return vec![aggregate(&data.iter().map(|&(_, v)| v).collect::<Vec<_>>())];
    }
    let keys: BTreeSet<Cell> = data.iter().map(|&(g, _)| g).collect();
    keys.into_iter()
        .filter_map(|key| {
            let values: Vec<_> = data
                .iter()
                .filter(|&&(g, _)| g == key)
                .map(|&(_, v)| v)
                .collect();
            let mut row = aggregate(&values);
            let keep = match mode {
                0 => true,
                1 => row[3].is_some_and(|sum| sum >= cut),
                2 => row[1] == Some(0),
                _ => row[0] > row[2],
            };
            row.insert(0, key);
            keep.then_some(row)
        })
        .collect()
}

fn grouping(rng: &mut StdRng) -> Sample {
    let a = rng.random_range(1..=9);
    let mut data = vec![
        (Some(1), Some(a)),
        (Some(1), Some(-a)),
        (Some(1), Some(a)),
        (Some(2), None),
        (Some(2), None),
        (None, Some(0)),
        (None, None),
    ];
    match rng.random_range(0..4) {
        0 => data.clear(),
        1 => data.retain(|&(g, _)| g.is_some()),
        2 => data.push((Some(3), Some(a + 1))),
        _ => {}
    }
    let mode = rng.random_range(0..5);
    let cut = rng.random_range(0..=a + 1);
    let selected = if mode == 4 { "" } else { "bucket, " };
    let grouping = if mode == 4 {
        "".to_owned()
    } else {
        let having = match mode {
            1 => format!("\nHAVING SUM(val) >= {cut}"),
            2 => "\nHAVING COUNT(val) = 0".to_owned(),
            3 => "\nHAVING COUNT(*) > COUNT(DISTINCT val)".to_owned(),
            _ => String::new(),
        };
        format!("\nGROUP BY bucket{having}\nORDER BY bucket")
    };
    let mut headers = vec!["n", "n_values", "n_distinct", "total"];
    if mode != 4 {
        headers.insert(0, "bucket");
    }
    Sample {
        setup: table(
            "observations",
            &["bucket", "val"],
            &data.iter().map(|&(g, v)| vec![g, v]).collect::<Vec<_>>(),
        ),
        query: format!(
            "SELECT {selected}COUNT(*) AS n, COUNT(val) AS n_values,\n       COUNT(DISTINCT val) AS n_distinct, SUM(val) AS total\nFROM observations{grouping};"
        ),
        headers,
        rows: group_rows(&data, mode, cut),
    }
}

// (id, partition, ordering key, value). IDs break ties only for ROWS/LAG.
type Event = (i32, i32, i32, i32);

fn window_rows(events: &[Event], mode: usize) -> Vec<Row> {
    let mut ordered = events.to_vec();
    ordered.sort_by_key(|&(id, partition, key, _)| (partition, key, id));
    ordered
        .iter()
        .enumerate()
        .map(|(position, &(id, partition, key, value))| {
            let prior: Vec<_> = ordered[..position]
                .iter()
                .filter(|&&(_, p, _, _)| p == partition)
                .collect();
            let mut row = vec![Some(id), Some(partition), Some(key), Some(value)];
            match mode {
                0 => {
                    let rows_sum = value + prior.iter().map(|&&(_, _, _, v)| v).sum::<i32>();
                    let range_sum = ordered
                        .iter()
                        .filter(|&&(_, p, k, _)| p == partition && k <= key)
                        .map(|&(_, _, _, v)| v)
                        .sum();
                    row.extend([Some(rows_sum), Some(range_sum)]);
                }
                1 => {
                    let lower = ordered
                        .iter()
                        .filter(|&&(_, p, k, _)| p == partition && k < key)
                        .count() as i32;
                    let lower_keys: BTreeSet<_> = prior
                        .iter()
                        .filter(|&&&(_, _, k, _)| k < key)
                        .map(|&&(_, _, k, _)| k)
                        .collect();
                    row.extend([Some(lower + 1), Some(lower_keys.len() as i32 + 1)]);
                }
                _ => {
                    let lag = prior.last().map(|&&(_, _, _, v)| v);
                    let moving_sum = value + lag.unwrap_or(0);
                    row.extend([lag, Some(moving_sum)]);
                }
            }
            row
        })
        .collect()
}

fn windows(rng: &mut StdRng) -> Sample {
    let n = rng.random_range(5..=9);
    let mut events = Vec::new();
    for id in 1..=n {
        events.push((
            id,
            if id <= 3 { 1 } else { rng.random_range(1..=2) },
            if id <= 2 { 2 } else { rng.random_range(1..=4) },
            rng.random_range(-4..=9),
        ));
    }
    // Keep a nonzero peer so ROWS and RANGE cannot accidentally agree everywhere.
    events[1].3 = rng.random_range(1..=9);
    let mode = rng.random_range(0..3);
    let (expressions, extras) = match mode {
        0 => (
            "SUM(val) OVER (PARTITION BY lane ORDER BY tick, id\n         ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) AS rows_total,\n       SUM(val) OVER (PARTITION BY lane ORDER BY tick\n         RANGE BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW) AS peer_total",
            ["rows_total", "peer_total"],
        ),
        1 => (
            "RANK() OVER (PARTITION BY lane ORDER BY tick) AS place,\n       DENSE_RANK() OVER (PARTITION BY lane ORDER BY tick) AS dense_place",
            ["place", "dense_place"],
        ),
        _ => (
            "LAG(val) OVER (PARTITION BY lane ORDER BY tick, id) AS previous,\n       SUM(val) OVER (PARTITION BY lane ORDER BY tick, id\n         ROWS BETWEEN 1 PRECEDING AND CURRENT ROW) AS pair_total",
            ["previous", "pair_total"],
        ),
    };
    let mut headers = vec!["id", "lane", "tick", "val"];
    headers.extend(extras);
    Sample {
        setup: table(
            "events",
            &["id", "lane", "tick", "val"],
            &events
                .iter()
                .map(|&(id, p, k, v)| vec![Some(id), Some(p), Some(k), Some(v)])
                .collect::<Vec<_>>(),
        ),
        query: format!(
            "SELECT id, lane, tick, val,\n       {expressions}\nFROM events\nORDER BY lane, tick, id;"
        ),
        headers,
        rows: window_rows(&events, mode),
    }
}

fn set_rows(a: &[Cell], b: &[Cell], mode: usize) -> Vec<Row> {
    let left: BTreeSet<_> = a.iter().copied().collect();
    let right: BTreeSet<_> = b.iter().copied().collect();
    let mut values: Vec<_> = match mode {
        0 => a.iter().chain(b).copied().collect(),
        1 => left.union(&right).copied().collect(),
        2 => left.intersection(&right).copied().collect(),
        _ => left.difference(&right).copied().collect(),
    };
    values.sort();
    values.into_iter().map(|v| vec![v]).collect()
}

fn sets(rng: &mut StdRng) -> Sample {
    let base = rng.random_range(-5..=5);
    let mut a = vec![Some(base), Some(base), Some(base + 1), None];
    let mut b = vec![Some(base), Some(base + 2), None, None];
    match rng.random_range(0..5) {
        0 => a.clear(),
        1 => b.clear(),
        2 => {
            a.retain(Option::is_some);
            b.retain(Option::is_some);
        }
        3 => b = a.clone(),
        _ => b.push(Some(base + 1)),
    }
    let mode = rng.random_range(0..4);
    let operator = ["UNION ALL", "UNION", "INTERSECT", "EXCEPT"][mode];
    Sample {
        setup: format!(
            "{}\n{}",
            table(
                "a",
                &["val"],
                &a.iter().map(|&v| vec![v]).collect::<Vec<_>>()
            ),
            table(
                "b",
                &["val"],
                &b.iter().map(|&v| vec![v]).collect::<Vec<_>>()
            )
        ),
        query: format!("SELECT val FROM a\n{operator}\nSELECT val FROM b\nORDER BY val;"),
        headers: vec!["val"],
        rows: set_rows(&a, &b, mode),
    }
}

fn reachable(edges: &[(i32, i32)], start: i32) -> BTreeSet<i32> {
    let mut seen = BTreeSet::from([start]);
    let mut frontier = vec![start];
    while let Some(node) = frontier.pop() {
        for &(a, b) in edges {
            if a == node && seen.insert(b) {
                frontier.push(b);
            }
        }
    }
    seen
}

fn walk_rows(edges: &[(i32, i32)], start: i32, depth: i32) -> Vec<Row> {
    let mut frontier = vec![start];
    let mut rows = vec![vec![Some(0), Some(start), Some(1)]];
    for d in 1..=depth {
        frontier = frontier
            .iter()
            .flat_map(|&node| {
                edges
                    .iter()
                    .filter(move |&&(a, _)| a == node)
                    .map(|&(_, b)| b)
            })
            .collect();
        let nodes: BTreeSet<_> = frontier.iter().copied().collect();
        for node in nodes {
            rows.push(vec![
                Some(d),
                Some(node),
                Some(frontier.iter().filter(|&&v| v == node).count() as i32),
            ]);
        }
    }
    rows
}

fn recursion(rng: &mut StdRng) -> Sample {
    let shift = rng.random_range(0..=20);
    let mut edges = match rng.random_range(0..5) {
        0 => vec![(1, 2), (2, 3), (3, 1), (3, 4)],
        1 => vec![(1, 2), (1, 3), (2, 4), (3, 4), (4, 5)],
        2 => vec![(1, 1), (1, 2), (1, 2), (2, 3)],
        3 => vec![(1, 2), (2, 3), (4, 5), (5, 4)],
        _ => vec![(2, 3), (3, 2)],
    };
    for (a, b) in &mut edges {
        *a += shift;
        *b += shift;
    }
    if rng.random_bool(0.35) {
        edges.push((shift + 3, shift + 1));
    }
    let start = shift + 1;
    let mode = rng.random_range(0..2);
    let (query, headers, rows) = if mode == 0 {
        (
            format!(
                "WITH RECURSIVE reached(node) AS (\n  VALUES ({start})\n  UNION\n  SELECT e.dst FROM edges e JOIN reached r ON e.src = r.node\n)\nSELECT node FROM reached ORDER BY node;"
            ),
            vec!["node"],
            reachable(&edges, start)
                .into_iter()
                .map(|v| vec![Some(v)])
                .collect(),
        )
    } else {
        let depth = rng.random_range(2..=5);
        (
            format!(
                "WITH RECURSIVE walks(node, depth) AS (\n  VALUES ({start}, 0)\n  UNION ALL\n  SELECT e.dst, w.depth + 1\n  FROM edges e JOIN walks w ON e.src = w.node\n  WHERE w.depth < {depth}\n)\nSELECT depth, node, COUNT(*) AS walks\nFROM walks\nGROUP BY depth, node\nORDER BY depth, node;"
            ),
            vec!["depth", "node", "walks"],
            walk_rows(&edges, start, depth),
        )
    };
    Sample {
        setup: table(
            "edges",
            &["src", "dst"],
            &edges
                .iter()
                .map(|&(a, b)| vec![Some(a), Some(b)])
                .collect::<Vec<_>>(),
        ),
        query,
        headers,
        rows,
    }
}

fn qualified(candidates: &[i32], required: &[i32], skills: &[(i32, i32)]) -> Vec<Row> {
    candidates
        .iter()
        .filter(|&&candidate| {
            required
                .iter()
                .all(|&skill| skills.contains(&(candidate, skill)))
        })
        .map(|&candidate| vec![Some(candidate)])
        .collect()
}

fn division(rng: &mut StdRng) -> Sample {
    let count = rng.random_range(3..=5);
    let candidates: Vec<_> = (1..=count).collect();
    let skill_count = rng.random_range(0..=3);
    let required: Vec<_> = (1..=skill_count).collect();
    let mut skills = Vec::new();
    for &candidate in &candidates {
        let mut duplicated = false;
        for skill in 1..=skill_count + 1 {
            if candidate == 1 || (candidate != count && rng.random_bool(0.65)) {
                skills.push((candidate, skill));
                if !duplicated && rng.random_bool(0.25) {
                    skills.push((candidate, skill));
                    duplicated = true;
                }
            }
        }
    }
    Sample {
        setup: format!("{}\n{}\n{}", table("candidates", &["id"], &candidates.iter().map(|&c| vec![Some(c)]).collect::<Vec<_>>()), table("required", &["skill"], &required.iter().map(|&s| vec![Some(s)]).collect::<Vec<_>>()), table("skills", &["candidate_id", "skill"], &skills.iter().map(|&(c, s)| vec![Some(c), Some(s)]).collect::<Vec<_>>())),
        query: "SELECT c.id\nFROM candidates c\nWHERE NOT EXISTS (\n  SELECT 1 FROM required r\n  WHERE NOT EXISTS (\n    SELECT 1 FROM skills s\n    WHERE s.candidate_id = c.id AND s.skill = r.skill\n  )\n)\nORDER BY c.id;".to_owned(),
        headers: vec!["id"],
        rows: qualified(&candidates, &required, &skills),
    }
}

fn sample(rng: &mut StdRng, kind: usize) -> Sample {
    match kind {
        0 => outer_join(rng),
        1 => anti_join(rng),
        2 => grouping(rng),
        3 => windows(rng),
        4 => sets(rng),
        5 => recursion(rng),
        _ => division(rng),
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let kind = rng.random_range(0..7);
    render(sample(rng, kind))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn outer_filter_placement_retains_different_customers() {
        let orders = [
            (1, 1, Some(9)),
            (2, 1, Some(10)),
            (3, 2, Some(9)),
            (4, 2, None),
        ];
        let high = vec![Some(1), Some(2), Some(10)];
        let absent = vec![Some(3), None, None];
        assert_eq!(
            left_rows(&[1, 2, 3], &orders, 10, 0),
            vec![high.clone(), vec![Some(2), None, None], absent.clone()]
        );
        assert_eq!(left_rows(&[1, 2, 3], &orders, 10, 1), vec![high.clone()]);
        assert_eq!(left_rows(&[1, 2, 3], &orders, 10, 2), vec![high, absent]);
    }

    #[test]
    fn not_in_truth_matrix_includes_empty_and_null_rhs() {
        let values = [None, Some(1), Some(2)];
        let rhs = [
            vec![],
            vec![None],
            vec![Some(1)],
            vec![Some(1), None],
            vec![Some(1), Some(2)],
        ];
        let expected = [
            [Some(true), Some(true), Some(true)],
            [None, None, None],
            [None, Some(false), Some(true)],
            [None, Some(false), None],
            [None, Some(false), Some(false)],
        ];
        for (banned, outcomes) in rhs.iter().zip(expected) {
            for (&value, result) in values.iter().zip(outcomes) {
                assert_eq!(sql_not_in(value, banned), result);
            }
        }
        assert_eq!(
            anti_rows(&values, &[Some(1), None], 1),
            vec![vec![Some(1), None], vec![Some(3), Some(2)]]
        );
        assert_eq!(anti_rows(&values, &[Some(1), None], 0), Vec::<Row>::new());
        assert_eq!(
            anti_rows(&values, &[Some(1), None], 2),
            vec![vec![Some(3), Some(2)]]
        );
    }

    #[test]
    fn aggregate_distinguishes_empty_all_null_zero_and_duplicates() {
        assert_eq!(aggregate(&[]), vec![Some(0), Some(0), Some(0), None]);
        assert_eq!(
            aggregate(&[None, None]),
            vec![Some(2), Some(0), Some(0), None]
        );
        assert_eq!(
            aggregate(&[Some(4), Some(-4), Some(4), None]),
            vec![Some(4), Some(3), Some(2), Some(4)]
        );
        assert_eq!(
            aggregate(&[Some(4), Some(-4)]),
            vec![Some(2), Some(2), Some(2), Some(0)]
        );
        assert!(group_rows(&[], 0, 0).is_empty());
        assert_eq!(group_rows(&[], 4, 0).len(), 1);
        assert_eq!(
            group_rows(&[(None, None), (None, None)], 0, 0),
            vec![vec![None, Some(2), Some(0), Some(0), None]]
        );
        assert!(group_rows(&[(Some(1), None)], 1, 0).is_empty());
    }

    #[test]
    fn peers_expand_range_and_rank_skips_positions() {
        let events = [(1, 1, 2, 3), (2, 1, 2, 5), (3, 1, 4, -1), (4, 2, 1, 7)];
        let sums = window_rows(&events, 0);
        assert_eq!(&sums[0][4..], &[Some(3), Some(8)]);
        assert_eq!(&sums[1][4..], &[Some(8), Some(8)]);
        assert_eq!(&sums[2][4..], &[Some(7), Some(7)]);
        let ranks = window_rows(&events, 1);
        assert_eq!(&ranks[2][4..], &[Some(3), Some(2)]);
        let lag = window_rows(&events, 2);
        assert_eq!(&lag[2][4..], &[Some(5), Some(4)]);
        assert_eq!(&lag[3][4..], &[None, Some(7)]);
    }

    #[test]
    fn set_operators_treat_null_as_one_distinct_value() {
        let a = [None, None, Some(1), Some(1), Some(2)];
        let b = [None, Some(1), Some(3)];
        assert_eq!(set_rows(&a, &b, 0).len(), 8);
        assert_eq!(
            set_rows(&a, &b, 1),
            vec![vec![None], vec![Some(1)], vec![Some(2)], vec![Some(3)]]
        );
        assert_eq!(set_rows(&a, &b, 2), vec![vec![None], vec![Some(1)]]);
        assert_eq!(set_rows(&a, &b, 3), vec![vec![Some(2)]]);
        assert_eq!(set_rows(&a, &[], 3), set_rows(&a, &[], 1));
    }

    #[test]
    fn recursive_set_closure_matches_floyd_warshall_on_every_three_node_graph() {
        for mask in 0..512 {
            let mut edges = Vec::new();
            let mut closure = [[false; 3]; 3];
            for (i, row) in closure.iter_mut().enumerate() {
                for (j, cell) in row.iter_mut().enumerate() {
                    if mask & (1 << (i * 3 + j)) != 0 {
                        edges.push((i as i32, j as i32));
                        *cell = true;
                    }
                }
                row[i] = true;
            }
            for k in 0..3 {
                for i in 0..3 {
                    for j in 0..3 {
                        closure[i][j] |= closure[i][k] && closure[k][j];
                    }
                }
            }
            for (start, expected) in closure.iter().enumerate() {
                let actual: BTreeSet<_> = expected
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &yes)| yes.then_some(i as i32))
                    .collect();
                assert_eq!(reachable(&edges, start as i32), actual);
            }
        }
    }

    #[test]
    fn recursive_bags_keep_parallel_edges_and_converging_paths() {
        let edges = [(1, 2), (1, 2), (1, 3), (2, 4), (3, 4), (4, 1)];
        assert_eq!(
            walk_rows(&edges, 1, 3),
            vec![
                vec![Some(0), Some(1), Some(1)],
                vec![Some(1), Some(2), Some(2)],
                vec![Some(1), Some(3), Some(1)],
                vec![Some(2), Some(4), Some(3)],
                vec![Some(3), Some(1), Some(3)]
            ]
        );
        assert_eq!(reachable(&edges, 1), BTreeSet::from([1, 2, 3, 4]));
    }

    #[test]
    fn relational_division_is_vacuous_for_empty_requirements() {
        let candidates = [1, 2, 3];
        let skills = [(1, 1), (1, 2), (1, 2), (2, 1), (3, 3)];
        assert_eq!(
            qualified(&candidates, &[1, 2], &skills),
            vec![vec![Some(1)]]
        );
        assert_eq!(
            qualified(&candidates, &[], &[]),
            vec![vec![Some(1)], vec![Some(2)], vec![Some(3)]]
        );
        assert_eq!(
            qualified(&candidates, &[1, 1], &skills),
            vec![vec![Some(1)], vec![Some(2)]]
        );
    }

    #[test]
    fn generated_tables_have_consistent_width_and_bounded_results() {
        let mut rng = StdRng::seed_from_u64(1847);
        for kind in 0..7 {
            for _ in 0..100 {
                let generated = sample(&mut rng, kind);
                assert!(generated.rows.len() <= 40);
                assert!(
                    generated
                        .rows
                        .iter()
                        .all(|r| r.len() == generated.headers.len())
                );
                assert!(!render(generated).ends_with('\n'));
            }
        }
    }
}
