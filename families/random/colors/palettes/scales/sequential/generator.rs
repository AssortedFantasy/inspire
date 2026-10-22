use rand::{Rng, rngs::StdRng};

fn linear_rgb(lightness: f64, chroma: f64, hue: f64) -> [f64; 3] {
    let a = chroma * hue.cos();
    let b = chroma * hue.sin();
    let l = (lightness + 0.3963377774 * a + 0.2158037573 * b).powi(3);
    let m = (lightness - 0.1055613458 * a - 0.0638541728 * b).powi(3);
    let s = (lightness - 0.0894841775 * a - 1.2914855480 * b).powi(3);
    [
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    ]
}

fn in_gamut(rgb: [f64; 3]) -> bool {
    rgb.iter().all(|c| (0.0..=1.0).contains(c))
}

fn fit_chroma(lightness: f64, chroma: f64, hue: f64) -> [f64; 3] {
    let rgb = linear_rgb(lightness, chroma, hue);
    if in_gamut(rgb) {
        return rgb;
    }
    // Fixed iteration count; reduce chroma while preserving hue and lightness.
    let (mut low, mut high) = (0.0, chroma);
    for _ in 0..18 {
        let mid = (low + high) / 2.0;
        if in_gamut(linear_rgb(lightness, mid, hue)) {
            low = mid;
        } else {
            high = mid;
        }
    }
    linear_rgb(lightness, low, hue)
}

fn encode(c: f64) -> u8 {
    let c = if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    };
    (c * 255.0).round().clamp(0.0, 255.0) as u8
}

fn ramp(hue: f64, light: f64, dark: f64, strength: f64, count: usize) -> Vec<[u8; 3]> {
    (0..count)
        .map(|i| {
            let t = i as f64 / (count - 1) as f64;
            let l = light + (dark - light) * t;
            // Pale tints start gently; richer middle tones taper at the dark end.
            let c = strength * (std::f64::consts::PI * l).sin();
            fit_chroma(l, c, hue).map(encode)
        })
        .collect()
}

pub fn generate(rng: &mut StdRng) -> String {
    let hue = rng.random_range(0.0_f64..360.0).to_radians();
    let light = rng.random_range(0.91..=0.96);
    let dark = rng.random_range(0.28..=0.42);
    let strength = rng.random_range(0.075..=0.16);
    let count = rng.random_range(5..=9);
    ramp(hue, light, dark, strength, count)
        .iter()
        .map(|[r, g, b]| format!("#{r:02x}{g:02x}{b:02x}"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn linear(channel: u8) -> f64 {
        let c = f64::from(channel) / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }

    fn oklab(rgb: [u8; 3]) -> [f64; 3] {
        let [r, g, b] = rgb.map(linear);
        let l = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b).cbrt();
        let m = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b).cbrt();
        let s = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b).cbrt();
        [
            0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
            1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
            0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
        ]
    }

    fn luminance(rgb: [u8; 3]) -> f64 {
        let [r, g, b] = rgb.map(linear);
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    #[test]
    fn transfers_and_reference_colors_match() {
        for c in 0..=255 {
            assert_eq!(encode(linear(c)), c);
        }
        for (rgb, lab) in [
            ([255; 3], [1.0, 0.0, 0.0]),
            ([255, 0, 0], [0.62795536, 0.22486306, 0.12584630]),
            ([0, 255, 0], [0.86643961, -0.23388757, 0.17949848]),
            ([0, 0, 255], [0.45201372, -0.03245698, -0.31152815]),
        ] {
            let actual = oklab(rgb);
            assert!((0..3).all(|i| (actual[i] - lab[i]).abs() < 1e-7));
            let hue = lab[2].atan2(lab[1]);
            let chroma = lab[1].hypot(lab[2]);
            assert_eq!(linear_rgb(lab[0], chroma, hue).map(encode), rgb);
        }
    }

    #[test]
    fn gamut_mapping_and_quantized_spacing_hold_at_parameter_boundaries() {
        for degrees in 0..360 {
            let hue = f64::from(degrees).to_radians();
            for light in [0.91, 0.96] {
                for dark in [0.28, 0.42] {
                    for strength in [0.075, 0.16] {
                        for count in 5..=9 {
                            let colors = ramp(hue, light, dark, strength, count);
                            for (i, &rgb) in colors.iter().enumerate() {
                                let l = light + (dark - light) * i as f64 / (count - 1) as f64;
                                let c = strength * (std::f64::consts::PI * l).sin();
                                let fitted = fit_chroma(l, c, hue);
                                assert!(in_gamut(fitted));
                                let lab = oklab(rgb);
                                assert!((lab[0] - l).abs() < 0.003);
                                let hue_error = lab[2].atan2(lab[1]) - hue;
                                assert!(
                                    hue_error.cos() > 0.98,
                                    "hue {degrees}, L {l}, strength {strength}: {rgb:?}, error {} degrees",
                                    hue_error.sin().atan2(hue_error.cos()).to_degrees()
                                );
                            }
                            for pair in colors.windows(2) {
                                assert!(luminance(pair[0]) > luminance(pair[1]));
                                assert!(oklab(pair[0])[0] - oklab(pair[1])[0] > 0.055);
                                assert_ne!(pair[0], pair[1]);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn seeded_outputs_are_distinct_and_reproducible() {
        let mut a = StdRng::seed_from_u64(18);
        let mut b = StdRng::seed_from_u64(18);
        let mut outputs = std::collections::HashSet::new();
        for _ in 0..2000 {
            let value = generate(&mut a);
            assert_eq!(value, generate(&mut b));
            assert!(!value.ends_with('\n'));
            assert!((5..=9).contains(&value.matches('#').count()));
            let colors: Vec<[u8; 3]> = value
                .split('#')
                .skip(1)
                .map(|part| {
                    std::array::from_fn(|i| {
                        u8::from_str_radix(&part[i * 2..i * 2 + 2], 16).unwrap()
                    })
                })
                .collect();
            for pair in colors.windows(2) {
                assert!(luminance(pair[0]) > luminance(pair[1]));
                assert!(oklab(pair[0])[0] - oklab(pair[1])[0] > 0.055);
            }
            assert!(outputs.insert(value));
        }
    }
}
