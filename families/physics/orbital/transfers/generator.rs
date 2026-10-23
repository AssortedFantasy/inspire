use rand::{Rng, rngs::StdRng};
use std::f64::consts::{PI, TAU};

include!("../../number.rs");

const EARTH_MU: f64 = 398_600.435_507;
const SUN_MU: f64 = 132_712_440_041.279_42;
const AU: f64 = 149_597_870.7;
const DAY: f64 = 86_400.0;

#[derive(Clone, Copy)]
struct Center {
    name: &'static str,
    mu: f64,
    low_radius: u32,
}

const CENTERS: [Center; 3] = [
    Center {
        name: "Earth",
        mu: EARTH_MU,
        low_radius: 6_600,
    },
    Center {
        name: "Moon",
        mu: 4_902.800_118,
        low_radius: 1_850,
    },
    Center {
        name: "Mars",
        mu: 42_828.375_816,
        low_radius: 3_700,
    },
];

fn circular(mu: f64, radius: f64) -> f64 {
    (mu / radius).sqrt()
}

fn apsis_speed(mu: f64, here: f64, other: f64) -> f64 {
    (2.0 * mu * other / (here * (here + other))).sqrt()
}

fn half_period(mu: f64, r1: f64, r2: f64) -> f64 {
    PI * (((r1 + r2) / 2.0).powi(3) / mu).sqrt()
}

#[derive(Clone, Copy)]
struct Hohmann {
    r: [f64; 2],
    circular: [f64; 2],
    transfer: [f64; 2],
    burns: [f64; 2],
    coast: f64,
}

impl Hohmann {
    fn new(mu: f64, r: [f64; 2]) -> Self {
        let circular = r.map(|radius| circular(mu, radius));
        let transfer = [apsis_speed(mu, r[0], r[1]), apsis_speed(mu, r[1], r[0])];
        Self {
            r,
            circular,
            transfer,
            burns: [transfer[0] - circular[0], circular[1] - transfer[1]],
            coast: half_period(mu, r[0], r[1]),
        }
    }

    fn total(self) -> f64 {
        self.burns.iter().map(|v| v.abs()).sum()
    }

    fn inclined_burns(self, turn: f64, split: f64) -> [f64; 2] {
        [
            vector_burn(self.circular[0], self.transfer[0], split),
            vector_burn(self.transfer[1], self.circular[1], turn - split),
        ]
    }

    fn inclined_total(self, turn: f64, split: f64) -> f64 {
        self.inclined_burns(turn, split).iter().sum()
    }
}

struct BiElliptic {
    burns: [f64; 3],
    coast: f64,
}

impl BiElliptic {
    fn new(mu: f64, r: [f64; 2], apex: f64) -> Self {
        Self {
            burns: [
                apsis_speed(mu, r[0], apex) - circular(mu, r[0]),
                apsis_speed(mu, apex, r[1]) - apsis_speed(mu, apex, r[0]),
                circular(mu, r[1]) - apsis_speed(mu, r[1], apex),
            ],
            coast: half_period(mu, r[0], apex) + half_period(mu, r[1], apex),
        }
    }

    fn total(&self) -> f64 {
        self.burns.iter().map(|v| v.abs()).sum()
    }
}

fn vector_burn(before: f64, after: f64, angle: f64) -> f64 {
    (before - after).hypot(2.0 * (before * after).sqrt() * (angle / 2.0).sin())
}

fn minimum_split(transfer: Hohmann, turn: f64) -> f64 {
    // The objective can have two minima near opposite ends. Bracket all sampled
    // minima, including both endpoint intervals, before refining each bracket.
    const GRID: usize = 128;
    let costs: [f64; GRID + 1] =
        std::array::from_fn(|i| transfer.inclined_total(turn, turn * i as f64 / GRID as f64));
    let mut best = 0.0;
    let mut best_cost = costs[0];
    if costs[GRID] < best_cost {
        best = turn;
        best_cost = costs[GRID];
    }
    for i in 0..=GRID {
        if i > 0 && i < GRID && !(costs[i] <= costs[i - 1] && costs[i] <= costs[i + 1]) {
            continue;
        }
        let mut low = turn * i.saturating_sub(1) as f64 / GRID as f64;
        let mut high = turn * (i + 1).min(GRID) as f64 / GRID as f64;
        let golden = (5.0_f64.sqrt() - 1.0) / 2.0;
        let mut a = high - golden * (high - low);
        let mut b = low + golden * (high - low);
        let mut fa = transfer.inclined_total(turn, a);
        let mut fb = transfer.inclined_total(turn, b);
        for _ in 0..56 {
            if fa < fb {
                high = b;
                b = a;
                fb = fa;
                a = high - golden * (high - low);
                fa = transfer.inclined_total(turn, a);
            } else {
                low = a;
                a = b;
                fa = fb;
                b = low + golden * (high - low);
                fb = transfer.inclined_total(turn, b);
            }
        }
        let split = (low + high) / 2.0;
        let cost = transfer.inclined_total(turn, split);
        if cost < best_cost {
            best = split;
            best_cost = cost;
        }
    }
    best
}

