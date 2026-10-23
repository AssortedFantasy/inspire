use rand::{Rng, rngs::StdRng};
use std::{collections::VecDeque, fmt::Write};

#[derive(Clone, Debug)]
struct Dfa {
    next: Vec<[usize; 2]>,
    final_states: Vec<bool>,
    start: usize,
}

impl Dfa {
    fn run_from(&self, mut state: usize, word: &str) -> usize {
        for symbol in word.bytes() {
            state = self.next[state][usize::from(symbol - b'0')];
        }
        state
    }

    fn accepts(&self, word: &str) -> bool {
        self.final_states[self.run_from(self.start, word)]
    }

    fn reachable(&self) -> Vec<bool> {
        let mut seen = vec![false; self.next.len()];
        let mut queue = VecDeque::from([self.start]);
        seen[self.start] = true;
        while let Some(q) = queue.pop_front() {
            for &r in &self.next[q] {
                if !seen[r] {
                    seen[r] = true;
                    queue.push_back(r);
                }
            }
        }
        seen
    }
}

fn word_label(word: &str) -> &str {
    if word.is_empty() { "ε" } else { word }
}

fn verdict(accepted: bool) -> &'static str {
    if accepted { "accept" } else { "reject" }
}

fn shortest(dfa: &Dfa, accepted: bool) -> Option<String> {
    let mut seen = vec![false; dfa.next.len()];
    seen[dfa.start] = true;
    let mut queue = VecDeque::from([(dfa.start, String::new())]);
    while let Some((q, word)) = queue.pop_front() {
        if dfa.final_states[q] == accepted {
            return Some(word);
        }
        for symbol in 0..2 {
            let r = dfa.next[q][symbol];
            if !seen[r] {
                seen[r] = true;
                let mut extended = word.clone();
                extended.push(char::from(b'0' + symbol as u8));
                queue.push_back((r, extended));
            }
        }
    }
    None
}

fn distinguish(a: &Dfa, p: usize, b: &Dfa, q: usize) -> Option<String> {
    let mut seen = vec![false; a.next.len() * b.next.len()];
    seen[p * b.next.len() + q] = true;
    let mut queue = VecDeque::from([(p, q, String::new())]);
    while let Some((p, q, word)) = queue.pop_front() {
        if a.final_states[p] != b.final_states[q] {
            return Some(word);
        }
        for symbol in 0..2 {
            let (r, s) = (a.next[p][symbol], b.next[q][symbol]);
            let index = r * b.next.len() + s;
            if !seen[index] {
                seen[index] = true;
                let mut extended = word.clone();
                extended.push(char::from(b'0' + symbol as u8));
                queue.push_back((r, s, extended));
            }
        }
    }
    None
}

fn minimize(dfa: &Dfa) -> (Dfa, Vec<Vec<usize>>) {
    let reachable = dfa.reachable();
    let mut classes: Vec<usize> = dfa.final_states.iter().map(|&f| usize::from(f)).collect();
    loop {
        let mut signatures = Vec::new();
        let mut refined = vec![usize::MAX; dfa.next.len()];
        for q in 0..dfa.next.len() {
            if reachable[q] {
                let signature = (
                    dfa.final_states[q],
                    classes[dfa.next[q][0]],
                    classes[dfa.next[q][1]],
                );
                let class = signatures
                    .iter()
                    .position(|s| *s == signature)
                    .unwrap_or_else(|| {
                        signatures.push(signature);
                        signatures.len() - 1
                    });
                refined[q] = class;
            }
        }
        if refined == classes {
            break;
        }
        classes = refined;
    }
    let count = classes.iter().filter(|&&c| c != usize::MAX).max().unwrap() + 1;
    let mut groups = vec![Vec::new(); count];
    for (q, &class) in classes.iter().enumerate() {
        if class != usize::MAX {
            groups[class].push(q);
        }
    }
    let next = groups
        .iter()
        .map(|group| {
            let q = group[0];
            [classes[dfa.next[q][0]], classes[dfa.next[q][1]]]
        })
        .collect();
    let final_states = groups
        .iter()
        .map(|group| dfa.final_states[group[0]])
        .collect();
    (
        Dfa {
            next,
            final_states,
            start: classes[dfa.start],
        },
        groups,
    )
}

#[derive(Clone, Copy)]
enum PatternKind {
    Suffix,
    Contains,
    Avoids,
}

