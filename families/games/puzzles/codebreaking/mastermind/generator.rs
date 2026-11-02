use rand::{Rng, rngs::StdRng, seq::SliceRandom};

type Code = [u8; 4];
type Feedback = (u8, u8);

fn codes() -> Vec<Code> {
    (0..1296)
        .map(|mut n| {
            std::array::from_fn(|_| {
                let digit = 1 + (n % 6) as u8;
                n /= 6;
                digit
            })
        })
        .collect()
}

fn score(secret: &Code, guess: &Code) -> Feedback {
    let exact = secret.iter().zip(guess).filter(|(a, b)| a == b).count() as u8;
    let mut a = [0u8; 7];
    let mut b = [0u8; 7];
    for (&s, &g) in secret.iter().zip(guess) {
        a[s as usize] += 1;
        b[g as usize] += 1;
    }
    let matched = a.iter().zip(b).map(|(&s, g)| s.min(g)).sum::<u8>();
    (exact, matched - exact)
}

fn solutions(all: &[Code], clues: &[(Code, Feedback)]) -> Vec<Code> {
    all.iter()
        .copied()
        .filter(|code| {
            clues
                .iter()
                .all(|(guess, feedback)| score(code, guess) == *feedback)
        })
        .collect()
}

fn resolving_probes(secret: &Code) -> [(Code, Feedback); 4] {
    std::array::from_fn(|position| {
        let mut guess = *secret;
        guess[position] = secret[position] % 6 + 1;
        (guess, score(secret, &guess))
    })
}

fn puzzle(rng: &mut StdRng) -> (Code, Vec<(Code, Feedback)>) {
    let all = codes();
    let secret = all[rng.random_range(0..all.len())];
    let mut remaining = all.clone();
    let mut clues = Vec::new();
    while remaining.len() > 1 && clues.len() < 8 {
        let mut probes = all.clone();
        probes.shuffle(rng);
        probes.truncate(48);
        // This non-answer probe guarantees at least one eliminated candidate.
        probes.push(*remaining.iter().find(|&&code| code != secret).unwrap());
        let mut best = None;
        for guess in probes {
            if guess == secret {
                continue;
            }
            let feedback = score(&secret, &guess);
            let survivors: Vec<_> = remaining
                .iter()
                .copied()
                .filter(|code| score(code, &guess) == feedback)
                .collect();
            if best
                .as_ref()
                .is_none_or(|(_, _, prior): &(Code, Feedback, Vec<Code>)| {
                    survivors.len() < prior.len()
                })
            {
                best = Some((guess, feedback, survivors));
            }
        }
        let (guess, feedback, survivors) = best.unwrap();
        clues.push((guess, feedback));
        remaining = survivors;
    }
    if remaining.len() > 1 {
        // Four probes each differ from the answer at one different position.
        // Their exact-match counts uniquely fix the answer without printing it.
        clues.extend(resolving_probes(&secret));
    }
    // A clue survives only if removing it would allow a second answer.
    clues.shuffle(rng);
    let mut i = 0;
    while i < clues.len() {
        let removed = clues.remove(i);
        if solutions(&all, &clues).len() != 1 {
            clues.insert(i, removed);
            i += 1;
        }
    }
    (secret, clues)
}

pub fn generate(rng: &mut StdRng) -> String {
    let (_, clues) = puzzle(rng);
    let mut text = String::from("Mastermind\nSymbols: 1 2 3 4 5 6\nGuess  Exact  Misplaced");
    for (guess, (exact, misplaced)) in clues {
        let digits: String = guess.into_iter().map(|n| char::from(b'0' + n)).collect();
        text.push_str(&format!("\n{digits}   {exact}      {misplaced}"));
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn duplicate_symbols_are_not_double_counted() {
        assert_eq!(score(&[1, 1, 2, 2], &[1, 2, 1, 1]), (1, 2));
        assert_eq!(score(&[1, 2, 3, 4], &[4, 3, 2, 1]), (0, 4));
        assert_eq!(score(&[1, 1, 1, 1], &[1, 2, 3, 4]), (1, 0));
        assert_eq!(codes().len(), 1296);
    }

    #[test]
    fn four_nonanswer_probes_resolve_every_code() {
        let all = codes();
        for secret in &all {
            let clues = resolving_probes(secret);
            assert!(clues.iter().all(|(guess, _)| guess != secret));
            assert_eq!(solutions(&all, &clues), [*secret]);
        }
    }

    #[test]
    fn puzzles_have_one_answer_and_no_redundant_clue() {
        let all = codes();
        let mut rng = StdRng::seed_from_u64(5811);
        let mut distinct = std::collections::HashSet::new();
        for _ in 0..100 {
            let (secret, clues) = puzzle(&mut rng);
            assert!(clues.len() <= 12);
            assert_eq!(solutions(&all, &clues), [secret]);
            assert!(!clues.iter().any(|(guess, _)| *guess == secret));
            for i in 0..clues.len() {
                let mut reduced = clues.clone();
                reduced.remove(i);
                assert!(solutions(&all, &reduced).len() > 1);
            }
            distinct.insert(secret);
        }
        assert!(distinct.len() > 90);
    }
}