fn signed(value: f64) -> String {
    format!(
        "{}{v}",
        if value < 0.0 { "−" } else { "+" },
        v = n(value.abs())
    )
}

fn duration(seconds: f64) -> String {
    if seconds < 3_600.0 {
        format!("{} min", n(seconds / 60.0))
    } else if seconds < 2.0 * DAY {
        format!("{} h", n(seconds / 3_600.0))
    } else if seconds < 730.5 * DAY {
        format!("{} d", n(seconds / DAY))
    } else {
        format!("{} yr", n(seconds / (365.25 * DAY)))
    }
}

fn coplanar(rng: &mut StdRng, regime: u32) -> String {
    let center = CENTERS[rng.random_range(0..CENTERS.len())];
    let inner = center.low_radius as f64 + rng.random_range(0..=100) as f64 * 10.0;
    let outer = if regime == 2 {
        inner + rng.random_range(2..=50) as f64
    } else if center.name == "Earth" && rng.random_bool(0.5) {
        42_164.0
    } else {
        inner * rng.random_range(15..=85) as f64 / 10.0
    };
    let r = if regime == 1 || (regime == 2 && rng.random_bool(0.5)) {
        [outer, inner]
    } else {
        [inner, outer]
    };
    let transfer = Hohmann::new(center.mu, r);
    format!(
        "Coplanar circular orbits about {}\nRadius {} → {} km\nHohmann ellipse a = {} km, e = {}\nDeparture burn {} km/s\nArrival burn {} km/s\nTotal Δv {} km/s\nCoast {}",
        center.name,
        n(r[0]),
        n(r[1]),
        n((r[0] + r[1]) / 2.0),
        n((r[0] - r[1]).abs() / (r[0] + r[1])),
        signed(transfer.burns[0]),
        signed(transfer.burns[1]),
        n(transfer.total()),
        duration(transfer.coast),
    )
}

fn bi_elliptic(rng: &mut StdRng, regime: u32) -> String {
    let inner = rng.random_range(4..=15) as f64 / 100.0 * AU;
    let ratio = if regime == 3 {
        rng.random_range(1_175..=1_250) as f64 / 100.0
    } else {
        let boundary = [13.0, 15.58, 16.0, 20.0][rng.random_range(0..4)];
        boundary + rng.random_range(-15..=15) as f64 / 100.0
    };
    let outer = inner * ratio;
    let factor = if regime == 3 {
        [2.0, 5.0, 20.0, 100.0][rng.random_range(0..4)]
    } else {
        rng.random_range(105..=1_500) as f64 / 100.0
    };
    let apex = outer * factor;
    let r = if rng.random_bool(0.5) {
        [inner, outer]
    } else {
        [outer, inner]
    };
    let hohmann = Hohmann::new(SUN_MU, r);
    let bi = BiElliptic::new(SUN_MU, r, apex);
    let difference = 1_000.0 * (hohmann.total() - bi.total());
    format!(
        "Circular solar orbits {} → {} au\nOuter/inner radius ratio {}\nBi-elliptic apex {} au\nThree burns {} / {} / {} km/s\nHohmann       Δv {} km/s, coast {}\nBi-elliptic   Δv {} km/s, coast {}\nBi-elliptic {} {} m/s and takes {}× as long",
        n(r[0] / AU),
        n(r[1] / AU),
        n(ratio),
        n(apex / AU),
        signed(bi.burns[0]),
        signed(bi.burns[1]),
        signed(bi.burns[2]),
        n(hohmann.total()),
        duration(hohmann.coast),
        n(bi.total()),
        duration(bi.coast),
        if difference >= 0.0 {
            "saves"
        } else {
            "costs an extra"
        },
        n(difference.abs()),
        n(bi.coast / hohmann.coast),
    )
}

