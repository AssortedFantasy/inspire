"""Generate original lift-plan and drawdown generators for four weave families."""
from pathlib import Path
import subprocess

BASE = Path(__file__).resolve().parent

COMMON = '''use rand::{rngs::StdRng, Rng};

struct Draft {
    shafts: usize,
    threading: Vec<usize>,
    lifts: Vec<Vec<usize>>,
    title: String,
}

#[cfg(test)]
fn drawdown(draft: &Draft) -> Vec<Vec<bool>> {
    draft.lifts.iter().map(|lift| draft.threading.iter().map(|shaft| lift.contains(shaft)).collect()).collect()
}

fn stripe_plan(rng: &mut StdRng, colors: (&str, &str)) -> String {
    let repeats: &[&[usize]] = &[&[1, 1], &[2, 2], &[4, 1], &[4, 2], &[1, 1, 2, 2], &[4, 1, 1, 1], &[6, 1, 1, 1], &[2, 1, 2, 3]];
    let repeat = repeats[rng.random_range(0..repeats.len())];
    repeat.iter().enumerate().map(|(i, count)| {
        format!("{}×{count}", if i % 2 == 0 { colors.0 } else { colors.1 })
    }).collect::<Vec<_>>().join(" ")
}

fn render(rng: &mut StdRng, draft: &Draft) -> String {
    let colors = [("indigo", "ivory"), ("charcoal", "silver"), ("olive", "ecru"), ("rust", "cream"), ("teal", "stone"), ("burgundy", "oatmeal"), ("ochre", "ink"), ("walnut", "flax")];
    let pair = colors[rng.random_range(0..colors.len())];
    let warp = stripe_plan(rng, pair);
    let weft = if rng.random_bool(0.5) { stripe_plan(rng, (pair.1, pair.0)) } else { pair.1.to_owned() };
    let list = |values: &[usize]| values.iter().map(usize::to_string).collect::<Vec<_>>().join(" ");
    let mut lines = vec![draft.title.clone(), format!("Warp {warp}"), format!("Weft {weft}")];
    if rng.random_bool(0.5) {
        lines.push(format!("{} shafts", draft.shafts));
        lines.push(format!("Threading {}", list(&draft.threading)));
        lines.push("Lift repeat".to_owned());
        for (i, lifted) in draft.lifts.iter().enumerate() {
            lines.push(format!("{:02}  {}", i + 1, list(lifted)));
        }
    } else {
        lines.push("Warp-over drawdown".to_owned());
        for lift in &draft.lifts {
            lines.push(draft.threading.iter().map(|shaft| if lift.contains(shaft) { 'X' } else { '.' }).collect());
        }
    }
    lines.join("\\n")
}

#[cfg(test)]
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
        title: "Plain weave".to_owned(),
    }
''',
    "basket": '''
    let ends = rng.random_range(2..=4);
    let picks = rng.random_range(2..=4);
    let first = rng.random_range(0..2);
    Draft {
        shafts: 2,
        threading: (0..2 * ends).map(|i| i / ends + 1).collect(),
        lifts: (0..2 * picks).map(|i| vec![(i / picks + first) % 2 + 1]).collect(),
        title: format!("Basket weave {ends} × {picks}"),
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
    let (name, threading) = match rng.random_range(0..3) {
        0 => ("straight", (1..=n).collect()),
        1 => ("point", (1..=n).chain((2..n).rev()).collect()),
        _ => ("broken", (1..=n).chain((0..n).map(|i| (n / 2 + n - 1 - i) % n + 1)).collect()),
    };
    Draft {
        shafts: n,
        threading,
        lifts,
        title: format!("{over}/{} {name} twill", n - over),
    }
''',
    "satin": '''
    let (n, steps): (usize, &[usize]) = [(5, &[2, 3][..]), (7, &[2, 3, 4, 5][..]), (8, &[3, 5][..])][rng.random_range(0..3)];
    let step = steps[rng.random_range(0..steps.len())];
    let offset = rng.random_range(0..n);
    let warp_faced = rng.random_bool(0.5);
    let lifts = (0..n).map(|pick| {
        let binding = (offset + pick * step) % n;
        (0..n).filter(|&shaft| (shaft != binding) == warp_faced).map(|shaft| shaft + 1).collect()
    }).collect();
    Draft {
        shafts: n,
        threading: (1..=n).collect(),
        lifts,
        title: format!("{n}-end {} satin, step {step}", if warp_faced { "warp-faced" } else { "weft-faced" }),
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
            let ends = d.threading.len() / 2;
            let picks = d.lifts.len() / 2;
            assert!((2..=4).contains(&ends));
            assert!((2..=4).contains(&picks));
            for row in &grid {
                assert!(row[..ends].iter().all(|&v| v == row[0]));
                assert!(row[ends..].iter().all(|&v| v != row[0]));
                assert_eq!(max_cyclic_run(row), ends);
            }
            for col in 0..2 * ends {
                let column: Vec<_> = grid.iter().map(|row| row[col]).collect();
                assert_eq!(max_cyclic_run(&column), picks);
                assert_eq!(max_cyclic_run(&column.iter().map(|v| !v).collect::<Vec<_>>()), picks);
            }
''',
    "twill": '''
            let n = d.shafts;
            let base: Vec<Vec<_>> = d.lifts.iter().map(|lift| (1..=n).map(|shaft| lift.contains(&shaft)).collect()).collect();
            let over = base[0].iter().filter(|&&v| v).count();
            let shift = (0..n).find(|&s| (0..n).all(|i| base[1][i] == base[0][(i + s) % n])).unwrap();
            assert!(shift == 1 || shift == n - 1);
            for i in 0..n {
                assert!((0..n).all(|col| base[(i + 1) % n][col] == base[i][(col + shift) % n]));
                assert_eq!(max_cyclic_run(&base[i]), over);
                let column: Vec<_> = base.iter().map(|row| row[i]).collect();
                assert_eq!(max_cyclic_run(&column), over);
            }
            let straight: Vec<_> = (1..=n).collect();
            let point: Vec<_> = (1..=n).chain((2..n).rev()).collect();
            let broken: Vec<_> = (1..=n).chain((0..n).map(|i| (n / 2 + n - 1 - i) % n + 1)).collect();
            assert!([straight, point, broken].contains(&d.threading));
''',
    "satin": '''
            let n = d.shafts;
            let binding_value = grid[0].iter().filter(|&&v| v).count() == 1;
            let positions: Vec<_> = grid.iter().map(|row| {
                assert_eq!(row.iter().filter(|&&v| v == binding_value).count(), 1);
                row.iter().position(|&v| v == binding_value).unwrap()
            }).collect();
            assert_eq!(positions.iter().copied().collect::<HashSet<_>>().len(), n);
            let step = (positions[1] + n - positions[0]) % n;
            assert!(step > 1 && step < n - 1);
            for i in 0..n {
                assert_eq!((positions[(i + 1) % n] + n - positions[i]) % n, step);
                let column: Vec<_> = grid.iter().map(|row| row[i] != binding_value).collect();
                assert_eq!(column.iter().filter(|&&v| !v).count(), 1);
                assert_eq!(max_cyclic_run(&column), n - 1);
            }
''',
}


def main():
    for kind, body in CASES.items():
        leaf = BASE / kind
        text = "// Generated by ../rebuild.py; original lift-plan designs.\n" + COMMON
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
        assert!(outputs.len() >= 100);
    }
}
'''.replace("CHECK", TESTS[kind])
        path = leaf / "generator.rs"
        path.write_text(text, encoding="utf-8")
        subprocess.run(["rustfmt", "--edition", "2024", str(path)], check=True)
    print("Generated four structurally tested weave-draft leaves.")


if __name__ == "__main__":
    main()
