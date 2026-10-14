use rand::{Rng, rngs::StdRng};

// Björn Ottosson's public-domain 2021 matrices; see README.md.
fn linear_srgb(lightness: f64, chroma: f64, hue: f64) -> [f64; 3] {
    let a = chroma * hue.to_radians().cos();
    let b = chroma * hue.to_radians().sin();
    let l = (lightness + 0.3963377774 * a + 0.2158037573 * b).powi(3);
    let m = (lightness - 0.1055613458 * a - 0.0638541728 * b).powi(3);
    let s = (lightness - 0.0894841775 * a - 1.2914855480 * b).powi(3);
    [
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    ]
}

fn in_gamut(lightness: f64, chroma: f64, hue: f64) -> bool {
    linear_srgb(lightness, chroma, hue)
        .into_iter()
        .all(|channel| (0.0..=1.0).contains(&channel))
}

fn maximum_chroma(lightness: f64, hue: f64) -> f64 {
    let (mut low, mut high) = (0.0, 0.4);
    for _ in 0..24 {
        let middle = (low + high) / 2.0;
        if in_gamut(lightness, middle, hue) {
            low = middle;
        } else {
            high = middle;
        }
    }
    low
}

pub fn generate(rng: &mut StdRng) -> String {
    let lightness = f64::from(rng.random_range(250..=900)) / 1000.0;
    let hue = f64::from(rng.random_range(0..3600)) / 10.0;
    let fraction = f64::from(rng.random_range(150..=950)) / 1000.0;
    // Stay inside the boundary; round downward so decimal output remains in gamut.
    let chroma = (maximum_chroma(lightness, hue) * fraction * 1_000_000.0).floor() / 1_000_000.0;
    format!("oklch({lightness:.3} {chroma:.6} {hue:.1})")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn known_srgb_red_and_neutral_coordinates_convert_correctly() {
        let red = linear_srgb(0.6279553606, 0.2576833077, 29.2338852);
        for (actual, expected) in red.into_iter().zip([1.0, 0.0, 0.0]) {
            assert!((actual - expected).abs() < 1e-6);
        }
        for lightness in [0.0_f64, 0.25, 0.5, 0.9, 1.0] {
            for channel in linear_srgb(lightness, 0.0, 120.0) {
                assert!((channel - lightness.powi(3)).abs() < 1e-9);
            }
        }
    }

    #[test]
    fn chroma_search_reaches_gamut_boundary_across_the_domain() {
        for lightness in (250..=900).step_by(25) {
            for hue in (0..360).step_by(3) {
                let l = f64::from(lightness) / 1000.0;
                let h = f64::from(hue);
                let c = maximum_chroma(l, h);
                assert!(c > 0.025 && c < 0.4);
                assert!(in_gamut(l, c, h));
                assert!(!in_gamut(l, c + 1e-5, h));
                assert!(in_gamut(l, c * 0.95, h));
            }
        }
    }

    #[test]
    fn emitted_decimal_colors_stay_in_gamut_and_cover_the_domain() {
        let mut rng = StdRng::seed_from_u64(923);
        let mut hues = [false; 12];
        let mut lightnesses = [false; 4];
        let mut chromas = [false; 2];
        for _ in 0..10_000 {
            let value = generate(&mut rng);
            let fields: Vec<f64> = value
                .strip_prefix("oklch(")
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_whitespace()
                .map(|field| field.parse().unwrap())
                .collect();
            assert_eq!(fields.len(), 3);
            let [l, c, h] = [fields[0], fields[1], fields[2]];
            assert!((0.25..=0.9).contains(&l));
            assert!((0.0..360.0).contains(&h));
            assert!(c > 0.0 && in_gamut(l, c, h), "{value}");
            hues[(h / 30.0) as usize] = true;
            lightnesses[((l - 0.25) / 0.163).min(3.0) as usize] = true;
            chromas[usize::from(c > 0.15)] = true;
        }
        assert!(hues.into_iter().all(|seen| seen));
        assert!(lightnesses.into_iter().all(|seen| seen));
        assert!(chromas.into_iter().all(|seen| seen));
    }
}
