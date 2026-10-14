"""Generate original, executable lift-plan draft generators for four weave families."""
from pathlib import Path

BASE = Path(__file__).resolve().parent

COMMON = '''use rand::{rngs::StdRng, Rng};

struct Draft {
    shafts: usize,
    threading: Vec<usize>,
    lifts: Vec<Vec<usize>>,
    title: String,
}

fn drawdown(draft: &Draft) -> Vec<Vec<bool>> {
    draft.lifts.iter().map(|lift| draft.threading.iter().map(|shaft| lift.contains(shaft)).collect()).collect()
}

fn render(rng: &mut StdRng, draft: &Draft) -> String {
    let colors = [("indigo", "ivory"), ("charcoal", "silver gray"), ("olive", "ecru"), ("rust", "cream"), ("teal", "stone"), ("burgundy", "oatmeal")];
    let (warp, weft) = colors[rng.random_range(0..colors.len())];
    let group = rng.random_range(1..=4);
    let alternate = rng.random_bool(0.5);
    let warp_colors: Vec<_> = (0..draft.threading.len()).map(|i| if alternate && (i / group) % 2 == 1 { weft } else { warp }).collect();
    let list = |values: &[usize]| values.iter().map(usize::to_string).collect::<Vec<_>>().join(" ");
    let mut lines = vec![
        draft.title.clone(),
        format!("Rising-shed lift plan; {} shafts. Threading entries are ends; repeat cyclically.", draft.shafts),
        format!("Threading, ends left to right: {}.", list(&draft.threading)),
        format!("Warp-color repeat, ends left to right: {}.", warp_colors.join(", ")),
        format!("Weft: {weft}. Picks repeat cyclically; unlisted shafts stay down."),
    ];
    for (i, lifted) in draft.lifts.iter().enumerate() {
        lines.push(format!("Pick {}: raise shafts {}.", i + 1, list(lifted)));
    }
    lines.push("Drawdown: X = warp over weft, . = weft over warp; columns are ends, rows picks.".to_owned());
    for (i, row) in drawdown(draft).iter().enumerate() {
        lines.push(format!("{:02} {}", i + 1, row.iter().map(|over| if *over { 'X' } else { '.' }).collect::<String>()));
    }
    lines.join("\\n")
}

fn max_cyclic_run(values: &[bool]) -> usize {
    let mut longest = 0;
    let mut run = 0;
    for &value in values.iter().cycle().take(2 * values.len()) {
        if value { run += 1; longest = longest.max(run); } else { run = 0; }
    }
    longest.min(values.len())
}

'''

CASES = {
    "plain": '''
    let first = rng.random_range(0..2);
    Draft {
        shafts: 2,
        threading: vec![1, 2],
        lifts: vec![vec![first + 1], vec![2 - first]],
        title: "Two-shaft plain-weave draft; 1/1 interlacement.".to_owned(),
    }
''',
    "basket": '''
    let group = rng.random_range(2..=4);
    let first = rng.random_range(0..2);
    Draft {
        shafts: 2,
        threading: (0..2 * group).map(|i| i / group + 1).collect(),
        lifts: (0..2 * group).map(|i| vec![(i / group + first) % 2 + 1]).collect(),
        title: format!("Two-shaft {group}x{group} basket-weave draft; grouped ends and picks."),
    }
''',
    "twill": '''
    let n = rng.random_range(3..=8);
    let over = rng.random_range(1..n);
    let reverse = rng.random_bool(0.5);
    let offset = rng.random_range(0..n);
    let lifts = (0..n).map(|pick| {
        let start = (offset + if reverse { n - pick } else { pick }) % n;
        (0..n).filter(|&shaft| (shaft + n - start) % n < over).map(|shaft| shaft + 1).collect()
    }).collect();
    Draft {
        shafts: n,
        threading: (1..=n).collect(),
        lifts,
        title: format!("{n}-shaft {over}/{} straight twill; one-shaft {} progression.", n - over, if reverse { "descending" } else { "ascending" }),
    }
''',
    "satin": '''
    let (n, steps): (usize, &[usize]) = [(5, &[2, 3][..]), (7, &[2, 3, 4, 5][..]), (8, &[3, 5][..])][rng.random_range(0..3)];
    let step = steps[rng.random_range(0..steps.len())];
    let offset = rng.random_range(0..n);
    let lifts = (0..n).map(|pick| {
        let binding = (offset + pick * step) % n;
        (0..n).filter(|&shaft| shaft != binding).map(|shaft| shaft + 1).collect()
    }).collect();
    Draft {
        shafts: n,
        threading: (1..=n).collect(),
        lifts,
        title: format!("{n}-shaft warp-faced satin; binding step {step}; maximum cyclic warp float {} picks.", n - 1),
    }
''',
}

