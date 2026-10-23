use rand::{Rng, rngs::StdRng};

// Encoding: https://docs.oracle.com/cd/E77782_01/html/E77791/z4000ac019878.html
// Rounding and NaN portability: https://doc.rust-lang.org/std/primitive.f32.html
const FRACTION: u32 = 0x007f_ffff;
const SIGN: u32 = 0x8000_0000;

pub fn generate(rng: &mut StdRng) -> String {
    match rng.random_range(0..12) {
        0..=3 => decomposition(rng),
        4..=5 => neighbors(rng),
        6..=7 => midpoint(rng),
        8 => underflow(rng),
        9 => association(rng),
        10 => special(rng),
        _ => nan(rng),
    }
}

fn dyadic(coefficient: i64, exponent: i32) -> String {
    if coefficient == 0 {
        return "0".into();
    }
    let shift = coefficient.unsigned_abs().trailing_zeros();
    let coefficient = coefficient >> shift;
    let exponent = exponent + shift as i32;
    match (coefficient, exponent) {
        (_, 0) => coefficient.to_string(),
        (1, _) => format!("2^{exponent}"),
        (-1, _) => format!("-2^{exponent}"),
        _ => format!("{coefficient} × 2^{exponent}"),
    }
}

fn components(bits: u32) -> (i64, i32) {
    let exponent = (bits >> 23) & 255;
    assert!(exponent < 255);
    let fraction = bits & FRACTION;
    let (significand, power) = if exponent == 0 {
        (fraction, -149)
    } else {
        ((1 << 23) | fraction, exponent as i32 - 150)
    };
    let sign = if bits & SIGN == 0 { 1 } else { -1 };
    (sign * i64::from(significand), power)
}

fn exact(bits: u32) -> String {
    if bits == SIGN {
        return "-0".into();
    }
    let (coefficient, exponent) = components(bits);
    dyadic(coefficient, exponent)
}

fn decomposition(rng: &mut StdRng) -> String {
    let sign = u32::from(rng.random_bool(0.5));
    let exponent: u32 = match rng.random_range(0..5) {
        0 => 0,
        1 => [1, 2, 253, 254][rng.random_range(0..4)],
        2 => rng.random_range(1..255),
        _ => rng.random_range(121..138),
    };
    let fraction = match rng.random_range(0..5) {
        0 => 1,
        1 => FRACTION,
        2 => 1 << rng.random_range(0..23),
        _ => rng.random_range(0..=FRACTION),
    };
    let bits = (sign << 31) | (exponent << 23) | fraction;
    let class = match (exponent, fraction) {
        (0, 0) => "zero",
        (0, _) => "subnormal",
        _ => "normal",
    };
    format!(
        "binary32 0x{bits:08x} ({class})\nsign      {sign}\nexponent  {exponent:08b}\nfraction  {fraction:023b}\n= {}",
        exact(bits)
    )
}

fn neighbors(rng: &mut StdRng) -> String {
    let exponent: u32 = match rng.random_range(0..4) {
        0 => 1,
        1 => 127,
        _ => rng.random_range(2..255),
    };
    let center = exponent << 23;
    let power = exponent as i32 - 127;
    let below = if exponent == 1 { -149 } else { power - 24 };
    let above = power - 23;
    format!(
        "binary32 neighbors of {}\n0x{:08x}  {}\n0x{center:08x}  {}\n0x{:08x}  {}\ngap below = {}\ngap above = {}",
        exact(center),
        center - 1,
        exact(center - 1),
        exact(center),
        center + 1,
        exact(center + 1),
        dyadic(1, below),
        dyadic(1, above)
    )
}

fn midpoint(rng: &mut StdRng) -> String {
    let exponent = rng.random_range(27..228_u32);
    let fraction = rng.random_range(0..FRACTION);
    let lower = (exponent << 23) | fraction;
    let half_ulp = exponent as i32 - 151;
    let rounded = lower + (lower & 1);
    format!(
        "binary32, roundTiesToEven\nx = {} (0x{lower:08x})\nx + {} = {} (0x{rounded:08x})\nmidpoint between 0x{lower:08x} and 0x{:08x}",
        exact(lower),
        dyadic(1, half_ulp),
        exact(rounded),
        lower + 1
    )
}

fn underflow(rng: &mut StdRng) -> String {
    let sign = if rng.random_bool(0.5) { SIGN } else { 0 };
    match rng.random_range(0..3) {
        0 => {
            let lower = rng.random_range(0..128_u32);
            let rounded = lower + (lower & 1);
            let input = dyadic(
                if sign == 0 { 1 } else { -1 } * i64::from(2 * lower + 1),
                -150,
            );
            format!(
                "binary32, roundTiesToEven\n{input} → {} (0x{:08x})",
                exact(sign | rounded),
                sign | rounded
            )
        }
        1 => format!(
            "binary32\n({}) + ({}) = {}\nresult 0x{:08x}",
            exact(sign | FRACTION),
            exact(sign | 1),
            exact(sign | 0x0080_0000),
            sign | 0x0080_0000
        ),
        _ => format!(
            "binary32\n({}) - ({}) = {}\nresult 0x{:08x}",
            exact(sign | 0x0080_0000),
            exact(sign | FRACTION),
            exact(sign | 1),
            sign | 1
        ),
    }
}

