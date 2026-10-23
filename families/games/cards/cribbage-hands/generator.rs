use rand::{Rng, rngs::StdRng, seq::SliceRandom};

#[derive(Debug, Default, PartialEq, Eq)]
struct Score {
    fifteens: u8,
    pairs: u8,
    run_length: u8,
    run_count: u8,
    flush: u8,
    nobs: u8,
}

impl Score {
    fn total(&self) -> u8 {
        2 * self.fifteens
            + 2 * self.pairs
            + self.run_length * self.run_count
            + self.flush
            + self.nobs
    }
}

fn rank(card: u8) -> u8 {
    card % 13 + 1
}

fn score(hand: &[u8; 4], starter: u8, crib: bool) -> Score {
    let cards = [hand[0], hand[1], hand[2], hand[3], starter];
    let mut result = Score::default();
    for mask in 1u8..32 {
        let mut ranks: Vec<_> = (0..5)
            .filter(|&i| mask & (1 << i) != 0)
            .map(|i| rank(cards[i]))
            .collect();
        if ranks.iter().map(|&r| r.min(10)).sum::<u8>() == 15 {
            result.fifteens += 1;
        }
        ranks.sort_unstable();
        if ranks.len() >= 3 && ranks.windows(2).all(|r| r[1] == r[0] + 1) {
            let length = ranks.len() as u8;
            if length > result.run_length {
                result.run_length = length;
                result.run_count = 1;
            } else if length == result.run_length {
                result.run_count += 1;
            }
        }
    }
    for i in 0..5 {
        for j in i + 1..5 {
            result.pairs += u8::from(rank(cards[i]) == rank(cards[j]));
        }
    }
    let suit = hand[0] / 13;
    if hand.iter().all(|&c| c / 13 == suit) {
        result.flush = if starter / 13 == suit {
            5
        } else if crib {
            0
        } else {
            4
        };
    }
    result.nobs = u8::from(
        hand.iter()
            .any(|&c| rank(c) == 11 && c / 13 == starter / 13),
    );
    result
}

fn suited_ranks(rng: &mut StdRng, ranks: &[u8; 5]) -> [u8; 5] {
    let mut suits = [[0, 1, 2, 3]; 13];
    for group in &mut suits {
        group.shuffle(rng);
    }
    let mut used = [0usize; 13];
    let mut cards = [0; 5];
    for (i, &rank) in ranks.iter().enumerate() {
        let index = (rank - 1) as usize;
        cards[i] = 13 * suits[index][used[index]] + rank - 1;
        used[index] += 1;
    }
    cards.shuffle(rng);
    cards
}

fn zero_hand(rng: &mut StdRng) -> [u8; 5] {
    let base = [[1, 7, 9], [2, 6, 8], [3, 6, 8], [1, 6, 7]][rng.random_range(0..4)];
    let mut faces = [10, 11, 12, 13];
    faces.shuffle(rng);
    if base[2] == 9 && faces[..2].contains(&10) && faces[..2].contains(&11) {
        faces.swap(1, 2);
    }
    let mut cards = suited_ranks(rng, &[base[0], base[1], base[2], faces[0], faces[1]]);
    let starter_suit = cards[4] / 13;
    for c in &mut cards[..4] {
        if rank(*c) == 11 {
            *c = 13 * ((starter_suit + 1) % 4) + 10;
        }
    }
    if cards[..4].iter().all(|c| c / 13 == cards[0] / 13) {
        cards[0] = (cards[0] + 13) % 52;
    }
    cards
}

