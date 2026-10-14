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

fn encode(c: f64) -> u8 {
    let c = if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    };
    (c * 255.0).round().clamp(0.0, 255.0) as u8
}

struct Scale {
    hues: [f64; 2],
    dark: f64,
    light: f64,
    chroma: f64,
    half: usize,
}

impl Scale {
    fn wing(&self, side: usize, step: usize, chroma: f64) -> [f64; 3] {
        let t = step as f64 / self.half as f64;
        linear_rgb(
            self.dark + (self.light - self.dark) * t,
            chroma * (1.0 - t),
            self.hues[side],
        )
    }

    fn fits(&self, chroma: f64) -> bool {
        (0..2).all(|side| (0..=self.half).all(|step| in_gamut(self.wing(side, step, chroma))))
    }

    fn fitted_chroma(&self) -> f64 {
        if self.fits(self.chroma) {
            return self.chroma;
        }
        // Fit both complete wings together to preserve equal chroma at paired steps.
        let (mut low, mut high) = (0.0, self.chroma);
        for _ in 0..18 {
            let mid = (low + high) / 2.0;
            if self.fits(mid) {
                low = mid;
            } else {
                high = mid;
            }
        }
        low
    }

    fn colors(&self) -> Vec<[u8; 3]> {
        let chroma = self.fitted_chroma();
        (0..=self.half)
            .map(|i| self.wing(0, i, chroma).map(encode))
            .chain(
                (0..self.half)
                    .rev()
                    .map(|i| self.wing(1, i, chroma).map(encode)),
            )
            .collect()
    }
}

fn sample(rng: &mut StdRng) -> Scale {
    let hue = rng.random_range(0.0_f64..360.0);
    Scale {
        hues: [
            hue.to_radians(),
            (hue + rng.random_range(120.0_f64..=240.0)).to_radians(),
        ],
        dark: rng.random_range(0.42..=0.55),
        light: rng.random_range(0.94..=0.97),
        chroma: rng.random_range(0.09..=0.15),
        half: rng.random_range(2..=4),
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let scale = sample(rng);
    let colors = scale
        .colors()
        .iter()
        .map(|[r, g, b]| format!("#{r:02x}{g:02x}{b:02x}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "Diverging palette, {} ordered steps through a pale neutral midpoint (matched OKLCH lightness and chroma on both wings; fitted to sRGB):\n{colors}",
        2 * scale.half + 1
    )
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

    fn check(scale: &Scale) {
        let colors = scale.colors();
        let chroma = scale.fitted_chroma();
        assert!(scale.fits(chroma));
        assert!(chroma > 0.05);
        assert_eq!(colors.len(), 2 * scale.half + 1);
        let center = colors[scale.half];
        assert_eq!(center, [encode(scale.light.powi(3)); 3]);
        assert_eq!(
            colors
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len(),
            colors.len()
        );
        for i in 0..scale.half {
            let left = oklab(colors[i]);
            let right = oklab(colors[2 * scale.half - i]);
            assert!((left[0] - right[0]).abs() < 0.004);
            assert!((left[1].hypot(left[2]) - right[1].hypot(right[2])).abs() < 0.004);
            let expected_l = scale.dark + (scale.light - scale.dark) * i as f64 / scale.half as f64;
            let expected_c = chroma * (1.0 - i as f64 / scale.half as f64);
            for (side, lab) in [left, right].into_iter().enumerate() {
                assert!((lab[0] - expected_l).abs() < 0.003);
                assert!((lab[1].hypot(lab[2]) - expected_c).abs() < 0.003);
                assert!((lab[2].atan2(lab[1]) - scale.hues[side]).cos() > 0.97);
            }
        }
        for wing in [
            colors[..=scale.half].to_vec(),
            colors[scale.half..].iter().rev().copied().collect(),
        ] {
            for pair in wing.windows(2) {
                assert!(luminance(pair[0]) < luminance(pair[1]));
                assert!(oklab(pair[1])[0] - oklab(pair[0])[0] > 0.09);
            }
        }
        let left = oklab(colors[0]);
        let right = oklab(*colors.last().unwrap());
        assert!((left[1] - right[1]).hypot(left[2] - right[2]) > 0.085);
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
            assert_eq!(
                linear_rgb(lab[0], lab[1].hypot(lab[2]), lab[2].atan2(lab[1])).map(encode),
                rgb
            );
        }
    }

    #[test]
    fn gamut_balance_and_order_hold_at_parameter_boundaries() {
        for hue in (0..360).step_by(10) {
            for separation in [120.0_f64, 180.0, 240.0] {
                for dark in [0.42, 0.55] {
                    for light in [0.94, 0.97] {
                        for chroma in [0.09, 0.15] {
                            for half in 2..=4 {
                                check(&Scale {
                                    hues: [
                                        f64::from(hue).to_radians(),
                                        (f64::from(hue) + separation).to_radians(),
                                    ],
                                    dark,
                                    light,
                                    chroma,
                                    half,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn seeded_palettes_are_balanced_distinct_and_reproducible() {
        let mut a = StdRng::seed_from_u64(34);
        let mut b = StdRng::seed_from_u64(34);
        let mut outputs = std::collections::HashSet::new();
        for _ in 0..2000 {
            check(&sample(&mut a));
            let value = generate(&mut b);
            assert!(matches!(value.matches('#').count(), 5 | 7 | 9));
            assert!(!value.ends_with('\n'));
            assert!(outputs.insert(value));
        }
        let mut a = StdRng::seed_from_u64(17);
        let mut b = StdRng::seed_from_u64(17);
        for _ in 0..100 {
            assert_eq!(generate(&mut a), generate(&mut b));
        }
    }
}