fn inclined(rng: &mut StdRng, regime: u32) -> String {
    let inner = rng.random_range(6_600..=8_000) as f64;
    let outer = if regime == 5 {
        42_164.0
    } else {
        inner * rng.random_range(1_005..=1_150) as f64 / 1_000.0
    };
    let r = if rng.random_bool(0.5) {
        [inner, outer]
    } else {
        [outer, inner]
    };
    let degrees = if regime == 5 {
        [5.0, 28.5, 51.6, 63.4][rng.random_range(0..4)] + rng.random_range(-10..=10) as f64 / 10.0
    } else {
        rng.random_range(400..=1_000) as f64 / 10.0
    };
    let turn = degrees.to_radians();
    let transfer = Hohmann::new(EARTH_MU, r);
    let split = minimum_split(transfer, turn);
    let burns = transfer.inclined_burns(turn, split);
    format!(
        "Circular Earth orbits {} → {} km\nPlane separation {}°\nTransfer apsides on the common nodal line\nAll turn at departure   Δv {} km/s\nAll turn at arrival     Δv {} km/s\nMinimum apsidal split   {}° + {}°\nSplit burns {} + {} km/s = {} km/s\nCoast {}",
        n(transfer.r[0]),
        n(transfer.r[1]),
        n(degrees),
        n(transfer.inclined_total(turn, turn)),
        n(transfer.inclined_total(turn, 0.0)),
        n(split.to_degrees()),
        n((turn - split).to_degrees()),
        n(burns[0]),
        n(burns[1]),
        n(burns.iter().sum()),
        duration(transfer.coast),
    )
}

fn phase_text(angle: f64) -> String {
    let signed = (angle + PI).rem_euclid(TAU) - PI;
    format!(
        "{}° {}",
        n(signed.to_degrees().abs()),
        if signed < 0.0 { "behind" } else { "ahead" },
    )
}

struct Window {
    required: f64,
    synodic: f64,
    wait: f64,
}

fn window(mu: f64, r: [f64; 2], current: f64) -> Window {
    let motion = r.map(|radius| (mu / radius.powi(3)).sqrt());
    let required = (PI - motion[1] * half_period(mu, r[0], r[1])).rem_euclid(TAU);
    let rate = motion[1] - motion[0];
    let synodic = TAU / rate.abs();
    Window {
        required,
        synodic,
        wait: ((required - current) / rate).rem_euclid(synodic),
    }
}

fn rendezvous(rng: &mut StdRng) -> String {
    // Rounded J2000 semimajor axes become radii of the circular model.
    let planets = [
        ("Mercury", 0.3871),
        ("Venus", 0.7233),
        ("Earth", 1.0),
        ("Mars", 1.5237),
        ("Jupiter", 5.2029),
    ];
    let departure = rng.random_range(0..planets.len());
    let other = rng.random_range(0..planets.len() - 1);
    let arrival = other + usize::from(other >= departure);
    let r = [planets[departure].1 * AU, planets[arrival].1 * AU];
    let current = (rng.random_range(0..3_600) as f64 / 10.0).to_radians();
    let transfer = Hohmann::new(SUN_MU, r);
    let timing = window(SUN_MU, r, current);
    format!(
        "{} → {}, circular coplanar solar orbits\nRadii {} → {} au\nHohmann coast {}\nDeparture speed mismatch {} km/s\nArrival speed mismatch {} km/s\nTarget needed {} at departure\nTarget now {}\nNext departure in {}\nWindows repeat every {}",
        planets[departure].0,
        planets[arrival].0,
        n(r[0] / AU),
        n(r[1] / AU),
        duration(transfer.coast),
        n(transfer.burns[0].abs()),
        n(transfer.burns[1].abs()),
        phase_text(timing.required),
        phase_text(current),
        duration(timing.wait),
        duration(timing.synodic),
    )
}

