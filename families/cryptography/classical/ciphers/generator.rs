use rand::{Rng, rngs::StdRng, seq::SliceRandom};

struct Passage {
    text: &'static str,
    chapter: &'static str,
}

// Exact contiguous passages from Lewis Carroll's 1865 text. See README.md.
const PASSAGES: &[Passage] = &[
    Passage {
        text: "In another moment down went Alice after it, never once considering how in the world she was to get out again.",
        chapter: "I",
    },
    Passage {
        text: "The rabbit-hole went straight on like a tunnel for some way, and then dipped suddenly down, so suddenly that Alice had not a moment to think about stopping herself before she found herself falling down a very deep well.",
        chapter: "I",
    },
    Passage {
        text: "Either the well was very deep, or she fell very slowly, for she had plenty of time as she went down to look about her and to wonder what was going to happen next.",
        chapter: "I",
    },
    Passage {
        text: "How brave they'll all think me at home!",
        chapter: "I",
    },
    Passage {
        text: "Would the fall never come to an end!",
        chapter: "I",
    },
    Passage {
        text: "There was nothing else to do, so Alice soon began talking again.",
        chapter: "I",
    },
    Passage {
        text: "I hope they'll remember her saucer of milk at tea-time.",
        chapter: "I",
    },
    Passage {
        text: "Do cats eat bats? Do cats eat bats?",
        chapter: "I",
    },
    Passage {
        text: "There were doors all round the hall, but they were all locked; and when Alice had been all the way down one side and up the other, trying every door, she walked sadly down the middle, wondering how she was ever to get out again.",
        chapter: "I",
    },
    Passage {
        text: "Alice opened the door and found that it led into a small passage, not much larger than a rat-hole: she knelt down and looked along the passage into the loveliest garden you ever saw.",
        chapter: "I",
    },
    Passage {
        text: "Oh, how I wish I could shut up like a telescope!",
        chapter: "I",
    },
    Passage {
        text: "It was all very well to say 'Drink me,' but the wise little Alice was not going to do that in a hurry.",
        chapter: "I",
    },
    Passage {
        text: "I wonder what I should be like then?",
        chapter: "I",
    },
    Passage {
        text: "Soon her eye fell on a little glass box that was lying under the table: she opened it, and found in it a very small cake, on which the words 'EAT ME' were beautifully marked in currants.",
        chapter: "I",
    },
    Passage {
        text: "So she set to work, and very soon finished off the cake.",
        chapter: "I",
    },
    Passage {
        text: "And she went on planning to herself how she would manage it.",
        chapter: "II",
    },
    Passage {
        text: "Just then her head struck against the roof of the hall: in fact she was now more than nine feet high, and she at once took up the little golden key and hurried off to the garden door.",
        chapter: "II",
    },
    Passage {
        text: "After a time she heard a little pattering of feet in the distance, and she hastily dried her eyes to see what was coming.",
        chapter: "II",
    },
    Passage {
        text: "I wonder if I've been changed in the night?",
        chapter: "II",
    },
    Passage {
        text: "As she said this she looked down at her hands, and was surprised to see that she had put on one of the Rabbit's little white kid gloves while she was talking.",
        chapter: "II",
    },
    Passage {
        text: "As she said these words her foot slipped, and in another moment, splash! she was up to her chin in salt water.",
        chapter: "II",
    },
    Passage {
        text: "The Mouse looked at her rather inquisitively, and seemed to her to wink with one of its little eyes, but it said nothing.",
        chapter: "II",
    },
    Passage {
        text: "The Mouse gave a sudden leap out of the water, and seemed to quiver all over with fright.",
        chapter: "II",
    },
    Passage {
        text: "Alice led the way, and the whole party swam to the shore.",
        chapter: "II",
    },
];

