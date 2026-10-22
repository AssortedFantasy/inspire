use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

include!("../../../../number.rs");

#[derive(Clone, Copy)]
struct Stage {
    driver: u32,
    driven: u32,
    idler: Option<u32>,
}

fn speeds(stages: &[Stage], input: f64) -> Vec<f64> {
    let mut values = vec![input];
    for stage in stages {
        let direction = if stage.idler.is_some() { 1.0 } else { -1.0 };
        values.push(values.last().unwrap() * direction * stage.driver as f64 / stage.driven as f64);
    }
    values
}

pub fn generate(rng: &mut StdRng) -> String {
    let stages: Vec<_> = (0..rng.random_range(2..=4))
        .map(|_| Stage {
            driver: rng.random_range(3..=12) * 6,
            driven: rng.random_range(3..=12) * 6,
            idler: rng.random_bool(0.3).then(|| rng.random_range(3..=12) * 6),
        })
        .collect();
    let input = rng.random_range(2..=12) as f64 * 60.0;
    let torque = rng.random_range(2..=20) as f64;
    let rpm = speeds(&stages, input);
    let mut out = "Lossless compound spur train\nLetters identify shafts".to_owned();
    for (index, stage) in stages.iter().enumerate() {
        let from = (b'A' + index as u8) as char;
        let to = (b'B' + index as u8) as char;
        if let Some(idler) = stage.idler {
            write!(
                out,
                "\n{from} {}T ↔ idler {idler}T ↔ {to} {}T",
                stage.driver, stage.driven
            )
            .unwrap();
        } else {
            write!(out, "\n{from} {}T ↔ {to} {}T", stage.driver, stage.driven).unwrap();
        }
    }
    let last = *rpm.last().unwrap();
    if rng.random_bool(0.5) {
        out.push_str("\n\nShaft | rpm");
        for (index, speed) in rpm.iter().enumerate() {
            write!(out, "\n{} | {}", (b'A' + index as u8) as char, n(*speed)).unwrap();
        }
        write!(
            out,
            "\n\nInput torque {} N·m\nOutput torque {} N·m",
            n(torque),
            n(torque * input / last.abs())
        )
        .unwrap();
    } else {
        write!(
            out,
            "\n\nωout/ωin = {}\n{} rpm → {} rpm\n{} N·m → {} N·m",
            n(last / input),
            n(input),
            n(last),
            n(torque),
            n(torque * input / last.abs())
        )
        .unwrap();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meshes_and_power_balance_with_optional_idlers() {
        for driver in 3..=12 {
            for driven in 3..=12 {
                for idler in [None, Some(18), Some(72)] {
                    let stage = Stage {
                        driver: driver * 6,
                        driven: driven * 6,
                        idler,
                    };
                    for count in 2..=4 {
                        let stages = vec![stage; count];
                        let rpm = speeds(&stages, 360.0);
                        for pair in rpm.windows(2) {
                            if let Some(teeth) = idler {
                                let middle = -pair[0] * stage.driver as f64 / teeth as f64;
                                assert!(
                                    (middle * teeth as f64 + pair[1] * stage.driven as f64).abs()
                                        < 1e-8
                                );
                            } else {
                                assert!(
                                    (pair[0] * stage.driver as f64 + pair[1] * stage.driven as f64)
                                        .abs()
                                        < 1e-8
                                );
                            }
                        }
                        let last = rpm.last().unwrap();
                        let out = 7.0 * 360.0 / last.abs();
                        assert!((out * last.abs() - 7.0 * 360.0).abs() < 1e-8);
                    }
                }
            }
        }
    }
}
