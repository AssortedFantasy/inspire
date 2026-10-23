use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn bezout(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut r, mut s, mut t) = ((a, b), (1, 0), (0, 1));
    while r.1 != 0 {
        let q = r.0 / r.1;
        r = (r.1, r.0 - q * r.1);
        s = (s.1, s.0 - q * s.1);
        t = (t.1, t.0 - q * t.1);
    }
    (r.0, s.0, t.0)
}

fn inverse(a: u64, modulus: u64) -> u64 {
    let (g, x, _) = bezout(a as i64, modulus as i64);
    assert_eq!(g, 1);
    x.rem_euclid(modulus as i64) as u64
}

fn power(mut a: u64, mut e: u64, modulus: u64) -> u64 {
    let mut result = 1;
    a %= modulus;
    while e != 0 {
        if e & 1 != 0 {
            result = result * a % modulus;
        }
        a = a * a % modulus;
        e >>= 1;
    }
    result
}

fn primes() -> Vec<u64> {
    (43..=499)
        .filter(|n| (2..).take_while(|d| d * d <= *n).all(|d| n % d != 0))
        .collect()
}

#[derive(Clone, Copy)]
struct Key {
    p: u64,
    q: u64,
    n: u64,
    lambda: u64,
    e: u64,
    d: u64,
}

impl Key {
    fn new(p: u64, q: u64, e: u64) -> Self {
        let lambda = (p - 1) / gcd(p - 1, q - 1) * (q - 1);
        Self {
            p,
            q,
            n: p * q,
            lambda,
            e,
            d: inverse(e, lambda),
        }
    }

    fn crt(self, c: u64) -> (u64, u64, u64, u64, u64) {
        let m_p = power(c, self.d % (self.p - 1), self.p);
        let m_q = power(c, self.d % (self.q - 1), self.q);
        let q_inverse = inverse(self.q, self.p);
        let h = ((m_p + self.p - m_q % self.p) % self.p) * q_inverse % self.p;
        (m_p, m_q, q_inverse, h, m_q + self.q * h)
    }

    fn display(self) -> String {
        let quotient = (self.e * self.d - 1) / self.lambda;
        format!(
            "p = {}, q = {}\nn = {} × {} = {}\nλ(n) = lcm({}, {}) = {}\ne = {}, d = {}\n{} × {} − {} × {} = 1",
            self.p,
            self.q,
            self.p,
            self.q,
            self.n,
            self.p - 1,
            self.q - 1,
            self.lambda,
            self.e,
            self.d,
            self.e,
            self.d,
            self.lambda,
            quotient
        )
    }
}

fn sample_key(rng: &mut StdRng, regime: usize) -> Key {
    let candidates: Vec<_> = primes()
        .into_iter()
        .filter(|p| regime != 3 || p % 3 == 2)
        .collect();
    let i = rng.random_range(0..candidates.len() - usize::from(regime == 2));
    let j = if regime == 2 {
        rng.random_range(i + 1..=(i + 4).min(candidates.len() - 1))
    } else {
        (i + rng.random_range(1..candidates.len())) % candidates.len()
    };
    let (p, q) = (
        candidates[i].min(candidates[j]),
        candidates[i].max(candidates[j]),
    );
    let lambda = (p - 1) / gcd(p - 1, q - 1) * (q - 1);
    let exponents: Vec<_> = [3, 5, 7, 11, 17, 257, 65537]
        .into_iter()
        .filter(|e| *e < p * q && gcd(*e, lambda) == 1)
        .collect();
    let e = if regime == 3 {
        3
    } else {
        exponents[rng.random_range(0..exponents.len())]
    };
    Key::new(p, q, e)
}

fn floor_sqrt(n: u64) -> u64 {
    let mut a = 0;
    while (a + 1) * (a + 1) <= n {
        a += 1;
    }
    a
}

fn fermat(n: u64) -> (u64, u64, u64) {
    let mut a = floor_sqrt(n);
    if a * a < n {
        a += 1;
    }
    let start = a;
    // The constructed factors are at most 499, so their midpoint bounds the search.
    for _ in 0..=499 {
        let b = floor_sqrt(a * a - n);
        if b * b == a * a - n {
            return (a, b, a - start + 1);
        }
        a += 1;
    }
    unreachable!("bounded factor pair")
}

