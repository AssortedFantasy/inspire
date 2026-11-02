use rand::{Rng, rngs::StdRng, seq::SliceRandom};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Item(usize, usize);

#[derive(Clone, Copy, Debug)]
enum Clue {
    At(Item, u8),
    Together(Item, Item),
    Apart(Item, Item),
    Before(Item, Item),
    Next(Item, Item),
    Gap(Item, Item, u8),
}

type Assignment = Vec<Vec<u8>>;

impl Clue {
    fn holds(self, assignment: &Assignment) -> bool {
        let pos = |Item(c, i): Item| assignment[c][i];
        match self {
            Self::At(a, p) => pos(a) == p,
            Self::Together(a, b) => pos(a) == pos(b),
            Self::Apart(a, b) => pos(a) != pos(b),
            Self::Before(a, b) => pos(a) < pos(b),
            Self::Next(a, b) => pos(a) + 1 == pos(b),
            Self::Gap(a, b, d) => pos(a).abs_diff(pos(b)) == d,
        }
    }
}

fn permutations(n: usize) -> Vec<Vec<u8>> {
    fn visit(prefix: &mut Vec<u8>, n: usize, out: &mut Vec<Vec<u8>>) {
        if prefix.len() == n {
            out.push(prefix.clone());
        } else {
            for value in 0..n as u8 {
                if !prefix.contains(&value) {
                    prefix.push(value);
                    visit(prefix, n, out);
                    prefix.pop();
                }
            }
        }
    }
    let mut result = Vec::new();
    visit(&mut Vec::new(), n, &mut result);
    result
}

fn universe(n: usize, categories: usize) -> Vec<Assignment> {
    let perms = permutations(n);
    let mut all = vec![Vec::new()];
    for _ in 0..categories {
        all = all
            .into_iter()
            .flat_map(|a| {
                perms.iter().map(move |p| {
                    let mut extended = a.clone();
                    extended.push(p.clone());
                    extended
                })
            })
            .collect();
    }
    all
}

fn puzzle(rng: &mut StdRng, n: usize, categories: usize) -> (Assignment, Vec<Clue>) {
    let all = universe(n, categories);
    let answer = all[rng.random_range(0..all.len())].clone();
    let items: Vec<_> = (0..categories)
        .flat_map(|c| (0..n).map(move |i| Item(c, i)))
        .collect();
    let mut pool = Vec::new();
    for &a in &items {
        pool.push(Clue::At(a, answer[a.0][a.1]));
        for &b in &items {
            if a == b {
                continue;
            }
            if a.0 != b.0 && a.0 < b.0 {
                pool.push(Clue::Together(a, b));
                pool.push(Clue::Apart(a, b));
            }
            pool.push(Clue::Before(a, b));
            pool.push(Clue::Next(a, b));
            if a.0 < b.0 || (a.0 == b.0 && a.1 < b.1) {
                for d in 1..n as u8 {
                    pool.push(Clue::Gap(a, b, d));
                }
            }
        }
    }
    pool.retain(|clue| clue.holds(&answer));
    pool.shuffle(rng);
    let mut remaining: Vec<_> = all.iter().collect();
    let mut clues = Vec::new();
    for clue in pool {
        if remaining.len() == 1 || clues.len() == 12 {
            break;
        }
        let before = remaining.len();
        remaining.retain(|a| clue.holds(a));
        if remaining.len() < before {
            clues.push(clue);
        }
    }
    if remaining.len() > 1 {
        // Finish with at most one positional fact per item, then remove all
        // redundant clues below. The puzzle stays complete at a bounded size.
        clues.extend(
            items
                .iter()
                .map(|&item| Clue::At(item, answer[item.0][item.1])),
        );
    }
    // Removing other constraints can only enlarge the solution set. One pass
    // therefore proves deletion-minimality, without claiming fewest clues.
    clues.shuffle(rng);
    let mut i = 0;
    while i < clues.len() {
        let necessary = all
            .iter()
            .filter(|a| {
                clues
                    .iter()
                    .enumerate()
                    .all(|(j, clue)| i == j || clue.holds(a))
            })
            .take(2)
            .count()
            > 1;
        if necessary {
            i += 1;
        } else {
            clues.remove(i);
        }
    }
    (answer, clues)
}

