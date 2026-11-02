use rand::{Rng, rngs::StdRng};

struct Area {
    name: &'static str,
    columns: (usize, usize),
    rows: (usize, usize),
}

struct Grid {
    name: &'static str,
    page: (usize, usize),
    margins: [usize; 4], // left, right, top, bottom
    columns: Vec<usize>,
    rows: Vec<usize>,
    gaps: (usize, usize),
    areas: Vec<Area>,
}

fn divide(total: usize, weights: &[usize]) -> Vec<usize> {
    let sum: usize = weights.iter().sum();
    let mut tracks: Vec<_> = weights.iter().map(|w| total * w / sum).collect();
    let mut order: Vec<_> = (0..weights.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(total * weights[i] % sum));
    let remainder = total - tracks.iter().sum::<usize>();
    for &i in order.iter().take(remainder) {
        tracks[i] += 1;
    }
    tracks
}

fn area(name: &'static str, columns: (usize, usize), rows: (usize, usize)) -> Area {
    Area {
        name,
        columns,
        rows,
    }
}

fn grid(rng: &mut StdRng) -> Grid {
    let regime = rng.random_range(0..4);
    if regime == 0 {
        let unit = rng.random_range(6..=12);
        let verso = rng.random_bool(0.5);
        return Grid {
            name: if verso {
                "Manuscript verso"
            } else {
                "Manuscript recto"
            },
            page: (24 * unit, 36 * unit),
            margins: if verso {
                [4 * unit, 2 * unit, 3 * unit, 6 * unit]
            } else {
                [2 * unit, 4 * unit, 3 * unit, 6 * unit]
            },
            columns: vec![18 * unit],
            rows: vec![27 * unit],
            gaps: (0, 0),
            areas: vec![area("Text", (1, 1), (1, 1))],
        };
    }
    let page = [(210, 297), (240, 320), (297, 420), (320, 480)][rng.random_range(0..4)];
    let margins = [
        rng.random_range(12..=22),
        rng.random_range(12..=22),
        rng.random_range(12..=24),
        rng.random_range(18..=30),
    ];
    let gap_x = rng.random_range(4..=8);
    let gap_y = rng.random_range(4..=10);
    let (name, weights, row_weights, areas) = match regime {
        1 => {
            let cols = rng.random_range(4..=6);
            let rows = rng.random_range(4..=7);
            let split = rng.random_range(1..cols);
            (
                "Modular page",
                vec![1; cols],
                vec![1; rows],
                vec![
                    area("Title", (1, cols), (1, 1)),
                    area("Figure", (1, split), (2, rows - 1)),
                    area("Caption", (1, split), (rows, rows)),
                    area("Text", (split + 1, cols), (2, rows)),
                ],
            )
        }
        2 => {
            let notes_right = rng.random_bool(0.5);
            let (notes, left, right, weights) = if notes_right {
                (3, 1, 2, vec![3, 3, 1])
            } else {
                (1, 2, 3, vec![1, 3, 3])
            };
            (
                "Editorial page with marginalia",
                weights,
                vec![1, 2, 2, 1],
                vec![
                    area("Marginalia", (notes, notes), (1, 4)),
                    area("Title", (left, right), (1, 1)),
                    area("Figure", (left, left), (2, 3)),
                    area("Caption", (left, left), (4, 4)),
                    area("Text", (right, right), (2, 4)),
                ],
            )
        }
        _ => {
            let cols = rng.random_range(3..=5);
            let rows = rng.random_range(3..=5);
            let mut weights = vec![2; cols];
            weights[cols - 1] = 1;
            let areas = if rng.random_bool(0.5) {
                vec![
                    area("Title", (1, cols), (1, 1)),
                    area("Image", (1, cols - 1), (2, rows)),
                    area("Details", (cols, cols), (2, rows)),
                ]
            } else {
                vec![
                    area("Image", (1, cols), (1, rows - 1)),
                    area("Title", (1, cols - 1), (rows, rows)),
                    area("Details", (cols, cols), (rows, rows)),
                ]
            };
            ("Asymmetric poster", weights, vec![1; rows], areas)
        }
    };
    let columns = divide(
        page.0 - margins[0] - margins[1] - gap_x * (weights.len() - 1),
        &weights,
    );
    let rows = divide(
        page.1 - margins[2] - margins[3] - gap_y * (row_weights.len() - 1),
        &row_weights,
    );
    Grid {
        name,
        page,
        margins,
        columns,
        rows,
        gaps: (gap_x, gap_y),
        areas,
    }
}

fn list(values: &[usize]) -> String {
    values
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(" + ")
}

fn span(axis: &str, (first, last): (usize, usize)) -> String {
    if first == last {
        format!("{axis} {first}")
    } else {
        format!("{axis}s {first}–{last}")
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let g = grid(rng);
    let [left, right, top, bottom] = g.margins;
    let mut lines = vec![
        g.name.to_owned(),
        format!("{} × {} mm", g.page.0, g.page.1),
        format!("Margins L {left}, R {right}, T {top}, B {bottom} mm"),
        format!("Column widths {} mm", list(&g.columns)),
        format!("Row heights {} mm", list(&g.rows)),
    ];
    if g.columns.len() > 1 {
        lines.push(format!(
            "Gutters {} mm across, {} mm down",
            g.gaps.0, g.gaps.1
        ));
        for a in &g.areas {
            lines.push(format!(
                "{}  {}, {}",
                a.name,
                span("column", a.columns),
                span("row", a.rows)
            ));
        }
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    #[test]
    fn track_geometry_closes_and_areas_tile_without_overlap() {
        let mut names = HashSet::new();
        for seed in 0..512 {
            let g = grid(&mut StdRng::seed_from_u64(seed));
            names.insert(g.name);
            let mut x = g.margins[0];
            for (i, &width) in g.columns.iter().enumerate() {
                assert!(width >= 10);
                x += width;
                if i + 1 < g.columns.len() {
                    x += g.gaps.0;
                }
            }
            assert_eq!(x + g.margins[1], g.page.0);
            let mut y = g.margins[2];
            for (i, &height) in g.rows.iter().enumerate() {
                assert!(height >= 10);
                y += height;
                if i + 1 < g.rows.len() {
                    y += g.gaps.1;
                }
            }
            assert_eq!(y + g.margins[3], g.page.1);
            let mut cells = vec![vec![0; g.columns.len()]; g.rows.len()];
            for a in g.areas {
                assert!(a.columns.0 >= 1 && a.columns.0 <= a.columns.1);
                assert!(a.rows.0 >= 1 && a.rows.0 <= a.rows.1);
                for row in &mut cells[a.rows.0 - 1..a.rows.1] {
                    for cell in &mut row[a.columns.0 - 1..a.columns.1] {
                        *cell += 1;
                    }
                }
            }
            assert!(cells.into_iter().flatten().all(|count| count == 1));
        }
        assert_eq!(names.len(), 5);
    }

    #[test]
    fn largest_remainders_preserve_proportions_to_within_one_mm() {
        for total in 100..300 {
            for weights in [&[1, 1, 1][..], &[1, 3, 3], &[2, 2, 2, 1]] {
                let tracks = divide(total, weights);
                assert_eq!(tracks.iter().sum::<usize>(), total);
                let sum = weights.iter().sum::<usize>();
                for (&track, &weight) in tracks.iter().zip(weights) {
                    assert!((track * sum).abs_diff(total * weight) < sum);
                }
            }
        }
    }
}
