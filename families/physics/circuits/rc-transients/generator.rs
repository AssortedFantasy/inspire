use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

include!("../../number.rs");

#[derive(Clone, Copy)]
struct Exponential {
    initial: f64,
    target: f64,
    tau: f64,
}

impl Exponential {
    fn at(self, t: f64) -> f64 {
        self.target + (self.initial - self.target) * (-t / self.tau).exp()
    }

    fn derivative(self, t: f64) -> f64 {
        (self.target - self.initial) / self.tau * (-t / self.tau).exp()
    }

    #[cfg(test)]
    fn integral(self, t: f64) -> f64 {
        self.target * t + (self.initial - self.target) * self.tau * -(-t / self.tau).exp_m1()
    }

    #[cfg(test)]
    fn square_integral(self, t: f64) -> f64 {
        let d = self.initial - self.target;
        self.target * self.target * t
            + 2.0 * self.target * d * self.tau * -(-t / self.tau).exp_m1()
            + d * d * self.tau / 2.0 * -(-2.0 * t / self.tau).exp_m1()
    }
}

#[derive(Clone, Copy)]
struct Rc {
    source: f64,
    resistance: f64,
    load: Option<f64>,
    capacitance: f64,
    initial: f64,
}

impl Rc {
    fn response(self) -> Exponential {
        let conductance = 1.0 / self.resistance + self.load.map_or(0.0, |r| 1.0 / r);
        Exponential {
            initial: self.initial,
            target: self.source / (self.resistance * conductance),
            tau: self.capacitance / conductance,
        }
    }

    fn render(self) -> String {
        let x = self.response();
        let mut out = if self.source == 0.0 {
            format!(
                "Capacitor discharging through {}\nC = {}, vC(0) = {} V",
                q(self.resistance, "Ω"),
                q(self.capacitance, "F"),
                n(self.initial)
            )
        } else {
            format!(
                "{}\nVs feeds R into node vC, with C to ground\nVs = {} V for t ≥ 0\nR = {}, C = {}, vC(0) = {} V",
                if self.load.is_some() {
                    "Loaded RC step"
                } else if self.initial * self.source < 0.0 {
                    "RC polarity reversal"
                } else {
                    "RC step charge"
                },
                n(self.source),
                q(self.resistance, "Ω"),
                q(self.capacitance, "F"),
                n(self.initial)
            )
        };
        if let Some(r) = self.load {
            write!(out, "\nLoad {} from vC to ground", q(r, "Ω")).unwrap();
        }
        write!(
            out,
            "\nτ = {}, vC(∞) = {} V\n\nt | vC (V) | iC into C",
            q(x.tau, "s"),
            n(x.target)
        )
        .unwrap();
        for u in [0.0, 0.2, 1.0, 3.0, 5.0] {
            let t = u * x.tau;
            write!(
                out,
                "\n{} | {} | {}",
                q(t, "s"),
                n(x.at(t)),
                q(self.capacitance * x.derivative(t), "A")
            )
            .unwrap();
        }
        if self.initial * x.target < 0.0 {
            write!(
                out,
                "\n\nvC crosses zero at {}",
                q(x.tau * ((x.target - self.initial) / x.target).ln(), "s")
            )
            .unwrap();
        } else if self.source == 0.0 {
            write!(
                out,
                "\n\nStored energy {} → 0\nHalf-energy time {}",
                q(0.5 * self.capacitance * self.initial.powi(2), "J"),
                q(x.tau * 2.0_f64.ln() / 2.0, "s")
            )
            .unwrap();
        } else {
            write!(
                out,
                "\n\nTime to 1% of final voltage error {}",
                q(x.tau * 100.0_f64.ln(), "s")
            )
            .unwrap();
        }
        out
    }
}

#[derive(Clone, Copy)]
struct Sharing {
    c1: f64,
    c2: f64,
    v1: f64,
    v2: f64,
    resistance: f64,
}

impl Sharing {
    fn ceq(self) -> f64 {
        self.c1 * self.c2 / (self.c1 + self.c2)
    }

    fn common(self) -> f64 {
        (self.c1 * self.v1 + self.c2 * self.v2) / (self.c1 + self.c2)
    }

