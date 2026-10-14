use rand::{Rng, rngs::StdRng};

pub fn generate(rng: &mut StdRng) -> String {
    let hue = rng.random_range(0..360);
    let saturation = rng.random_range(20..=100);
    let lightness = rng.random_range(15..=85);
    format!("hsl({hue} {saturation}% {lightness}%)")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn colors_cover_hue_saturation_and_lightness_without_powerless_hues() {
        let mut rng = StdRng::seed_from_u64(419);
        let mut hues = [false; 36];
        let mut saturations = [false; 81];
        let mut lightnesses = [false; 71];
        for _ in 0..10_000 {
            let value = generate(&mut rng);
            let components: Vec<u16> = value
                .strip_prefix("hsl(")
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split_whitespace()
                .map(|value| value.trim_end_matches('%').parse().unwrap())
                .collect();
            assert_eq!(components.len(), 3);
            let [h, s, l] = [components[0], components[1], components[2]];
            assert!(h < 360 && (20..=100).contains(&s) && (15..=85).contains(&l));
            // HSL chroma is positive for this domain, so hue is meaningful.
            let chroma = (1.0 - (2.0 * f64::from(l) / 100.0 - 1.0).abs()) * f64::from(s) / 100.0;
            assert!(chroma >= 0.06 - 1e-12 && chroma <= 1.0);
            hues[usize::from(h / 10)] = true;
            saturations[usize::from(s - 20)] = true;
            lightnesses[usize::from(l - 15)] = true;
        }
        assert!(hues.into_iter().all(|seen| seen));
        assert!(saturations.into_iter().all(|seen| seen));
        assert!(lightnesses.into_iter().all(|seen| seen));
    }
}
