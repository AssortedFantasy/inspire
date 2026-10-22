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

fn oklab(rgb: [u8; 3]) -> [f64; 3] {
    let [r, g, b] = rgb.map(|v| {
        let c = f64::from(v) / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    });
    let l = (0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b).cbrt();
    let m = (0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b).cbrt();
    let s = (0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b).cbrt();
    [
        0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
        1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
        0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
    ]
}

fn distance_squared(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| (a - b).powi(2)).sum()
}

fn candidate(rng: &mut StdRng) -> [u8; 3] {
    let lightness = rng.random_range(0.48..=0.84);
    let chroma = rng.random_range(0.08..=0.24);
    let hue = rng.random_range(0.0_f64..360.0).to_radians();
    fit_chroma(lightness, chroma, hue).map(encode)
}

fn palette(rng: &mut StdRng, count: usize) -> Vec<[u8; 3]> {
    let mut colors = vec![candidate(rng)];
    let mut positions = vec![oklab(colors[0])];
    while colors.len() < count {
        let mut best = ([0; 3], [0.0; 3], -1.0);
        for _ in 0..128 {
            let color = candidate(rng);
            let position = oklab(color);
            let nearest = positions
                .iter()
                .map(|&other| distance_squared(position, other))
                .fold(f64::INFINITY, f64::min);
            if nearest > best.2 {
                best = (color, position, nearest);
            }
        }
        colors.push(best.0);
        positions.push(best.1);
    }
    colors
}

pub fn generate(rng: &mut StdRng) -> String {
    let count = rng.random_range(4..=8);
    palette(rng, count)
        .iter()
        .map(|[r, g, b]| format!("#{r:02x}{g:02x}{b:02x}"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn reference_coordinates_and_distances() {
        for (rgb, expected) in [
            ([255, 0, 0], [0.6279553606, 0.2248630611, 0.1258462985]),
            ([0, 0, 0], [0.0; 3]),
            ([255, 255, 255], [1.0, 0.0, 0.0]),
        ] {
            for (actual, expected) in oklab(rgb).into_iter().zip(expected) {
                assert!((actual - expected).abs() < 1e-6);
            }
        }
        assert_eq!(distance_squared([0.0; 3], [1.0, 2.0, 3.0]), 14.0);
    }

    #[test]
    fn seeded_palettes_are_separated_in_quantized_oklab() {
        let mut rng = StdRng::seed_from_u64(182);
        let mut sizes = std::collections::HashSet::new();
        let mut hues = [false; 12];
        for _ in 0..2000 {
            let count = rng.random_range(4..=8);
            sizes.insert(count);
            let colors = palette(&mut rng, count);
            assert_eq!(colors.len(), count);
            let positions: Vec<_> = colors.into_iter().map(oklab).collect();
            for (i, &a) in positions.iter().enumerate() {
                assert!((0.475..=0.845).contains(&a[0]));
                let hue = a[2].atan2(a[1]).to_degrees().rem_euclid(360.0);
                hues[(hue / 30.0) as usize] = true;
                for &b in &positions[..i] {
                    assert!(distance_squared(a, b) > 0.01, "{positions:?}");
                }
            }
        }
        assert_eq!(sizes.len(), 5);
        assert!(hues.into_iter().all(|seen| seen));
    }

    #[test]
    fn seeded_output_replays_without_repeating_palettes() {
        let mut rng = StdRng::seed_from_u64(7);
        let mut replay = rng.clone();
        let mut outputs = std::collections::HashSet::new();
        for _ in 0..100 {
            let value = generate(&mut rng);
            assert_eq!(value, generate(&mut replay));
            assert!((4..=8).contains(&value.matches('#').count()));
            assert!(outputs.insert(value));
        }
    }
}
