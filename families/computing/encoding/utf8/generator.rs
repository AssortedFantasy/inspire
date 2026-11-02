use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

mod names;
use names::NAMES;

#[derive(Debug, Eq, PartialEq)]
struct Issue {
    offset: usize,
    reason: &'static str,
}

#[derive(Debug, Eq, PartialEq)]
struct Decoded {
    scalars: Vec<(usize, u32, usize)>,
    consumed: usize,
}

fn decode(bytes: &[u8], final_input: bool) -> Result<Decoded, Issue> {
    let mut scalars = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let lead = bytes[i];
        let (width, mut cp) = match lead {
            0..=0x7f => (1, u32::from(lead)),
            0xc2..=0xdf => (2, u32::from(lead & 0x1f)),
            0xe0..=0xef => (3, u32::from(lead & 0x0f)),
            0xf0..=0xf4 => (4, u32::from(lead & 0x07)),
            0x80..=0xbf => {
                return Err(Issue {
                    offset: i,
                    reason: "unexpected continuation byte",
                });
            }
            0xc0..=0xc1 => {
                return Err(Issue {
                    offset: i,
                    reason: "non-shortest encoding",
                });
            }
            _ => {
                return Err(Issue {
                    offset: i,
                    reason: "invalid leading byte",
                });
            }
        };
        for j in 1..width {
            let Some(&byte) = bytes.get(i + j) else {
                return if final_input {
                    Err(Issue {
                        offset: i,
                        reason: "truncated sequence",
                    })
                } else {
                    Ok(Decoded {
                        scalars,
                        consumed: i,
                    })
                };
            };
            let reason = if !(0x80..=0xbf).contains(&byte) {
                Some("expected continuation byte")
            } else if j == 1 && ((lead == 0xe0 && byte < 0xa0) || (lead == 0xf0 && byte < 0x90)) {
                Some("non-shortest encoding")
            } else if j == 1 && lead == 0xed && byte >= 0xa0 {
                Some("surrogate encoding")
            } else if j == 1 && lead == 0xf4 && byte >= 0x90 {
                Some("above U+10FFFF")
            } else {
                None
            };
            if let Some(reason) = reason {
                return Err(Issue { offset: i, reason });
            }
            cp = (cp << 6) | u32::from(byte & 0x3f);
        }
        scalars.push((i, cp, width));
        i += width;
    }
    Ok(Decoded {
        scalars,
        consumed: i,
    })
}

fn encoded(cp: u32) -> Vec<u8> {
    let mut bytes = [0; 4];
    char::from_u32(cp)
        .unwrap()
        .encode_utf8(&mut bytes)
        .as_bytes()
        .to_vec()
}

