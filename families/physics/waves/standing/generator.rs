use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

include!("../../number.rs");

#[derive(Clone, Copy, Debug, PartialEq)]
enum Boundary {
    String,
    OpenPipe,
    StoppedPipe,
}

struct Resonator {
    boundary: Boundary,
    length: f64,
    speed: f64,
    touch: Option<u32>,
    description: String,
}

impl Resonator {
    fn harmonic(&self, index: u32) -> u32 {
        if self.boundary == Boundary::StoppedPipe {
            2 * index + 1
        } else {
            (index + 1) * self.touch.unwrap_or(1)
        }
    }

    fn wavelength(&self, harmonic: u32) -> f64 {
        let factor = if self.boundary == Boundary::StoppedPipe {
            4.0
        } else {
            2.0
        };
        factor * self.length / harmonic as f64
    }

    fn frequency(&self, harmonic: u32) -> f64 {
        self.speed / self.wavelength(harmonic)
    }

    fn extrema(&self, harmonic: u32, pressure: bool) -> (Vec<f64>, Vec<f64>) {
        let mut nodes = Vec::new();
        let mut antinodes = Vec::new();
        let odd_boundary = self.boundary == Boundary::StoppedPipe;
        let steps = if odd_boundary { harmonic } else { 2 * harmonic };
        let cosine = (self.boundary == Boundary::OpenPipe) != pressure;
        for j in 0..=steps {
            let x = if j == steps {
                self.length
            } else {
                self.length * j as f64 / steps as f64
            };
            if (j % 2 == 0) != cosine {
                nodes.push(x);
            } else {
                antinodes.push(x);
            }
        }
        (nodes, antinodes)
    }
}

fn air_speed(celsius: f64) -> f64 {
    331.0 * ((celsius + 273.15) / 273.15).sqrt()
}

fn sample(rng: &mut StdRng) -> Resonator {
    let regime = rng.random_range(0..4);
    sample_regime(rng, regime)
}

fn sample_regime(rng: &mut StdRng, regime: u32) -> Resonator {
    if regime < 2 {
        let length = rng.random_range(30..=180) as f64 / 100.0;
        let tension = rng.random_range(15..=150) as f64;
        let density = rng.random_range(15..=400) as f64 / 100_000.0;
        let touch = if regime == 1 {
            Some(rng.random_range(2..=5))
        } else {
            None
        };
        let mut description = format!(
            "Ideal string fixed at both ends\nL = {} m\nTension = {} N\nLinear density = {} g/m",
            n(length),
            n(tension),
            n(1000.0 * density)
        );
        if let Some(divisor) = touch {
            write!(
                description,
                "\nLight touch at x = L/{divisor} = {} m",
                n(length / divisor as f64)
            )
            .unwrap();
        }
        Resonator {
            boundary: Boundary::String,
            length,
            speed: (tension / density).sqrt(),
            touch,
            description,
        }
    } else {
        let length = rng.random_range(8..=180) as f64 / 100.0;
        let temperature = rng.random_range(-10..=40) as f64;
        let boundary = if regime == 2 {
            Boundary::OpenPipe
        } else {
            Boundary::StoppedPipe
        };
        let description = format!(
            "{}\nL = {} m\nDry air at {} °C\nc ≈ {} m/s",
            if boundary == Boundary::OpenPipe {
                "Ideal open–open pipe"
            } else {
                "Ideal pipe closed at x = 0, open at x = L"
            },
            n(length),
            n(temperature),
            n(air_speed(temperature))
        );
        Resonator {
            boundary,
            length,
            speed: air_speed(temperature),
            touch: None,
            description,
        }
    }
}

fn pitch(frequency: f64) -> String {
    let semitones = 69.0 + 12.0 * (frequency / 440.0).log2();
    let midi = semitones.round() as i32;
    let names = [
        "C", "C♯", "D", "E♭", "E", "F", "F♯", "G", "A♭", "A", "B♭", "B",
    ];
    let cents = ((semitones - midi as f64) * 100.0).round() as i32;
    format!(
        "{}{} {cents:+}¢",
        names[midi.rem_euclid(12) as usize],
        midi.div_euclid(12) - 1
    )
}