fn deal(rng: &mut StdRng) -> ([u8; 4], u8, bool) {
    let crib = rng.random_bool(0.5);
    let regime = rng.random_range(0..32);
    let cards = match regime {
        0..=17 => {
            let mut deck: Vec<_> = (0..52).collect();
            deck.shuffle(rng);
            deck[..5].try_into().unwrap()
        }
        18..=25 => {
            let offsets = [
                [0, 0, 1, 1, 2],
                [0, 0, 0, 1, 2],
                [0, 1, 1, 2, 3],
                [0, 1, 2, 3, 4],
            ][rng.random_range(0..4)];
            let start = rng.random_range(1..=13 - offsets[4]);
            suited_ranks(rng, &offsets.map(|offset| start + offset))
        }
        26..=28 => {
            let suit = rng.random_range(0..4);
            let mut ranks: Vec<_> = (1..=13).collect();
            ranks.shuffle(rng);
            if rng.random_bool(0.5) {
                let jack = ranks.iter().position(|&r| r == 11).unwrap();
                ranks.swap(0, jack);
            }
            let starter_suit = if rng.random_bool(0.5) {
                suit
            } else {
                (suit + rng.random_range(1..=3)) % 4
            };
            [
                suit * 13 + ranks[0] - 1,
                suit * 13 + ranks[1] - 1,
                suit * 13 + ranks[2] - 1,
                suit * 13 + ranks[3] - 1,
                starter_suit * 13 + ranks[4] - 1,
            ]
        }
        30 => zero_hand(rng),
        _ => {
            let ranks = if regime == 31 && rng.random_bool(0.125) {
                [5, 5, 5, 5, rng.random_range(10..=13)]
            } else {
                [
                    [4, 5, 5, 6, 10],
                    [3, 3, 4, 5, 5],
                    [2, 3, 4, 6, 9],
                    [1, 4, 5, 5, 10],
                    [5, 5, 6, 9, 10],
                    [7, 7, 8, 8, 9],
                    [6, 6, 7, 8, 9],
                    [1, 2, 3, 4, 5],
                ][rng.random_range(0..8)]
            };
            suited_ranks(rng, &ranks)
        }
    };
    (cards[..4].try_into().unwrap(), cards[4], crib)
}

fn card(card: u8) -> String {
    format!(
        "{}{}",
        [
            "A", "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K"
        ][(card % 13) as usize],
        ["C", "D", "H", "S"][(card / 13) as usize]
    )
}

