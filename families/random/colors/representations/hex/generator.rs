use rand::{Rng, rngs::StdRng};

fn encode([red, green, blue]: [u8; 3]) -> String {
    format!("#{red:02x}{green:02x}{blue:02x}")
}

pub fn generate(rng: &mut StdRng) -> String {
    encode(rng.random())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn decode(value: &str) -> [u8; 3] {
        assert_eq!(value.len(), 7);
        assert!(value.starts_with('#'));
        assert!(
            value[1..]
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        );
        [1, 3, 5].map(|start| u8::from_str_radix(&value[start..start + 2], 16).unwrap())
    }

    #[test]
    fn all_byte_values_roundtrip_in_every_channel() {
        for channel in 0..3 {
            for byte in 0..=255 {
                let mut channels = [0, 128, 255];
                channels[channel] = byte;
                assert_eq!(decode(&encode(channels)), channels);
            }
        }
        assert_eq!(encode([0, 0, 0]), "#000000");
        assert_eq!(encode([255, 255, 255]), "#ffffff");
    }

    #[test]
    fn generated_colors_preserve_uniform_rgb_draws() {
        let mut actual = StdRng::seed_from_u64(715);
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