fn positions(points: &[f64]) -> String {
    points.iter().map(|x| n(*x)).collect::<Vec<_>>().join(", ")
}

pub fn generate(rng: &mut StdRng) -> String {
    let resonator = sample(rng);
    let mut out = resonator.description.clone();
    if rng.random_bool(0.55) {
        if resonator.touch.is_some() {
            out.push_str("\n\nSurviving full-string modes");
        }
        out.push_str("\n\nHarmonic | frequency (Hz) | nearest pitch (A4 = 440 Hz)");
        for index in 0..5 {
            let harmonic = resonator.harmonic(index);
            let frequency = resonator.frequency(harmonic);
            write!(
                out,
                "\n{harmonic} | {} | {}",
                n(frequency),
                pitch(frequency)
            )
            .unwrap();
        }
    } else {
        let harmonic = resonator.harmonic(rng.random_range(0..3));
        let pressure = resonator.boundary != Boundary::String && rng.random_bool(0.5);
        let (nodes, antinodes) = resonator.extrema(harmonic, pressure);
        write!(
            out,
            "\n\nHarmonic {harmonic}\nf = {} Hz\nλ = {} m\n{} nodes x (m) = [{}]\n{} antinodes x (m) = [{}]",
            n(resonator.frequency(harmonic)),
            n(resonator.wavelength(harmonic)),
            if pressure { "Pressure" } else { "Displacement" },
            positions(&nodes),
            if pressure { "Pressure" } else { "Displacement" },
            positions(&antinodes)
        )
        .unwrap();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::f64::consts::PI;

    #[test]
    fn eigenmodes_obey_boundary_conditions_and_the_wave_equation() {
        for (regime, seed) in (0..4).flat_map(|regime| (0..128).map(move |seed| (regime, seed))) {
            let resonator = sample_regime(&mut StdRng::seed_from_u64(seed), regime);
            for index in 0..5 {
                let harmonic = resonator.harmonic(index);
                let k = 2.0 * PI / resonator.wavelength(harmonic);
                let frequency = resonator.frequency(harmonic);
                assert!((2.0 * PI * frequency - k * resonator.speed).abs() < 1e-8);
                if resonator.boundary == Boundary::StoppedPipe {
                    assert_eq!(harmonic % 2, 1);
                    assert!((k * resonator.length).cos().abs() < 1e-12);
                } else {
                    assert!((k * resonator.length).sin().abs() < 1e-12);
                }
                if let Some(divisor) = resonator.touch {
                    assert!((k * resonator.length / divisor as f64).sin().abs() < 1e-12);
                }
                for pressure in [false, true] {
                    if pressure && resonator.boundary == Boundary::String {
                        continue;
                    }
                    let cosine = (resonator.boundary == Boundary::OpenPipe) != pressure;
                    let field = |x: f64| if cosine { (k * x).cos() } else { (k * x).sin() };
                    let (nodes, antinodes) = resonator.extrema(harmonic, pressure);
                    for x in nodes {
                        assert!(x >= 0.0 && x <= resonator.length);
                        assert!(field(x).abs() < 1e-12);
                    }
                    for x in antinodes {
                        assert!(x >= 0.0 && x <= resonator.length);
                        assert!((field(x).abs() - 1.0).abs() < 1e-12);
                    }
                }
            }
        }
    }

    #[test]
    fn stopped_pipe_has_half_the_open_pipe_fundamental() {
        let mut resonator = Resonator {
            boundary: Boundary::OpenPipe,
            length: 0.85,
            speed: 340.0,
            touch: None,
            description: String::new(),
        };
        let open_fundamental = resonator.frequency(1);
        resonator.boundary = Boundary::StoppedPipe;
        assert_eq!(2.0 * resonator.frequency(1), open_fundamental);
        assert!(air_speed(-10.0) < air_speed(20.0));
        assert!(air_speed(20.0) < air_speed(40.0));
    }
}
