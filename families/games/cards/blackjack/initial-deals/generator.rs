use rand::{rngs::StdRng, seq::SliceRandom};

fn total(cards: &[u8]) -> (u8, bool) {
    let low: u8 = cards.iter().map(|c| (c % 13 + 1).min(10)).sum();
    let soft = cards.iter().any(|c| c % 13 == 0) && low + 10 <= 21;
    (low + if soft { 10 } else { 0 }, soft)
}
fn card(c: u8) -> String {
    format!(
        "{}{}",
        [
            "A", "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K"
        ][(c % 13) as usize],
        ["C", "D", "H", "S"][(c / 13) as usize]
    )
}
pub fn generate(rng: &mut StdRng) -> String {
    let mut deck: Vec<u8> = (0..52).collect();
    deck.shuffle(rng);
    let player = [deck[0], deck[2]];
    let dealer = [deck[1], deck[3]];
    let (value, soft) = total(&player);
    format!(
        "Blackjack\nPlayer: {} {} ({value}, {}{})\nDealer: {} ??",
        card(player[0]),
        card(player[1]),
        if soft { "soft" } else { "hard" },
        if value == 21 { ", natural" } else { "" },
        card(dealer[0])
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ace_adjustment() {
        assert_eq!(total(&[0, 13]), (12, true));
        assert_eq!(total(&[0, 9]), (21, true));
        assert_eq!(total(&[9, 22]), (20, false));
        assert_eq!(total(&[0, 9, 22]), (21, false));
    }
}
