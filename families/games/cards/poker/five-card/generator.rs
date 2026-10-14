use rand::{rngs::StdRng, seq::SliceRandom};

fn rank(hand: &[u8]) -> (u8, Vec<u8>) {
    let mut counts = [0u8; 15];
    for &card in hand {
        counts[(card % 13 + 2) as usize] += 1;
    }
    let mut groups: Vec<(u8, u8)> = (2..=14)
        .filter(|&r| counts[r] > 0)
        .map(|r| (counts[r], r as u8))
        .collect();
    groups.sort_unstable_by(|a, b| b.cmp(a));
    let flush = hand.iter().all(|c| c / 13 == hand[0] / 13);
    let mut ranks: Vec<_> = groups.iter().map(|&(_, r)| r).collect();
    ranks.sort_unstable_by(|a, b| b.cmp(a));
    let straight = if ranks == [14, 5, 4, 3, 2] {
        Some(5)
    } else if ranks.len() == 5 && ranks[0] - ranks[4] == 4 {
        Some(ranks[0])
    } else {
        None
    };
    let grouped: Vec<_> = groups.iter().map(|&(_, r)| r).collect();
    if let Some(high) = straight
        && flush
    {
        (8, vec![high])
    } else if groups[0].0 == 4 {
        (7, grouped)
    } else if groups[0].0 == 3 && groups[1].0 == 2 {
        (6, grouped)
    } else if flush {
        (5, ranks)
    } else if let Some(high) = straight {
        (4, vec![high])
    } else if groups[0].0 == 3 {
        (3, grouped)
    } else if groups[0].0 == 2 && groups[1].0 == 2 {
        (2, grouped)
    } else if groups[0].0 == 2 {
        (1, grouped)
    } else {
        (0, ranks)
    }
}

fn card(card: u8) -> String {
    format!(
        "{}{}",
        [
            "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"
        ][(card % 13) as usize],
        ["C", "D", "H", "S"][(card / 13) as usize]
    )
}

pub fn generate(rng: &mut StdRng) -> String {
    let mut deck: Vec<u8> = (0..52).collect();
    deck.shuffle(rng);
    let hand = &deck[..5];
    let (category, tiebreak) = rank(hand);
    format!(
        "Five-card poker; one standard 52-card deck, no jokers.\nHand: {}\nCategory: {}\nTie-break ranks (descending priority; ace=14, wheel high=5): {}",
        hand.iter().map(|&c| card(c)).collect::<Vec<_>>().join(" "),
        [
            "high card",
            "one pair",
            "two pair",
            "three of a kind",
            "straight",
            "flush",
            "full house",
            "four of a kind",
            "straight flush"
        ][category as usize],
        tiebreak
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn categories_and_wheel_tiebreak() {
        let hands = [
            ([0, 14, 28, 42, 6], 0),
            ([0, 13, 28, 42, 6], 1),
            ([0, 13, 1, 14, 6], 2),
            ([0, 13, 26, 42, 6], 3),
            ([0, 14, 28, 42, 4], 4),
            ([0, 2, 4, 6, 8], 5),
            ([0, 13, 26, 1, 14], 6),
            ([0, 13, 26, 39, 6], 7),
            ([0, 1, 2, 3, 4], 8),
        ];
        for (hand, category) in hands {
            assert_eq!(rank(&hand).0, category);
        }
        assert_eq!(rank(&[12, 13, 27, 41, 3]), (4, vec![5]));
        assert!(rank(&[0, 13, 26, 39, 12]) > rank(&[0, 13, 26, 39, 11]));
    }
}
