use rand::{Rng, rngs::StdRng};
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rational(i64, i64);

impl Rational {
    fn new(n: i64, d: i64) -> Self {
        assert!(d > 0);
        let (mut a, mut b) = (n.abs(), d);
        while b != 0 {
            (a, b) = (b, a % b);
        }
        Self(n / a, d / a)
    }

    fn integer(n: i64) -> Self {
        Self(n, 1)
    }

    fn text(self) -> String {
        if self.1 == 1 {
            self.0.to_string()
        } else {
            format!("{}/{}", self.0, self.1)
        }
    }
}

impl Add for Rational {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.0 * rhs.1 + rhs.0 * self.1, self.1 * rhs.1)
    }
}

impl Sub for Rational {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.0 * rhs.1 - rhs.0 * self.1, self.1 * rhs.1)
    }
}

impl Mul for Rational {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Div for Rational {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        assert!(rhs.0 > 0);
        Self::new(self.0 * rhs.1, self.1 * rhs.0)
    }
}

impl Ord for Rational {
    fn cmp(&self, rhs: &Self) -> Ordering {
        (self.0 * rhs.1).cmp(&(rhs.0 * self.1))
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

#[derive(Clone, Copy, Debug)]
struct Market {
    a: i64,
    b: i64,
    c: i64,
    s: i64,
    capacity: Option<i64>,
}

#[derive(Clone, Copy, Debug)]
struct Clearing {
    q: Rational,
    buyer: Rational,
    seller: Rational,
    capacity_rent: Rational,
}

impl Market {
    fn clear(self, tax: i64) -> Clearing {
        let zero = Rational::integer(0);
        let mut q = Rational::new((self.a - self.c - tax).max(0), self.b + self.s);
        if let Some(capacity) = self.capacity {
            q = q.min(Rational::integer(capacity));
        }
        let buyer = Rational::integer(self.a) - Rational::integer(self.b) * q;
        let seller = buyer - Rational::integer(tax);
        let marginal_cost = Rational::integer(self.c) + Rational::integer(self.s) * q;
        Clearing {
            q,
            buyer,
            seller,
            capacity_rent: (seller - marginal_cost).max(zero),
        }
    }

    fn surplus(self, q: Rational) -> Rational {
        Rational::integer(self.a - self.c) * q - Rational::new(self.b + self.s, 2) * q * q
    }

    fn demand(self, p: Rational) -> Rational {
        ((Rational::integer(self.a) - p) / Rational::integer(self.b)).max(Rational::integer(0))
    }

    fn supply(self, p: Rational) -> Rational {
        ((p - Rational::integer(self.c)) / Rational::integer(self.s)).max(Rational::integer(0))
    }
}

#[derive(Clone, Copy, Debug)]
enum Policy {
    None,
    Tax(i64),
    Ceiling(Rational),
    Floor(Rational),
}

fn sample(rng: &mut StdRng, regime: usize) -> (Market, Policy) {
    let b = rng.random_range(1..=8);
    let s = rng.random_range(1..=8);
    let c = rng.random_range(0..=12);
    let mut market = Market {
        a: c + rng.random_range(16..=70),
        b,
        c,
        s,
        capacity: None,
    };
    let policy = match regime {
        0 => Policy::None,
        1 => {
            // Strong incidence in either direction, and the equal-slope boundary.
            match rng.random_range(0..3) {
                0 => market.b = market.s,
                1 => market.b = 4 * market.s,
                _ => market.s = 4 * market.b,
            }
            Policy::Tax(rng.random_range(1..market.a - c))
        }
        2 | 3 => {
            let base = market.clear(0);
            let gap = if regime == 2 {
                base.buyer - Rational::integer(c)
            } else {
                Rational::integer(market.a) - base.buyer
            };
            let delta = gap / Rational::integer(rng.random_range(2..=4));
            let offset = match rng.random_range(0..3) {
                0 => Rational::integer(0),
                1 => delta,
                _ => Rational::integer(0) - delta,
            };
            if regime == 2 {
                Policy::Ceiling(base.buyer + offset)
            } else {
                Policy::Floor(base.buyer + offset)
            }
        }
        4 => {
            let gap = market.a - c;
            Policy::Tax(match rng.random_range(0..3) {
                0 => gap - 1,
                1 => gap,
                _ => gap + rng.random_range(1..=12),
            })
        }
        _ => {
            let capacity = rng.random_range(3..=12);
            let rent = rng.random_range(2..=16);
            market.a = c + (b + s) * capacity + rent;
            market.capacity = Some(capacity);
            Policy::Tax(match rng.random_range(0..4) {
                0 => rent - 1,
                1 => rent,
                2 => rent + 1,
                _ => rent + (b + s) * rng.random_range(1..capacity),
            })
        }
    };
    (market, policy)
}

fn render(market: Market, policy: Policy) -> String {
    let base = market.clear(0);
    let term = |slope| {
        if slope == 1 {
            "Q".to_owned()
        } else {
            format!("{slope}Q")
        }
    };
    let mut lines = vec![
        format!("Demand P_b = {} - {}", market.a, term(market.b)),
        if market.c == 0 {
            format!("Supply P_s = {}", term(market.s))
        } else {
            format!("Supply P_s = {} + {}", market.c, term(market.s))
        },
    ];
    if let Some(capacity) = market.capacity {
        lines.push(format!("Hard capacity Q ≤ {capacity}"));
    }
    lines.push(format!(
        "\nCompetitive price {}, quantity {}",
        base.buyer.text(),
        base.q.text()
    ));
    match policy {
        Policy::None => {
            let cs = (Rational::integer(market.a) - base.buyer) * base.q / Rational::integer(2);
            let ps = market.surplus(base.q) - cs;
            lines.push(format!(
                "Consumer surplus {}, producer surplus {}",
                cs.text(),
                ps.text()
            ));
        }
        Policy::Tax(tax) => {
            let after = market.clear(tax);
            lines.push(format!("Per-unit tax {tax}"));
            if after.q == Rational::integer(0) {
                lines.push(format!(
                    "No trade. Tax {} the reservation-price gap {}.",
                    if tax == market.a - market.c {
                        "equals"
                    } else {
                        "exceeds"
                    },
                    market.a - market.c
                ));
            } else {
                lines.push(format!(
                    "After tax Q {}, buyer price {}, seller receipt {}",
                    after.q.text(),
                    after.buyer.text(),
                    after.seller.text()
                ));
                lines.push(format!(
                    "Buyer price rise {}, seller receipt fall {}",
                    (after.buyer - base.buyer).text(),
                    (base.seller - after.seller).text()
                ));
            }
            if let Some(capacity) = market.capacity {
                lines.push(format!(
                    "Capacity {}. Marginal capacity rent {} → {}",
                    if after.q == Rational::integer(capacity) {
                        "fully used"
                    } else {
                        "slack"
                    },
                    base.capacity_rent.text(),
                    after.capacity_rent.text()
                ));
            }
            lines.push(format!(
                "Tax revenue {}, surplus loss {}",
                (Rational::integer(tax) * after.q).text(),
                (market.surplus(base.q) - market.surplus(after.q)).text()
            ));
        }
        Policy::Ceiling(limit) | Policy::Floor(limit) => {
            let ceiling = matches!(policy, Policy::Ceiling(_));
            let binding = if ceiling {
                limit < base.buyer
            } else {
                limit > base.buyer
            };
            let state = if binding {
                "binding"
            } else if limit == base.buyer {
                "at competitive price"
            } else {
                "nonbinding"
            };
            lines.push(format!(
                "Price {} {} ({state})",
                if ceiling { "ceiling" } else { "floor" },
                limit.text()
            ));
            let p = if binding { limit } else { base.buyer };
            let demand = market.demand(p);
            let supply = market.supply(p);
            lines.push(format!(
                "Price {}, quantity demanded {}, quantity offered {}",
                p.text(),
                demand.text(),
                supply.text()
            ));
            lines.push(format!("Quantity traded {}", demand.min(supply).text()));
            if binding {
                lines.push(if ceiling {
                    format!("Shortage {}", (demand - supply).text())
                } else {
                    format!("Unsold supply {}", (supply - demand).text())
                });
                lines.push("Short side fully trades. No government purchases.".to_owned());
            }
        }
    }
    lines.join("\n")
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..6);
    let (market, policy) = sample(rng, regime);
    render(market, policy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn clearing_satisfies_wedge_and_supply_complementarity() {
        let mut rng = StdRng::seed_from_u64(16_038);
        for regime in 0..6 {
            for _ in 0..120 {
                let (market, policy) = sample(&mut rng, regime);
                let tax = if let Policy::Tax(t) = policy { t } else { 0 };
                let result = market.clear(tax);
                let q = result.q;
                assert!(q >= Rational::integer(0));
                if q == Rational::integer(0) {
                    assert!(tax >= market.a - market.c);
                    continue;
                }
                assert_eq!(result.buyer - result.seller, Rational::integer(tax));
                assert_eq!(market.demand(result.buyer), q);
                let cost = Rational::integer(market.c) + Rational::integer(market.s) * q;
                if let Some(capacity) = market.capacity {
                    assert!(q <= Rational::integer(capacity));
                    if q < Rational::integer(capacity) {
                        assert_eq!(result.seller, cost);
                    } else {
                        assert!(result.seller >= cost);
                    }
                } else {
                    assert_eq!(result.seller, cost);
                    assert_eq!(market.supply(result.seller), q);
                }
            }
        }
    }

    #[test]
    fn tax_transfers_and_welfare_balance() {
        let market = Market {
            a: 31,
            b: 3,
            c: 7,
            s: 5,
            capacity: None,
        };
        let before = market.clear(0);
        for tax in 0..=24 {
            let after = market.clear(tax);
            let cs = (Rational::integer(market.a) - after.buyer) * after.q / Rational::integer(2);
            let ps = (after.seller - Rational::integer(market.c)) * after.q / Rational::integer(2);
            let revenue = Rational::integer(tax) * after.q;
            assert_eq!(cs + ps + revenue, market.surplus(after.q));
            let reduction = before.q - after.q;
            assert_eq!(
                market.surplus(before.q) - market.surplus(after.q),
                Rational::new(market.b + market.s, 2) * reduction * reduction
            );
        }
    }

    #[test]
    fn capacity_tax_boundary_and_shutdown_are_exact() {
        let market = Market {
            a: 34,
            b: 2,
            c: 4,
            s: 3,
            capacity: Some(5),
        };
        assert_eq!(market.clear(0).q, Rational::integer(5));
        assert_eq!(market.clear(4).capacity_rent, Rational::integer(1));
        assert_eq!(market.clear(5).capacity_rent, Rational::integer(0));
        assert_eq!(market.clear(5).q, Rational::integer(5));
        assert_eq!(market.clear(6).q, Rational::new(24, 5));
        assert_eq!(market.clear(30).q, Rational::integer(0));
        assert_eq!(market.clear(31).q, Rational::integer(0));
    }

    #[test]
    fn control_short_side_and_boundary_match_independent_equations() {
        let market = Market {
            a: 40,
            b: 2,
            c: 4,
            s: 4,
            capacity: None,
        };
        let price = market.clear(0).buyer;
        assert_eq!(price, Rational::integer(28));
        for p in [Rational::integer(16), price, Rational::integer(34)] {
            let qd = market.demand(p);
            let qs = market.supply(p);
            assert_eq!(Rational::integer(2) * qd + p, Rational::integer(40));
            assert_eq!(Rational::integer(4) * qs + Rational::integer(4), p);
            assert_eq!(qd.cmp(&qs), price.cmp(&p));
        }
    }
}