pub fn generate(rng: &mut StdRng) -> String {
    match rng.random_range(0..8) {
        regime @ 0..=2 => coplanar(rng, regime),
        regime @ 3..=4 => bi_elliptic(rng, regime),
        regime @ 5..=6 => inclined(rng, regime),
        _ => rendezvous(rng),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) {
        assert!(
            (a - b).abs() <= 2e-10 * a.abs().max(b.abs()).max(1.0),
            "{a} ≠ {b}"
        );
    }

    #[test]
    fn hohmann_conserves_energy_and_angular_momentum_and_reverses() {
        for center in CENTERS {
            for ratio in [1.0001, 1.01, 2.0, 6.32, 12.0, 20.0] {
                let r = [center.low_radius as f64, center.low_radius as f64 * ratio];
                let t = Hohmann::new(center.mu, r);
                let reverse = Hohmann::new(center.mu, [r[1], r[0]]);
                let energy = -center.mu / (r[0] + r[1]);
                for (radius, speed) in r.into_iter().zip(t.transfer) {
                    close(speed * speed / 2.0 - center.mu / radius, energy);
                }
                close(r[0] * t.transfer[0], r[1] * t.transfer[1]);
                close(t.total(), reverse.total());
                close(t.coast, reverse.coast);
                close(t.burns[0], -reverse.burns[1]);
                close(t.burns[1], -reverse.burns[0]);
                assert!(t.burns.iter().all(|v| *v > 0.0));
                assert!(reverse.burns.iter().all(|v| *v < 0.0));
            }
        }
        let same = Hohmann::new(EARTH_MU, [7_000.0; 2]);
        close(same.total(), 0.0);
        close(same.coast, PI * (7_000.0_f64.powi(3) / EARTH_MU).sqrt());
    }

    #[test]
    fn finite_bielliptic_apex_changes_the_winner_and_costs_time() {
        let h = Hohmann::new(1.0, [1.0, 12.0]);
        let short = BiElliptic::new(1.0, h.r, 120.0);
        let long = BiElliptic::new(1.0, h.r, 1_200.0);
        assert!(short.total() > h.total());
        assert!(long.total() < h.total());
        assert!(short.coast > h.coast && long.coast > short.coast);
        for ratio in [10.0, 12.0, 15.58, 16.0, 20.0] {
            let r = [1.0, ratio];
            for factor in [1.05, 2.0, 20.0, 100.0] {
                let apex = ratio * factor;
                let b = BiElliptic::new(1.0, r, apex);
                let reverse = BiElliptic::new(1.0, [r[1], r[0]], apex);
                close(b.total(), reverse.total());
                close(b.coast, reverse.coast);
                for i in 0..3 {
                    close(b.burns[i], -reverse.burns[2 - i]);
                }
                for radius in r {
                    let near = apsis_speed(1.0, radius, apex);
                    let far = apsis_speed(1.0, apex, radius);
                    close(radius * near, apex * far);
                    close(
                        near * near / 2.0 - 1.0 / radius,
                        far * far / 2.0 - 1.0 / apex,
                    );
                }
                assert!(b.coast > Hohmann::new(1.0, r).coast);
            }
        }
        let ratio: f64 = 20.0;
        let limiting = (2.0_f64.sqrt() - 1.0) * (1.0 + 1.0 / ratio.sqrt());
        let finite = BiElliptic::new(1.0, [1.0, ratio], 1e12);
        assert!((finite.total() - limiting).abs() < 1e-9);
    }

    #[test]
    fn split_search_handles_competing_minima_and_matches_dense_grid() {
        for ratio in [1.0, 1.001, 1.01, 1.15, 2.0, 6.32, 12.0] {
            for degrees in [1.0_f64, 5.0, 28.5, 63.4, 90.0, 100.0] {
                let turn = degrees.to_radians();
                let t = Hohmann::new(1.0, [1.0, ratio]);
                let split = minimum_split(t, turn);
                let cost = t.inclined_total(turn, split);
                assert!((0.0..=turn).contains(&split));
                assert!(cost >= t.total() - 1e-12);
                assert!(cost <= t.inclined_total(turn, 0.0) + 1e-12);
                assert!(cost <= t.inclined_total(turn, turn) + 1e-12);
                for i in 0..=4_000 {
                    let candidate = turn * i as f64 / 4_000.0;
                    assert!(cost <= t.inclined_total(turn, candidate) + 1e-10);
                }
                let reverse = Hohmann::new(1.0, [ratio, 1.0]);
                close(cost, reverse.inclined_total(turn, turn - split));
            }
        }
        close(vector_burn(3.0, 3.0, PI / 2.0), 3.0 * 2.0_f64.sqrt());
        close(vector_burn(3.0, 3.0, 0.0), 0.0);
    }

    #[test]
    fn departure_windows_close_the_phase_and_repeat_in_both_directions() {
        for r in [
            [AU, 1.5237 * AU],
            [1.5237 * AU, AU],
            [5.2029 * AU, 0.3871 * AU],
        ] {
            let motion = r.map(|radius| (SUN_MU / radius.powi(3)).sqrt());
            for degrees in [0.0_f64, 0.1, 44.0, 180.0, 359.9] {
                let current = degrees.to_radians();
                let w = window(SUN_MU, r, current);
                let arrival = w.required + motion[1] * half_period(SUN_MU, r[0], r[1]);
                close((arrival - PI).sin(), 0.0);
                close((arrival - PI).cos(), 1.0);
                let launch = current + (motion[1] - motion[0]) * w.wait;
                close((launch - w.required).sin(), 0.0);
                close((launch - w.required).cos(), 1.0);
                assert!((0.0..w.synodic).contains(&w.wait));
                close((motion[1] - motion[0]).abs() * w.synodic, TAU);
                close(w.synodic, window(SUN_MU, [r[1], r[0]], current).synodic);
            }
        }
    }
}
