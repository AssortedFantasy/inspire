use rand::{Rng, rngs::StdRng, seq::SliceRandom};

#[derive(Clone, Copy, Debug)]
struct Bidder {
    value: i64,
    bid: i64,
}

struct Auction {
    bidders: Vec<Bidder>,
    reserve: i64,
    priority: Vec<usize>,
}

#[derive(Clone, Copy, Debug)]
enum Rule {
    FirstPrice,
    SecondPrice,
    AllPay,
}

const RULES: [Rule; 3] = [Rule::FirstPrice, Rule::SecondPrice, Rule::AllPay];

struct Outcome {
    winner: Option<usize>,
    payments: Vec<i64>,
    utilities: Vec<i64>,
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..9);
    let auction = sample(rng, regime);
    render(&auction)
}

fn sample(rng: &mut StdRng, regime: usize) -> Auction {
    let n = rng.random_range(3..=6);
    let level = rng.random_range(30..=110);
    let mut bidders: Vec<_> = (0..n)
        .map(|_| {
            let value = rng.random_range(10..=level);
            Bidder {
                value,
                bid: rng.random_range(0..=value.min(level - 5)),
            }
        })
        .collect();
    let mut reserve = 0;
    match regime {
        0 => {
            bidders[0] = Bidder {
                value: level + rng.random_range(20..=40),
                bid: level + rng.random_range(1..=10),
            };
        }
        1 => {
            bidders[0] = Bidder {
                value: level + rng.random_range(40..=80),
                bid: level - rng.random_range(2..=10),
            };
            bidders[1] = Bidder {
                value: level + 15,
                bid: level + 5,
            };
        }
        2 => {
            reserve = level;
            bidders[0] = Bidder {
                value: level + rng.random_range(10..=50),
                bid: level - 1,
            };
        }
        3 => {
            reserve = level;
            bidders[0] = Bidder {
                value: level + rng.random_range(10..=50),
                bid: level + rng.random_range(1..=9),
            };
        }
        4 => {
            reserve = level;
            bidders[0] = Bidder {
                value: level,
                bid: level,
            };
        }
        5 => {
            reserve = level - rng.random_range(0..=10);
            let tied = rng.random_range(2..=n);
            for bidder in bidders.iter_mut().take(tied) {
                *bidder = Bidder {
                    value: level + rng.random_range(1..=40),
                    bid: level,
                };
            }
        }
        6 => {
            for (i, bidder) in bidders.iter_mut().enumerate() {
                let bid = level - i as i64;
                *bidder = Bidder {
                    value: bid + rng.random_range(0..=4),
                    bid,
                };
            }
            bidders[0].value = level + 5;
        }
        7 => {
            bidders[0] = Bidder {
                value: level + 2,
                bid: level + 10,
            };
            bidders[1] = Bidder {
                value: level + 20,
                bid: level - 1,
            };
        }
        8 => {
            reserve = rng.random_range(0..=170);
            for bidder in &mut bidders {
                *bidder = Bidder {
                    value: rng.random_range(0..=150),
                    bid: rng.random_range(0..=150),
                };
            }
        }
        _ => unreachable!(),
    }
    bidders.shuffle(rng);
    let mut priority: Vec<_> = (0..n).collect();
    priority.shuffle(rng);
    Auction {
        bidders,
        reserve,
        priority,
    }
}

fn resolve(auction: &Auction, rule: Rule) -> Outcome {
    let mut winner: Option<usize> = None;
    for &i in &auction.priority {
        let bid = auction.bidders[i].bid;
        if bid >= auction.reserve && winner.is_none_or(|w| bid > auction.bidders[w].bid) {
            winner = Some(i);
        }
    }
    let mut payments = vec![0; auction.bidders.len()];
    match rule {
        Rule::AllPay => {
            for (payment, bidder) in payments.iter_mut().zip(&auction.bidders) {
                *payment = bidder.bid;
            }
        }
        Rule::FirstPrice | Rule::SecondPrice => {
            if let Some(w) = winner {
                payments[w] = match rule {
                    Rule::FirstPrice => auction.bidders[w].bid,
                    Rule::SecondPrice => auction
                        .bidders
                        .iter()
                        .enumerate()
                        .filter(|&(i, _)| i != w)
                        .map(|(_, bidder)| bidder.bid)
                        .max()
                        .unwrap_or(0)
                        .max(auction.reserve),
                    Rule::AllPay => unreachable!(),
                };
            }
        }
    }
    let utilities = auction
        .bidders
        .iter()
        .zip(&payments)
        .enumerate()
        .map(|(i, (bidder, payment))| {
            if winner == Some(i) {
                bidder.value - payment
            } else {
                -payment
            }
        })
        .collect();
    Outcome {
        winner,
        payments,
        utilities,
    }
}