fn pattern_dfa(pattern: &str, kind: PatternKind) -> Dfa {
    let n = pattern.len();
    let mut next = vec![[0; 2]; n + 1];
    for (q, transitions) in next.iter_mut().enumerate() {
        for (symbol, target) in transitions.iter_mut().enumerate() {
            if q == n && !matches!(kind, PatternKind::Suffix) {
                *target = n;
                continue;
            }
            let mut extended = pattern[..q].to_owned();
            extended.push(char::from(b'0' + symbol as u8));
            *target = (0..=n)
                .rev()
                .find(|&k| extended.ends_with(&pattern[..k]))
                .unwrap();
        }
    }
    let final_states = (0..=n)
        .map(|q| {
            if matches!(kind, PatternKind::Avoids) {
                q < n
            } else {
                q == n
            }
        })
        .collect();
    Dfa {
        next,
        final_states,
        start: 0,
    }
}

fn residues(modulus: usize, residue: usize, positional: bool) -> Dfa {
    let next = (0..modulus)
        .map(|q| {
            let base = if positional { 2 * q } else { q };
            [base % modulus, (base + 1) % modulus]
        })
        .collect();
    Dfa {
        next,
        final_states: (0..modulus).map(|q| q == residue).collect(),
        start: 0,
    }
}

fn bounded_balance(bound: usize) -> Dfa {
    let sink = bound + 1;
    let mut next: Vec<_> = (0..=bound)
        .map(|q| {
            [
                if q < bound { q + 1 } else { sink },
                if q > 0 { q - 1 } else { sink },
            ]
        })
        .collect();
    next.push([sink; 2]);
    let mut final_states = vec![false; sink + 1];
    final_states[0] = true;
    Dfa {
        next,
        final_states,
        start: 0,
    }
}

fn codebook(words: &[String]) -> Dfa {
    let mut edges = vec![[None; 2]];
    let mut final_states = vec![false];
    for word in words {
        let mut q = 0;
        for byte in word.bytes() {
            let symbol = usize::from(byte - b'0');
            q = if let Some(target) = edges[q][symbol] {
                target
            } else {
                let target = edges.len();
                edges[q][symbol] = Some(target);
                edges.push([None; 2]);
                final_states.push(false);
                target
            };
        }
        final_states[q] = true;
    }
    let sink = edges.len();
    let mut next: Vec<_> = edges
        .into_iter()
        .map(|row| [row[0].unwrap_or(sink), row[1].unwrap_or(sink)])
        .collect();
    next.push([sink; 2]);
    final_states.push(false);
    Dfa {
        next,
        final_states,
        start: 0,
    }
}

fn inflated(dfa: &Dfa) -> Dfa {
    let mut next = Vec::new();
    let mut final_states = Vec::new();
    for (q, row) in dfa.next.iter().enumerate() {
        for memory in 0..2 {
            next.push([2 * row[0] + 1 - memory, 2 * row[1] + memory]);
            final_states.push(dfa.final_states[q]);
        }
    }
    let junk = next.len();
    next.push([junk; 2]);
    final_states.push(!dfa.final_states[dfa.start]);
    Dfa {
        next,
        final_states,
        start: 2 * dfa.start,
    }
}

#[derive(Clone, Copy)]
enum Operation {
    And,
    Or,
    Xor,
    Difference,
}

impl Operation {
    fn apply(self, a: bool, b: bool) -> bool {
        match self {
            Self::And => a && b,
            Self::Or => a || b,
            Self::Xor => a ^ b,
            Self::Difference => a && !b,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::And => "A ∩ B",
            Self::Or => "A ∪ B",
            Self::Xor => "A △ B",
            Self::Difference => "A \\ B",
        }
    }
}

fn product(a: &Dfa, b: &Dfa, operation: Operation) -> (Dfa, Vec<(usize, usize)>) {
    let mut pairs = vec![(a.start, b.start)];
    let mut next = Vec::new();
    let mut final_states = Vec::new();
    let mut index = 0;
    while index < pairs.len() {
        let (p, q) = pairs[index];
        let mut row = [0; 2];
        for (symbol, target) in row.iter_mut().enumerate() {
            let pair = (a.next[p][symbol], b.next[q][symbol]);
            *target = pairs
                .iter()
                .position(|&old| old == pair)
                .unwrap_or_else(|| {
                    pairs.push(pair);
                    pairs.len() - 1
                });
        }
        next.push(row);
        final_states.push(operation.apply(a.final_states[p], b.final_states[q]));
        index += 1;
    }
    (
        Dfa {
            next,
            final_states,
            start: 0,
        },
        pairs,
    )
}

