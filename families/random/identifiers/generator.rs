use rand::{Rng, rngs::StdRng};

pub fn generate(rng: &mut StdRng) -> String {
    let (kind, value) = sample(rng);
    format!("Synthetic {kind} {value}")
}

fn digits(rng: &mut StdRng, n: usize) -> Vec<u8> {
    (0..n).map(|_| rng.random_range(0..=9)).collect()
}

fn text(digits: &[u8]) -> String {
    digits.iter().map(|d| char::from(b'0' + d)).collect()
}

fn mod11(body: &[u8]) -> char {
    let sum: u32 = body
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| (i as u32 + 2) * u32::from(d))
        .sum();
    let check = (11 - sum % 11) % 11;
    if check == 10 {
        'X'
    } else {
        char::from(b'0' + check as u8)
    }
}

fn boundary_mod11(rng: &mut StdRng, body: &mut [u8]) {
    let target = match rng.random_range(0..3) {
        0 => 'X',
        1 => '0',
        _ => return,
    };
    let n = body.len();
    let mut candidates = Vec::new();
    for pair in 0..100 {
        body[n - 2] = pair / 10;
        body[n - 1] = pair % 10;
        if mod11(body) == target {
            candidates.push(pair);
        }
    }
    let pair = candidates[rng.random_range(0..candidates.len())];
    body[n - 2] = pair / 10;
    body[n - 1] = pair % 10;
}

fn ean13(body: &[u8]) -> u8 {
    let sum: u32 = body
        .iter()
        .enumerate()
        .map(|(i, &d)| u32::from(d) * if i % 2 == 0 { 1 } else { 3 })
        .sum();
    ((10 - sum % 10) % 10) as u8
}

fn luhn(body: &[u8]) -> u8 {
    let sum: u32 = body
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| {
            let d = u32::from(d) * if i % 2 == 0 { 2 } else { 1 };
            if d > 9 { d - 9 } else { d }
        })
        .sum();
    ((10 - sum % 10) % 10) as u8
}

fn remainder97(value: &str) -> u32 {
    value.bytes().fold(0, |remainder, b| {
        if b.is_ascii_digit() {
            (remainder * 10 + u32::from(b - b'0')) % 97
        } else {
            (remainder * 100 + u32::from(b - b'A' + 10)) % 97
        }
    })
}

fn german_iban(bban: &str) -> String {
    let checksum = 98 - remainder97(&format!("{bban}DE00"));
    format!("DE{checksum:02}{bban}")
}