    fn at(self, t: f64) -> (f64, f64, f64) {
        let difference = (self.v1 - self.v2) * (-t / (self.resistance * self.ceq())).exp();
        (
            self.common() + self.c2 / (self.c1 + self.c2) * difference,
            self.common() - self.c1 / (self.c1 + self.c2) * difference,
            difference / self.resistance,
        )
    }

    fn heat(self) -> f64 {
        0.5 * self.ceq() * (self.v1 - self.v2).powi(2)
    }

    fn render(self) -> String {
        let tau = self.resistance * self.ceq();
        let mut out = format!(
            "Capacitor charge sharing\nBottom plates share ground. At t = 0, R joins the top plates\nC1 = {}, C2 = {}, R = {}\nv1(0) = {} V, v2(0) = {} V\nτ = {}\n\nt | v1 (V) | v2 (V) | i from 1 to 2",
            q(self.c1, "F"),
            q(self.c2, "F"),
            q(self.resistance, "Ω"),
            n(self.v1),
            n(self.v2),
            q(tau, "s")
        );
        for u in [0.0, 0.2, 1.0, 3.0, 5.0] {
            let (v1, v2, i) = self.at(u * tau);
            write!(
                out,
                "\n{} | {} | {} | {}",
                q(u * tau, "s"),
                n(v1),
                n(v2),
                q(i, "A")
            )
            .unwrap();
        }
        write!(
            out,
            "\n\nCommon final voltage {} V\nHeat in R {}",
            n(self.common()),
            q(self.heat(), "J")
        )
        .unwrap();
        out
    }
}

#[derive(Clone, Copy)]
struct Pulse {
    voltage: f64,
    resistance: f64,
    capacitance: f64,
    width: f64,
    high_pass: bool,
}

impl Pulse {
    fn tau(self) -> f64 {
        self.resistance * self.capacitance
    }

    fn capacitor(self, t: f64) -> f64 {
        if t < self.width {
            self.voltage * -(-t / self.tau()).exp_m1()
        } else {
            self.voltage
                * -(-self.width / self.tau()).exp_m1()
                * (-(t - self.width) / self.tau()).exp()
        }
    }

    fn render(self) -> String {
        let mut out = format!(
            "{}\n{}\nInput 0 → {} V at t = 0, back to 0 at {}\nR = {}, C = {}, τ = {}\n\nt | input (V) | output (V)",
            if self.high_pass {
                "RC edge detector"
            } else {
                "RC pulse capture"
            },
            if self.high_pass {
                "Input through C to output, R from output to ground"
            } else {
                "Input through R to output, C from output to ground"
            },
            n(self.voltage),
            q(self.width, "s"),
            q(self.resistance, "Ω"),
            q(self.capacitance, "F"),
            q(self.tau(), "s")
        );
        for (t, left_limit) in [
            (0.0, false),
            (self.width / 2.0, false),
            (self.width, true),
            (self.width, false),
            (self.width + self.tau(), false),
            (self.width + 5.0 * self.tau(), false),
        ] {
            let input = if t < self.width || left_limit {
                self.voltage
            } else {
                0.0
            };
            let cap = self.capacitor(t);
            let output = if self.high_pass { input - cap } else { cap };
            write!(
                out,
                "\n{}{} | {} | {}",
                q(t, "s"),
                if t == self.width {
                    if left_limit { "−" } else { "+" }
                } else {
                    ""
                },
                n(input),
                n(output)
            )
            .unwrap();
        }
        if self.high_pass {
            write!(
                out,
                "\n\nFalling-edge output {} V",
                n(-self.capacitor(self.width))
            )
            .unwrap();
        } else {
            write!(
                out,
                "\n\nPeak stored energy {}",
                q(
                    0.5 * self.capacitance * self.capacitor(self.width).powi(2),
                    "J"
                )
            )
            .unwrap();
        }
        out
    }
}

#[derive(Clone, Copy)]
struct Periodic {
    high: f64,
    low: f64,
    r_high: f64,
    r_low: f64,
    capacitance: f64,
    on: f64,
    off: f64,
}

impl Periodic {
    fn limits(self) -> (f64, f64) {
        let a = -(-self.on / (self.r_high * self.capacitance)).exp_m1();
        let b = -(-self.off / (self.r_low * self.capacitance)).exp_m1();
        let bottom = self.low + (self.high - self.low) * a * (1.0 - b) / (a + b - a * b);
        let top = self.high - (self.high - bottom) * (1.0 - a);
        (bottom, top)
    }

