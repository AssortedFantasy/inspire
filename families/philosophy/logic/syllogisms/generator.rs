use rand::{Rng, rngs::StdRng};

#[derive(Clone, Copy, Debug)]
struct Form {
    mood: [u8; 3],
    figure: usize,
}

// Term bits are S=1, P=2, M=4. A model is a nonempty set of occupied
// membership regions, with one representative per region.
fn statement(kind: u8, subject: u8, predicate: u8, model: u16) -> bool {
    let mut overlap = false;
    let mut outside = false;
    for region in 0..8 {
        if model & (1 << region) != 0 && region & subject != 0 {
            overlap |= region & predicate != 0;
            outside |= region & predicate == 0;
        }
    }
    match kind {
        b'A' => !outside,
        b'E' => !overlap,
        b'I' => overlap,
        b'O' => outside,
        _ => unreachable!(),
    }
}

impl Form {
    fn terms(self) -> [(u8, u8); 3] {
        let premises = match self.figure {
            1 => [(4, 2), (1, 4)],
            2 => [(2, 4), (1, 4)],
            3 => [(4, 2), (4, 1)],
            4 => [(2, 4), (4, 1)],
            _ => unreachable!(),
        };
        [premises[0], premises[1], (1, 2)]
    }

    fn countermodels(self) -> Vec<u16> {
        let terms = self.terms();
        (1..256)
            .filter(|&model| {
                statement(self.mood[0], terms[0].0, terms[0].1, model)
                    && statement(self.mood[1], terms[1].0, terms[1].1, model)
                    && !statement(self.mood[2], 1, 2, model)
            })
            .collect()
    }

    fn code(self) -> String {
        format!(
            "{}{}{}–{}",
            self.mood[0] as char, self.mood[1] as char, self.mood[2] as char, self.figure
        )
    }
}

fn inhabited(model: u16) -> u8 {
    (0..8)
        .filter(|region| model & (1 << region) != 0)
        .fold(0, |terms, region| terms | region as u8)
}

fn forms() -> impl Iterator<Item = Form> {
    (1..=4).flat_map(|figure| {
        (0..64).map(move |code| Form {
            mood: [b"AEIO"[code / 16], b"AEIO"[code / 4 % 4], b"AEIO"[code % 4]],
            figure,
        })
    })
}

fn regime(countermodels: &[u16]) -> usize {
    if countermodels.is_empty() {
        0
    } else if countermodels.iter().all(|&model| inhabited(model) != 7) {
        1
    } else {
        2
    }
}

fn term_name(term: u8) -> &'static str {
    match term {
        1 => "S",
        2 => "P",
        4 => "M",
        _ => unreachable!(),
    }
}

fn sentence(kind: u8, subject: u8, predicate: u8) -> String {
    let (s, p) = (term_name(subject), term_name(predicate));
    match kind {
        b'A' => format!("All {s} are {p}."),
        b'E' => format!("No {s} are {p}."),
        b'I' => format!("Some {s} are {p}."),
        b'O' => format!("Some {s} are not {p}."),
        _ => unreachable!(),
    }
}

