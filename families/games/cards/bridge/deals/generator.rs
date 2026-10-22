use rand::{rngs::StdRng, seq::SliceRandom};

fn hcp(cards: &[u8]) -> u8 {
    cards.iter().map(|c| (c % 13).saturating_sub(8)).sum()
}

fn hand(cards: &[u8]) -> String {
    (0..4)
        .rev()
        .map(|suit| {
            let mut ranks: Vec<_> = cards
                .iter()
                .filter(|&&c| c / 13 == suit)
                .map(|c| c % 13)
                .collect();
            ranks.sort_unstable_by(|a, b| b.cmp(a));
            let text = ranks
                .iter()
                .map(|&r| {
                    [
                        "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A",
                    ][r as usize]
                })
                .collect::<Vec<_>>()
                .join("");
            format!(
                "{}:{}",
                ["C", "D", "H", "S"][suit as usize],
                if text.is_empty() { "-" } else { &text }
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn generate(rng: &mut StdRng) -> String {
    let mut deck: Vec<u8> = (0..52).collect();
    deck.shuffle(rng);
    let mut text = String::from("Bridge");
    for (i, name) in ["North", "East", "South", "West"].iter().enumerate() {
        let cards = &deck[i * 13..(i + 1) * 13];
        text.push_str(&format!("\n{name}: {} ({} HCP)", hand(cards), hcp(cards)));
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn high_card_points_conserve_forty() {
        let cards: Vec<_> = (0..52).collect();
        assert_eq!(hcp(&cards), 40);
        assert_eq!(hcp(&[9, 10, 11, 12]), 10);
        assert_eq!(hand(&[12, 25, 38, 51]), "S:A H:A D:A C:A");
    }
}
