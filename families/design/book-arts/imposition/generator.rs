use rand::{Rng, rngs::StdRng};

#[derive(Debug)]
struct Sheet {
    front: [usize; 2],
    back: [usize; 2],
}

struct Gathering {
    sheets: Vec<Sheet>, // outside to inside
}

struct Plan {
    name: &'static str,
    right_bound: bool,
    first: usize,
    last_printed: usize,
    page: (usize, usize),
    gatherings: Vec<Gathering>,
}

fn impose(first: usize, sheets: usize, right_bound: bool) -> Gathering {
    let last = first + 4 * sheets - 1;
    Gathering {
        sheets: (0..sheets)
            .map(|i| {
                let mut front = [last - 2 * i, first + 2 * i];
                let mut back = [first + 2 * i + 1, last - 2 * i - 1];
                if right_bound {
                    front.reverse();
                    back.reverse();
                }
                Sheet { front, back }
            })
            .collect(),
    }
}

fn plan(rng: &mut StdRng) -> Plan {
    let (name, sizes) = match rng.random_range(0..3) {
        0 => ("Nested booklet", vec![rng.random_range(1..=6)]),
        1 => (
            "Sewn gatherings",
            vec![rng.random_range(2..=4); rng.random_range(2..=4)],
        ),
        _ => {
            let full = rng.random_range(3..=5);
            let mut sizes = vec![full; rng.random_range(1..=3)];
            sizes.push(rng.random_range(1..full));
            ("Sewn gatherings with short final section", sizes)
        }
    };
    let right_bound = rng.random_bool(0.5);
    let first = [1, 1, 1, 17, 33, 65][rng.random_range(0..6)];
    let blanks = if rng.random_bool(0.5) {
        0
    } else {
        rng.random_range(1..=3)
    };
    let count = sizes.iter().sum::<usize>() * 4;
    let mut next = first;
    let gatherings = sizes
        .into_iter()
        .map(|sheets| {
            let gathering = impose(next, sheets, right_bound);
            next += sheets * 4;
            gathering
        })
        .collect();
    Plan {
        name,
        right_bound,
        first,
        last_printed: first + count - blanks - 1,
        page: [(148, 210), (105, 148), (160, 240), (180, 240)][rng.random_range(0..4)],
        gatherings,
    }
}

fn render(plan: &Plan) -> String {
    let hand = if plan.right_bound { "right" } else { "left" };
    let mut lines = vec![
        format!("{}, {hand} bound", plan.name),
        format!("Page {} × {} mm", plan.page.0, plan.page.1),
        format!(
            "Sheet {} × {} mm, one vertical fold",
            2 * plan.page.0,
            plan.page.1
        ),
        format!("Pages {}–{}", plan.first, plan.last_printed),
        "Each face viewed separately, heads up. Front | back, left to right.".to_owned(),
    ];
    for (g, gathering) in plan.gatherings.iter().enumerate() {
        lines.push(format!("Gathering {}", char::from(b'A' + g as u8)));
        let label = |n: usize| {
            if n <= plan.last_printed {
                n.to_string()
            } else {
                "blank".to_owned()
            }
        };
        for (s, sheet) in gathering.sheets.iter().enumerate() {
            lines.push(format!(
                "  {}  [{}, {}] | [{}, {}]",
                s + 1,
                label(sheet.front[0]),
                label(sheet.front[1]),
                label(sheet.back[0]),
                label(sheet.back[1])
            ));
        }
    }
    lines.push(if plan.right_bound {
        "Fold right half behind left. Nest each gathering in sheet order.".to_owned()
    } else {
        "Fold left half behind right. Nest each gathering in sheet order.".to_owned()
    });
    lines.join("\n")
}

pub fn generate(rng: &mut StdRng) -> String {
    render(&plan(rng))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    // Fold each physical sheet into two leaves. The reverse side of a sheet's
    // right panel is its back-left panel when both faces are viewed heads up.
    // Nesting puts one leaf of each sheet before the centre, one after it.
    fn read_folded(g: &Gathering, right_bound: bool) -> Vec<usize> {
        let leading = usize::from(!right_bound);
        let trailing = 1 - leading;
        let mut leaves = Vec::new();
        for sheet in &g.sheets {
            leaves.push([sheet.front[leading], sheet.back[trailing]]);
        }
        for sheet in g.sheets.iter().rev() {
            leaves.push([sheet.back[leading], sheet.front[trailing]]);
        }
        leaves.into_iter().flatten().collect()
    }

    #[test]
    fn physical_folding_and_nesting_recover_every_page_in_reading_order() {
        let mut architectures = HashSet::new();
        let mut blank_counts = HashSet::new();
        for seed in 0..512 {
            let p = plan(&mut StdRng::seed_from_u64(seed));
            architectures.insert(p.name);
            let mut read = Vec::new();
            let mut cells = Vec::new();
            for g in &p.gatherings {
                read.extend(read_folded(g, p.right_bound));
                cells.extend(
                    g.sheets
                        .iter()
                        .flat_map(|s| s.front.into_iter().chain(s.back)),
                );
            }
            let expected: Vec<_> = (p.first..p.first + cells.len()).collect();
            assert_eq!(read, expected);
            cells.sort_unstable();
            assert_eq!(cells, expected, "every page has one physical position");
            let blanks = read.iter().filter(|&&n| n > p.last_printed).count();
            blank_counts.insert(blanks);
            assert!(blanks <= 3);
            assert!(p.first % 2 == 1);
            assert_eq!(
                read.iter().position(|&n| n > p.last_printed),
                if blanks == 0 {
                    None
                } else {
                    Some(read.len() - blanks)
                }
            );
        }
        assert_eq!(architectures.len(), 3);
        assert_eq!(blank_counts.len(), 4);
    }

    #[test]
    fn binding_direction_mirrors_both_faces_without_changing_the_book() {
        for first in [1, 17, 65] {
            for n in 1..=6 {
                let left = impose(first, n, false);
                let right = impose(first, n, true);
                for (a, b) in left.sheets.iter().zip(&right.sheets) {
                    assert_eq!([a.front[1], a.front[0]], b.front);
                    assert_eq!([a.back[1], a.back[0]], b.back);
                }
                assert_eq!(read_folded(&left, false), read_folded(&right, true));
            }
        }
    }
}