fn sample(rng: &mut StdRng) -> (&'static str, String) {
    let groups = [0, 1, 2, 3, 4, 5, 7];
    match rng.random_range(0..6) {
        0 => {
            let mut body = digits(rng, 9);
            body[0] = groups[rng.random_range(0..groups.len())];
            boundary_mod11(rng, &mut body);
            ("ISBN-10", format!("{}{}", text(&body), mod11(&body)))
        }
        1 => {
            let mut body = vec![9, 7, 8, groups[rng.random_range(0..groups.len())]];
            body.extend(digits(rng, 8));
            ("ISBN-13", format!("{}{}", text(&body), ean13(&body)))
        }
        2 => {
            let mut body = digits(rng, 7);
            boundary_mod11(rng, &mut body);
            let value = format!("{}{}", text(&body), mod11(&body));
            ("ISSN", format!("{}-{}", &value[..4], &value[4..]))
        }
        3 => {
            let mut body = vec![9, 7, 9, 0];
            body.extend(digits(rng, 8));
            let registrant_length = match body[4] {
                0 => 3,
                1..=3 => 4,
                4..=6 => 5,
                7..=8 => 6,
                _ => 7,
            };
            let value = text(&body);
            (
                "ISMN",
                format!(
                    "979-0-{}-{}-{}",
                    &value[4..4 + registrant_length],
                    &value[4 + registrant_length..],
                    ean13(&body)
                ),
            )
        }
        4 => {
            let n = rng.random_range(7..=18);
            let mut body = digits(rng, n);
            if rng.random_bool(0.25) {
                body[0] = 0;
                body[1] = 0;
            }
            ("Luhn", format!("{}{}", text(&body), luhn(&body)))
        }
        _ => {
            let mut bban = digits(rng, 18);
            if rng.random_bool(0.5) {
                bban[8] = 0;
                bban[9] = 0;
                bban[10] = 0;
            }
            let value = german_iban(&text(&bban));
            (
                "IBAN",
                value
                    .as_bytes()
                    .chunks(4)
                    .map(|chunk| std::str::from_utf8(chunk).unwrap())
                    .collect::<Vec<_>>()
                    .join(" "),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    fn clean(value: &str) -> String {
        value.chars().filter(|c| !matches!(c, ' ' | '-')).collect()
    }

    fn valid(kind: &str, value: &str) -> bool {
        let value = clean(value);
        if kind == "IBAN" {
            if value.len() != 22
                || !value.starts_with("DE")
                || !value[2..].bytes().all(|b| b.is_ascii_digit())
            {
                return false;
            }
            let rotated = format!("{}{}", &value[4..], &value[..4]);
            // Expand letters to decimal and reduce digit by digit, separately from the producer.
            let expanded = rotated
                .chars()
                .map(|c| {
                    if c.is_ascii_digit() {
                        c.to_string()
                    } else {
                        (c as u32 - 'A' as u32 + 10).to_string()
                    }
                })
                .collect::<String>();
            return expanded
                .bytes()
                .fold(0, |r, b| (10 * r + u32::from(b - b'0')) % 97)
                == 1;
        }
        let digits: Vec<u32> = value
            .bytes()
            .map(|b| if b == b'X' { 10 } else { u32::from(b - b'0') })
            .collect();
        match kind {
            "ISBN-10" | "ISSN" => {
                let n = if kind == "ISSN" { 8 } else { 10 };
                digits.len() == n
                    && digits
                        .iter()
                        .enumerate()
                        .map(|(i, &d)| d * (n - i) as u32)
                        .sum::<u32>()
                        % 11
                        == 0
            }
            "ISBN-13" | "ISMN" => {
                digits.len() == 13
                    && digits
                        .iter()
                        .enumerate()
                        .map(|(i, &d)| d * if i % 2 == 0 { 1 } else { 3 })
                        .sum::<u32>()
                        % 10
                        == 0
            }
            _ => {
                digits
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(i, &d)| {
                        let product = d * if i % 2 == 0 { 1 } else { 2 };
                        product / 10 + product % 10
                    })
                    .sum::<u32>()
                    % 10
                    == 0
            }
        }
    }

    #[test]
    fn complete_checksums_pass_and_every_single_digit_mutation_fails() {
        let mut rng = StdRng::seed_from_u64(329);
        let mut kinds = HashSet::new();
        let mut x_checks = 0;
        for _ in 0..2000 {
            let (kind, value) = sample(&mut rng);
            kinds.insert(kind);
            assert!(valid(kind, &value), "{kind} {value}");
            x_checks += usize::from(value.ends_with('X'));
            if kind == "ISMN" {
                assert!(value.starts_with("979-0-"));
            }
            let original = clean(&value);
            for (i, b) in original.bytes().enumerate() {
                if b.is_ascii_digit() {
                    let mut changed = original.clone().into_bytes();
                    changed[i] = b'0' + (b - b'0' + 1) % 10;
                    assert!(
                        !valid(kind, std::str::from_utf8(&changed).unwrap()),
                        "{kind} {original}, changed {i}"
                    );
                }
            }
        }
        assert_eq!(kinds.len(), 6);
        assert!(x_checks > 180);
    }

    #[test]
    fn published_examples_and_boundary_targets() {
        for (kind, value) in [
            ("ISBN-10", "080442957X"),
            ("ISBN-13", "9780306406157"),
            ("ISSN", "0317-8471"),
            ("ISMN", "979-0-2600-0043-8"),
            ("Luhn", "79927398713"),
            ("IBAN", "DE89 3704 0044 0532 0130 00"),
        ] {
            assert!(valid(kind, value), "{kind} {value}");
        }
        assert_eq!(german_iban("370400440532013000"), "DE89370400440532013000");
        let mut rng = StdRng::seed_from_u64(331);
        for n in [7, 9] {
            let mut checks = HashSet::new();
            for _ in 0..500 {
                let mut body = digits(&mut rng, n);
                boundary_mod11(&mut rng, &mut body);
                checks.insert(mod11(&body));
            }
            assert!(checks.contains(&'X') && checks.contains(&'0'));
        }
    }
}