fn name(i: usize) -> char {
    char::from(b'A' + i as u8)
}

fn vector(values: &[i64]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn render(auction: &Auction) -> String {
    let mut lines = vec![
        format!("{} private-value bidders, one item", auction.bidders.len()),
        format!("Reserve {}", auction.reserve),
        format!(
            "Tie priority {}",
            auction
                .priority
                .iter()
                .map(|&i| name(i).to_string())
                .collect::<Vec<_>>()
                .join(" > ")
        ),
        "All-pay forfeits every bid, including when the item is unsold.".to_owned(),
        "\nBidder   Private value   Submitted bid".to_owned(),
    ];
    for (i, bidder) in auction.bidders.iter().enumerate() {
        lines.push(format!(
            "{:<6}   {:>13}   {:>13}",
            name(i),
            bidder.value,
            bidder.bid
        ));
    }
    let names = (0..auction.bidders.len())
        .map(|i| name(i).to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let mut rows = vec![[
        "Rule".to_owned(),
        "Winner".to_owned(),
        "Seller revenue".to_owned(),
        format!("Payments [{names}]"),
        format!("Utilities [{names}]"),
    ]];
    for rule in RULES {
        let result = resolve(auction, rule);
        rows.push([
            match rule {
                Rule::FirstPrice => "First-price",
                Rule::SecondPrice => "Second-price",
                Rule::AllPay => "All-pay",
            }
            .to_owned(),
            result
                .winner
                .map_or_else(|| "unsold".to_owned(), |i| name(i).to_string()),
            result.payments.iter().sum::<i64>().to_string(),
            vector(&result.payments),
            vector(&result.utilities),
        ]);
    }
    let mut widths = [0; 5];
    for row in &rows {
        for (width, cell) in widths.iter_mut().zip(row) {
            *width = (*width).max(cell.len());
        }
    }
    lines.push(String::new());
    for row in rows {
        lines.push(format!(
            "{:<w0$}  {:<w1$}  {:>w2$}  {:>w3$}  {:>w4$}",
            row[0],
            row[1],
            row[2],
            row[3],
            row[4],
            w0 = widths[0],
            w1 = widths[1],
            w2 = widths[2],
            w3 = widths[3],
            w4 = widths[4],
        ));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::cmp::Reverse;

    const PRIORITIES: [[usize; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];

    #[test]
    fn exhaustive_small_bids_match_sorted_order_statistics_and_conserve_surplus() {
        for encoded in 0..64 {
            let bids = [encoded % 4, encoded / 4 % 4, encoded / 16];
            for reserve in 0..=4 {
                for priority in PRIORITIES {
                    let auction = Auction {
                        bidders: bids
                            .iter()
                            .zip([1, 3, 2])
                            .map(|(&bid, value)| Bidder { value, bid })
                            .collect(),
                        reserve,
                        priority: priority.to_vec(),
                    };
                    let mut ordered = priority;
                    ordered.sort_by_key(|&i| Reverse(bids[i]));
                    let expected = (bids[ordered[0]] >= reserve).then_some(ordered[0]);
                    let mut prices = bids;
                    prices.sort();
                    for rule in RULES {
                        let result = resolve(&auction, rule);
                        assert_eq!(result.winner, expected);
                        for (i, &bid) in bids.iter().enumerate() {
                            let expected_payment = match rule {
                                Rule::AllPay => bid,
                                Rule::FirstPrice if expected == Some(i) => prices[2],
                                Rule::SecondPrice if expected == Some(i) => prices[1].max(reserve),
                                _ => 0,
                            };
                            assert_eq!(result.payments[i], expected_payment);
                            assert_eq!(
                                result.utilities[i] + result.payments[i],
                                if expected == Some(i) {
                                    auction.bidders[i].value
                                } else {
                                    0
                                }
                            );
                        }
                        assert_eq!(
                            result.utilities.iter().sum::<i64>()
                                + result.payments.iter().sum::<i64>(),
                            expected.map_or(0, |w| auction.bidders[w].value)
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn second_price_truthful_bid_weakly_dominates_every_small_deviation() {
        for value in 0..=4 {
            for left_bid in 0..=4 {
                for right_bid in 0..=4 {
                    for reserve in 0..=5 {
                        for priority in PRIORITIES {
                            let mut auction = Auction {
                                bidders: vec![
                                    Bidder { value, bid: value },
                                    Bidder {
                                        value: 0,
                                        bid: left_bid,
                                    },
                                    Bidder {
                                        value: 0,
                                        bid: right_bid,
                                    },
                                ],
                                reserve,
                                priority: priority.to_vec(),
                            };
                            let truthful = resolve(&auction, Rule::SecondPrice).utilities[0];
                            for bid in 0..=5 {
                                auction.bidders[0].bid = bid;
                                let deviation = resolve(&auction, Rule::SecondPrice).utilities[0];
                                assert!(truthful >= deviation);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn constructed_profiles_reach_their_promised_boundaries() {
        let mut rng = StdRng::seed_from_u64(0xa0c710);
        for regime in 0..9 {
            for _ in 0..128 {
                let auction = sample(&mut rng, regime);
                let first = resolve(&auction, Rule::FirstPrice);
                let second = resolve(&auction, Rule::SecondPrice);
                let all_pay = resolve(&auction, Rule::AllPay);
                let highest = auction.bidders.iter().map(|b| b.value).max().unwrap();
                let winner = first.winner;
                let eligible: Vec<_> = auction
                    .bidders
                    .iter()
                    .filter(|b| b.bid >= auction.reserve)
                    .collect();
                match regime {
                    0 => {
                        let w = winner.unwrap();
                        assert_eq!(auction.bidders[w].value, highest);
                        assert!(first.utilities[w] > 0);
                    }
                    1 => {
                        assert!(auction.bidders[winner.unwrap()].value < highest);
                        assert!(auction.bidders.iter().all(|b| b.bid <= b.value));
                    }
                    2 => {
                        assert!(winner.is_none());
                        assert!(highest > auction.reserve);
                        assert!(auction.bidders.iter().all(|b| b.value > 0));
                        assert_eq!(first.payments.iter().sum::<i64>(), 0);
                        assert_eq!(second.payments.iter().sum::<i64>(), 0);
                        assert!(all_pay.utilities.iter().sum::<i64>() < 0);
                    }
                    3 => {
                        assert_eq!(eligible.len(), 1);
                        let w = winner.unwrap();
                        assert!(auction.bidders[w].bid > auction.reserve);
                        assert_eq!(second.payments[w], auction.reserve);
                    }
                    4 => {
                        assert_eq!(eligible.len(), 1);
                        let w = winner.unwrap();
                        assert_eq!(auction.bidders[w].bid, auction.reserve);
                        assert_eq!(first.utilities[w], 0);
                        assert_eq!(second.utilities[w], 0);
                    }
                    5 => {
                        let top_bid = auction.bidders[winner.unwrap()].bid;
                        let tied: Vec<_> = auction
                            .priority
                            .iter()
                            .copied()
                            .filter(|&i| auction.bidders[i].bid == top_bid)
                            .collect();
                        assert!(tied.len() >= 2);
                        assert_eq!(winner, Some(tied[0]));
                        assert_eq!(first.payments, second.payments);
                    }
                    6 => {
                        assert!(auction.bidders.iter().all(|b| b.bid <= b.value));
                        assert!(first.utilities.iter().sum::<i64>() > 0);
                        assert!(all_pay.utilities.iter().sum::<i64>() < 0);
                    }
                    7 => {
                        let w = winner.unwrap();
                        assert!(first.utilities[w] < 0);
                        assert!(second.utilities[w] > 0);
                        assert!(all_pay.utilities[w] < 0);
                    }
                    _ => (),
                }
            }
        }
    }
}