    fn render(self) -> String {
        let (bottom, top) = self.limits();
        let mut out = format!(
            "Switched RC in periodic steady state\nC = {} to ground\nConnect to {} V through {} for {}, then {} V through {} for {}\n\nPhase | elapsed | vC (V)",
            q(self.capacitance, "F"),
            n(self.high),
            q(self.r_high, "Ω"),
            q(self.on, "s"),
            n(self.low),
            q(self.r_low, "Ω"),
            q(self.off, "s")
        );
        for (phase, time, initial, target, resistance) in [
            ("high", self.on, bottom, self.high, self.r_high),
            ("low", self.off, top, self.low, self.r_low),
        ] {
            let response = Exponential {
                initial,
                target,
                tau: resistance * self.capacitance,
            };
            for fraction in [0.0, 0.5, 1.0] {
                write!(
                    out,
                    "\n{phase} | {} | {}",
                    q(fraction * time, "s"),
                    n(response.at(fraction * time))
                )
                .unwrap();
            }
        }
        write!(
            out,
            "\n\nRipple {} V peak to peak\nFrequency {}",
            n(top - bottom),
            q(1.0 / (self.on + self.off), "Hz")
        )
        .unwrap();
        out
    }
}

#[derive(Clone, Copy)]
struct Rl {
    source: f64,
    resistance: f64,
    inductance: f64,
    initial: f64,
}

impl Rl {
    fn response(self) -> Exponential {
        Exponential {
            initial: self.initial,
            target: self.source / self.resistance,
            tau: self.inductance / self.resistance,
        }
    }

    fn render(self) -> String {
        let x = self.response();
        let mut out = format!(
            "{}\n{}\nR = {}, L = {}, iL(0) = {}\nτ = {}\n\nt | iL | vL along iL (V)",
            if self.source == 0.0 {
                "Resistive coil turnoff"
            } else if self.initial < 0.0 {
                "RL current reversal"
            } else {
                "RL current rise"
            },
            if self.source == 0.0 {
                "Source removed at t = 0, R and L form a closed loop".to_owned()
            } else {
                format!("Vs = {} V drives series R and L for t ≥ 0", n(self.source))
            },
            q(self.resistance, "Ω"),
            q(self.inductance, "H"),
            q(self.initial, "A"),
            q(x.tau, "s")
        );
        for u in [0.0, 0.2, 1.0, 3.0, 5.0] {
            let t = u * x.tau;
            write!(
                out,
                "\n{} | {} | {}",
                q(t, "s"),
                q(x.at(t), "A"),
                n(self.inductance * x.derivative(t))
            )
            .unwrap();
        }
        if self.initial < 0.0 {
            write!(
                out,
                "\n\nCurrent crosses zero at {}",
                q(x.tau * ((x.target - x.initial) / x.target).ln(), "s")
            )
            .unwrap();
        } else {
            write!(
                out,
                "\n\nMagnetic energy {} → {}",
                q(0.5 * self.inductance * self.initial.powi(2), "J"),
                q(0.5 * self.inductance * x.target.powi(2), "J")
            )
            .unwrap();
        }
        out
    }
}

#[derive(Clone, Copy)]
struct Clamp {
    resistance: f64,
    inductance: f64,
    initial: f64,
    voltage: f64,
}

impl Clamp {
    fn extinction(self) -> f64 {
        self.inductance / self.resistance * (self.resistance * self.initial / self.voltage).ln_1p()
    }

    fn current(self, t: f64) -> f64 {
        if t >= self.extinction() {
            0.0
        } else {
            self.initial
                + (self.initial + self.voltage / self.resistance)
                    * (-t * self.resistance / self.inductance).exp_m1()
        }
    }

    fn clamp_heat(self) -> f64 {
        self.voltage * (self.inductance * self.initial - self.voltage * self.extinction())
            / self.resistance
    }