TESTS = {
    "plain": '''
            assert_eq!(d.shafts, 2);
            for row in &grid { assert_eq!(row.iter().filter(|&&v| v).count(), 1); }
            for (&a, &b) in grid[0].iter().zip(&grid[1]) { assert_ne!(a, b); }
''',
    "basket": '''
            let group = d.threading.len() / 2;
            assert!((2..=4).contains(&group));
            for row in &grid {
                assert!(row[..group].iter().all(|&v| v == row[0]));
                assert!(row[group..].iter().all(|&v| v != row[0]));
            }
            for col in 0..2 * group {
                let column: Vec<_> = grid.iter().map(|row| row[col]).collect();
                assert_eq!(max_cyclic_run(&column), group);
                assert_eq!(max_cyclic_run(&column.iter().map(|v| !v).collect::<Vec<_>>()), group);
            }
''',
    "twill": '''
            let n = d.shafts;
            let over = grid[0].iter().filter(|&&v| v).count();
            let shift = (0..n).find(|&s| (0..n).all(|i| grid[1][i] == grid[0][(i + s) % n])).unwrap();
            assert!(shift == 1 || shift == n - 1);
            for row in &grid {
                assert_eq!(row.iter().filter(|&&v| v).count(), over);
                assert_eq!(max_cyclic_run(row), over);
            }
            for col in 0..n {
                let column: Vec<_> = grid.iter().map(|row| row[col]).collect();
                assert_eq!(max_cyclic_run(&column), over);
            }
''',
    "satin": '''
            let n = d.shafts;
            let positions: Vec<_> = grid.iter().map(|row| {
                assert_eq!(row.iter().filter(|&&v| !v).count(), 1);
                row.iter().position(|&v| !v).unwrap()
            }).collect();
            assert_eq!(positions.iter().copied().collect::<HashSet<_>>().len(), n);
            for i in 0..n {
                let shift = (positions[(i + 1) % n] + n - positions[i]) % n;
                assert!(shift > 1 && shift < n - 1);
            }
            for col in 0..n {
                let column: Vec<_> = grid.iter().map(|row| row[col]).collect();
                assert_eq!(column.iter().filter(|&&v| !v).count(), 1);
                assert_eq!(max_cyclic_run(&column), n - 1);
            }
''',
}

def main():
    for kind, body in CASES.items():
        leaf = BASE / kind
        leaf.mkdir(parents=True, exist_ok=True)
        text = "// Generated by ../rebuild.py; original lift-plan designs.\n" + COMMON
        # Float helpers are used only by tests; a plain weave does not need one.
        text = text.replace("fn max_cyclic_run", "#[cfg(test)]\nfn max_cyclic_run")
        if kind == "plain":
            text = text[:text.index("#[cfg(test)]\nfn max_cyclic_run")]
        text += "fn draft(rng: &mut StdRng) -> Draft {\n" + body + "}\n"
        text += "\npub fn generate(rng: &mut StdRng) -> String {\n    let d = draft(rng);\n    render(rng, &d)\n}\n"
        text += '''
#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    #[test]
    fn draft_interlaces_on_both_axes_and_preserves_weave_structure() {
        let mut rng = StdRng::seed_from_u64(73);
        let mut outputs = HashSet::new();
        for _ in 0..500 {
            let d = draft(&mut rng);
            let grid = drawdown(&d);
            for row in &grid { assert!(row.contains(&true) && row.contains(&false)); }
            for col in 0..d.threading.len() {
                let column: Vec<_> = grid.iter().map(|row| row[col]).collect();
                assert!(column.contains(&true) && column.contains(&false));
            }
            for &shaft in &d.threading { assert!((1..=d.shafts).contains(&shaft)); }
            for lift in &d.lifts {
                assert_eq!(lift.iter().copied().collect::<HashSet<_>>().len(), lift.len());
                assert!(lift.iter().all(|s| (1..=d.shafts).contains(s)));
            }
CHECK
            let text = render(&mut rng, &d);
            assert!(!text.ends_with('\\n'));
            outputs.insert(text);
        }
        assert!(outputs.len() >= MINIMUM);
    }
}
'''.replace("CHECK", TESTS[kind]).replace("MINIMUM", "12" if kind == "plain" else "50")
        (leaf / "generator.rs").write_text(text, encoding="utf-8")
    print("Generated four structurally tested weave-draft leaves.")


if __name__ == "__main__":
    main()