pub fn generate(rng: &mut StdRng) -> String {
    let (title, labels, names): (&str, [&str; 3], [[&str; 4]; 3]) = match rng.random_range(0..3) {
        0 => (
            "Deliveries, earliest to latest",
            ["Recipient", "Parcel", "Courier"],
            [
                ["Mara", "Ivo", "Nia", "Sam"],
                ["blue parcel", "red parcel", "gold parcel", "green parcel"],
                ["bicycle", "van", "tram", "boat"],
            ],
        ),
        1 => (
            "Recital, first to last",
            ["Performer", "Instrument", "Piece"],
            [
                ["Asha", "Leon", "Mei", "Omar"],
                ["cello", "flute", "harp", "piano"],
                ["prelude", "waltz", "nocturne", "fugue"],
            ],
        ),
        _ => (
            "Shops, left to right",
            ["Owner", "Trade", "Door"],
            [
                ["Ada", "Finn", "Rosa", "Tariq"],
                ["bakery", "florist", "bookshop", "cobbler"],
                ["oak door", "blue door", "glass door", "red door"],
            ],
        ),
    };
    let n = if rng.random_bool(0.5) { 3 } else { 4 };
    let categories = if n == 3 && rng.random_bool(0.5) { 3 } else { 2 };
    let (_, clues) = puzzle(rng, n, categories);
    let mut text = format!("{title}\nPositions: 1–{n}");
    for c in 0..categories {
        text.push_str(&format!("\n{}: {}", labels[c], names[c][..n].join(", ")));
    }
    let name = |Item(c, i): Item| names[c][i];
    for (i, clue) in clues.into_iter().enumerate() {
        let line = match clue {
            Clue::At(a, p) => format!("{} is at position {}.", name(a), p + 1),
            Clue::Together(a, b) => format!("{} and {} share a position.", name(a), name(b)),
            Clue::Apart(a, b) => format!("{} and {} occupy different positions.", name(a), name(b)),
            Clue::Before(a, b) => format!("{} comes before {}.", name(a), name(b)),
            Clue::Next(a, b) => format!("{} is immediately before {}.", name(a), name(b)),
            Clue::Gap(a, b, 1) => format!("{} and {} are adjacent.", name(a), name(b)),
            Clue::Gap(a, b, d) => format!("{} and {} are {d} positions apart.", name(a), name(b)),
        };
        text.push_str(&format!("\n{}. {line}", i + 1));
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn enumeration_has_every_bijection_once() {
        assert_eq!(permutations(4).len(), 24);
        assert_eq!(universe(4, 2).len(), 576);
        assert_eq!(universe(3, 3).len(), 216);
        for p in permutations(4) {
            let mut sorted = p;
            sorted.sort_unstable();
            assert_eq!(sorted, [0, 1, 2, 3]);
        }
    }

    #[test]
    fn every_puzzle_has_one_answer_and_every_clue_is_necessary() {
        let mut rng = StdRng::seed_from_u64(80421);
        let mut kinds = [false; 6];
        for (n, categories) in [(3, 2), (3, 3), (4, 2)] {
            let all = universe(n, categories);
            for _ in 0..40 {
                let (answer, clues) = puzzle(&mut rng, n, categories);
                assert!(clues.len() <= 12 + n * categories);
                let solutions: Vec<_> = all
                    .iter()
                    .filter(|a| clues.iter().all(|c| c.holds(a)))
                    .collect();
                assert_eq!(solutions, [&answer]);
                for (i, clue) in clues.iter().enumerate() {
                    kinds[match clue {
                        Clue::At(..) => 0,
                        Clue::Together(..) => 1,
                        Clue::Apart(..) => 2,
                        Clue::Before(..) => 3,
                        Clue::Next(..) => 4,
                        Clue::Gap(..) => 5,
                    }] = true;
                    assert!(
                        all.iter()
                            .filter(|a| clues.iter().enumerate().all(|(j, c)| i == j || c.holds(a)))
                            .take(2)
                            .count()
                            > 1
                    );
                }
            }
        }
        assert!(kinds.into_iter().all(|seen| seen));
    }
}