    fn render(self) -> String {
        let end = self.extinction();
        let mut out = format!(
            "Clamped coil turnoff\nAt t = 0, source removed and positive current recirculates through R and a reverse clamp\nL = {}, R = {}, iL(0) = {}\nClamp drop {} V until current reaches zero\n\nt | iL | vL along iL (V)",
            q(self.inductance, "H"),
            q(self.resistance, "Ω"),
            q(self.initial, "A"),
            n(self.voltage)
        );
        for fraction in [0.0, 0.1, 0.5, 0.9, 1.0, 1.2] {
            let t = fraction * end;
            let i = self.current(t);
            let vl = if t < end {
                -self.resistance * i - self.voltage
            } else {
                0.0
            };
            write!(out, "\n{} | {} | {}", q(t, "s"), q(i, "A"), n(vl)).unwrap();
        }
        write!(
            out,
            "\n\nCurrent extinguishes at {}\nHeat in clamp {}, heat in R {}",
            q(end, "s"),
            q(self.clamp_heat(), "J"),
            q(
                0.5 * self.inductance * self.initial.powi(2) - self.clamp_heat(),
                "J"
            )
        )
        .unwrap();
        out
    }
}

enum Circuit {
    Rc(Rc),
    Sharing(Sharing),
    Pulse(Pulse),
    Periodic(Periodic),
    Rl(Rl),
    Clamp(Clamp),
}

impl Circuit {
    fn render(self) -> String {
        match self {
            Self::Rc(c) => c.render(),
            Self::Sharing(c) => c.render(),
            Self::Pulse(c) => c.render(),
            Self::Periodic(c) => c.render(),
            Self::Rl(c) => c.render(),
            Self::Clamp(c) => c.render(),
        }
    }
}

fn choose(rng: &mut StdRng, choices: &[f64]) -> f64 {
    choices[rng.random_range(0..choices.len())]
}

fn sample_regime(rng: &mut StdRng, regime: u32) -> Circuit {
    let r = choose(
        rng,
        &[100.0, 330.0, 1000.0, 3300.0, 10_000.0, 33_000.0, 100_000.0],
    );
    let c = choose(rng, &[1e-9, 1e-8, 1e-7, 1e-6, 1e-5, 47e-6]);
    let v = choose(rng, &[1.5, 3.3, 5.0, 9.0, 12.0, 24.0]);
    match regime {
        0..=3 => Circuit::Rc(Rc {
            source: if regime == 3 { 0.0 } else { v },
            resistance: r,
            load: (regime == 2).then(|| r * choose(rng, &[0.1, 0.5, 1.0, 5.0, 20.0])),
            capacitance: c,
            initial: match regime {
                1 => -v * choose(rng, &[0.5, 1.0, 2.0]),
                3 => v,
                _ => 0.0,
            },
        }),
        4..=5 => {
            let ratio = choose(rng, &[0.25, 0.5, 1.0, 2.0, 4.0]);
            Circuit::Sharing(Sharing {
                c1: c,
                c2: c * ratio,
                v1: v,
                v2: if regime == 5 { -v / ratio } else { 0.0 },
                resistance: r,
            })
        }
        6 | 8 => Circuit::Pulse(Pulse {
            voltage: v,
            resistance: r,
            capacitance: c,
            width: r * c * choose(rng, &[0.02, 0.2, 1.0, 3.0, 8.0]),
            high_pass: regime == 8,
        }),
        7 => {
            let period = r * c * choose(rng, &[0.02, 0.2, 1.0, 4.0, 12.0]);
            let duty = choose(rng, &[0.1, 0.25, 0.5, 0.75, 0.9]);
            Circuit::Periodic(Periodic {
                high: v,
                low: if rng.random_bool(0.25) { -v } else { 0.0 },
                r_high: r,
                r_low: r * choose(rng, &[0.2, 1.0, 5.0]),
                capacitance: c,
                on: period * duty,
                off: period * (1.0 - duty),
            })
        }
        9..=11 => {
            let resistance = choose(rng, &[10.0, 22.0, 47.0, 100.0, 470.0]);
            let inductance = choose(rng, &[100e-6, 470e-6, 2.2e-3, 10e-3, 47e-3, 220e-3, 1.0]);
            Circuit::Rl(Rl {
                source: if regime == 11 { 0.0 } else { v },
                resistance,
                inductance,
                initial: match regime {
                    10 => -v / resistance * choose(rng, &[0.5, 1.0, 2.0]),
                    11 => v / resistance,
                    _ => 0.0,
                },
            })
        }
        _ => Circuit::Clamp(Clamp {
            resistance: choose(rng, &[2.2, 4.7, 10.0, 22.0]),
            inductance: choose(rng, &[1e-3, 4.7e-3, 22e-3, 100e-3, 220e-3]),
            initial: rng.random_range(1..=8) as f64 / 10.0,
            voltage: if regime == 12 {
                0.7
            } else {
                choose(rng, &[12.0, 24.0, 36.0, 48.0])
            },
        }),
    }
}

