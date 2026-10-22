use rand::{Rng, rngs::StdRng};

fn palette(hue: i32, step: i32, saturation: i32, lightness: i32) -> Vec<(i32, i32, i32)> {
    (-2..=2)
        .map(|offset| ((hue + offset * step).rem_euclid(360), saturation, lightness))
        .collect()
}

pub fn generate(rng: &mut StdRng) -> String {
    let hue = rng.random_range(0..360);
    let step = rng.random_range(10..=20);
    let saturation = rng.random_range(40..=75);
    let lightness = rng.random_range(38..=65);
    palette(hue, step, saturation, lightness)
        .iter()
        .map(|(h, s, l)| format!("hsl({h} {s}% {l}%)"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn rgb8(h: i32, s: i32, l: i32) -> [u8; 3] {
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

    #[test]
    fn wraps_hue_preserves_steps_and_shared_saturation_lightness() {
        for hue in 0..360 {
            for step in 10..=20 {
                let colors = palette(hue, step, 60, 50);
                assert_eq!(colors.len(), 5);
                for pair in colors.windows(2) {
                    assert_eq!((pair[1].0 - pair[0].0).rem_euclid(360), step);
                }
                assert!(
                    colors
                        .iter()
                        .all(|&(h, s, l)| (0..360).contains(&h) && s == 60 && l == 50)
                );
            }
        }
    }
    #[test]
    fn generated_palettes_are_reproducible_and_distinct() {
        let mut a = StdRng::seed_from_u64(8);
        let mut b = StdRng::seed_from_u64(8);
        let samples: std::collections::HashSet<_> = (0..1000)
            .map(|_| {
                let value = generate(&mut a);
                assert_eq!(value, generate(&mut b));
                assert_eq!(value.matches("hsl(").count(), 5);
                value
            })
            .collect();
        assert!(samples.len() > 990);
    }

    #[test]
    fn emitted_colors_preserve_the_arc_and_cover_the_parameter_space() {
        let mut rng = StdRng::seed_from_u64(79);
        let mut centers = std::collections::HashSet::new();
        let mut steps = std::collections::HashSet::new();
        let mut saturations = std::collections::HashSet::new();
        let mut lightnesses = std::collections::HashSet::new();
        for _ in 0..10_000 {
            let value = generate(&mut rng);
            assert!(!value.ends_with('\n'));
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
            assert_eq!(colors.len(), 5);
            let step = (colors[1][0] - colors[0][0]).rem_euclid(360);
            assert!((10..=20).contains(&step));
            for &[h, s, l] in &colors {
                assert!((0..360).contains(&h));
                assert!((40..=75).contains(&s));
                assert!((38..=65).contains(&l));
                assert_eq!([s, l], [colors[2][1], colors[2][2]]);
            }
            for pair in colors.windows(2) {
                assert_eq!((pair[1][0] - pair[0][0]).rem_euclid(360), step);
            }
            assert_eq!((colors[4][0] - colors[0][0]).rem_euclid(360), 4 * step);
            centers.insert(colors[2][0]);
            steps.insert(step);
            saturations.insert(colors[2][1]);
            lightnesses.insert(colors[2][2]);
        }
        assert_eq!(centers.len(), 360);
        assert_eq!(steps.len(), 11);
        assert_eq!(saturations.len(), 36);
        assert_eq!(lightnesses.len(), 28);
    }

    #[test]
    fn swatches_remain_distinct_after_rgb_byte_rounding() {
        // Check all hue rotations and spacing choices at the least chromatic
        // saturation and both lightness limits, including crossings at 0°.
        for hue in 0..360 {
            for step in 10..=20 {
                for lightness in [38, 65] {
                    let colors: Vec<_> = palette(hue, step, 40, lightness)
                        .into_iter()
                        .map(|(h, s, l)| rgb8(h, s, l))
                        .collect();
                    let unique: std::collections::HashSet<_> = colors.iter().collect();
                    assert_eq!(unique.len(), 5);
                    for pair in colors.windows(2) {
                        let distance_squared: i32 = (0..3)
                            .map(|i| (i32::from(pair[0][i]) - i32::from(pair[1][i])).pow(2))
                            .sum();
                        assert!(
                            distance_squared >= 49,
                            "hue {hue}, step {step}, lightness {lightness}: {pair:?}, distance² {distance_squared}"
                        );
                    }
                }
            }
        }
    }
}