fn set(items: impl Iterator<Item = usize>) -> String {
    let labels = items.map(|i| format!("a{i}")).collect::<Vec<_>>();
    if labels.is_empty() {
        "∅".into()
    } else {
        format!("{{{}}}", labels.join(", "))
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let target = rng.random_range(0..3);
    let candidates = forms()
        .filter_map(|form| {
            let countermodels = form.countermodels();
            (regime(&countermodels) == target).then_some((form, countermodels))
        })
        .collect::<Vec<_>>();
    let (form, countermodels) = &candidates[rng.random_range(0..candidates.len())];
    let terms = form.terms();
    let mut lines = (0..3)
        .map(|i| {
            let text = sentence(form.mood[i], terms[i].0, terms[i].1);
            if i == 2 { format!("∴ {text}") } else { text }
        })
        .collect::<Vec<_>>();
    lines.push(form.code());
    match target {
        0 => lines.push("Valid with empty term extensions allowed.".into()),
        1 => {
            let assumptions = (1u8..8)
                .filter(|&assumption| {
                    countermodels
                        .iter()
                        .all(|&model| inhabited(model) & assumption != assumption)
                })
                .collect::<Vec<_>>();
            let minimal = assumptions
                .iter()
                .filter(|&&a| !assumptions.iter().any(|&b| b != a && b & a == b))
                .map(|&a| {
                    [1, 2, 4]
                        .into_iter()
                        .filter(|&term| a & term != 0)
                        .map(|term| format!("{} ≠ ∅", term_name(term)))
                        .collect::<Vec<_>>()
                        .join(" and ")
                })
                .collect::<Vec<_>>()
                .join(" or ");
            lines.push(format!("Valid assuming {minimal}."));
            lines.push("Invalid with empty term extensions allowed.".into());
        }
        _ => lines.push("Invalid even assuming S, P and M are all nonempty.".into()),
    }
    if target != 0 {
        let eligible = countermodels
            .iter()
            .copied()
            .filter(|&model| target != 2 || inhabited(model) == 7)
            .collect::<Vec<_>>();
        let size = eligible
            .iter()
            .map(|model| model.count_ones())
            .min()
            .unwrap();
        let smallest = eligible
            .iter()
            .copied()
            .filter(|model| model.count_ones() == size)
            .collect::<Vec<_>>();
        let model = smallest[rng.random_range(0..smallest.len())];
        let regions = (0..8)
            .filter(|region| model & (1 << region) != 0)
            .collect::<Vec<_>>();
        lines.push("Countermodel".into());
        lines.push(format!("D = {}", set(0..regions.len())));
        for term in [1, 2, 4] {
            lines.push(format!(
                "{} = {}",
                term_name(term),
                set((0..regions.len()).filter(|&i| regions[i] & term != 0))
            ));
        }
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Independent individual-based semantics; unlike region occupancy this
    // permits duplicate membership types and explicitly quantifies objects.
    fn individual_truth(kind: u8, s: u8, p: u8, domain: u8) -> bool {
        match kind {
            b'A' => (0..domain).all(|x| s & (1 << x) == 0 || p & (1 << x) != 0),
            b'E' => (0..domain).all(|x| s & p & (1 << x) == 0),
            b'I' => (0..domain).any(|x| s & p & (1 << x) != 0),
            b'O' => (0..domain).any(|x| s & (1 << x) != 0 && p & (1 << x) == 0),
            _ => unreachable!(),
        }
    }

    #[test]
    fn all_256_forms_match_exhaustive_domains_of_up_to_three_objects() {
        let mut counts = [0; 3];
        for form in forms() {
            let countermodels = form.countermodels();
            counts[regime(&countermodels)] += 1;
            let mut found = [false; 8];
            for domain in 1..=3 {
                for s in 0..1 << domain {
                    for p in 0..1 << domain {
                        for m in 0..1 << domain {
                            let extension = |term| match term {
                                1 => s,
                                2 => p,
                                4 => m,
                                _ => unreachable!(),
                            };
                            let truths = form.terms().map(|(a, b)| (extension(a), extension(b)));
                            if individual_truth(form.mood[0], truths[0].0, truths[0].1, domain)
                                && individual_truth(form.mood[1], truths[1].0, truths[1].1, domain)
                                && !individual_truth(form.mood[2], s, p, domain)
                            {
                                let nonempty = u8::from(s != 0)
                                    | (u8::from(p != 0) << 1)
                                    | (u8::from(m != 0) << 2);
                                for assumption in 0..8 {
                                    found[assumption as usize] |=
                                        nonempty & assumption == assumption;
                                }
                            }
                        }
                    }
                }
            }
            for assumption in 0..8 {
                assert_eq!(
                    found[assumption as usize],
                    countermodels
                        .iter()
                        .any(|&model| inhabited(model) & assumption == assumption),
                    "{} with nonempty mask {assumption}",
                    form.code()
                );
            }
        }
        assert_eq!(counts, [15, 9, 232]);
    }

    #[test]
    fn familiar_boundary_cases_have_the_expected_import() {
        for (mood, figure, expected) in [
            (*b"AAA", 1, 0),
            (*b"EAE", 1, 0),
            (*b"AAI", 1, 1),
            (*b"AAI", 3, 1),
            (*b"AAA", 2, 2),
            (*b"AAA", 3, 2),
        ] {
            assert_eq!(regime(&Form { mood, figure }.countermodels()), expected);
        }
    }
}