pub fn generate(rng: &mut StdRng) -> String {
    let (mut hand, starter, crib) = deal(rng);
    let points = score(&hand, starter, crib);
    hand.sort_unstable_by_key(|&c| (rank(c), c / 13));
    let mut lines = vec![
        format!(
            "{}  {}",
            if crib { "Crib" } else { "Hand" },
            hand.iter().map(|&c| card(c)).collect::<Vec<_>>().join(" ")
        ),
        format!("Starter  {}", card(starter)),
        format!(
            "{} {}",
            points.total(),
            if points.total() == 1 {
                "point"
            } else {
                "points"
            }
        ),
    ];
    if points.fifteens > 0 {
        lines.push(format!(
            "Fifteens  {} × 2 = {}",
            points.fifteens,
            points.fifteens * 2
        ));
    }
    if points.pairs > 0 {
        lines.push(format!(
            "Pairs  {} × 2 = {}",
            points.pairs,
            points.pairs * 2
        ));
    }
    if points.run_count > 0 {
        lines.push(format!(
            "Runs  {} × {} = {}",
            points.run_count,
            points.run_length,
            points.run_count * points.run_length
        ));
    }
    if points.flush > 0 {
        lines.push(format!("Flush  {}", points.flush));
    }
    if points.nobs > 0 {
        lines.push("Nobs  1".into());
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn c(rank: u8, suit: u8) -> u8 {
        suit * 13 + rank - 1
    }

    fn reference_rank_score(ranks: [u8; 5]) -> (u8, u8, u8, u8) {
        let mut counts = [0u8; 14];
        for rank in ranks {
            counts[rank as usize] += 1;
        }
        let choose = [
            [1u8, 0, 0, 0, 0],
            [1, 1, 0, 0, 0],
            [1, 2, 1, 0, 0],
            [1, 3, 3, 1, 0],
            [1, 4, 6, 4, 1],
        ];
        let mut sums = [0u8; 16];
        sums[0] = 1;
        let mut pairs = 0;
        for (rank, &count) in counts.iter().enumerate().skip(1) {
            pairs += choose[count as usize][2];
            let previous = sums;
            sums.fill(0);
            for (sum, &ways) in previous.iter().enumerate() {
                for (chosen, &combinations) in choose[count as usize]
                    .iter()
                    .enumerate()
                    .take(count as usize + 1)
                {
                    let next = sum + chosen * rank.min(10);
                    if next <= 15 {
                        sums[next] += ways * combinations;
                    }
                }
            }
        }
        let mut run_length = 0;
        let mut run_count = 0;
        for streak in counts[1..].split(|&count| count == 0) {
            if streak.len() >= 3 {
                let length = streak.len() as u8;
                let multiplicity = streak.iter().product::<u8>();
                if length > run_length {
                    run_length = length;
                    run_count = multiplicity;
                } else if length == run_length {
                    run_count += multiplicity;
                }
            }
        }
        (sums[15], pairs, run_length, run_count)
    }

    #[test]
    fn every_rank_multiset_matches_independent_frequency_scoring() {
        let mut multisets = 0;
        for a in 1..=13 {
            for b in a..=13 {
                for c in b..=13 {
                    for d in c..=13 {
                        for e in d..=13 {
                            if a == e {
                                continue;
                            }
                            multisets += 1;
                            let ranks = [a, b, c, d, e];
                            let expected = reference_rank_score(ranks);
                            for placement in 0..4 {
                                let mut used = [0u8; 14];
                                let mut cards = ranks.map(|rank| {
                                    let suit = (used[rank as usize] + placement * rank) % 4;
                                    used[rank as usize] += 1;
                                    suit * 13 + rank - 1
                                });
                                cards.rotate_left(placement as usize);
                                let actual =
                                    score(&cards[..4].try_into().unwrap(), cards[4], false);
                                assert_eq!(
                                    (
                                        actual.fifteens,
                                        actual.pairs,
                                        actual.run_length,
                                        actual.run_count
                                    ),
                                    expected,
                                    "ranks {ranks:?}, placement {placement}"
                                );
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(multisets, 6175);
    }

    #[test]
    fn constructed_zero_hands_have_no_accidental_flush_or_nobs() {
        let mut rng = StdRng::seed_from_u64(2221);
        for _ in 0..2000 {
            let cards = zero_hand(&mut rng);
            let hand = cards[..4].try_into().unwrap();
            assert_eq!(score(&hand, cards[4], false).total(), 0, "{cards:?}");
            assert_eq!(score(&hand, cards[4], true).total(), 0, "{cards:?}");
        }
    }

    #[test]
    fn twenty_nine_counts_eight_fifteens_six_pairs_and_nobs() {
        let points = score(&[c(5, 0), c(5, 1), c(5, 2), c(11, 3)], c(5, 3), false);
        assert_eq!(points.fifteens, 8);
        assert_eq!(points.pairs, 6);
        assert_eq!(points.nobs, 1);
        assert_eq!(points.total(), 29);
    }

    #[test]
    fn only_maximal_runs_count_with_rank_multiplicity() {
        let cases = [
            ([c(2, 0), c(3, 1), c(4, 2), c(5, 3)], c(6, 0), 5, 1),
            ([c(2, 0), c(3, 1), c(3, 2), c(4, 3)], c(4, 0), 3, 4),
            ([c(2, 0), c(2, 1), c(2, 2), c(3, 3)], c(4, 0), 3, 3),
            ([c(9, 0), c(10, 1), c(11, 2), c(12, 3)], c(1, 0), 4, 1),
            ([c(11, 0), c(12, 1), c(13, 2), c(1, 3)], c(2, 0), 3, 1),
        ];
        for (hand, starter, length, count) in cases {
            let points = score(&hand, starter, false);
            assert_eq!((points.run_length, points.run_count), (length, count));
        }
    }

    #[test]
    fn hand_and_crib_flushes_and_nobs_use_the_starter_correctly() {
        let hand = [c(1, 0), c(3, 0), c(7, 0), c(11, 0)];
        assert_eq!(score(&hand, c(9, 1), false).flush, 4);
        assert_eq!(score(&hand, c(9, 1), true).flush, 0);
        assert_eq!(score(&hand, c(9, 0), false).flush, 5);
        assert_eq!(score(&hand, c(9, 0), true).flush, 5);
        assert_eq!(score(&hand, c(9, 0), true).nobs, 1);
        assert_eq!(score(&hand, c(9, 1), false).nobs, 0);
        assert_eq!(score(&hand, c(11, 1), false).nobs, 0);
        let three = [c(1, 0), c(3, 0), c(7, 0), c(11, 1)];
        assert_eq!(score(&three, c(9, 0), false).flush, 0);
    }

    #[test]
    fn generated_deals_are_distinct_and_scores_ignore_hand_order() {
        let mut rng = StdRng::seed_from_u64(4772);
        let mut totals = [false; 30];
        for _ in 0..6000 {
            let (mut hand, starter, crib) = deal(&mut rng);
            let mut cards = hand.to_vec();
            cards.push(starter);
            cards.sort_unstable();
            cards.dedup();
            assert_eq!(cards.len(), 5);
            assert!(cards.iter().all(|&c| c < 52));
            let points = score(&hand, starter, crib);
            assert!(points.total() <= 29);
            totals[points.total() as usize] = true;
            hand.shuffle(&mut rng);
            assert_eq!(score(&hand, starter, crib), points);
        }
        assert!(totals[0] && totals[16] && totals[24]);
        for impossible in [19, 25, 26, 27] {
            assert!(!totals[impossible]);
        }
    }
}
