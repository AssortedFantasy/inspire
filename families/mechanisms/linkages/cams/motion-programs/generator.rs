use rand::{Rng, rngs::StdRng};
use std::{f64::consts::TAU, fmt::Write};

include!("../../../number.rs");

#[derive(Clone, Copy)]
enum Law {
    Cycloidal,
    Quintic,
}

impl Law {
    fn value(self, u: f64) -> f64 {
        match self {
            Self::Cycloidal => u - (TAU * u).sin() / TAU,
            Self::Quintic => u.powi(3) * (10.0 - 15.0 * u + 6.0 * u * u),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Cycloidal => "u − sin(2πu)/(2π)",
            Self::Quintic => "10u³ − 15u⁴ + 6u⁵",
        }
    }
}

struct Program {
    lift: f64,
    rise: f64,
    dwell: f64,
    fall: f64,
    up: Law,
    down: Law,
}

impl Program {
    fn position(&self, angle: f64) -> f64 {
        let angle = angle.rem_euclid(360.0);
        if angle < self.rise {
            self.lift * self.up.value(angle / self.rise)
        } else if angle < self.rise + self.dwell {
            self.lift
        } else if angle < self.rise + self.dwell + self.fall {
            let u = (angle - self.rise - self.dwell) / self.fall;
            self.lift * (1.0 - self.down.value(u))
        } else {
            0.0
        }
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let rise = rng.random_range(4..=10) * 15;
    let fall = rng.random_range(4..=10) * 15;
    let dwell = rng.random_range(1..(360 - rise - fall) / 15) * 15;
    let program = Program {
        lift: rng.random_range(1..=12) as f64 * 5.0,
        rise: rise as f64,
        dwell: dwell as f64,
        fall: fall as f64,
        up: if rng.random_bool(0.5) {
            Law::Cycloidal
        } else {
            Law::Quintic
        },
        down: if rng.random_bool(0.5) {
            Law::Cycloidal
        } else {
            Law::Quintic
        },
    };
    let rpm = rng.random_range(1..=12) * 30;
    let top = rise + dwell;
    let bottom = top + fall;
    let mut out = format!(
        "Cam motion program, {rpm} rpm\nLift {} mm\n\nθ (°) | follower\n0–{rise} | rise\n{rise}–{top} | high dwell\n{top}–{bottom} | return\n{bottom}–360 | low dwell\n\nRise s/h = {}\nReturn s/h = 1 − [{}]\nu runs 0→1 within each moving segment",
        n(program.lift),
        program.up.name(),
        program.down.name()
    );
    if rng.random_bool(0.5) {
        out.push_str("\n\nθ (°) | s (mm)");
        for angle in (0..360).step_by(30) {
            write!(out, "\n{angle} | {}", n(program.position(angle as f64))).unwrap();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn motion_laws_have_smooth_stationary_endpoints() {
        for law in [Law::Cycloidal, Law::Quintic] {
            assert!(law.value(0.0).abs() < 1e-12);
            assert!((law.value(1.0) - 1.0).abs() < 1e-12);
            for endpoint in [0.0, 1.0] {
                let h = 1e-4;
                let slope = (law.value(endpoint + h) - law.value(endpoint - h)) / (2.0 * h);
                let curvature = (law.value(endpoint + h) - 2.0 * law.value(endpoint)
                    + law.value(endpoint - h))
                    / (h * h);
                assert!(slope.abs() < 1e-6 && curvature.abs() < 1e-5);
            }
            let program = Program {
                lift: 35.0,
                rise: 120.0,
                dwell: 60.0,
                fall: 90.0,
                up: law,
                down: law,
            };
            for angle in 0..720 {
                let x = program.position(angle as f64);
                assert!((0.0..=35.0).contains(&x));
                assert!((x - program.position(angle as f64 + 360.0)).abs() < 1e-12);
            }
            assert_eq!(program.position(120.0), 35.0);
            assert_eq!(program.position(180.0), 35.0);
            assert_eq!(program.position(270.0), 0.0);
        }
    }
}
