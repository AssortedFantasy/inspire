use rand::{Rng, rngs::StdRng, seq::SliceRandom};
mod rank;

fn best(cards: &[u8; 7]) -> (u8, Vec<u8>) {
    let mut highest = (0, Vec::new());
    for omit_a in 0..7 {
        for omit_b in omit_a + 1..7 {
            let hand: Vec<_> = cards
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != omit_a && i != omit_b)
                .map(|(_, &card)| card)
                .collect();
            highest = highest.max(rank::rank(&hand));
        }
    }
    highest
}

fn card(c: u8) -> String {
    format!(
        "{}{}",
        [
            "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"
        ][(c % 13) as usize],
        ["C", "D", "H", "S"][(c / 13) as usize]
    )
}

fn deal(rng: &mut StdRng) -> ([u8; 5], Vec<[u8; 2]>) {
    let mut deck: Vec<u8> = (0..52).collect();
    deck.shuffle(rng);
    let board = deck[..5].try_into().unwrap();
    let players = (0..rng.random_range(2..=6))
        .map(|i| [deck[5 + i * 2], deck[6 + i * 2]])
        .collect();
    (board, players)
}

pub fn generate(rng: &mut StdRng) -> String {
    let (board, players) = deal(rng);
    let ranks: Vec<_> = players
        .iter()
        .map(|hand| {
            best(&[
                board[0], board[1], board[2], board[3], board[4], hand[0], hand[1],
            ])
        })
        .collect();
    let highest = ranks.iter().max().unwrap();
    let winners: Vec<_> = ranks
        .iter()
        .enumerate()
        .filter(|(_, rank)| *rank == highest)
        .map(|(i, _)| format!("P{}", i + 1))
        .collect();
    let mut text = format!(
        "Texas hold'em\nBoard: {}",
        board.into_iter().map(card).collect::<Vec<_>>().join(" ")
    );
    for (i, hand) in players.iter().enumerate() {
        let category = [
            "high card",
            "one pair",
            "two pair",
            "three of a kind",
            "straight",
            "flush",
            "full house",
            "four of a kind",
            "straight flush",
        ][ranks[i].0 as usize];
        text.push_str(&format!(
            "\nP{}: {} {} ({category})",
            i + 1,
            card(hand[0]),
            card(hand[1])
        ));
    }
    if winners.len() == 1 {
        text.push_str(&format!("\n{} wins", winners[0]));
    } else {
        text.push_str(&format!("\n{} split", winners.join(", ")));
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn best_hand_may_play_the_board_or_use_both_hole_cards() {
        assert_eq!(best(&[8, 9, 10, 11, 12, 13, 27]), (8, vec![14]));
        assert_eq!(best(&[0, 13, 26, 1, 14, 27, 12]), (6, vec![3, 2]));
        assert_eq!(best(&[12, 13, 27, 41, 3, 6, 20]), (4, vec![5]));
        let board = [8, 9, 10, 11, 12];
        assert_eq!(
            best(&[board[0], board[1], board[2], board[3], board[4], 13, 27]),
            best(&[board[0], board[1], board[2], board[3], board[4], 28, 42])
        );
    }
    #[test]
    fn every_deal_conserves_distinct_cards_and_evaluates_all_players() {
        let mut rng = StdRng::seed_from_u64(371);
        for _ in 0..1000 {
            let (board, players) = deal(&mut rng);
            assert!((2..=6).contains(&players.len()));
            let mut cards: Vec<_> = board
                .into_iter()
                .chain(players.iter().flatten().copied())
                .collect();
            let count = cards.len();
            cards.sort_unstable();
            cards.dedup();
            assert_eq!(cards.len(), count);
            assert!(cards.into_iter().all(|card| card < 52));
            for hand in players {
                let rank = best(&[
                    board[0], board[1], board[2], board[3], board[4], hand[0], hand[1],
                ]);
                assert!(rank.0 <= 8 && !rank.1.is_empty());
            }
        }
    }
}