fn cube_limit(n: u64) -> u64 {
    let mut m = 0_u64;
    while (m + 1).pow(3) < n {
        m += 1;
    }
    m
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..6);
    let key = sample_key(rng, regime);
    let mut out = key.display();
    match regime {
        0 | 1 => {
            let m = match rng.random_range(0..8) {
                0 => 0,
                1 => key.n - 1,
                _ => rng.random_range(2..key.n - 1),
            };
            let c = power(m, key.e, key.n);
            write!(out, "\nm = {m}\nc = {m}^{} mod {} = {c}", key.e, key.n).unwrap();
            if regime == 0 {
                write!(
                    out,
                    "\n{c}^{} mod {} = {}",
                    key.d,
                    key.n,
                    power(c, key.d, key.n)
                )
                .unwrap();
            } else {
                let (m_p, m_q, q_inverse, h, recovered) = key.crt(c);
                write!(
                    out,
                    "\ndP = {}, dQ = {}, qInv = {q_inverse}\nmP = {c}^{} mod {} = {m_p}\nmQ = {c}^{} mod {} = {m_q}\nh = ({m_p} − {m_q}) × {q_inverse} mod {} = {h}\nm = {m_q} + {} × {h} = {recovered}",
                    key.d % (key.p - 1), key.d % (key.q - 1),
                    key.d % (key.p - 1), key.p, key.d % (key.q - 1), key.q,
                    key.p, key.q
                ).unwrap();
            }
        }
        2 => {
            let m = rng.random_range(2..key.n - 1);
            let c = power(m, key.e, key.n);
            let (a, b, attempts) = fermat(key.n);
            write!(
                out,
                "\nm = {m}\nc = {m}^{} mod {} = {c}\n{c}^{} mod {} = {}\n\nFermat factorization ({attempts} tested midpoint{})\n{a}² − {} = {} = {b}²\n{} = ({a} − {b})({a} + {b}) = {} × {}",
                key.e, key.n, key.d, key.n, power(c, key.d, key.n),
                if attempts == 1 { "" } else { "s" }, key.n, b * b,
                key.n, a - b, a + b
            ).unwrap();
        }
        3 => {
            let m = rng.random_range(2..=cube_limit(key.n));
            let c = power(m, 3, key.n);
            write!(
                out,
                "\nm = {m}\nm³ = {c} < {}\nc = {m}³ mod {} = {c}\n∛{c} = {m}\n{c}^{} mod {} = {}",
                key.n,
                key.n,
                key.d,
                key.n,
                power(c, key.d, key.n)
            )
            .unwrap();
        }
        4 => {
            let a = rng.random_range(2..key.n - 1);
            let b = rng.random_range(2..key.n - 1);
            let c_a = power(a, key.e, key.n);
            let c_b = power(b, key.e, key.n);
            let product = a * b % key.n;
            let c_product = c_a * c_b % key.n;
            write!(
                out,
                "\nm₁ = {a}, c₁ = {a}^{} mod {} = {c_a}\nm₂ = {b}, c₂ = {b}^{} mod {} = {c_b}\nc₁c₂ mod {} = {c_product}\n{c_product}^{} mod {} = {} = m₁m₂ mod {}",
                key.e, key.n, key.e, key.n, key.n,
                key.d, key.n, power(c_product, key.d, key.n), key.n
            ).unwrap();
            debug_assert_eq!(power(c_product, key.d, key.n), product);
        }
        _ => {
            let m = key.p * rng.random_range(1..key.q);
            let c = power(m, key.e, key.n);
            write!(
                out,
                "\nm = {m}\ngcd({m}, {}) = {}\nc = {m}^{} mod {} = {c}\n{c}^{} mod {} = {}",
                key.n,
                gcd(m, key.n),
                key.e,
                key.n,
                key.d,
                key.n,
                power(c, key.d, key.n)
            )
            .unwrap();
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn exponentiation_and_bezout_match_independent_arithmetic() {
        for modulus in 2..80 {
            for a in 0..modulus {
                let mut repeated = 1;
                for exponent in 0..25 {
                    assert_eq!(power(a, exponent, modulus), repeated);
                    repeated = repeated * a % modulus;
                }
                let (g, x, y) = bezout(a as i64, modulus as i64);
                assert_eq!(a as i64 * x + modulus as i64 * y, g);
                assert_eq!(g as u64, gcd(a, modulus));
            }
        }
    }

    #[test]
    fn rsa_known_vector_and_crt() {
        let key = Key::new(61, 53, 17);
        assert_eq!((key.n, key.lambda, key.d), (3233, 780, 413));
        assert_eq!(power(65, key.e, key.n), 2790);
        assert_eq!(power(2790, key.d, key.n), 65);
        assert_eq!(key.crt(2790).4, 65);
    }

    #[test]
    fn every_residue_round_trips_including_nonunits() {
        let small = [3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
        for (i, &p) in small.iter().enumerate() {
            for &q in &small[i + 1..] {
                let lambda = (p - 1) / gcd(p - 1, q - 1) * (q - 1);
                let e = (3..p * q).find(|e| gcd(*e, lambda) == 1).unwrap();
                let key = Key::new(p, q, e);
                for m in 0..key.n {
                    let c = power(m, e, key.n);
                    assert_eq!(power(c, key.d, key.n), m, "{p}, {q}, {m}");
                    assert_eq!(key.crt(c).4, m, "{p}, {q}, {m}");
                }
            }
        }
    }

    #[test]
    fn constructed_regimes_have_the_claimed_structure() {
        for seed in 0..512 {
            let mut rng = StdRng::seed_from_u64(seed);
            for regime in 0..6 {
                let key = sample_key(&mut rng, regime);
                assert_ne!(key.p, key.q);
                assert!((2..key.p).all(|d| !key.p.is_multiple_of(d)));
                assert!((2..key.q).all(|d| !key.q.is_multiple_of(d)));
                assert_eq!(key.e * key.d % key.lambda, 1);
                assert!(key.e < key.n);
                if regime == 2 {
                    let (a, b, _) = fermat(key.n);
                    assert_eq!((a - b, a + b), (key.p, key.q));
                }
                if regime == 3 {
                    assert_eq!(key.e, 3);
                    let limit = cube_limit(key.n);
                    assert!(limit.pow(3) < key.n);
                    assert!((limit + 1).pow(3) >= key.n);
                    for m in 2..=limit {
                        assert_eq!(power(m, 3, key.n), m.pow(3));
                    }
                }
                let (a, b) = (key.p, key.q + 1);
                assert_eq!(
                    power(a, key.e, key.n) * power(b, key.e, key.n) % key.n,
                    power(a * b % key.n, key.e, key.n)
                );
            }
        }
    }
}
