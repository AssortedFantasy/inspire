use rand::{Rng, rngs::StdRng};

// The key signature supplies accidentals; each pitch retains its diatonic letter.
const KEYS: [(&str, i8); 10] = [
    ("C", 0),
    ("G", 4),
    ("D", 1),
    ("A", 5),
    ("E", 2),
    ("F", 3),
    ("Bb", 6),
    ("Eb", 2),
    ("Ab", 5),
    ("B", 6),
];
const MODES: [(&str, usize); 5] = [("", 0), ("dor", 1), ("mix", 4), ("min", 5), ("lyd", 3)];
const SIMPLE: [&[u8]; 8] = [
    &[2, 2],
    &[1, 1, 2],
    &[2, 1, 1],
    &[1, 2, 1],
    &[3, 1],
    &[1, 3],
    &[1, 1, 1, 1],
    &[4],
];
const COMPOUND: [&[u8]; 6] = [
    &[3, 3],
    &[2, 1, 3],
    &[3, 1, 2],
    &[1, 2, 2, 1],
    &[1, 1, 1, 3],
    &[6],
];

#[derive(Clone, Debug)]
struct Event {
    degree: Option<i8>,
    duration: u8,
}

type Bar = Vec<Event>;

#[derive(Clone, Copy)]
struct Meter {
    numerator: u8,
    denominator: u8,
    units: u8,
}

const METERS: [Meter; 5] = [
    Meter {
        numerator: 2,
        denominator: 4,
        units: 4,
    },
    Meter {
        numerator: 3,
        denominator: 4,
        units: 6,
    },
    Meter {
        numerator: 4,
        denominator: 4,
        units: 8,
    },
    Meter {
        numerator: 6,
        denominator: 8,
        units: 6,
    },
    Meter {
        numerator: 9,
        denominator: 8,
        units: 9,
    },
];

fn rhythm(rng: &mut StdRng, meter: Meter) -> Vec<u8> {
    let mut result = Vec::new();
    let mut remaining = meter.units;
    while remaining > 0 {
        let cells = if meter.denominator == 8 {
            &COMPOUND[..]
        } else {
            &SIMPLE[..]
        };
        let choices: Vec<_> = cells
            .iter()
            .filter(|c| c.iter().sum::<u8>() <= remaining)
            .collect();
        // A final quarter or dotted quarter also lets odd-length bars breathe.
        if choices.is_empty() || remaining <= 3 {
            if remaining > 1 && rng.random_bool(0.4) {
                result.extend(std::iter::repeat_n(1, remaining as usize));
            } else {
                result.push(remaining);
            }
            break;
        }
        let cell = choices[rng.random_range(0..choices.len())];
        result.extend_from_slice(cell);
        remaining -= cell.iter().sum::<u8>();
    }
    result
}

fn step(rng: &mut StdRng, previous: i8, direction: i8, last_step: i8) -> i8 {
    let interval = if last_step.abs() >= 3 {
        -last_step.signum()
    } else {
        match rng.random_range(0..12) {
            0 => 0,
            1..=5 => direction,
            6..=8 => -direction,
            9..=10 => 2 * direction,
            _ => 3 * direction,
        }
    };
    // Reflect at the range boundary rather than accumulating repeated high notes.
    if (-4..=10).contains(&(previous + interval)) {
        interval
    } else {
        -interval
    }
}

fn motif(rng: &mut StdRng, meter: Meter) -> Vec<Bar> {
    let shape = rng.random_range(0..4);
    let mut previous = [0, 2, 4][rng.random_range(0..3)];
    let mut last_step = 0;
    let mut result = Vec::new();
    for b in 0..2 {
        let durations = rhythm(rng, meter);
        let count = durations.len();
        let mut bar = Vec::new();
        for (i, duration) in durations.into_iter().enumerate() {
            let direction = match shape {
                0 => {
                    if b == 0 {
                        1
                    } else {
                        -1
                    }
                }
                1 => -1,
                2 => 1,
                _ => {
                    if (b * count + i) % 4 < 2 {
                        1
                    } else {
                        -1
                    }
                }
            };
            if b != 0 || i != 0 {
                last_step = step(rng, previous, direction, last_step);
                previous += last_step;
            }
            let rest = i > 0 && i + 1 == count && duration <= 2 && rng.random_bool(0.18);
            bar.push(Event {
                degree: (!rest).then_some(previous),
                duration,
            });
        }
        result.push(bar);
    }
    result
}

