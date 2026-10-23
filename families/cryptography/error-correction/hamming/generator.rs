use rand::{Rng, rngs::StdRng};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Verdict {
    Clean,
    Corrected(u8),
    DoubleError,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Decoded {
    syndrome: u8,
    odd: bool,
    verdict: Verdict,
    word: Option<u8>,
}

fn bit(position: u8) -> u8 {
    1 << (position - 1)
}

fn encode(data: u8, extended: bool) -> u8 {
    let mut word = 0;
    for (index, position) in [3, 5, 6, 7].into_iter().enumerate() {
        if data & (1 << (3 - index)) != 0 {
            word |= bit(position);
        }
    }
    for check in [1, 2, 4] {
        if syndrome(word) & check != 0 {
            word |= bit(check);
        }
    }
    if extended && !word.count_ones().is_multiple_of(2) {
        word |= bit(8);
    }
    word
}

fn extract(word: u8) -> u8 {
    [3, 5, 6, 7].into_iter().fold(0, |data, position| {
        (data << 1) | u8::from(word & bit(position) != 0)
    })
}

fn syndrome(word: u8) -> u8 {
    (1..=7).fold(0, |value, position| {
        value
            ^ if word & bit(position) != 0 {
                position
            } else {
                0
            }
    })
}

fn decode(word: u8, extended: bool) -> Decoded {
    let syndrome = syndrome(word);
    let odd = !word.count_ones().is_multiple_of(2);
    let verdict = if extended {
        match (syndrome, odd) {
            (0, false) => Verdict::Clean,
            (0, true) => Verdict::Corrected(8),
            (_, true) => Verdict::Corrected(syndrome),
            (_, false) => Verdict::DoubleError,
        }
    } else if syndrome == 0 {
        Verdict::Clean
    } else {
        Verdict::Corrected(syndrome)
    };
    let corrected = match verdict {
        Verdict::Clean => Some(word),
        Verdict::Corrected(position) => Some(word ^ bit(position)),
        Verdict::DoubleError => None,
    };
    Decoded {
        syndrome,
        odd,
        verdict,
        word: corrected,
    }
}

fn bits(word: u8, width: u8) -> String {
    (1..=width)
        .map(|position| if word & bit(position) == 0 { '0' } else { '1' })
        .collect()
}

fn payload(data: &[u8]) -> String {
    data.iter()
        .map(|value| char::from_digit(u32::from(*value), 16).unwrap())
        .collect()
}

fn action(decoded: Decoded, original: u8) -> String {
    let Some(word) = decoded.word else {
        return "reject".into();
    };
    let data = extract(word);
    let mut result = match decoded.verdict {
        Verdict::Clean => format!("accept {data:x}"),
        Verdict::Corrected(position) => format!("flip {position} → {data:x}"),
        Verdict::DoubleError => unreachable!(),
    };
    if data != original {
        result.push_str(" WRONG");
    }
    result
}

fn random_data(rng: &mut StdRng, count: usize) -> Vec<u8> {
    (0..count).map(|_| rng.random_range(0..16)).collect()
}

fn random_pair(rng: &mut StdRng, width: u8) -> u8 {
    let first = rng.random_range(1..=width);
    let other = rng.random_range(1..width);
    let second = if other >= first { other + 1 } else { other };
    bit(first) | bit(second)
}

fn trace(rng: &mut StdRng, mode: u8) -> String {
    let extended = mode >= 4 || rng.random_bool(0.5);
    let width = if extended { 8 } else { 7 };
    let count = rng.random_range(4..=8);
    let data = random_data(rng, count);
    let mut masks: Vec<u8> = (0..count)
        .map(|_| {
            if mode == 0 || rng.random_bool(0.45) {
                0
            } else {
                bit(rng.random_range(1..=width))
            }
        })
        .collect();
    if mode != 0 {
        let focus = rng.random_range(0..count);
        masks[focus] = match mode {
            1 => bit([1, 2, 4][rng.random_range(0..3)]),
            2 => bit([3, 5, 6, 7][rng.random_range(0..4)]),
            3 => bit(rng.random_range(1..=width)),
            4 => bit(8),
            5 => bit(8) | bit(rng.random_range(1..=7)),
            _ => random_pair(rng, 7),
        };
        masks[(focus + 1) % count] = 0;
    }
    let mut output = format!(
        "Hamming({width},4), even parity\nbit positions {}\npayload {}\n\nword  received  syndrome{}  decoder",
        if extended { "12345678" } else { "1234567" },
        payload(&data),
        if extended { "  parity" } else { "" },
    );
    for (index, (&value, &mask)) in data.iter().zip(&masks).enumerate() {
        let received = encode(value, extended) ^ mask;
        let decoded = decode(received, extended);
        let parity = if extended {
            format!("       {:4}", if decoded.odd { "odd" } else { "even" })
        } else {
            String::new()
        };
        output.push_str(&format!(
            "\n{:>4}  {:<8}  {:03b}{parity}    {}",
            index + 1,
            bits(received, width),
            decoded.syndrome,
            action(decoded, value),
        ));
    }
    output
}

fn comparison(rng: &mut StdRng) -> String {
    let count = rng.random_range(4..=7);
    let data = random_data(rng, count);
    let focus = rng.random_range(0..count);
    let mut output = format!(
        "Hamming(7,4) / extended Hamming(8,4), even parity\nbit positions 12345678\npayload {}\n\nword  received  syndrome  Hamming(7,4)       Hamming(8,4)",
        payload(&data),
    );
    for (index, &value) in data.iter().enumerate() {
        let mask = if index == focus || rng.random_bool(0.45) {
            random_pair(rng, 7)
        } else if rng.random_bool(0.5) {
            bit(rng.random_range(1..=7))
        } else {
            0
        };
        let received = encode(value, true) ^ mask;
        let standard = decode(received & 0x7f, false);
        let extended = decode(received, true);
        output.push_str(&format!(
            "\n{:>4}  {}  {:03b}       {:<19}{}",
            index + 1,
            bits(received, 8),
            standard.syndrome,
            action(standard, value),
            action(extended, value),
        ));
    }
    output
}

fn burst_masks(lanes: usize, start: usize, length: usize, interleaved: bool) -> Vec<u8> {
    let mut masks = vec![0; lanes];
    for index in start..start + length {
        let (lane, position) = if interleaved {
            (index % lanes, index / lanes)
        } else {
            (index / 8, index % 8)
        };
        masks[lane] |= bit(position as u8 + 1);
    }
    masks
}

fn burst(rng: &mut StdRng) -> String {
    let count = rng.random_range(4..=7);
    let length = count - 1 + rng.random_range(0..=2);
    let start = rng.random_range(0..=8 * count - length);
    let data = random_data(rng, count);
    let serial = burst_masks(count, start, length, false);
    let interleaved = burst_masks(count, start, length, true);
    let mut output = format!(
        "Hamming(8,4), even parity, {count}-lane column interleave\nbit positions 12345678\npayload {}\nburst at stream bits {}..{}\n\nlane  row-major received / decoder                interleaved received / decoder",
        payload(&data),
        start + 1,
        start + length,
    );
    for (index, &value) in data.iter().enumerate() {
        let sent = encode(value, true);
        let row_received = sent ^ serial[index];
        let column_received = sent ^ interleaved[index];
        let row_result = format!(
            "{}  {}",
            bits(row_received, 8),
            action(decode(row_received, true), value)
        );
        let column_result = format!(
            "{}  {}",
            bits(column_received, 8),
            action(decode(column_received, true), value)
        );
        output.push_str(&format!(
            "\n{:>4}  {row_result:<44}{column_result}",
            index + 1
        ));
    }
    output
}

pub fn generate(rng: &mut StdRng) -> String {
    match rng.random_range(0..12) {
        mode @ 0..=6 => trace(rng, mode),
        7..=8 => comparison(rng),
        _ => burst(rng),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn codebook(extended: bool) -> Vec<u8> {
        // Independent parity constraints, rather than the encoder's construction.
        (0..=if extended { 255 } else { 127 })
            .filter(|word: &u8| {
                [0b0101_0101, 0b0110_0110, 0b0111_1000]
                    .iter()
                    .all(|mask| (word & mask).count_ones().is_multiple_of(2))
                    && (!extended || word.count_ones().is_multiple_of(2))
            })
            .collect()
    }

    #[test]
    fn exhaustive_codebooks_and_minimum_distances() {
        for (extended, expected_distance) in [(false, 3), (true, 4)] {
            let words = codebook(extended);
            assert_eq!(words.len(), 16);
            let mut minimum = 8;
            for data in 0..16 {
                let encoded = encode(data, extended);
                assert!(words.contains(&encoded));
                assert_eq!(extract(encoded), data);
                for &other in &words {
                    if other != encoded {
                        minimum = minimum.min((other ^ encoded).count_ones());
                    }
                }
            }
            assert_eq!(minimum, expected_distance);
        }
    }

    #[test]
    fn every_received_word_agrees_with_nearest_codewords() {
        for extended in [false, true] {
            let words = codebook(extended);
            for received in 0..=if extended { 255 } else { 127 } {
                let distance = words
                    .iter()
                    .map(|word| (word ^ received).count_ones())
                    .min()
                    .unwrap();
                let nearest: Vec<_> = words
                    .iter()
                    .copied()
                    .filter(|word| (word ^ received).count_ones() == distance)
                    .collect();
                let decoded = decode(received, extended);
                if distance < 2 {
                    assert_eq!(nearest.len(), 1);
                    assert_eq!(decoded.word, Some(nearest[0]));
                    assert_eq!(decoded.verdict == Verdict::Clean, distance == 0);
                } else {
                    assert!(extended);
                    assert_eq!(distance, 2);
                    assert_eq!(nearest.len(), 4);
                    assert_eq!(decoded.verdict, Verdict::DoubleError);
                    assert_eq!(decoded.word, None);
                }
            }
        }
    }

    #[test]
    fn all_single_errors_correct_including_overall_parity() {
        for extended in [false, true] {
            for data in 0..16 {
                let encoded = encode(data, extended);
                assert_eq!(decode(encoded, extended).verdict, Verdict::Clean);
                for position in 1..=if extended { 8 } else { 7 } {
                    let decoded = decode(encoded ^ bit(position), extended);
                    assert_eq!(decoded.word, Some(encoded));
                    assert_eq!(decoded.verdict, Verdict::Corrected(position));
                    assert_eq!(decoded.syndrome, if position == 8 { 0 } else { position });
                    if extended {
                        assert!(decoded.odd);
                    }
                }
            }
        }
    }

    #[test]
    fn every_double_error_is_detected_or_miscorrected_as_shown() {
        for data in 0..16 {
            for first in 1..=8 {
                for second in first + 1..=8 {
                    let mask = bit(first) | bit(second);
                    let encoded = encode(data, true);
                    let decoded = decode(encoded ^ mask, true);
                    assert_eq!(decoded.verdict, Verdict::DoubleError);
                    assert_eq!(decoded.word, None);
                    assert!(!decoded.odd);
                    if second <= 7 {
                        let standard = encode(data, false);
                        let decoded = decode(standard ^ mask, false);
                        let wrong = decoded.word.unwrap();
                        assert_eq!(decoded.verdict, Verdict::Corrected(first ^ second));
                        assert_eq!((wrong ^ standard).count_ones(), 3);
                        assert_ne!(extract(wrong), data);
                        assert_eq!(wrong, encode(extract(wrong), false));
                    }
                }
            }
        }
    }

    #[test]
    fn interleave_burst_boundary_is_exact_for_every_alignment() {
        for lanes in 4..=7 {
            for length in 1..=lanes + 1 {
                for start in 0..=8 * lanes - length {
                    let masks = burst_masks(lanes, start, length, true);
                    assert_eq!(
                        masks.iter().map(|mask| mask.count_ones()).sum::<u32>(),
                        length as u32
                    );
                    for data in 0..16 {
                        let sent = encode(data, true);
                        for &mask in &masks {
                            let decoded = decode(sent ^ mask, true);
                            if mask.count_ones() <= 1 {
                                assert_eq!(decoded.word, Some(sent));
                            } else {
                                assert_eq!(decoded.verdict, Verdict::DoubleError);
                            }
                        }
                    }
                    if length <= lanes {
                        assert!(masks.iter().all(|mask| mask.count_ones() <= 1));
                    } else {
                        assert_eq!(
                            masks.iter().filter(|mask| mask.count_ones() == 2).count(),
                            1
                        );
                    }
                }
            }
        }
    }
}