fn association(rng: &mut StdRng) -> String {
    let power = rng.random_range(-30..80);
    let large = dyadic(1, power);
    let small = dyadic(1, power - 25);
    format!(
        "binary32, roundTiesToEven\na = {large}\nb = -{large}\nc = {small}\n(a + b) + c = {small}\na + (b + c) = 0"
    )
}

fn special(rng: &mut StdRng) -> String {
    match rng.random_range(0..3) {
        0 => "binary32\n+0 = 0x00000000\n-0 = 0x80000000\n+0 == -0 → true\n1 / +0 → +∞\n1 / -0 → -∞".into(),
        1 => "binary32, roundTiesToEven\n0x7f7fffff × 2 → +∞ (0x7f800000)\n0xff7fffff × 2 → -∞ (0xff800000)".into(),
        _ => "binary32\n+∞ = 0x7f800000\n-∞ = 0xff800000\n1 / +∞ → +0\n1 / -∞ → -0\n+∞ + -∞ → NaN".into(),
    }
}

fn nan(rng: &mut StdRng) -> String {
    let payload = rng.random_range(1..0x0040_0000_u32);
    let quiet = rng.random_bool(0.5);
    let sign = u32::from(rng.random_bool(0.5));
    let fraction = payload | (u32::from(quiet) << 22);
    let bits = (sign << 31) | 0x7f80_0000 | fraction;
    let kind = if quiet { "quiet" } else { "signaling" };
    format!(
        "binary32 0x{bits:08x} ({kind} NaN)\nsign      {sign}\nexponent  11111111\nfraction  {fraction:023b}\npayload   0x{payload:06x}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn finite_decoding_matches_hardware_exactly() {
        let mut rng = StdRng::seed_from_u64(754);
        for _ in 0..10_000 {
            let bits = rng.random::<u32>();
            if (bits >> 23) & 255 == 255 {
                continue;
            }
            let (coefficient, exponent) = components(bits);
            let decoded = coefficient as f64 * 2.0_f64.powi(exponent);
            assert_eq!(decoded, f64::from(f32::from_bits(bits)));
        }
        assert_eq!(components(0x0080_0000), (1 << 23, -149));
        assert_eq!(components(0x007f_ffff), ((1 << 23) - 1, -149));
        assert_eq!(exact(0x8000_0000), "-0");
    }

    #[test]
    fn binade_gaps_double_except_at_gradual_underflow() {
        for exponent in 1..255_u32 {
            let center = exponent << 23;
            let value = f64::from(f32::from_bits(center));
            let below = value - f64::from(f32::from_bits(center - 1));
            let above = f64::from(f32::from_bits(center + 1)) - value;
            assert_eq!(above, 2.0_f64.powi(exponent as i32 - 150));
            assert_eq!(above / below, if exponent == 1 { 1.0 } else { 2.0 });
        }
    }

    #[test]
    fn exact_midpoints_round_to_even_including_zero_and_subnormals() {
        for exponent in [0, 1, 27, 127, 227, 254_u32] {
            for fraction in [0, 1, 2, 127, FRACTION - 1] {
                let lower = (exponent << 23) | fraction;
                let midpoint =
                    (f64::from(f32::from_bits(lower)) + f64::from(f32::from_bits(lower + 1))) / 2.0;
                assert_eq!((midpoint as f32).to_bits(), lower + (lower & 1));
                assert_eq!((-midpoint as f32).to_bits(), SIGN | (lower + (lower & 1)));
            }
        }
    }

    #[test]
    fn association_and_underflow_constructions_hold() {
        for power in -30..80 {
            let a = 2.0_f32.powi(power);
            let b = -a;
            let c = 2.0_f32.powi(power - 25);
            assert_eq!((a + b) + c, c);
            assert_eq!(a + (b + c), 0.0);
        }
        for sign in [0, SIGN] {
            let smallest = f32::from_bits(sign | 1);
            let subnormal = f32::from_bits(sign | FRACTION);
            let normal = f32::from_bits(sign | 0x0080_0000);
            assert_eq!((subnormal + smallest).to_bits(), normal.to_bits());
            assert_eq!((normal - subnormal).to_bits(), smallest.to_bits());
        }
    }

    #[test]
    fn both_nan_classes_have_nonzero_fractions_and_disjoint_payloads() {
        for sign in [0, SIGN] {
            for payload in [1, 2, 0x003f_ffff] {
                for quiet in [0, 0x0040_0000] {
                    let bits = sign | 0x7f80_0000 | quiet | payload;
                    assert!(f32::from_bits(bits).is_nan());
                    assert_eq!(bits & 0x003f_ffff, payload);
                    assert_eq!(bits & 0x0040_0000, quiet);
                }
            }
        }
    }
}