fn nearest_tonic(degree: i8) -> i8 {
    [0_i8, 7]
        .into_iter()
        .min_by_key(|t| t.abs_diff(degree))
        .unwrap()
}

fn cadence(bar: &mut Bar, final_close: bool) {
    if final_close && bar.len() >= 3 && bar.last().unwrap().duration == 1 {
        let tail = bar.pop().unwrap();
        bar.last_mut().unwrap().duration += tail.duration;
    }
    let last = bar.len() - 1;
    let previous = bar[..last].iter().rev().find_map(|e| e.degree).unwrap_or(0);
    let target = if final_close {
        nearest_tonic(previous)
    } else {
        [-3_i8, 4]
            .into_iter()
            .min_by_key(|t| t.abs_diff(previous))
            .unwrap()
    };
    // Resolve by step. With a single sustained note, no approach is necessary.
    if last > 0 {
        let before = if previous < target {
            target - 1
        } else {
            target + 1
        };
        bar[last - 1].degree = Some(before);
    }
    bar[last].degree = Some(target);
}

fn phrase(rng: &mut StdRng, meter: Meter, bars: usize) -> Vec<Bar> {
    let source = motif(rng, meter);
    let shift = [1, 2, 3, -2][rng.random_range(0..4)];
    let invert = rng.random_bool(0.25);
    let mut result = Vec::new();
    for b in 0..bars {
        let mut bar = source[b % 2].clone();
        if b / 2 % 2 == 1 {
            for event in &mut bar {
                if let Some(degree) = event.degree {
                    let altered = if invert && b % 4 >= 2 {
                        4 - degree
                    } else {
                        degree + shift
                    };
                    event.degree = Some(altered.clamp(-4, 10));
                }
            }
        } else if b >= 4 {
            // A returning phrase acquires a passing note, retaining its landmarks.
            if let Some(index) = bar
                .iter()
                .position(|e| e.duration >= 2 && e.degree.is_some())
            {
                let duration = bar[index].duration;
                let degree = bar[index].degree.unwrap();
                bar[index].duration = duration - 1;
                bar.insert(
                    index + 1,
                    Event {
                        degree: Some(if degree == 10 { 9 } else { degree + 1 }),
                        duration: 1,
                    },
                );
            }
        }
        if b % 4 == 3 || b + 1 == bars {
            cadence(&mut bar, b + 1 == bars);
        } else if b % 4 == 1 {
            cadence(&mut bar, false);
        }
        result.push(bar);
    }
    result
}

fn key_name(major: &str, mode: usize) -> String {
    const LETTERS: &[u8] = b"CDEFGAB";
    const MAJOR_STEPS: [i8; 7] = [0, 2, 4, 5, 7, 9, 11];
    const NATURAL: [i8; 7] = [0, 2, 4, 5, 7, 9, 11];
    let letter = LETTERS
        .iter()
        .position(|&c| c == major.as_bytes()[0])
        .unwrap();
    let accidental = if major.ends_with('b') { -1 } else { 0 };
    let target = (NATURAL[letter] + accidental + MAJOR_STEPS[mode]) % 12;
    let root_letter = (letter + mode) % 7;
    let alter = (target - NATURAL[root_letter] + 18) % 12 - 6;
    format!(
        "{}{}",
        LETTERS[root_letter] as char,
        match alter {
            -1 => "b",
            1 => "#",
            0 => "",
            _ => unreachable!("modal key needs an unsupported accidental"),
        }
    )
}

