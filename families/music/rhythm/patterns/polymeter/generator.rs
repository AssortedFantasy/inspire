use rand::{Rng, rngs::StdRng};

const PERIODS: [(usize, usize); 4] = [(6, 8), (6, 10), (8, 10), (8, 12)];

fn cells(period: usize) -> &'static [&'static str] {
    match period {
        6 => &["x.x...", "x..xx.", "xx..x.", "x...x."],
        8 => &["x...x.x.", "x..x..x.", "x...xx..", "xx..x..."],
        10 => &["x...x.x...", "x..x..x...", "x.x...x...", "x...x...x."],
        12 => &[
            "x...x.x..x..",
            "x..x..x...x.",
            "x.x...x..x..",
            "x...x...xx..",
        ],
        _ => unreachable!(),
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn part(rng: &mut StdRng, period: usize, length: usize) -> Vec<bool> {
    let choices = cells(period);
    let cell = choices[rng.random_range(0..choices.len())].as_bytes();
    let rotation = rng.random_range(0..period);
    (0..length)
        .map(|step| cell[(step + rotation) % period] == b'x')
        .collect()
}

fn grid(part: &[bool]) -> String {
    part.chunks(4)
        .map(|beat| {
            beat.iter()
                .map(|&hit| if hit { 'x' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn generate(rng: &mut StdRng) -> String {
    let (a, b) = PERIODS[rng.random_range(0..PERIODS.len())];
    let length = a / gcd(a, b) * b;
    let first = part(rng, a, length);
    let second = part(rng, b, length);
    let tempo = rng.random_range(72..=132);
    format!(
        "1/4 = {tempo} · 1/8 grid\nBell {}/4 |: {} :|\nWood {}/4 |: {} :|",
        a / 2,
        grid(&first),
        b / 2,
        grid(&second),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn voices_meet_after_their_least_common_cycle() {
        let mut rng = StdRng::seed_from_u64(31);
        for (a, b) in PERIODS {
            let length = a / gcd(a, b) * b;
            for period in [a, b] {
                for cell in cells(period) {
                    assert_eq!(cell.len(), period);
                    for shorter in 1..period {
                        assert!(
                            (0..period)
                                .any(|i| cell.as_bytes()[i]
                                    != cell.as_bytes()[(i + shorter) % period])
                        );
                    }
                }
                for _ in 0..100 {
                    let voice = part(&mut rng, period, length);
                    assert_eq!(voice.len(), length);
                    assert!(
                        voice
                            .iter()
                            .enumerate()
                            .all(|(i, &hit)| hit == voice[(i + period) % length])
                    );
                }
            }
            assert!((1..length).all(|i| !i.is_multiple_of(a) || !i.is_multiple_of(b)));
        }
    }
}