fn q(value: f64, unit: &str) -> String {
    if value == 0.0 {
        return format!("0 {unit}");
    }
    for (scale, prefix) in [
        (1e9, "G"),
        (1e6, "M"),
        (1e3, "k"),
        (1.0, ""),
        (1e-3, "m"),
        (1e-6, "µ"),
        (1e-9, "n"),
        (1e-12, "p"),
    ] {
        if value.abs() >= scale {
            return format!("{} {prefix}{unit}", n(value / scale));
        }
    }
    format!("{value:.3e} {unit}")
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..14);
    sample_regime(rng, regime).render()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn near(a: f64, b: f64) {
        near_scaled(a, b, (a.abs() + b.abs()).max(1e-12));
    }

    fn near_scaled(a: f64, b: f64, scale: f64) {
        assert!((a - b).abs() <= 1e-9 * scale, "{a} != {b}");
    }

    fn integrate(f: impl Fn(f64) -> f64, end: f64) -> f64 {
        let count = 2048;
        let h = end / count as f64;
        let mut total = f(0.0) + f(end);
        for k in 1..count {
            total += if k % 2 == 0 { 2.0 } else { 4.0 } * f(k as f64 * h);
        }
        total * h / 3.0
    }

    #[test]
    fn rc_and_rl_obey_kirchhoff_and_energy_balances() {
        for regime in [0, 1, 2, 3, 9, 10, 11] {
            for seed in 0..64 {
                match sample_regime(&mut StdRng::seed_from_u64(seed), regime) {
                    Circuit::Rc(c) => {
                        let x = c.response();
                        near(x.at(0.0), c.initial);
                        for u in [0.0, 0.1, 0.7, 3.0, 10.0] {
                            let t = u * x.tau;
                            near(
                                (c.source - x.at(t)) / c.resistance,
                                c.capacitance * x.derivative(t)
                                    + c.load.map_or(0.0, |r| x.at(t) / r),
                            );
                        }
                        let end = 5.0 * x.tau;
                        let source_energy =
                            c.source * (c.source * end - x.integral(end)) / c.resistance;
                        let resistor_heat = (c.source.powi(2) * end
                            - 2.0 * c.source * x.integral(end)
                            + x.square_integral(end))
                            / c.resistance;
                        let load_heat = c.load.map_or(0.0, |r| x.square_integral(end) / r);
                        let stored = 0.5 * c.capacitance * (x.at(end).powi(2) - c.initial.powi(2));
                        near(source_energy - stored, resistor_heat + load_heat);
                        if c.initial < 0.0 {
                            near_scaled(
                                x.at(x.tau * ((x.target - c.initial) / x.target).ln()),
                                0.0,
                                c.initial.abs() + x.target.abs(),
                            );
                        }
                    }
                    Circuit::Rl(c) => {
                        let x = c.response();
                        near(x.at(0.0), c.initial);
                        for u in [0.0, 0.1, 0.7, 3.0, 10.0] {
                            let t = u * x.tau;
                            near_scaled(
                                c.source,
                                c.resistance * x.at(t) + c.inductance * x.derivative(t),
                                c.source.abs() + c.resistance * c.initial.abs(),
                            );
                        }
                        let end = 5.0 * x.tau;
                        near(
                            c.source * x.integral(end)
                                - 0.5 * c.inductance * (x.at(end).powi(2) - c.initial.powi(2)),
                            c.resistance * x.square_integral(end),
                        );
                        if c.initial < 0.0 {
                            near_scaled(
                                x.at(x.tau * ((x.target - c.initial) / x.target).ln()),
                                0.0,
                                c.initial.abs() + x.target.abs(),
                            );
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    #[test]
    fn sharing_conserves_charge_and_accounts_for_all_lost_energy() {
        for regime in [4, 5] {
            for seed in 0..128 {
                let Circuit::Sharing(c) = sample_regime(&mut StdRng::seed_from_u64(seed), regime)
                else {
                    unreachable!()
                };
                let initial_energy = 0.5 * (c.c1 * c.v1.powi(2) + c.c2 * c.v2.powi(2));
                let final_energy = 0.5 * (c.c1 + c.c2) * c.common().powi(2);
                near(initial_energy, final_energy + c.heat());
                if regime == 5 {
                    near_scaled(c.common(), 0.0, c.v1.abs() + c.v2.abs());
                }
                let tau = c.resistance * c.ceq();
                for u in [0.0, 0.2, 1.0, 3.0, 8.0] {
                    let (v1, v2, i) = c.at(u * tau);
                    near_scaled(
                        c.c1 * v1 + c.c2 * v2,
                        c.c1 * c.v1 + c.c2 * c.v2,
                        c.c1 * c.v1.abs() + c.c2 * c.v2.abs(),
                    );
                    near(v1 - v2, i * c.resistance);
                    near_scaled(
                        initial_energy - 0.5 * (c.c1 * v1.powi(2) + c.c2 * v2.powi(2)),
                        c.heat() * -(-2.0 * u).exp_m1(),
                        initial_energy,
                    );
                }
            }
        }
    }

    #[test]
    fn pulse_capacitor_is_continuous_and_resistor_edge_is_exact() {
        for regime in [6, 8] {
            for seed in 0..128 {
                let Circuit::Pulse(c) = sample_regime(&mut StdRng::seed_from_u64(seed), regime)
                else {
                    unreachable!()
                };
                near(c.capacitor(0.0), 0.0);
                let peak = c.voltage * -(-c.width / c.tau()).exp_m1();
                near(c.capacitor(c.width), peak);
                near(c.capacitor(c.width + c.tau()), peak / std::f64::consts::E);
                near((c.voltage - peak) - (-peak), c.voltage);
                let charge =
                    integrate(|t| c.voltage / c.resistance * (-t / c.tau()).exp(), c.width);
                near(charge, c.capacitance * peak);
            }
        }
    }

    #[test]
    fn periodic_extrema_close_the_cycle_and_stay_between_the_rails() {
        for seed in 0..512 {
            let Circuit::Periodic(c) = sample_regime(&mut StdRng::seed_from_u64(seed), 7) else {
                unreachable!()
            };
            let (bottom, top) = c.limits();
            assert!(c.low <= bottom && bottom < top && top <= c.high);
            near_scaled(
                Exponential {
                    initial: bottom,
                    target: c.high,
                    tau: c.r_high * c.capacitance,
                }
                .at(c.on),
                top,
                c.high - c.low,
            );
            near_scaled(
                Exponential {
                    initial: top,
                    target: c.low,
                    tau: c.r_low * c.capacitance,
                }
                .at(c.off),
                bottom,
                c.high - c.low,
            );
            let high_charge = c.capacitance * (top - bottom);
            let low_charge = c.capacitance * (bottom - top);
            near(high_charge + low_charge, 0.0);
        }
    }

    #[test]
    fn clamp_never_reverses_current_and_dissipates_the_initial_magnetic_energy() {
        for regime in [12, 13] {
            for seed in 0..128 {
                let Circuit::Clamp(c) = sample_regime(&mut StdRng::seed_from_u64(seed), regime)
                else {
                    unreachable!()
                };
                let end = c.extinction();
                near(c.current(0.0), c.initial);
                near(c.current(end), 0.0);
                near(c.current(end * 2.0), 0.0);
                let mut previous = c.initial;
                for k in 1..=100 {
                    let i = c.current(end * k as f64 / 100.0);
                    assert!(i >= 0.0 && i <= previous);
                    previous = i;
                }
                let clamp_heat = integrate(|t| c.voltage * c.current(t), end);
                let resistor_heat = integrate(|t| c.resistance * c.current(t).powi(2), end);
                near(clamp_heat, c.clamp_heat());
                near(
                    clamp_heat + resistor_heat,
                    0.5 * c.inductance * c.initial.powi(2),
                );
                let higher = Clamp {
                    voltage: 2.0 * c.voltage,
                    ..c
                };
                assert!(higher.extinction() < end);
                assert!(higher.clamp_heat() > c.clamp_heat());
            }
        }
    }
}