fn note(degree: i8, root_letter: i8) -> String {
    let position = root_letter + degree;
    let octave = position.div_euclid(7);
    let letter = b"CDEFGAB"[position.rem_euclid(7) as usize] as char;
    match octave {
        -1 => format!("{letter},"),
        0 => letter.to_string(),
        1 => letter.to_ascii_lowercase().to_string(),
        _ => format!("{}'", letter.to_ascii_lowercase()),
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let (major, major_letter) = KEYS[rng.random_range(0..KEYS.len())];
    let (mode, rotation) = MODES[rng.random_range(0..MODES.len())];
    let key = key_name(major, rotation);
    let root_letter = (major_letter + rotation as i8) % 7;
    let meter = METERS[rng.random_range(0..METERS.len())];
    let bars = [2, 4, 4, 8][rng.random_range(0..4)];
    let phrase = phrase(rng, meter, bars);
    let (beat, tempo) = if meter.denominator == 8 {
        ("3/8", rng.random_range(48..=96))
    } else {
        ("1/4", rng.random_range(64..=132))
    };
    let lines: Vec<_> = phrase
        .chunks(4)
        .map(|row| {
            row.iter()
                .map(|bar| {
                    bar.iter()
                        .map(|e| {
                            let pitch = e
                                .degree
                                .map_or_else(|| "z".into(), |d| note(d, root_letter));
                            if e.duration == 1 {
                                pitch
                            } else {
                                format!("{pitch}{}", e.duration)
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect::<Vec<_>>()
                .join(" | ")
        })
        .collect();
    format!(
        "X:1\nM:{}/{}\nL:1/8\nQ:{beat}={tempo}\nK:{key}{mode}\n{} |]",
        meter.numerator,
        meter.denominator,
        lines.join(" |\n")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::BTreeSet;

    #[test]
    fn phrases_fill_every_bar_and_resolve_within_the_diatonic_range() {
        let mut rng = StdRng::seed_from_u64(91);
        let mut rhythms = BTreeSet::new();
        let mut contours = BTreeSet::new();
        for meter in METERS {
            for bars in [2, 4, 8] {
                for _ in 0..200 {
                    let p = phrase(&mut rng, meter, bars);
                    assert_eq!(p.len(), bars);
                    for bar in &p {
                        assert_eq!(bar.iter().map(|e| e.duration).sum::<u8>(), meter.units);
                        assert!(bar.iter().all(|e| e.duration > 0));
                        assert!(
                            bar.iter()
                                .filter_map(|e| e.degree)
                                .all(|d| (-4..=10).contains(&d))
                        );
                    }
                    let final_bar = p.last().unwrap();
                    let tonic = final_bar.last().unwrap().degree.unwrap();
                    assert!([0, 7].contains(&tonic));
                    if final_bar.len() > 1 {
                        assert_eq!(
                            final_bar[final_bar.len() - 2]
                                .degree
                                .unwrap()
                                .abs_diff(tonic),
                            1
                        );
                    }
                    rhythms.insert(p[0].iter().map(|e| e.duration).collect::<Vec<_>>());
                    contours.insert(p[0].iter().filter_map(|e| e.degree).collect::<Vec<_>>());
                }
            }
        }
        assert!(rhythms.len() > 100);
        assert!(contours.len() > 100);
    }

    #[test]
    fn all_modal_tonics_match_their_key_signature() {
        let natural = [0_i8, 2, 4, 5, 7, 9, 11];
        for (major, letter) in KEYS {
            assert_eq!(major.as_bytes()[0], b"CDEFGAB"[letter as usize]);
            let major_pc = natural[letter as usize] - i8::from(major.ends_with('b'));
            for (_, rotation) in MODES {
                let name = key_name(major, rotation);
                let root_letter = b"CDEFGAB"
                    .iter()
                    .position(|&c| c == name.as_bytes()[0])
                    .unwrap();
                let accidental = i8::from(name.ends_with('#')) - i8::from(name.ends_with('b'));
                assert_eq!(
                    (natural[root_letter] + accidental).rem_euclid(12),
                    (major_pc + natural[rotation]).rem_euclid(12)
                );
                assert_eq!(root_letter, (letter as usize + rotation) % 7);
            }
        }
    }

    #[test]
    fn modal_keys_keep_the_relative_major_signature() {
        assert_eq!(key_name("C", 1), "D");
        assert_eq!(key_name("Bb", 5), "G");
        assert_eq!(key_name("E", 3), "A");
        assert_eq!(key_name("B", 3), "E");
        assert_eq!(key_name("Ab", 4), "Eb");
        assert_eq!(key_name("B", 4), "F#");
        assert_eq!(note(-1, 0), "B,");
        assert_eq!(note(7, 0), "c");
    }
}
