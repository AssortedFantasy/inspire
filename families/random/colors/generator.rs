use rand::{Rng, rngs::StdRng};

fn color(rng: &mut StdRng, style: u32) -> String {
    let [r, g, b]: [u8; 3] = rng.random();
    match style {
        0 => format!("#{r:02x}{g:02x}{b:02x}"),
        1 => {
            let a: u8 = rng.random_range(64..=255);
            format!("#{r:02x}{g:02x}{b:02x}{a:02x}")
        }
        2 => format!("rgb({r} {g} {b})"),
        3 => format!(
            "rgb({}% {}% {}%)",
            rng.random_range(0..=100),
            rng.random_range(0..=100),
            rng.random_range(0..=100)
        ),
        4 => format!(
            "rgba({r}, {g}, {b}, {:.2})",
            rng.random_range(25..=100) as f64 / 100.0
        ),
        5 => format!(
            "hsl({} {}% {}%)",
            rng.random_range(0..360),
            rng.random_range(0..=100),
            rng.random_range(0..=100)
        ),
        _ => format!(
            "hsl({} {}% {}% / {}%)",
            rng.random_range(0..360),
            rng.random_range(0..=100),
            rng.random_range(0..=100),
            rng.random_range(25..=100)
        ),
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let style = rng.random_range(0..7);
    color(rng, style)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn channels_and_alpha_are_in_css_ranges() {
        let mut rng = StdRng::seed_from_u64(13);
        for style in 0..7 {
            for _ in 0..1000 {
                let value = color(&mut rng, style);
                if style < 2 {
                    assert_eq!(value.len(), if style == 0 { 7 } else { 9 });
                    assert!(value[1..].bytes().all(|b| b.is_ascii_hexdigit()));
                    continue;
                }
                let inside = value.split_once('(').unwrap().1.strip_suffix(')').unwrap();
                let numbers: Vec<f64> = inside
                    .split([' ', ',', '/', '%'])
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse().unwrap())
                    .collect();
                assert_eq!(numbers.len(), if style == 4 || style == 6 { 4 } else { 3 });
                for (i, &n) in numbers.iter().enumerate() {
                    let max = match (style, i) {
                        (4, 3) => 1.0,
                        (5 | 6, 0) => 359.0,
                        (2 | 4, _) => 255.0,
                        _ => 100.0,
                    };
                    assert!((0.0..=max).contains(&n), "{value}");
                }
            }
        }
    }

    #[test]
    fn samples_are_bare_colors_and_cover_every_notation() {
        let mut rng = StdRng::seed_from_u64(3);
        let mut seen = [false; 7];
        for _ in 0..1000 {
            let sample = generate(&mut rng);
            let style = if sample.starts_with('#') {
                usize::from(sample.len() == 9)
            } else if sample.starts_with("rgb(") {
                if sample.contains('%') { 3 } else { 2 }
            } else if sample.starts_with("rgba(") {
                4
            } else if sample.contains('/') {
                6
            } else {
                assert!(sample.starts_with("hsl("));
                5
            };
            seen[style] = true;
            assert!(!sample.contains(['\n', ':', ';']));
        }
        assert!(seen.into_iter().all(|s| s));
    }
}
