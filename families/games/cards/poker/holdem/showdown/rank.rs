pub(super) fn rank(hand: &[u8]) -> (u8, Vec<u8>) {
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
