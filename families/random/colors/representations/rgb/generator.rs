use rand::{Rng, rngs::StdRng};

fn encode([red, green, blue]: [u8; 3]) -> String {
    format!("rgb({red} {green} {blue})")
}

pub fn generate(rng: &mut StdRng) -> String {
    encode(rng.random())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn decode(value: &str) -> [u8; 3] {
        let channels: Vec<u8> = value
            .strip_prefix("rgb(")
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_whitespace()
            .map(|value| value.parse().unwrap())
            .collect();
        channels.try_into().unwrap()
    }

    #[test]
    fn decimal_encoding_preserves_every_channel_value() {
        for channel in 0..3 {
            for byte in 0..=255 {
                let mut channels = [0, 128, 255];
                channels[channel] = byte;
                assert_eq!(decode(&encode(channels)), channels);
            }
        }
        assert_eq!(encode([0, 0, 0]), "rgb(0 0 0)");
        assert_eq!(encode([255, 255, 255]), "rgb(255 255 255)");
    }

    #[test]
    fn generated_colors_preserve_uniform_rgb_draws() {
        let mut actual = StdRng::seed_from_u64(317);
        let mut expected = actual.clone();
        let mut seen = [[false; 256]; 3];
        for _ in 0..10_000 {
            let channels = decode(&generate(&mut actual));
            assert_eq!(channels, expected.random::<[u8; 3]>());
            for (channel, byte) in channels.into_iter().enumerate() {
                seen[channel][usize::from(byte)] = true;
            }
        }
        assert!(seen.into_iter().flatten().all(|value| value));
    }
}
