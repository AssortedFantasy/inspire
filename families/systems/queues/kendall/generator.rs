use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

include!("../../number.rs");

#[derive(Clone, Copy, Debug)]
struct Queue {
    arrival: f64,
    service: f64,
    servers: usize,
    capacity: Option<usize>,
}

#[derive(Debug)]
struct Stationary {
    p0: f64,
    delay: f64,
    blocked: f64,
    throughput: f64,
    length: f64,
    queued: f64,
    probabilities: Vec<f64>,
}

fn stationary(q: Queue) -> Option<Stationary> {
    let c = q.servers;
    let load = q.arrival / q.service;
    let rho = load / c as f64;
    if let Some(k) = q.capacity {
        let mut p = vec![1.0];
        for count in 1..=k {
            p.push(p[count - 1] * load / count.min(c) as f64);
        }
        let sum: f64 = p.iter().sum();
        for v in &mut p {
            *v /= sum;
        }
        let blocked = p[k];
        let throughput = q.arrival * (1.0 - blocked);
        let length = p.iter().enumerate().map(|(i, p)| i as f64 * p).sum();
        let queued = p
            .iter()
            .enumerate()
            .map(|(i, p)| i.saturating_sub(c) as f64 * p)
            .sum();
        // Arrival sees the stationary occupancy (PASTA); condition on admission.
        let delay = p[c..k].iter().sum::<f64>() / (1.0 - blocked);
        Some(Stationary {
            p0: p[0],
            delay,
            blocked,
            throughput,
            length,
            queued,
            probabilities: p,
        })
    } else {
        if rho >= 1.0 {
            return None;
        }
        let mut weights = vec![1.0];
        for count in 1..=c {
            weights.push(weights[count - 1] * load / count as f64);
        }
        let normalizer = weights[..c].iter().sum::<f64>() + weights[c] / (1.0 - rho);
        let p0 = 1.0 / normalizer;
        let delay = weights[c] * p0 / (1.0 - rho);
        let queued = delay * rho / (1.0 - rho);
        Some(Stationary {
            p0,
            delay,
            blocked: 0.0,
            throughput: q.arrival,
            length: queued + load,
            queued,
            probabilities: Vec::new(),
        })
    }
}

fn make(rng: &mut StdRng, mode: usize) -> (Queue, bool) {
    let service = rng.random_range(6..=30) as f64;
    let servers = if matches!(mode, 3 | 5) {
        rng.random_range(2..=5)
    } else {
        1
    };
    let ratios: &[f64] = match mode {
        0 => &[0.2, 0.35, 0.5, 0.65],
        1 | 5 => &[0.9, 0.95, 0.98, 0.99],
        2 => &[1.0, 1.05, 1.2],
        3 => &[0.5, 0.75, 0.9, 0.97],
        _ => &[0.5, 0.95, 1.0, 1.05, 1.5, 2.0],
    };
    let ratio = ratios[rng.random_range(0..ratios.len())];
    (
        Queue {
            arrival: service * servers as f64 * ratio,
            service,
            servers,
            capacity: (mode == 4).then(|| rng.random_range(2..=12)),
        },
        mode == 3,
    )
}

