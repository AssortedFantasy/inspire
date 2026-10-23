use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

include!("../../number.rs");

const R: f64 = 0.287;
const GAMMA: f64 = 1.4;
const CV: f64 = R / (GAMMA - 1.0);
const CP: f64 = GAMMA * CV;

#[derive(Clone, Copy, Debug)]
struct State {
    t: f64,
    v: f64,
}

impl State {
    fn p(self) -> f64 {
        R * self.t / self.v
    }

    fn entropy_from(self, reference: Self) -> f64 {
        CV * (self.t / reference.t).ln() + R * (self.v / reference.v).ln()
    }
}

#[derive(Clone, Copy, Debug)]
enum Process {
    Isentropic,
    Isothermal,
    ConstantVolume,
    ConstantPressure,
}

impl Process {
    fn work(self, a: State, b: State) -> f64 {
        match self {
            Self::Isentropic => CV * (a.t - b.t),
            Self::Isothermal => R * a.t * (b.v / a.v).ln(),
            Self::ConstantVolume => 0.0,
            Self::ConstantPressure => a.p() * (b.v - a.v),
        }
    }

    fn heat(self, a: State, b: State) -> f64 {
        match self {
            Self::Isentropic => 0.0,
            Self::Isothermal => self.work(a, b),
            Self::ConstantVolume => CV * (b.t - a.t),
            Self::ConstantPressure => CP * (b.t - a.t),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Isentropic => "isentropic",
            Self::Isothermal => "isothermal",
            Self::ConstantVolume => "constant volume",
            Self::ConstantPressure => "constant pressure",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind {
    Carnot,
    Refrigerator,
    HeatPump,
    Otto,
    Diesel,
    Brayton,
    Stirling,
}

struct Cycle {
    title: &'static str,
    detail: String,
    kind: Kind,
    states: [State; 4],
    processes: [Process; 4],
    regenerator: f64,
}

impl Cycle {
    fn leg(&self, i: usize) -> (State, State) {
        (self.states[i], self.states[(i + 1) % 4])
    }

    fn work(&self) -> f64 {
        (0..4)
            .map(|i| {
                let (a, b) = self.leg(i);
                self.processes[i].work(a, b)
            })
            .sum()
    }

    fn external_heat(&self, i: usize) -> f64 {
        let (a, b) = self.leg(i);
        let heat = self.processes[i].heat(a, b);
        if self.kind == Kind::Stirling && matches!(self.processes[i], Process::ConstantVolume) {
            (1.0 - self.regenerator) * heat
        } else {
            heat
        }
    }

    fn heat_in(&self) -> f64 {
        (0..4).map(|i| self.external_heat(i).max(0.0)).sum()
    }

    fn heat_out(&self) -> f64 {
        -(0..4).map(|i| self.external_heat(i).min(0.0)).sum::<f64>()
    }

    fn temperatures(&self) -> (f64, f64) {
        self.states
            .iter()
            .fold((f64::INFINITY, 0.0_f64), |(lo, hi), s| {
                (lo.min(s.t), hi.max(s.t))
            })
    }
}

fn carnot(tc: f64, th: f64, p: f64, ratio: f64) -> [State; 4] {
    let v = R * th / p;
    let expansion = (th / tc).powf(1.0 / (GAMMA - 1.0));
    [
        State { t: th, v },
        State {
            t: th,
            v: v * ratio,
        },
        State {
            t: tc,
            v: v * ratio * expansion,
        },
        State {
            t: tc,
            v: v * expansion,
        },
    ]
}

fn otto(t: f64, p: f64, ratio: f64, rise: f64) -> [State; 4] {
    let v = R * t / p;
    let compression = ratio.powf(GAMMA - 1.0);
    let hot = t * compression + rise;
    [
        State { t, v },
        State {
            t: t * compression,
            v: v / ratio,
        },
        State {
            t: hot,
            v: v / ratio,
        },
        State {
            t: hot / compression,
            v,
        },
    ]
}

fn diesel(t: f64, p: f64, ratio: f64, cutoff: f64) -> [State; 4] {
    let v = R * t / p;
    let compressed = t * ratio.powf(GAMMA - 1.0);
    [
        State { t, v },
        State {
            t: compressed,
            v: v / ratio,
        },
        State {
            t: compressed * cutoff,
            v: v * cutoff / ratio,
        },
        State {
            t: t * cutoff.powf(GAMMA),
            v,
        },
    ]
}

fn brayton(tc: f64, th: f64, p: f64, temperature_ratio: f64) -> [State; 4] {
    let high_p = p * temperature_ratio.powf(GAMMA / (GAMMA - 1.0));
    let temperatures = [tc, tc * temperature_ratio, th, th / temperature_ratio];
    std::array::from_fn(|i| State {
        t: temperatures[i],
        v: R * temperatures[i] / if i == 1 || i == 2 { high_p } else { p },
    })
}

fn stirling(tc: f64, th: f64, p: f64, ratio: f64) -> [State; 4] {
    let large_v = R * tc / p;
    [
        State {
            t: th,
            v: large_v / ratio,
        },
        State { t: th, v: large_v },
        State { t: tc, v: large_v },
        State {
            t: tc,
            v: large_v / ratio,
        },
    ]
}

fn sample_regime(rng: &mut StdRng, regime: u32) -> Cycle {
    use Process::{ConstantPressure as P, ConstantVolume as V, Isentropic as S, Isothermal as T};
    let tc = rng.random_range(280..=340) as f64;
    let p = rng.random_range(80..=140) as f64;
    let mut cycle = Cycle {
        title: "Carnot engine",
        detail: String::new(),
        kind: Kind::Carnot,
        states: [State { t: tc, v: 1.0 }; 4],
        processes: [T, S, T, S],
        regenerator: 0.0,
    };
    match regime {
        0..=2 => {
            let th = if regime == 0 {
                rng.random_range(650..=1100) as f64
            } else {
                tc + rng.random_range(5..=30) as f64
            };
            let ratio = rng.random_range(15..=50) as f64 / 10.0;
            cycle.states = carnot(tc, th, p * 5.0, ratio);
            cycle.detail = format!("Isothermal volume ratio = {}", n(ratio));
            if regime == 1 {
                cycle.title = "Carnot engine, small temperature lift";
            } else if regime == 2 {
                cycle.states = [
                    cycle.states[0],
                    cycle.states[3],
                    cycle.states[2],
                    cycle.states[1],
                ];
                cycle.processes = [S, T, S, T];
                if rng.random_bool(0.5) {
                    cycle.title = "Carnot refrigerator";
                    cycle.kind = Kind::Refrigerator;
                } else {
                    cycle.title = "Carnot heat pump";
                    cycle.kind = Kind::HeatPump;
                }
            }
        }
        3..=4 => {
            cycle.kind = Kind::Otto;
            cycle.title = if regime == 3 {
                "Otto cycle"
            } else {
                "Otto cycle, weak heat addition"
            };
            let ratio: f64 = rng.random_range(60..=140) as f64 / 10.0;
            let rise = if regime == 3 {
                rng.random_range(400..=1300) as f64
            } else {
                tc * ratio.powf(GAMMA - 1.0) * rng.random_range(1..=4) as f64 / 100.0
            };
            cycle.states = otto(tc, p, ratio, rise);
            cycle.processes = [S, V, S, V];
            cycle.detail = format!("Compression ratio = {}", n(ratio));
        }
        5..=7 => {
            cycle.kind = Kind::Diesel;
            let ratio = if regime == 7 {
                rng.random_range(20..=35) as f64 / 10.0
            } else {
                rng.random_range(120..=200) as f64 / 10.0
            };
            let cutoff = match regime {
                6 => 1.0 + rng.random_range(1..=4) as f64 / 100.0,
                7 => ratio * rng.random_range(94..=98) as f64 / 100.0,
                _ => rng.random_range(13..=22) as f64 / 10.0,
            };
            cycle.title = match regime {
                6 => "Diesel cycle, cutoff approaching unity",
                7 => "Diesel cycle, short isentropic expansion",
                _ => "Diesel cycle",
            };
            cycle.states = diesel(tc, p, ratio, cutoff);
            cycle.processes = [S, P, S, V];
            cycle.detail = format!(
                "Compression ratio = {}\nCutoff ratio = {}",
                n(ratio),
                n(cutoff)
            );
        }
        8..=10 => {
            cycle.kind = Kind::Brayton;
            let th = rng.random_range(1000..=1400) as f64;
            let theta = th / tc;
            let ratio = match regime {
                9 => theta.sqrt(),
                10 => theta * rng.random_range(96..=99) as f64 / 100.0,
                _ => 1.0 + (theta - 1.0) * rng.random_range(15..=75) as f64 / 100.0,
            };
            cycle.title = match regime {
                9 => "Brayton cycle at maximum specific work",
                10 => "Brayton cycle near the heat-addition limit",
                _ => "Brayton cycle",
            };
            cycle.states = brayton(tc, th, p, ratio);
            cycle.processes = [S, P, S, P];
            cycle.detail = format!(
                "Pressure ratio = {}\nCompressor temperature ratio = {}",
                n(cycle.states[1].p() / p),
                n(ratio)
            );
        }
        _ => {
            cycle.kind = Kind::Stirling;
            let th = rng.random_range(500..=1100) as f64;
            let ratio = rng.random_range(15..=60) as f64 / 10.0;
            cycle.regenerator = match regime {
                11 => 1.0,
                12 => 0.0,
                _ => rng.random_range(4..=19) as f64 / 20.0,
            };
            cycle.title = "Stirling cycle";
            cycle.states = stirling(tc, th, p, ratio);
            cycle.processes = [T, V, T, V];
            cycle.detail = format!(
                "Volume ratio = {}\nRegenerator effectiveness = {}%",
                n(ratio),
                n(100.0 * cycle.regenerator)
            );
        }
    }
    cycle
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..14);
    let cycle = sample_regime(rng, regime);
    let mut out = format!(
        "{}\nConstant-capacity ideal gas, γ = 1.4, R = 0.287 kJ/(kg K)\n{}\n\nState | p (kPa) | v (m³/kg) | T (K) | s − s₁ (kJ/(kg K))",
        cycle.title, cycle.detail
    );
    for (i, &s) in cycle.states.iter().enumerate() {
        write!(
            out,
            "\n{} | {} | {} | {} | {}",
            i + 1,
            n(s.p()),
            n(s.v),
            n(s.t),
            n(s.entropy_from(cycle.states[0]))
        )
        .unwrap();
    }
    out.push_str("\n\nLeg | process | external heat (kJ/kg)");
    for (i, process) in cycle.processes.iter().enumerate() {
        write!(
            out,
            "\n{} → {} | {} | {}",
            i + 1,
            (i + 1) % 4 + 1,
            process.name(),
            n(cycle.external_heat(i))
        )
        .unwrap();
    }
    let work = cycle.work();
    let (tc, th) = cycle.temperatures();
    match cycle.kind {
        Kind::Refrigerator | Kind::HeatPump => {
            let (label, useful) = if cycle.kind == Kind::Refrigerator {
                ("Refrigerator COP", cycle.heat_in())
            } else {
                ("Heat-pump COP", cycle.heat_out())
            };
            write!(
                out,
                "\n\nWork input = {} kJ/kg\nHeat extracted at {} K = {} kJ/kg\nHeat delivered at {} K = {} kJ/kg\n{} = {}",
                n(-work),
                n(tc),
                n(cycle.heat_in()),
                n(th),
                n(cycle.heat_out()),
                label,
                n(useful / -work)
            )
            .unwrap();
        }
        _ => {
            write!(
                out,
                "\n\nNet work output = {} kJ/kg\nEfficiency = {}%\nCarnot bound at {}–{} K = {}%",
                n(work),
                n(100.0 * work / cycle.heat_in()),
                n(tc),
                n(th),
                n(100.0 * (1.0 - tc / th))
            )
            .unwrap();
            if cycle.kind == Kind::Brayton {
                let compressor = CP * (cycle.states[1].t - cycle.states[0].t);
                let turbine = CP * (cycle.states[2].t - cycle.states[3].t);
                write!(
                    out,
                    "\nCompressor / turbine shaft work = {} / {} kJ/kg",
                    n(compressor),
                    n(turbine)
                )
                .unwrap();
            } else if cycle.kind == Kind::Stirling && cycle.regenerator > 0.0 {
                write!(
                    out,
                    "\nRegenerator stores and returns {} kJ/kg per cycle",
                    n(cycle.regenerator * CV * (th - tc))
                )
                .unwrap();
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn close(a: f64, b: f64) {
        assert!(
            (a - b).abs() < 2e-9 * (1.0 + a.abs() + b.abs()),
            "{a} != {b}"
        );
    }

    #[test]
    fn all_regimes_obey_first_law_process_constraints_and_carnot_bound() {
        let mut rng = StdRng::seed_from_u64(0x05ea_d017);
        for regime in 0..14 {
            for _ in 0..128 {
                let c = sample_regime(&mut rng, regime);
                let mut entropy_sum = 0.0;
                for i in 0..4 {
                    let (a, b) = c.leg(i);
                    assert!(a.t.is_finite() && a.t > 0.0);
                    assert!(a.v.is_finite() && a.v > 0.0);
                    assert!(a.p().is_finite() && a.p() > 0.0);
                    let process = c.processes[i];
                    close(process.heat(a, b) - process.work(a, b), CV * (b.t - a.t));
                    let ds = b.entropy_from(a);
                    entropy_sum += ds;
                    match process {
                        Process::Isentropic => {
                            close(ds, 0.0);
                            close(a.p() * a.v.powf(GAMMA), b.p() * b.v.powf(GAMMA));
                        }
                        Process::Isothermal => {
                            close(a.t, b.t);
                            close(ds, process.heat(a, b) / a.t);
                        }
                        Process::ConstantVolume => {
                            close(a.v, b.v);
                            close(ds, CV * (b.t / a.t).ln());
                        }
                        Process::ConstantPressure => {
                            close(a.p(), b.p());
                            close(ds, CP * (b.t / a.t).ln());
                        }
                    }
                }
                close(entropy_sum, 0.0);
                close(c.heat_in() - c.heat_out(), c.work());
                let (tc, th) = c.temperatures();
                if matches!(c.kind, Kind::Refrigerator | Kind::HeatPump) {
                    assert!(c.work() < 0.0);
                    close(c.heat_in() / -c.work(), tc / (th - tc));
                    close(c.heat_out() / -c.work(), th / (th - tc));
                } else {
                    assert!(c.work() > 0.0);
                    let efficiency = c.work() / c.heat_in();
                    assert!(efficiency <= 1.0 - tc / th + 1e-12);
                    assert!(efficiency > 0.0);
                }
            }
        }
    }

    #[test]
    fn efficiencies_match_cycle_identities() {
        let mut rng = StdRng::seed_from_u64(8841);
        for regime in 0..14 {
            for _ in 0..64 {
                let c = sample_regime(&mut rng, regime);
                let s = c.states;
                let actual = c.work() / c.heat_in();
                match c.kind {
                    Kind::Carnot => {
                        let (tc, th) = c.temperatures();
                        close(actual, 1.0 - tc / th);
                    }
                    Kind::Otto => {
                        close(actual, 1.0 - (s[1].v / s[0].v).powf(GAMMA - 1.0));
                    }
                    Kind::Diesel => {
                        let r = s[0].v / s[1].v;
                        let cutoff = s[2].v / s[1].v;
                        let ideal = 1.0
                            - (cutoff.powf(GAMMA) - 1.0)
                                / (GAMMA * (cutoff - 1.0) * r.powf(GAMMA - 1.0));
                        close(actual, ideal);
                        assert!(actual < 1.0 - r.powf(1.0 - GAMMA));
                    }
                    Kind::Brayton => {
                        let rp = s[1].p() / s[0].p();
                        close(actual, 1.0 - rp.powf((1.0 - GAMMA) / GAMMA));
                        close(c.work(), CP * ((s[2].t - s[3].t) - (s[1].t - s[0].t)));
                    }
                    Kind::Stirling => {
                        let (tc, th) = c.temperatures();
                        let log_ratio = (s[1].v / s[0].v).ln();
                        let q = R * th * log_ratio + (1.0 - c.regenerator) * CV * (th - tc);
                        close(actual, R * (th - tc) * log_ratio / q);
                        if c.regenerator == 1.0 {
                            close(actual, 1.0 - tc / th);
                            close(c.external_heat(1), 0.0);
                            close(c.external_heat(3), 0.0);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    #[test]
    fn boundary_regimes_are_constructed_on_the_intended_limits() {
        let mut rng = StdRng::seed_from_u64(781);
        for _ in 0..128 {
            let low_lift = sample_regime(&mut rng, 1);
            let (tc, th) = low_lift.temperatures();
            assert!((5.0 - 1e-9..=30.0 + 1e-9).contains(&(th - tc)));
            let weak = sample_regime(&mut rng, 4);
            let fraction = weak.states[2].t / weak.states[1].t - 1.0;
            assert!((0.01 - 1e-9..=0.04 + 1e-9).contains(&fraction));
            let diesel_limit = sample_regime(&mut rng, 6);
            let r = diesel_limit.states[0].v / diesel_limit.states[1].v;
            let otto_efficiency = 1.0 - r.powf(1.0 - GAMMA);
            assert!(otto_efficiency - diesel_limit.work() / diesel_limit.heat_in() < 0.004);
            let short = sample_regime(&mut rng, 7);
            let expansion = short.states[3].v / short.states[2].v;
            assert!((1.0 / 0.98 - 1e-9..=1.0 / 0.94 + 1e-9).contains(&expansion));
            let pinched = sample_regime(&mut rng, 10);
            let s = pinched.states;
            assert!(s[2].t > s[1].t);
            assert!(s[1].t / s[2].t >= 0.96 - 1e-12);
            assert!(CP * (s[1].t - s[0].t) / (CP * (s[2].t - s[3].t)) > 0.96 - 1e-12);
        }
    }

    #[test]
    fn maximum_work_brayton_has_equal_intermediate_temperatures() {
        let mut rng = StdRng::seed_from_u64(911);
        for _ in 0..128 {
            let c = sample_regime(&mut rng, 9);
            let s = c.states;
            close(s[1].t, s[3].t);
            close(s[1].t, (s[0].t * s[2].t).sqrt());
            let optimum = s[1].t / s[0].t;
            for factor in [0.9, 1.1] {
                let perturbed = brayton(s[0].t, s[2].t, s[0].p(), optimum * factor);
                let work =
                    CP * ((perturbed[2].t - perturbed[3].t) - (perturbed[1].t - perturbed[0].t));
                assert!(c.work() > work);
            }
        }
    }
}