fn nth_from_end(distance: usize, bit: usize) -> (Dfa, Vec<usize>) {
    let mut subsets = vec![1usize];
    let mut next = Vec::new();
    let mut final_states = Vec::new();
    let mask = (1 << (distance + 1)) - 1;
    let mut index = 0;
    while index < subsets.len() {
        let subset = subsets[index];
        let mut row = [0; 2];
        for (symbol, target) in row.iter_mut().enumerate() {
            let mut reached = 1 | ((subset & !1) << 1 & mask);
            if symbol == bit {
                reached |= 2;
            }
            *target = subsets
                .iter()
                .position(|&old| old == reached)
                .unwrap_or_else(|| {
                    subsets.push(reached);
                    subsets.len() - 1
                });
        }
        next.push(row);
        final_states.push(subset & (1 << distance) != 0);
        index += 1;
    }
    (
        Dfa {
            next,
            final_states,
            start: 0,
        },
        subsets,
    )
}

fn random_word(rng: &mut StdRng, min: usize, max: usize) -> String {
    let len = rng.random_range(min..=max);
    (0..len)
        .map(|_| if rng.random_bool(0.5) { '0' } else { '1' })
        .collect()
}

fn language(rng: &mut StdRng) -> (String, Dfa) {
    match rng.random_range(0..7) {
        mode @ 0..=2 => {
            let pattern = if rng.random_bool(0.5) {
                let patterns = ["00", "010", "101", "0010", "0101", "10101"];
                patterns[rng.random_range(0..patterns.len())].to_owned()
            } else {
                random_word(rng, 2, 5)
            };
            let (verb, kind) = match mode {
                0 => ("ending in", PatternKind::Suffix),
                1 => ("containing", PatternKind::Contains),
                _ => ("avoiding", PatternKind::Avoids),
            };
            (
                format!("Binary words {verb} {pattern}"),
                pattern_dfa(&pattern, kind),
            )
        }
        3 | 4 => {
            let modulus = rng.random_range(2..=6);
            let residue = rng.random_range(0..modulus);
            let positional = rng.random_bool(0.5);
            let title = if positional {
                "Binary value"
            } else {
                "Number of 1s"
            };
            (
                format!("{title} ≡ {residue} (mod {modulus})"),
                residues(modulus, residue, positional),
            )
        }
        5 => {
            let bound = rng.random_range(1..=4);
            (
                format!("0 opens, 1 closes: balanced words of depth at most {bound}"),
                bounded_balance(bound),
            )
        }
        _ => {
            let distance = rng.random_range(1..=3);
            let bit = rng.random_range(0..=1);
            (
                format!("Bit {distance} from the right is {bit}"),
                nth_from_end(distance, bit).0,
            )
        }
    }
}

fn table(text: &mut String, dfa: &Dfa, labels: Option<&[String]>) {
    let name = |q: usize| labels.map_or_else(|| format!("q{q}"), |names| names[q].clone());
    writeln!(text, "Start {}", name(dfa.start)).unwrap();
    let finals = dfa
        .final_states
        .iter()
        .enumerate()
        .filter(|(_, f)| **f)
        .map(|(q, _)| name(q))
        .collect::<Vec<_>>()
        .join(", ");
    writeln!(text, "Accept {{{finals}}}").unwrap();
    writeln!(text, "state          0              1").unwrap();
    for (q, row) in dfa.next.iter().enumerate() {
        writeln!(
            text,
            "{:<14} {:<14} {}",
            name(q),
            name(row[0]),
            name(row[1])
        )
        .unwrap();
    }
}

fn witnesses(text: &mut String, dfa: &Dfa) {
    for accepted in [true, false] {
        match shortest(dfa, accepted) {
            Some(word) => writeln!(
                text,
                "Shortest {}: {}",
                verdict(accepted),
                word_label(&word)
            )
            .unwrap(),
            None => writeln!(
                text,
                "No {}ing word",
                if accepted { "accept" } else { "reject" }
            )
            .unwrap(),
        }
    }
}

