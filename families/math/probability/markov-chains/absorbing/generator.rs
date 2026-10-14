use rand::{Rng, rngs::StdRng, seq::SliceRandom};

pub fn generate(rng: &mut StdRng) -> String {
    let (d, a, b) = sample(rng);
    let (ua, ub, ta, tb, det) = answers(d, a, b);
    let row = |values: [i64; 4]| values.map(|n| fraction(n, d)).join(", ");
    format!(
        "Absorbing Markov chain, states in order [A, B, U, V]; P[i,j] is the probability of the next state j given current state i.\nTransition matrix: [[{}], [{}], [0,0,1,0], [0,0,0,1]].\nU and V are absorbing; A and B are transient. Absorption occurs with probability 1.\nStarting in A: P(final state U)={}; P(final state V)={}; expected steps to absorption={}.\nStarting in B: P(final state U)={}; P(final state V)={}; expected steps to absorption={}.",
        row(a),
        row(b),
        fraction(ua, det),
        fraction(det - ua, det),
        fraction(ta, det),
        fraction(ub, det),
        fraction(det - ub, det),
        fraction(tb, det),
    )
}

fn sample(rng: &mut StdRng) -> (i64, [i64; 4], [i64; 4]) {
    let d = rng.random_range(6..=20);
    let mut composition = || {
        let mut cuts: Vec<i64> = (1..d).collect();
        cuts.shuffle(rng);
        cuts.truncate(3);
        cuts.sort_unstable();
        [cuts[0], cuts[1] - cuts[0], cuts[2] - cuts[1], d - cuts[2]]
    };
    (d, composition(), composition())
}

fn answers(d: i64, a: [i64; 4], b: [i64; 4]) -> (i64, i64, i64, i64, i64) {
    let det = (d - a[0]) * (d - b[1]) - a[1] * b[0];
    (
        (d - b[1]) * a[2] + a[1] * b[2],
        b[0] * a[2] + (d - a[0]) * b[2],
        d * (d - b[1] + a[1]),
        d * (b[0] + d - a[0]),
        det,
    )
}

fn fraction(n: i64, d: i64) -> String {
    let (mut a, mut b) = (n, d);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    if d / a == 1 {
        (n / a).to_string()
    } else {
        format!("{}/{}", n / a, d / a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn exact_answers_obey_first_step_equations_and_probability_bounds() {
        let mut rng = StdRng::seed_from_u64(296);
        for _ in 0..2_000 {
            let (d, a, b) = sample(&mut rng);
            let (ua, ub, ta, tb, det) = answers(d, a, b);
            assert_eq!(a.iter().sum::<i64>(), d);
            assert_eq!(b.iter().sum::<i64>(), d);
            assert!(a.iter().chain(&b).all(|&n| n > 0));
            assert!(det > 0);
            assert!(ua > 0 && ua < det && ub > 0 && ub < det);
            assert!(ta >= det && tb >= det);
            assert_eq!(d * ua, a[0] * ua + a[1] * ub + a[2] * det);
            assert_eq!(d * ub, b[0] * ua + b[1] * ub + b[2] * det);
            assert_eq!(d * ta, d * det + a[0] * ta + a[1] * tb);
            assert_eq!(d * tb, d * det + b[0] * ta + b[1] * tb);
        }
    }

    #[test]
    fn independent_probability_flow_matches_absorption_answer() {
        let (d, a, b) = (10, [3, 2, 4, 1], [1, 4, 2, 3]);
        let (ua, _, ta, _, det) = answers(d, a, b);
        let (mut pa, mut pb, mut absorbed_u, mut steps) = (1.0, 0.0, 0.0, 0.0);
        for _ in 0..200 {
            steps += pa + pb;
            absorbed_u += (pa * a[2] as f64 + pb * b[2] as f64) / d as f64;
            (pa, pb) = (
                (pa * a[0] as f64 + pb * b[0] as f64) / d as f64,
                (pa * a[1] as f64 + pb * b[1] as f64) / d as f64,
            );
        }
        assert!((absorbed_u - ua as f64 / det as f64).abs() < 1e-12);
        assert!((steps - ta as f64 / det as f64).abs() < 1e-12);
    }
}
