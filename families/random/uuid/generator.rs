use rand::{Rng, rngs::StdRng};

pub fn generate(rng: &mut StdRng) -> String {
    let mut bytes = [0u8; 16];
    rng.fill(&mut bytes);
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    format!(
        "{}-{}-{}-{}-{}",
        &hex[..8],
        &hex[8..12],
        &hex[12..16],
        &hex[16..20],
        &hex[20..]
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashSet;

    #[test]
    fn samples_have_version_variant_and_canonical_shape() {
        let mut rng = StdRng::seed_from_u64(17);
        let mut seen = HashSet::new();
        for _ in 0..10000 {
            let uuid = generate(&mut rng);
            assert_eq!(uuid.len(), 36);
            assert_eq!(uuid.as_bytes()[14], b'4');
            assert!(matches!(uuid.as_bytes()[19], b'8' | b'9' | b'a' | b'b'));
            for (i, b) in uuid.bytes().enumerate() {
                if [8, 13, 18, 23].contains(&i) {
                    assert_eq!(b, b'-');
                } else {
                    assert!(b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
                }
            }
            assert!(seen.insert(uuid));
        }
    }

    #[test]
    fn supplied_seed_reproduces_the_sequence() {
        let mut a = StdRng::seed_from_u64(23);
        let mut b = StdRng::seed_from_u64(23);
        for _ in 0..100 {
            assert_eq!(generate(&mut a), generate(&mut b));
        }
    }
}