const KEYWORDS: &[&str] = &[
    "LANTERN",
    "RIVER",
    "CHERRY",
    "CHESTNUT",
    "BALLOON",
    "THUNDER",
    "ORCHARD",
    "CIPHER",
    "CABBAGE",
    "MARBLE",
    "TUESDAY",
    "JASMINE",
    "COPPER",
    "KETTLE",
    "BANANA",
    "MIRROR",
    "LAGOON",
    "WINDOW",
    "OCTOBER",
    "FOXGLOVE",
    "MARMALADE",
    "WILLOW",
    "SILVER",
    "GARDEN",
    "SAPPHIRE",
    "COMPASS",
    "BICYCLE",
    "FERRY",
    "QUARTZ",
    "VIOLET",
    "MONDAY",
    "NOTEBOOK",
];

enum Cipher {
    Vigenere {
        key: Vec<u8>,
        autokey: bool,
    },
    Playfair {
        square: [u8; 25],
        keyword: Option<&'static str>,
    },
    RailFence {
        rails: usize,
        offset: usize,
        inverse: bool,
    },
    Columnar {
        keyword: &'static str,
        padding: usize,
        complete: bool,
    },
}

struct Specimen {
    passage: &'static Passage,
    excerpt: &'static str,
    cipher: Cipher,
    ciphertext: Vec<u8>,
}

