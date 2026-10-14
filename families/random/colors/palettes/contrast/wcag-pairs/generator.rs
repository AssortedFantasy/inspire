use rand::{Rng, rngs::StdRng};

fn luminance(rgb: [u8; 3]) -> f64 {
    let c = rgb.map(|v| {
        let x = f64::from(v) / 255.0;
        if x <= 0.04045 {
            x / 12.92
        } else {
            ((x + 0.055) / 1.055).powf(2.4)
        }
    });
    0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2]
}
fn contrast(a: [u8; 3], b: [u8; 3]) -> f64 {
    let (a, b) = (luminance(a), luminance(b));
    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}
fn pair(rng: &mut StdRng) -> ([u8; 3], [u8; 3]) {
    let background: [u8; 3] = rng.random();
    let dark = contrast(background, [0; 3]) >= contrast(background, [255; 3]);
    for _ in 0..48 {
        let foreground = std::array::from_fn(|_| {
            if dark {
                rng.random_range(0..=95)
            } else {
                rng.random_range(160..=255)
            }
        });
        if contrast(foreground, background) >= 4.5 {
            return (foreground, background);
        }
    }
    (if dark { [0; 3] } else { [255; 3] }, background)
}
pub fn generate(rng: &mut StdRng) -> String {
    let (fg, bg) = pair(rng);
    let [r, g, b] = fg;
    let [br, bg, bb] = bg;
    let ratio = (contrast(fg, [br, bg, bb]) * 100.0).floor() / 100.0;
    format!(
        "Opaque sRGB text/background pair (WCAG 2 contrast ≥4.5:1):\nforeground: #{r:02x}{g:02x}{b:02x}\nbackground: #{br:02x}{bg:02x}{bb:02x}\ncontrast: {ratio:.2}:1 (rounded down)"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn primary_luminances_and_srgb_transfer_boundary() {
        for (rgb, expected) in [
            ([255, 0, 0], 0.2126),
            ([0, 255, 0], 0.7152),
            ([0, 0, 255], 0.0722),
            ([10; 3], 0.003035269835488375),
            ([11; 3], 0.003346535763899161),
        ] {
            assert!((luminance(rgb) - expected).abs() < 1e-14);
        }
    }
    #[test]
    fn standard_reference_ratios() {
        assert!((contrast([0; 3], [255; 3]) - 21.0).abs() < 1e-12);
        assert!((contrast([119; 3], [255; 3]) - 4.478089).abs() < 1e-5);
        assert!((contrast([118; 3], [255; 3]) - 4.542225).abs() < 1e-5);
        assert_eq!(contrast([70; 3], [70; 3]), 1.0);
    }
    #[test]
    fn every_quantized_pair_passes_aa_normal_text() {
        let mut rng = StdRng::seed_from_u64(91);
        let mut black = 0;
        let mut white = 0;
        let mut chromatic = 0;
        for _ in 0..10000 {
            let (fg, bg) = pair(&mut rng);
            assert!(contrast(fg, bg) >= 4.5);
            assert_eq!(contrast(fg, bg), contrast(bg, fg));
            if fg == [0; 3] {
                black += 1;
            } else if fg == [255; 3] {
                white += 1;
            } else {
                chromatic += 1;
            }
        }
        assert!(chromatic > 9000);
        assert!(black + white < 1000);
    }
    #[test]
    fn output_ratios_are_conservative_and_parseable() {
        let mut rng = StdRng::seed_from_u64(52);
        for _ in 0..500 {
            let value = generate(&mut rng);
            let lines: Vec<_> = value.lines().collect();
            let parse = |line: &str| {
                let hex = line.split_once('#').unwrap().1;
                std::array::from_fn(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap())
            };
            let exact = contrast(parse(lines[1]), parse(lines[2]));
            let printed: f64 = lines[3]
                .split_whitespace()
                .nth(1)
                .unwrap()
                .trim_end_matches(":1")
                .parse()
                .unwrap();
            assert!(printed >= 4.5 && printed <= exact && exact - printed < 0.0100001);
        }
    }
}
