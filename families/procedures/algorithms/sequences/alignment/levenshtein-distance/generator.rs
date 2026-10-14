use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

fn execute(left: &[u8], right: &[u8]) -> (Vec<Vec<usize>>, String, String) {
    let mut table = vec![vec![0; right.len() + 1]; left.len() + 1];
    for (i, row) in table.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, value) in table[0].iter_mut().enumerate() {
        *value = j;
    }
    for i in 1..=left.len() {
        for j in 1..=right.len() {
            table[i][j] = (table[i - 1][j] + 1)
                .min(table[i][j - 1] + 1)
                .min(table[i - 1][j - 1] + usize::from(left[i - 1] != right[j - 1]));
        }
    }
    let (mut i, mut j) = (left.len(), right.len());
    let (mut a, mut b) = (Vec::new(), Vec::new());
    while i > 0 || j > 0 {
        if i > 0
            && j > 0
            && table[i][j] == table[i - 1][j - 1] + usize::from(left[i - 1] != right[j - 1])
        {
            a.push(left[i - 1]);
            b.push(right[j - 1]);
            i -= 1;
            j -= 1;
        } else if i > 0 && table[i][j] == table[i - 1][j] + 1 {
            a.push(left[i - 1]);
            b.push(b'-');
            i -= 1;
        } else {
            a.push(b'-');
            b.push(right[j - 1]);
            j -= 1;
        }
    }
    a.reverse();
    b.reverse();
    (
        table,
        String::from_utf8(a).unwrap(),
        String::from_utf8(b).unwrap(),
    )
}
pub fn generate(rng: &mut StdRng) -> String {
    const FAMILIES: &[&[&str]] = &[
        &["planet", "plane", "plan", "plant", "planted"],
        &["stone", "tones", "notes", "toned", "stony"],
        &["grain", "train", "trail", "rain", "brain"],
        &["flame", "frame", "fame", "name", "game"],
        &["spare", "spear", "pear", "pearl", "sparse"],
        &["money", "honey", "hone", "home", "homes"],
        &["sample", "simple", "simply", "samples", "simpler"],
        &["crane", "cane", "can", "craned", "crank"],
        &["learn", "lean", "leant", "yearn", "earn"],
        &["music", "musics", "muse", "mused", "mosaic"],
    ];
    // Shared spelling makes insertions, deletions and alignments visible,
    // instead of producing mostly unrelated all-substitution examples.
    let words = FAMILIES[rng.random_range(0..FAMILIES.len())];
    let i = rng.random_range(0..words.len());
    let mut j = rng.random_range(0..words.len() - 1);
    if j >= i {
        j += 1;
    }
    let (left, right) = (words[i], words[j]);
    let (table, a, b) = execute(left.as_bytes(), right.as_bytes());
    let mut text = format!(
        "Levenshtein-distance execution\nSource: {left}; target: {right}.\nInsertion, deletion, and substitution each cost 1; a matching letter costs 0.\nRows are source-prefix lengths 0..{}; columns are target-prefix lengths 0..{}.",
        left.len(),
        right.len()
    );
    for (i, row) in table.iter().enumerate() {
        write!(text, "\nRow {i}: {row:?}").unwrap();
    }
    write!(text,"\nMinimum edit cost: {}\nOne optimal alignment (- denotes a gap):\n{a}\n{b}\nTraceback tie order: diagonal, deletion, insertion.",table[left.len()][right.len()]).unwrap();
    text
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashSet, VecDeque};
    fn shortest_edit_path(start: &[u8], goal: &[u8]) -> usize {
        let mut visited = HashSet::from([start.to_vec()]);
        let mut queue = VecDeque::from([(start.to_vec(), 0)]);
        while let Some((word, cost)) = queue.pop_front() {
            if word == goal {
                return cost;
            }
            let mut neighbors = Vec::new();
            for i in 0..word.len() {
                let mut deleted = word.clone();
                deleted.remove(i);
                neighbors.push(deleted);
                for letter in b'a'..=b'b' {
                    let mut replaced = word.clone();
                    replaced[i] = letter;
                    neighbors.push(replaced);
                }
            }
            if word.len() < 3 {
                for i in 0..=word.len() {
                    for letter in b'a'..=b'b' {
                        let mut inserted = word.clone();
                        inserted.insert(i, letter);
                        neighbors.push(inserted);
                    }
                }
            }
            for next in neighbors {
                if visited.insert(next.clone()) {
                    queue.push_back((next, cost + 1));
                }
            }
        }
        unreachable!()
    }
    #[test]
    fn optimal_cost_matches_independent_breadth_first_edit_graph() {
        let words: Vec<Vec<u8>> = (0..=3)
            .flat_map(|len| {
                (0usize..1 << len)
                    .map(move |mask| (0..len).map(|i| b'a' + ((mask >> i) & 1) as u8).collect())
            })
            .collect();
        for left in &words {
            for right in &words {
                let (table, a, b) = execute(left, right);
                assert_eq!(
                    table[left.len()][right.len()],
                    shortest_edit_path(left, right)
                );
                assert_eq!(a.replace('-', "").as_bytes(), left);
                assert_eq!(b.replace('-', "").as_bytes(), right);
                assert_eq!(a.len(), b.len());
                assert_eq!(
                    a.bytes().zip(b.bytes()).filter(|(x, y)| x != y).count(),
                    table[left.len()][right.len()]
                );
            }
        }
    }
}