pub fn generate(rng: &mut StdRng) -> String {
    let mode = rng.random_range(0..6);
    let (q, pooling) = make(rng, mode);
    let name = q
        .capacity
        .map_or_else(|| format!("M/M/{}", q.servers), |k| format!("M/M/1/{k}"));
    let rho = q.arrival / (q.servers as f64 * q.service);
    let mut out = format!(
        "{name}, FCFS\nλ = {}/h, μ = {}/h per server\nρ = {}",
        n(q.arrival),
        n(q.service),
        n(rho)
    );
    let Some(s) = stationary(q) else {
        write!(
            out,
            "\nNo stationary distribution (λ {} cμ)",
            if rho == 1.0 { "=" } else { ">" }
        )
        .unwrap();
        return out;
    };
    write!(
        out,
        "\nMean queue wait = {} min\nMean time in system = {} min\nLq = {}, L = {}",
        n(60.0 * s.queued / s.throughput),
        n(60.0 * s.length / s.throughput),
        n(s.queued),
        n(s.length)
    )
    .unwrap();
    if let Some(k) = q.capacity {
        write!(out, "\nBlocked arrivals = {}%\nAdmitted throughput = {}/h\nP(wait | admitted) = {}\np[0..{k}] = [{}]", n(100.0 * s.blocked), n(s.throughput), n(s.delay), s.probabilities.iter().map(|v| n(*v)).collect::<Vec<_>>().join(", ")).unwrap();
    } else if q.servers == 1 {
        write!(out, "\nP(N = n) = {} × {}^n, n ≥ 0", n(s.p0), n(rho)).unwrap();
    } else {
        let c = q.servers;
        let a = q.arrival / q.service;
        write!(out, "\nP(wait) = {}\np0 = {}\np_n = p0 × {}^n / n! for n < {c}\np_n = p0 × {}^{c} / {c}! × {}^(n − {c}) for n ≥ {c}", n(s.delay), n(s.p0), n(a), n(a), n(rho)).unwrap();
    }
    if pooling {
        let split = stationary(Queue {
            arrival: q.arrival / q.servers as f64,
            servers: 1,
            ..q
        })
        .unwrap();
        write!(out, "\n\n{} separate queues, independent uniform arrival splitting\nMean queue wait = {} min (pooled {} min)", q.servers, n(60.0 * split.queued / split.throughput), n(60.0 * s.queued / s.throughput)).unwrap();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn independent_birth_death_chain_agrees() {
        for seed in 0..240 {
            let mut rng = StdRng::seed_from_u64(seed);
            for mode in 0..6 {
                let (q, pooling) = make(&mut rng, mode);
                let Some(s) = stationary(q) else {
                    assert!(q.capacity.is_none() && q.arrival >= q.servers as f64 * q.service);
                    continue;
                };
                let limit = q.capacity.unwrap_or(12_000);
                let mut p = vec![1.0];
                for i in 1..=limit {
                    p.push(p[i - 1] * q.arrival / (q.service * i.min(q.servers) as f64));
                }
                let sum: f64 = p.iter().sum();
                for v in &mut p {
                    *v /= sum;
                }
                let length: f64 = p.iter().enumerate().map(|(i, p)| i as f64 * p).sum();
                let queued: f64 = p
                    .iter()
                    .enumerate()
                    .map(|(i, p)| i.saturating_sub(q.servers) as f64 * p)
                    .sum();
                assert!((length - s.length).abs() < 1e-7);
                assert!((queued - s.queued).abs() < 1e-7);
                assert!((p[0] - s.p0).abs() < 1e-10);
                let admitted = q.capacity.map_or(1.0, |k| 1.0 - p[k]);
                let waiting_states = q.capacity.unwrap_or(limit + 1);
                let delay = p[q.servers..waiting_states].iter().sum::<f64>() / admitted;
                assert!((delay - s.delay).abs() < 1e-9);
                // A customer seeing i >= c waits for i-c+1 completions while
                // all c servers remain busy. PASTA weights the arrival states.
                let wait = p[q.servers..waiting_states]
                    .iter()
                    .enumerate()
                    .map(|(i, p)| p * (i + 1) as f64 / (q.servers as f64 * q.service))
                    .sum::<f64>()
                    / admitted;
                assert!((wait - s.queued / s.throughput).abs() < 1e-7);
                let completions: f64 = p
                    .iter()
                    .enumerate()
                    .map(|(i, p)| i.min(q.servers) as f64 * q.service * p)
                    .sum();
                assert!((completions - s.throughput).abs() < 1e-8);
                for i in 1..=limit {
                    assert!(
                        (p[i - 1] * q.arrival - p[i] * q.service * i.min(q.servers) as f64).abs()
                            < 1e-10
                    );
                }
                if pooling {
                    let split = stationary(Queue {
                        arrival: q.arrival / q.servers as f64,
                        servers: 1,
                        ..q
                    })
                    .unwrap();
                    assert!(s.queued / s.throughput < split.queued / split.throughput);
                }
            }
        }
    }

    #[test]
    fn finite_critical_load_is_uniform() {
        let s = stationary(Queue {
            arrival: 8.0,
            service: 8.0,
            servers: 1,
            capacity: Some(7),
        })
        .unwrap();
        assert!(s.probabilities.iter().all(|p| (*p - 0.125).abs() < 1e-12));
        assert!((s.length - 3.5).abs() < 1e-12);
        assert!((s.throughput - 7.0).abs() < 1e-12);
    }
}