pub fn generate(rng: &mut StdRng) -> String {
    let architecture = rng.random_range(0..4);
    let specimen = sample(rng, architecture);
    let (header, group) = match &specimen.cipher {
        Cipher::Vigenere { key, autokey } => (
            format!(
                "Vigenère{}\n{} {}",
                if *autokey { " plaintext autokey" } else { "" },
                if *autokey { "Primer" } else { "Repeating key" },
                letters(key),
            ),
            5,
        ),
        Cipher::Playfair { square, keyword } => {
            let key = if let Some(word) = keyword {
                format!("Keyword {word}")
            } else {
                square
                    .chunks(5)
                    .map(|row| grouped(row, 1))
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            (
                format!("Playfair\nI/J merged, X filler (Q after X)\n{key}"),
                2,
            )
        }
        Cipher::RailFence {
            rails,
            offset,
            inverse,
        } => (
            format!(
                "Rail fence, {}\n{rails} rails, offset {offset}",
                if *inverse {
                    "rows to zigzag"
                } else {
                    "zigzag to rows"
                },
            ),
            5,
        ),
        Cipher::Columnar {
            keyword,
            padding,
            complete,
        } => (
            format!(
                "Columnar transposition\nKeyword {keyword}, ties left to right\n{}",
                if *complete {
                    format!("Complete rectangle, {padding} trailing X padding")
                } else {
                    "Unpadded rectangle".into()
                },
            ),
            5,
        ),
    };
    format!(
        "{header}\n\n{}\n— Lewis Carroll, Alice's Adventures in Wonderland, chapter {}\n\n{}",
        specimen.excerpt,
        specimen.passage.chapter,
        grouped(&specimen.ciphertext, group),
    )
}

fn sample(rng: &mut StdRng, architecture: usize) -> Specimen {
    let (passage, excerpt) = select_passage(rng);
    let plain = normalize(excerpt, false);
    let keyword = KEYWORDS[rng.random_range(0..KEYWORDS.len())];
    let (cipher, ciphertext) = match architecture {
        0 => {
            let autokey = rng.random_bool(0.4);
            let key = if rng.random_bool(0.5) {
                keyword.as_bytes().to_vec()
            } else {
                let length = if rng.random_bool(0.5) {
                    rng.random_range(2..=4)
                } else {
                    rng.random_range(7..=12)
                };
                let mut key: Vec<_> = (0..length).map(|_| rng.random_range(b'A'..=b'Z')).collect();
                if key.iter().all(|&letter| letter == key[0]) {
                    key[1] = b'A' + (key[0] - b'A' + 1) % 26;
                }
                key
            };
            let ciphertext = vigenere(&plain, &key, autokey);
            (Cipher::Vigenere { key, autokey }, ciphertext)
        }
        1 => {
            let keyword = rng.random_bool(0.7).then_some(keyword);
            let mut square = playfair_square(keyword.unwrap_or(""));
            if keyword.is_none() {
                square.shuffle(rng);
            }
            let prepared = playfair_prepare(&normalize(excerpt, true));
            let ciphertext = playfair(&prepared, &square);
            (Cipher::Playfair { square, keyword }, ciphertext)
        }
        2 => {
            let rails = if rng.random_bool(0.3) {
                2
            } else {
                rng.random_range(3..=7)
            };
            let cycle = 2 * (rails - 1);
            let offset = match rng.random_range(0..4) {
                0 => 0,
                1 => rails - 1,
                _ => rng.random_range(0..cycle),
            };
            let inverse = rng.random_bool(0.25);
            let ciphertext = rail_fence(&plain, rails, offset, inverse);
            (
                Cipher::RailFence {
                    rails,
                    offset,
                    inverse,
                },
                ciphertext,
            )
        }
        _ => {
            let complete = rng.random_bool(0.4);
            let width = keyword.len();
            let padding = if complete {
                (width - plain.len() % width) % width
            } else {
                0
            };
            let mut prepared = plain;
            prepared.resize(prepared.len() + padding, b'X');
            let ciphertext = columnar(&prepared, keyword);
            (
                Cipher::Columnar {
                    keyword,
                    padding,
                    complete,
                },
                ciphertext,
            )
        }
    };
    Specimen {
        passage,
        excerpt,
        cipher,
        ciphertext,
    }
}

fn span_count(words: usize) -> usize {
    (10..=32.min(words)).map(|length| words - length + 1).sum()
}

fn select_passage(rng: &mut StdRng) -> (&'static Passage, &'static str) {
    if rng.random_bool(0.4) {
        let passage = &PASSAGES[rng.random_range(0..PASSAGES.len())];
        return (passage, passage.text);
    }
    let counts: Vec<_> = PASSAGES
        .iter()
        .map(|p| span_count(p.text.split_whitespace().count()))
        .collect();
    let mut ticket = rng.random_range(0..counts.iter().sum::<usize>());
    for (passage, count) in PASSAGES.iter().zip(counts) {
        if ticket >= count {
            ticket -= count;
            continue;
        }
        let starts: Vec<_> = std::iter::once(0)
            .chain(passage.text.match_indices(' ').map(|(i, _)| i + 1))
            .collect();
        for length in 10..=32.min(starts.len()) {
            let choices = starts.len() - length + 1;
            if ticket >= choices {
                ticket -= choices;
            } else {
                let end = starts
                    .get(ticket + length)
                    .map_or(passage.text.len(), |i| i - 1);
                return (passage, &passage.text[starts[ticket]..end]);
            }
        }
    }
    unreachable!("ticket selects an enumerated source span")
}

fn normalize(text: &str, merge_j: bool) -> Vec<u8> {
    text.bytes()
        .filter(u8::is_ascii_alphabetic)
        .map(|b| {
            let letter = b.to_ascii_uppercase();
            if merge_j && letter == b'J' {
                b'I'
            } else {
                letter
            }
        })
        .collect()
}

fn letters(text: &[u8]) -> String {
    String::from_utf8(text.to_vec()).expect("ASCII letters")
}

fn grouped(text: &[u8], size: usize) -> String {
    text.chunks(size).map(letters).collect::<Vec<_>>().join(" ")
}

fn vigenere(plain: &[u8], key: &[u8], autokey: bool) -> Vec<u8> {
    plain
        .iter()
        .enumerate()
        .map(|(i, &letter)| {
            let shift = if autokey && i >= key.len() {
                plain[i - key.len()]
            } else {
                key[i % key.len()]
            };
            b'A' + (letter - b'A' + shift - b'A') % 26
        })
        .collect()
}

fn playfair_square(keyword: &str) -> [u8; 25] {
    let mut square = [0; 25];
    let mut used = [false; 26];
    used[(b'J' - b'A') as usize] = true;
    let mut position = 0;
    for letter in normalize(keyword, true).into_iter().chain(b'A'..=b'Z') {
        if !used[(letter - b'A') as usize] {
            square[position] = letter;
            position += 1;
            used[(letter - b'A') as usize] = true;
        }
    }
    square
}

fn playfair_prepare(plain: &[u8]) -> Vec<u8> {
    let mut prepared = Vec::with_capacity(2 * plain.len());
    let mut i = 0;
    while i < plain.len() {
        let first = plain[i];
        let second = plain.get(i + 1).copied();
        prepared.push(first);
        if let Some(second) = second.filter(|&letter| letter != first) {
            prepared.push(second);
            i += 2;
        } else {
            prepared.push(if first == b'X' { b'Q' } else { b'X' });
            i += 1;
        }
    }
    prepared
}

fn playfair(plain: &[u8], square: &[u8; 25]) -> Vec<u8> {
    let mut positions = [0; 26];
    for (position, &letter) in square.iter().enumerate() {
        positions[(letter - b'A') as usize] = position;
    }
    let mut ciphertext = Vec::with_capacity(plain.len());
    for pair in plain.as_chunks::<2>().0 {
        let a = positions[(pair[0] - b'A') as usize];
        let b = positions[(pair[1] - b'A') as usize];
        let (row_a, col_a, row_b, col_b) = (a / 5, a % 5, b / 5, b % 5);
        let (a, b) = if row_a == row_b {
            (5 * row_a + (col_a + 1) % 5, 5 * row_b + (col_b + 1) % 5)
        } else if col_a == col_b {
            (5 * ((row_a + 1) % 5) + col_a, 5 * ((row_b + 1) % 5) + col_b)
        } else {
            (5 * row_a + col_b, 5 * row_b + col_a)
        };
        ciphertext.extend([square[a], square[b]]);
    }
    ciphertext
}

fn rail_fence(text: &[u8], rails: usize, offset: usize, inverse: bool) -> Vec<u8> {
    let cycle = 2 * (rails - 1);
    // The sorted visit order is the encryption permutation; swapping its sides
    // gives the ACA's other direction without a second traversal algorithm.
    let mut order: Vec<_> = (0..text.len()).collect();
    order.sort_by_key(|&i| {
        let phase = (i + offset) % cycle;
        phase.min(cycle - phase)
    });
    if inverse {
        let mut output = vec![0; text.len()];
        for (i, &position) in order.iter().enumerate() {
            output[position] = text[i];
        }
        output
    } else {
        order.iter().map(|&position| text[position]).collect()
    }
}

fn columnar(plain: &[u8], keyword: &str) -> Vec<u8> {
    let width = keyword.len();
    let mut columns: Vec<_> = (0..width).collect();
    columns.sort_by_key(|&i| (keyword.as_bytes()[i], i));
    columns
        .into_iter()
        .flat_map(|col| plain.iter().skip(col).step_by(width).copied())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn decrypt_vigenere(ciphertext: &[u8], key: &[u8], autokey: bool) -> Vec<u8> {
        let mut plaintext = Vec::with_capacity(ciphertext.len());
        for &letter in ciphertext {
            let i = plaintext.len();
            let shift = if autokey && i >= key.len() {
                plaintext[i - key.len()]
            } else {
                key[i % key.len()]
            };
            plaintext.push(b'A' + (26 + letter - shift) % 26);
        }
        plaintext
    }

    fn decrypt_playfair(ciphertext: &[u8], square: &[u8; 25]) -> Vec<u8> {
        let mut plain = Vec::with_capacity(ciphertext.len());
        for pair in ciphertext.as_chunks::<2>().0 {
            let a = square.iter().position(|&b| b == pair[0]).unwrap();
            let b = square.iter().position(|&b| b == pair[1]).unwrap();
            let a_row = &square[a / 5 * 5..a / 5 * 5 + 5];
            let b_row = &square[b / 5 * 5..b / 5 * 5 + 5];
            if a / 5 == b / 5 {
                plain.extend([a_row[(a % 5 + 4) % 5], b_row[(b % 5 + 4) % 5]]);
            } else if a % 5 == b % 5 {
                plain.extend([square[(a + 20) % 25], square[(b + 20) % 25]]);
            } else {
                plain.extend([a_row[b % 5], b_row[a % 5]]);
            }
        }
        plain
    }

    // Walk the fence with a signed direction, independently of the production
    // phase formula and sorting permutation.
    fn fence_path(len: usize, rails: usize, offset: usize) -> Vec<usize> {
        let mut row = 0_isize;
        let mut direction = 1;
        let mut path = Vec::with_capacity(len);
        for i in 0..len + offset {
            if i >= offset {
                path.push(row as usize);
            }
            row += direction;
            if row == 0 || row == rails as isize - 1 {
                direction = -direction;
            }
        }
        path
    }

    fn decrypt_fence(text: &[u8], rails: usize, offset: usize, inverse: bool) -> Vec<u8> {
        let path = fence_path(text.len(), rails, offset);
        let mut rows = vec![Vec::new(); rails];
        if inverse {
            for (&letter, &row) in text.iter().zip(&path) {
                rows[row].push(letter);
            }
            rows.into_iter().flatten().collect()
        } else {
            let mut cursor = 0;
            for (r, row) in rows.iter_mut().enumerate() {
                let count = path.iter().filter(|&&p| p == r).count();
                row.extend_from_slice(&text[cursor..cursor + count]);
                row.reverse();
                cursor += count;
            }
            path.into_iter()
                .map(|row| rows[row].pop().unwrap())
                .collect()
        }
    }

    fn decrypt_columnar(text: &[u8], keyword: &str) -> Vec<u8> {
        let width = keyword.len();
        let height = text.len().div_ceil(width);
        let mut grid = vec![vec![None; width]; height];
        let mut cursor = 0;
        for letter in b'A'..=b'Z' {
            for (col, &key_letter) in keyword.as_bytes().iter().enumerate() {
                if key_letter == letter {
                    for (row, cells) in grid.iter_mut().enumerate() {
                        if row * width + col < text.len() {
                            cells[col] = Some(text[cursor]);
                            cursor += 1;
                        }
                    }
                }
            }
        }
        grid.into_iter().flatten().flatten().collect()
    }

    #[test]
    fn published_known_vectors() {
        assert_eq!(
            letters(&vigenere(b"ATTACKATDAWN", b"LEMON", false)),
            "LXFOPVEFRNHR"
        );
        // ACA's Vigenere and plaintext-autokey worked examples.
        let plain = normalize(
            "In the Vigenere C equals K plus P where A is zero B is one etc",
            false,
        );
        assert_eq!(
            letters(&vigenere(&plain, b"POLYALPHABETIC", false)),
            "XBEFEGXNEOIKMETEFYLDZWL VWIEJTFPYIDOLRP FBAQCSPRC".replace(' ', "")
        );
        let plain = normalize(
            "The autokey can be used with Vigenere Variant Beaufort or Porta",
            false,
        );
        assert_eq!(
            letters(&vigenere(&plain, b"PRIMER", true)),
            "IYMMYKHRIYWTBLISUEQXMNZZLCMGLMMBEEMRROBVIUSHSXOLUCIMO"
        );
        let square = playfair_square("LOGARITHM");
        let plain = playfair_prepare(&normalize("Come quickly we need help", true));
        assert_eq!(
            letters(&playfair(&plain, &square)),
            "DLHFSNCNCRZXCQQGFEEQON"
        );
        assert_eq!(
            letters(&rail_fence(b"CIVILWARFIELDCIPHER", 3, 0, false)),
            "CLFDHIIWRILCPEVAEIR"
        );
        assert_eq!(
            letters(&rail_fence(b"CIVILWARFIELDCIPHER", 3, 0, true)),
            "CWIAIRPFVIHEILEDLCR"
        );
        assert_eq!(letters(&columnar(b"FILLEDBLOCKX", "CAB")), "IELKLDOXFLBC");
        assert_eq!(letters(&columnar(b"UNFILLEDBLOCK", "CAB")), "NLDOFLBCUIELK");
    }

    #[test]
    fn playfair_preparation_and_all_pair_geometries() {
        assert_eq!(letters(&playfair_prepare(b"BALLOON")), "BALXLOON");
        assert_eq!(letters(&playfair_prepare(b"XXX")), "XQXQXQ");
        assert_eq!(letters(&playfair_prepare(b"X")), "XQ");
        assert_eq!(letters(&normalize("Jigsaw, jinx!", true)), "IIGSAWIINX");
        let square = playfair_square("JASMINE");
        assert_eq!(square.iter().filter(|&&c| c == b'I').count(), 1);
        assert!(!square.contains(&b'J'));
        for &first in &square {
            for &second in &square {
                if first == second {
                    continue;
                }
                let plain = [first, second];
                let encrypted = playfair(&plain, &square);
                assert_ne!(encrypted[0], encrypted[1]);
                assert_eq!(decrypt_playfair(&encrypted, &square), plain);
            }
        }
    }

    #[test]
    fn transpositions_cover_short_rows_ties_and_cycle_boundaries() {
        for len in 1..=80 {
            let plain: Vec<_> = (0..len).map(|i| b'A' + (i % 26) as u8).collect();
            for rails in 2..=9 {
                for offset in 0..2 * (rails - 1) {
                    for inverse in [false, true] {
                        let encrypted = rail_fence(&plain, rails, offset, inverse);
                        assert_eq!(decrypt_fence(&encrypted, rails, offset, inverse), plain);
                    }
                }
            }
            for keyword in ["A", "AAAA", "BALLOON", "BANANA", "ZEBRA", "MARMALADE"] {
                assert_eq!(decrypt_columnar(&columnar(&plain, keyword), keyword), plain);
            }
        }
    }

    #[test]
    fn seeded_specimens_decrypt_independently() {
        let mut rng = StdRng::seed_from_u64(0xC1F3_2026);
        for architecture in 0..4 {
            for _ in 0..512 {
                let specimen = sample(&mut rng, architecture);
                assert!(specimen.ciphertext.iter().all(u8::is_ascii_uppercase));
                assert!(specimen.passage.text.contains(specimen.excerpt));
                let original = normalize(specimen.excerpt, false);
                match specimen.cipher {
                    Cipher::Vigenere { key, autokey } => assert_eq!(
                        decrypt_vigenere(&specimen.ciphertext, &key, autokey),
                        original
                    ),
                    Cipher::Playfair { square, .. } => {
                        let decoded = decrypt_playfair(&specimen.ciphertext, &square);
                        let merged = normalize(specimen.excerpt, true);
                        // Check the preparation independently: consume genuine
                        // pairs, or exactly one legal inserted filler.
                        let mut cursor = 0;
                        for pair in decoded.as_chunks::<2>().0 {
                            assert_eq!(pair[0], merged[cursor]);
                            cursor += 1;
                            if cursor < merged.len() && merged[cursor] != pair[0] {
                                assert_eq!(pair[1], merged[cursor]);
                                cursor += 1;
                            } else {
                                assert_eq!(pair[1], if pair[0] == b'X' { b'Q' } else { b'X' });
                            }
                        }
                        assert_eq!(cursor, merged.len());
                    }
                    Cipher::RailFence {
                        rails,
                        offset,
                        inverse,
                    } => assert_eq!(
                        decrypt_fence(&specimen.ciphertext, rails, offset, inverse),
                        original
                    ),
                    Cipher::Columnar {
                        keyword,
                        padding,
                        complete,
                    } => {
                        let decoded = decrypt_columnar(&specimen.ciphertext, keyword);
                        assert_eq!(&decoded[..original.len()], original);
                        assert_eq!(decoded.len(), original.len() + padding);
                        assert!(decoded[original.len()..].iter().all(|&b| b == b'X'));
                        if complete {
                            assert_eq!(decoded.len() % keyword.len(), 0);
                        }
                    }
                }
            }
        }
    }
}
