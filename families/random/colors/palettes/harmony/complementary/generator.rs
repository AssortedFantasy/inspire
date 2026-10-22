use rand::{Rng, rngs::StdRng};

fn hues(hue: i32) -> [i32; 2] {
    [hue, (hue + 180).rem_euclid(360)]
}

pub fn generate(rng: &mut StdRng) -> String {
    let [base, accent] = hues(rng.random_range(0..360));
    let saturation = rng.random_range(45..=75);
    let middle = rng.random_range(42..=56);
    let tint = rng.random_range(80..=90);
    let shade = rng.random_range(18..=28);
    format!(
        "base: hsl({base} {saturation}% {middle}%)\nbase tint: hsl({base} {}% {tint}%)\nbase shade: hsl({base} {}% {shade}%)\naccent: hsl({accent} {saturation}% {middle}%)\naccent tint: hsl({accent} {}% {tint}%)",
        saturation - 15,
        saturation - 10,
        saturation - 15
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn rgb8([h, s, l]: [i32; 3]) -> [u8; 3] {
        let (s, l) = (s as f64 / 100.0, l as f64 / 100.0);
        let chroma = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let sector = h as f64 / 60.0;
        let x = chroma * (1.0 - (sector % 2.0 - 1.0).abs());
        let channels = match h / 60 {
            0 => [chroma, x, 0.0],
            1 => [x, chroma, 0.0],
            2 => [0.0, chroma, x],
            3 => [0.0, x, chroma],
            4 => [x, 0.0, chroma],
            _ => [chroma, 0.0, x],
        };
        channels.map(|v| ((v + l - chroma / 2.0) * 255.0).round() as u8)
    }

    fn luminance(rgb: [u8; 3]) -> f64 {
        let linear = rgb.map(|v| {
            let v = f64::from(v) / 255.0;
            if v <= 0.04045 {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        });
        0.2126 * linear[0] + 0.7152 * linear[1] + 0.0722 * linear[2]
    }
    #[test]
    fn opposite_hues_cover_every_rotation() {
        for h in 0..360 {
            let [a, b] = hues(h);
            assert!((0..360).contains(&b));
            assert_eq!((b - a).rem_euclid(360), 180);
        }
    }
    #[test]
    fn emitted_palettes_preserve_harmony_roles_and_brightness() {
        let mut rng = StdRng::seed_from_u64(5);
        let mut replay = StdRng::seed_from_u64(5);
        let mut seen = std::collections::HashSet::new();
        let mut parameters: [_; 5] = std::array::from_fn(|_| std::collections::HashSet::new());
        for _ in 0..10_000 {
            let value = generate(&mut rng);
            assert_eq!(value, generate(&mut replay));
            assert!(!value.ends_with('\n'));
            assert_eq!(value.matches("hsl(").count(), 5);
            let colors: Vec<[i32; 3]> = value
                .split("hsl(")
                .skip(1)
                .map(|part| {
                    part.split(')')
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .map(|v| v.trim_end_matches('%').parse().unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect();
            let [base, tint, shade, accent, accent_tint] = colors[..] else {
                panic!("expected five swatches");
            };
            assert!((0..360).contains(&base[0]));
            assert!((0..360).contains(&accent[0]));
            assert_eq!((accent[0] - base[0]).rem_euclid(360), 180);
            assert_eq!([tint[0], shade[0]], [base[0]; 2]);
            assert_eq!(accent_tint[0], accent[0]);
            assert_eq!(accent[1..], base[1..]);
            assert!((45..=75).contains(&base[1]));
            assert!((42..=56).contains(&base[2]));
            assert!((80..=90).contains(&tint[2]));
            assert!((18..=28).contains(&shade[2]));
            assert_eq!(tint[1], base[1] - 15);
            assert_eq!(shade[1], base[1] - 10);
            assert_eq!(accent_tint[1..], tint[1..]);
            let rgb: Vec<_> = colors.into_iter().map(rgb8).collect();
            assert_eq!(
                rgb.iter().collect::<std::collections::HashSet<_>>().len(),
                5
            );
            assert!(luminance(rgb[2]) < luminance(rgb[0]));
            assert!(luminance(rgb[0]) < luminance(rgb[1]));
            assert!(luminance(rgb[3]) < luminance(rgb[4]));
            for (set, parameter) in parameters
                .iter_mut()
                .zip([base[0], base[1], base[2], tint[2], shade[2]])
            {
                set.insert(parameter);
            }
            seen.insert(value);
        }
        assert!(seen.len() > 9900);
        assert_eq!(parameters.map(|set| set.len()), [360, 31, 15, 11, 11]);
    }
}