fn trace(text: &mut String, dfa: &Dfa, word: &str, name: &str) {
    write!(text, "{name} {}: q{}", word_label(word), dfa.start).unwrap();
    let mut q = dfa.start;
    for byte in word.bytes() {
        q = dfa.next[q][usize::from(byte - b'0')];
        write!(text, " -{}→ q{q}", char::from(byte)).unwrap();
    }
    writeln!(text, "  {}", verdict(dfa.final_states[q])).unwrap();
}

pub fn generate(rng: &mut StdRng) -> String {
    let mut text = String::new();
    match rng.random_range(0..6) {
        0 => {
            let (title, dfa) = language(rng);
            writeln!(text, "{title}").unwrap();
            table(&mut text, &dfa, None);
            witnesses(&mut text, &dfa);
            let word = random_word(rng, 4, 11);
            trace(&mut text, &dfa, &word, "Trace");
        }
        1 => {
            let (title, base) = language(rng);
            let dfa = inflated(&base);
            let (minimal, groups) = minimize(&dfa);
            writeln!(text, "{title}\nBefore minimization").unwrap();
            table(&mut text, &dfa, None);
            let unreachable = dfa
                .reachable()
                .iter()
                .enumerate()
                .filter(|(_, seen)| !**seen)
                .map(|(q, _)| format!("q{q}"))
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(text, "Unreachable {{{unreachable}}}").unwrap();
            for (index, group) in groups.iter().enumerate() {
                writeln!(
                    text,
                    "m{index} = {{{}}}",
                    group
                        .iter()
                        .map(|q| format!("q{q}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
                .unwrap();
            }
            writeln!(
                text,
                "{} states → {} reachable minimal states",
                dfa.next.len(),
                minimal.next.len()
            )
            .unwrap();
            let labels = (0..minimal.next.len())
                .map(|q| format!("m{q}"))
                .collect::<Vec<_>>();
            table(&mut text, &minimal, Some(&labels));
        }
        2 => {
            let (title_a, a, title_b, b) = if rng.random_bool(0.45) {
                let m = rng.random_range(2..=3);
                let n = m * 2;
                let r = rng.random_range(0..m);
                let s = rng.random_range(0..n);
                (
                    format!("Binary value ≡ {r} (mod {m})"),
                    residues(m, r, true),
                    format!("Binary value ≡ {s} (mod {n})"),
                    residues(n, s, true),
                )
            } else {
                let patterns = ["00", "01", "10", "11", "010", "101"];
                let p = patterns[rng.random_range(0..patterns.len())];
                let parity = rng.random_range(0..2);
                (
                    format!("Words containing {p}"),
                    pattern_dfa(p, PatternKind::Contains),
                    format!("Number of 1s ≡ {parity} (mod 2)"),
                    residues(2, parity, false),
                )
            };
            let operation = [
                Operation::And,
                Operation::Or,
                Operation::Xor,
                Operation::Difference,
            ][rng.random_range(0..4)];
            let (combined, pairs) = product(&a, &b, operation);
            writeln!(text, "A = {title_a}\nB = {title_b}\n{}", operation.label()).unwrap();
            let labels = pairs
                .iter()
                .map(|(p, q)| format!("({p},{q})"))
                .collect::<Vec<_>>();
            table(&mut text, &combined, Some(&labels));
            writeln!(
                text,
                "{} reachable pairs of {}\n{} minimal states",
                pairs.len(),
                a.next.len() * b.next.len(),
                minimize(&combined).0.next.len()
            )
            .unwrap();
            witnesses(&mut text, &combined);
        }
        3 => {
            let (title, a) = language(rng);
            let mut b = a.clone();
            let change = match rng.random_range(0..3) {
                0 => {
                    b = inflated(&a);
                    "state expansion".to_owned()
                }
                1 => {
                    let q = rng.random_range(0..b.next.len());
                    b.final_states[q] = !b.final_states[q];
                    format!("acceptance of q{q} flipped")
                }
                _ => {
                    let q = rng.random_range(0..b.next.len());
                    let symbol = rng.random_range(0..2);
                    let old = b.next[q][symbol];
                    let new = (old + rng.random_range(1..b.next.len())) % b.next.len();
                    b.next[q][symbol] = new;
                    format!("q{q} on {symbol}: q{old} → q{new}")
                }
            };
            writeln!(text, "A: {title}").unwrap();
            table(&mut text, &a, None);
            writeln!(text, "B: {change}").unwrap();
            table(&mut text, &b, None);
            if let Some(word) = distinguish(&a, a.start, &b, b.start) {
                writeln!(text, "Shortest distinguishing word {}", word_label(&word)).unwrap();
                trace(&mut text, &a, &word, "A");
                trace(&mut text, &b, &word, "B");
            } else {
                writeln!(text, "A and B accept the same language").unwrap();
            }
        }
        4 => {
            let stem = random_word(rng, 0, 2);
            let tail = random_word(rng, 1, 3);
            let mut words = vec![
                format!("{stem}0{tail}"),
                format!("{stem}1{tail}"),
                format!("{stem}{tail}"),
            ];
            if rng.random_bool(0.5) {
                words.push(stem);
            }
            if rng.random_bool(0.5) {
                words.push(random_word(rng, 1, 5));
            }
            words.sort();
            words.dedup();
            let trie = codebook(&words);
            let (minimal, _) = minimize(&trie);
            writeln!(
                text,
                "Exactly {{{}}}",
                words
                    .iter()
                    .map(|word| word_label(word))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .unwrap();
            writeln!(
                text,
                "{} trie states → {} minimal states",
                trie.next.len(),
                minimal.next.len()
            )
            .unwrap();
            table(&mut text, &minimal, None);
            let word = words[rng.random_range(0..words.len())].clone();
            trace(&mut text, &minimal, &word, "Trace");
            trace(&mut text, &minimal, &format!("{word}0"), "Extension");
        }
        _ => {
            let distance = rng.random_range(2..=4);
            let bit = rng.random_range(0..=1);
            let (dfa, subsets) = nth_from_end(distance, bit);
            writeln!(
                text,
                "Bit {distance} from the right is {bit}\nNFA start 0, accept {distance}"
            )
            .unwrap();
            writeln!(
                text,
                "0 on 0 → {{0{}}}\n0 on 1 → {{0{}}}",
                if bit == 0 { ",1" } else { "" },
                if bit == 1 { ",1" } else { "" }
            )
            .unwrap();
            for q in 1..distance {
                writeln!(text, "{q} on 0 or 1 → {{{}}}", q + 1).unwrap();
            }
            writeln!(text, "{distance} on 0 or 1 → {{}}\nReachable subsets").unwrap();
            let labels = subsets
                .iter()
                .map(|&subset| {
                    format!(
                        "{{{}}}",
                        (0..=distance)
                            .filter(|&q| subset & (1 << q) != 0)
                            .map(|q| q.to_string())
                            .collect::<Vec<_>>()
                            .join(",")
                    )
                })
                .collect::<Vec<_>>();
            table(&mut text, &dfa, Some(&labels));
            let word = random_word(rng, distance - 1, distance + 5);
            writeln!(
                text,
                "{}  {}",
                word_label(&word),
                verdict(dfa.accepts(&word))
            )
            .unwrap();
            writeln!(
                text,
                "{} reachable subsets, {} minimal states",
                subsets.len(),
                minimize(&dfa).0.next.len()
            )
            .unwrap();
        }
    }
    text.trim_end().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn words(max_length: usize) -> Vec<String> {
        let mut words = vec![String::new()];
        for length in 1..=max_length {
            for value in 0..1 << length {
                words.push(format!("{value:0length$b}"));
            }
        }
        words
    }

    #[test]
    fn pattern_languages_match_string_operations() {
        for pattern in ["0", "1", "00", "010", "10101", "00100"] {
            let suffix = pattern_dfa(pattern, PatternKind::Suffix);
            let contains = pattern_dfa(pattern, PatternKind::Contains);
            let avoids = pattern_dfa(pattern, PatternKind::Avoids);
            for word in words(9) {
                assert_eq!(suffix.accepts(&word), word.ends_with(pattern));
                assert_eq!(contains.accepts(&word), word.contains(pattern));
                assert_eq!(avoids.accepts(&word), !word.contains(pattern));
            }
        }
    }

    #[test]
    fn arithmetic_and_prefix_balance_match_direct_evaluation() {
        for word in words(9) {
            let value = usize::from_str_radix(&format!("0{word}"), 2).unwrap();
            let ones = word.bytes().filter(|&byte| byte == b'1').count();
            for modulus in 2..=6 {
                for residue in 0..modulus {
                    assert_eq!(
                        residues(modulus, residue, true).accepts(&word),
                        value % modulus == residue
                    );
                    assert_eq!(
                        residues(modulus, residue, false).accepts(&word),
                        ones % modulus == residue
                    );
                }
            }
            for bound in 1..=4 {
                let mut depth = 0i32;
                let mut valid = true;
                for bit in word.bytes() {
                    depth += if bit == b'0' { 1 } else { -1 };
                    valid &= (0..=bound as i32).contains(&depth);
                }
                assert_eq!(bounded_balance(bound).accepts(&word), valid && depth == 0);
            }
        }
    }

    #[test]
    fn subset_construction_matches_indexed_bits_and_is_minimal() {
        for distance in 1..=4 {
            for bit in 0..2 {
                let (dfa, subsets) = nth_from_end(distance, bit);
                assert_eq!(subsets.len(), 1 << distance);
                assert_eq!(minimize(&dfa).0.next.len(), 1 << distance);
                for word in words(9) {
                    let expected = word.len() >= distance
                        && usize::from(word.as_bytes()[word.len() - distance] - b'0') == bit;
                    assert_eq!(dfa.accepts(&word), expected);
                }
            }
        }
    }

    #[test]
    fn minimization_agrees_with_independent_pair_search() {
        let mut rng = StdRng::seed_from_u64(91);
        for _ in 0..120 {
            let (_, original) = language(&mut rng);
            let expanded = inflated(&original);
            let (minimal, groups) = minimize(&expanded);
            assert!(distinguish(&expanded, expanded.start, &minimal, minimal.start).is_none());
            let reached = expanded.reachable();
            for p in 0..expanded.next.len() {
                for q in p + 1..expanded.next.len() {
                    if reached[p] && reached[q] {
                        let merged = groups
                            .iter()
                            .any(|group| group.contains(&p) && group.contains(&q));
                        assert_eq!(merged, distinguish(&expanded, p, &expanded, q).is_none());
                    }
                }
            }
            for p in 0..minimal.next.len() {
                for q in p + 1..minimal.next.len() {
                    assert!(distinguish(&minimal, p, &minimal, q).is_some());
                }
            }
            assert!(!reached[expanded.next.len() - 1]);
        }
    }

    #[test]
    fn product_closure_and_shortest_witnesses() {
        let a = pattern_dfa("010", PatternKind::Contains);
        let b = residues(3, 1, false);
        for operation in [
            Operation::And,
            Operation::Or,
            Operation::Xor,
            Operation::Difference,
        ] {
            let combined = product(&a, &b, operation).0;
            for word in words(8) {
                assert_eq!(
                    combined.accepts(&word),
                    operation.apply(a.accepts(&word), b.accepts(&word))
                );
            }
            for accepted in [true, false] {
                if let Some(witness) = shortest(&combined, accepted) {
                    assert_eq!(combined.accepts(&witness), accepted);
                    for shorter in words(witness.len().saturating_sub(1)) {
                        if shorter.len() < witness.len() {
                            assert_ne!(combined.accepts(&shorter), accepted);
                        }
                    }
                }
            }
        }
        let empty = product(&residues(2, 0, true), &residues(4, 1, true), Operation::And).0;
        assert!(shortest(&empty, true).is_none());
        assert_eq!(minimize(&empty).0.next.len(), 1);
    }

    #[test]
    fn finite_language_matches_exact_membership() {
        let language = ["", "0", "010", "110", "10110"].map(str::to_owned);
        let trie = codebook(&language);
        let minimal = minimize(&trie).0;
        assert!(minimal.next.len() < trie.next.len());
        for word in words(8) {
            assert_eq!(minimal.accepts(&word), language.contains(&word));
        }
    }

    #[test]
    fn witnesses_are_shortest_even_when_difference_is_epsilon() {
        let a = pattern_dfa("0101", PatternKind::Suffix);
        let b = pattern_dfa("0101", PatternKind::Contains);
        let witness = distinguish(&a, a.start, &b, b.start).unwrap();
        assert_ne!(a.accepts(&witness), b.accepts(&witness));
        for word in words(witness.len() - 1) {
            assert_eq!(a.accepts(&word), b.accepts(&word));
        }
        let mut complement = a.clone();
        complement.final_states.iter_mut().for_each(|f| *f = !*f);
        assert_eq!(
            distinguish(&a, a.start, &complement, complement.start),
            Some(String::new())
        );
        assert!(distinguish(&a, a.start, &inflated(&a), 0).is_none());
    }
}