fn utf16(cp: u32) -> Vec<u16> {
    if cp < 0x10000 {
        vec![cp as u16]
    } else {
        let n = cp - 0x10000;
        vec![0xd800 | (n >> 10) as u16, 0xdc00 | (n & 0x3ff) as u16]
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn codes(values: &[u32]) -> String {
    values
        .iter()
        .map(|cp| format!("U+{cp:04X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn named(rng: &mut StdRng) -> u32 {
    const BLOCKS: [&str; 10] = [
        "Basic Latin",
        "Latin-1 Supplement",
        "Combining Diacritical Marks",
        "Greek and Coptic",
        "General Punctuation",
        "Arrows",
        "Mathematical Operators",
        "Miscellaneous Technical",
        "Musical Symbols",
        "Emoticons",
    ];
    let block = BLOCKS[rng.random_range(0..BLOCKS.len())];
    let candidates: Vec<_> = NAMES.iter().filter(|row| row.2 == block).collect();
    candidates[rng.random_range(0..candidates.len())].0
}

fn description(cp: u32) -> String {
    match NAMES.binary_search_by_key(&cp, |row| row.0) {
        Ok(i) => format!("U+{cp:04X}  {}\n{}", NAMES[i].1, NAMES[i].2),
        Err(_) => format!("U+{cp:04X}"),
    }
}

fn scalar(rng: &mut StdRng) -> String {
    let cp = named(rng);
    let mut out = description(cp);
    if rng.random_bool(0.5) {
        write!(
            out,
            "\n\nUTF-8         {}\nUTF-16 units  {}",
            hex(&encoded(cp)),
            utf16(cp)
                .iter()
                .map(|u| format!("{u:04X}"))
                .collect::<Vec<_>>()
                .join(" ")
        )
        .unwrap();
    } else {
        write!(
            out,
            "\n\nUTF-8   {}\n        {}",
            hex(&encoded(cp)),
            encoded(cp)
                .iter()
                .map(|b| format!("{b:08b}"))
                .collect::<Vec<_>>()
                .join(" ")
        )
        .unwrap();
    }
    out
}

fn boundary(rng: &mut StdRng) -> String {
    let groups: &[&[u32]] = &[
        &[0x7e, 0x7f, 0x80, 0x81],
        &[0x7fe, 0x7ff, 0x800, 0x801],
        &[0xd7fe, 0xd7ff, 0xe000, 0xe001],
        &[0xfffe, 0xffff, 0x10000, 0x10001],
        &[0x10fffc, 0x10fffd, 0x10fffe, 0x10ffff],
    ];
    let values = groups[rng.random_range(0..groups.len())];
    let mut out = String::from("scalar     UTF-8          UTF-16 units\n");
    for &cp in values {
        writeln!(
            out,
            "U+{cp:06X}   {:<14} {}",
            hex(&encoded(cp)),
            utf16(cp)
                .iter()
                .map(|u| format!("{u:04X}"))
                .collect::<Vec<_>>()
                .join(" ")
        )
        .unwrap();
    }
    out.trim_end().to_owned()
}

fn sequence(rng: &mut StdRng) -> Vec<u32> {
    match rng.random_range(0..5) {
        0 => vec![rng.random_range(0x41..=0x5a), 0x0301, named(rng)],
        1 => vec![
            0x1f469,
            0x200d,
            if rng.random_bool(0.5) {
                0x1f4bb
            } else {
                0x1f52c
            },
        ],
        2 => vec![0x61, 0x00e9, 0x20ac, 0x1d11e],
        3 => vec![named(rng), 0x200b, named(rng), 0xfeff],
        _ => (0..rng.random_range(3..=4)).map(|_| named(rng)).collect(),
    }
}

fn offsets(rng: &mut StdRng) -> String {
    let values = sequence(rng);
    let bytes: Vec<_> = values.iter().flat_map(|&cp| encoded(cp)).collect();
    let decoded = decode(&bytes, true).unwrap();
    let mut out = format!(
        "{}\n{} bytes, {} scalars\n\nbyte offset  scalar     UTF-8\n",
        codes(&values),
        bytes.len(),
        values.len()
    );
    for (offset, cp, width) in decoded.scalars {
        writeln!(
            out,
            "{offset:<13}U+{cp:06X}   {}",
            hex(&bytes[offset..offset + width])
        )
        .unwrap();
    }
    out.trim_end().to_owned()
}

fn invalid(rng: &mut StdRng) -> String {
    let mut bytes: Vec<_> = if rng.random_bool(0.5) {
        encoded(named(rng))
    } else {
        vec![]
    };
    let bad = match rng.random_range(0..7) {
        0 => {
            let cp = rng.random_range(0..=0x7f_u8);
            vec![0xc0 | (cp >> 6), 0x80 | (cp & 0x3f)]
        }
        1 => vec![
            0xed,
            rng.random_range(0xa0..=0xbf),
            rng.random_range(0x80..=0xbf),
        ],
        2 => vec![
            0xf4,
            rng.random_range(0x90..=0xbf),
            rng.random_range(0x80..=0xbf),
            rng.random_range(0x80..=0xbf),
        ],
        3 => vec![rng.random_range(0x80..=0xbf)],
        4 => {
            let mut b = encoded(if rng.random_bool(0.5) {
                0x1d11e
            } else {
                0x20ac
            });
            b.truncate(rng.random_range(1..b.len()));
            b
        }
        5 => vec![
            0xe2,
            rng.random_range(0x20..=0x7e),
            rng.random_range(0x80..=0xbf),
        ],
        _ => vec![rng.random_range(0xf5..=0xff), 0x80, 0x80, 0x80],
    };
    bytes.extend(bad);
    let issue = decode(&bytes, true).unwrap_err();
    format!(
        "{}\n\nRejected at byte {}\n{}",
        hex(&bytes),
        issue.offset,
        issue.reason
    )
}

fn chunks(rng: &mut StdRng) -> String {
    let values = sequence(rng);
    let bytes: Vec<_> = values.iter().flat_map(|&cp| encoded(cp)).collect();
    let mut out = String::from("chunk          emitted scalars                   pending bytes\n");
    let mut pending = Vec::new();
    let mut cursor = 0;
    while cursor < bytes.len() {
        let end = (cursor + rng.random_range(1..=4)).min(bytes.len());
        let chunk = &bytes[cursor..end];
        pending.extend_from_slice(chunk);
        let decoded = decode(&pending, false).unwrap();
        let emitted = decoded
            .scalars
            .iter()
            .map(|(_, cp, _)| *cp)
            .collect::<Vec<_>>();
        let tail = pending[decoded.consumed..].to_vec();
        writeln!(
            out,
            "{:<15}{:<34} {}",
            hex(chunk),
            if emitted.is_empty() {
                "—".to_owned()
            } else {
                codes(&emitted)
            },
            if tail.is_empty() {
                "—".to_owned()
            } else {
                hex(&tail)
            }
        )
        .unwrap();
        pending = tail;
        cursor = end;
    }
    write!(out, "\nEnd of stream  accepted\n{}", codes(&values)).unwrap();
    out
}

pub fn generate(rng: &mut StdRng) -> String {
    match rng.random_range(0..5) {
        0 => scalar(rng),
        1 => boundary(rng),
        2 => offsets(rng),
        3 => invalid(rng),
        _ => chunks(rng),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn all_scalar_values_decode_and_utf16_matches_standard() {
        for cp in 0..=0x10ffff {
            if let Some(ch) = char::from_u32(cp) {
                let bytes = encoded(cp);
                assert_eq!(
                    decode(&bytes, true).unwrap().scalars,
                    vec![(0, cp, ch.len_utf8())]
                );
                let mut units = [0; 2];
                assert_eq!(utf16(cp), ch.encode_utf16(&mut units));
            }
        }
    }

    #[test]
    fn validity_and_error_offsets_match_standard() {
        fn check(bytes: &[u8]) {
            match (decode(bytes, true), std::str::from_utf8(bytes)) {
                (Ok(a), Ok(b)) => assert_eq!(
                    a.scalars.iter().map(|x| x.1).collect::<Vec<_>>(),
                    b.chars().map(u32::from).collect::<Vec<_>>()
                ),
                (Err(a), Err(b)) => assert_eq!(a.offset, b.valid_up_to(), "{bytes:?}"),
                result => panic!("decoder disagreement on {bytes:?}: {result:?}"),
            }
        }
        for a in 0..=255 {
            check(&[a]);
            for b in 0..=255 {
                check(&[a, b]);
            }
        }
        let edges = [
            0, 0x7f, 0x80, 0x8f, 0x90, 0x9f, 0xa0, 0xbf, 0xc0, 0xc2, 0xdf, 0xe0, 0xed, 0xef, 0xf0,
            0xf4, 0xf5, 0xff,
        ];
        for a in edges {
            for b in edges {
                for c in edges {
                    check(&[a, b, c]);
                    for d in edges {
                        check(&[a, b, c, d]);
                    }
                }
            }
        }
    }

    #[test]
    fn every_stream_split_preserves_scalars() {
        let values = [0x41, 0x301, 0x7ff, 0x800, 0xd7ff, 0xe000, 0x10000, 0x10ffff];
        let bytes: Vec<_> = values.iter().flat_map(|&cp| encoded(cp)).collect();
        for first in 0..=bytes.len() {
            for second in first..=bytes.len() {
                let mut pending = Vec::new();
                let mut decoded = Vec::new();
                for chunk in [&bytes[..first], &bytes[first..second], &bytes[second..]] {
                    pending.extend_from_slice(chunk);
                    let step = decode(&pending, false).unwrap();
                    decoded.extend(step.scalars.iter().map(|x| x.1));
                    pending.drain(..step.consumed);
                }
                assert!(pending.is_empty());
                assert_eq!(decoded, values);
            }
        }
    }

    #[test]
    fn name_subset_is_sorted_and_every_width_is_present() {
        assert!(NAMES.windows(2).all(|pair| pair[0].0 < pair[1].0));
        let mut widths = [false; 4];
        for &(cp, name, block) in NAMES {
            widths[encoded(cp).len() - 1] = true;
            assert!(!name.is_empty() && !block.is_empty());
        }
        assert!(widths.into_iter().all(|present| present));
        let mut rng = StdRng::seed_from_u64(130);
        for _ in 0..256 {
            assert!(!generate(&mut rng).is_empty());
        }
    }
}
