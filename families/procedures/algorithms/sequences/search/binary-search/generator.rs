use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

fn lower_bound(data: &[i32], key: i32) -> (usize, Vec<(usize, usize, usize)>) {
    let (mut lo, mut hi) = (0, data.len());
    let mut trace = Vec::new();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        trace.push((lo, hi, mid));
        if data[mid] < key {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    (lo, trace)
}
pub fn generate(rng: &mut StdRng) -> String {
    let mut data: Vec<_> = (0..rng.random_range(7..=12))
        .map(|_| rng.random_range(-5..=20))
        .collect();
    data.sort();
    let key = if rng.random_bool(0.5) {
        data[rng.random_range(0..data.len())]
    } else {
        rng.random_range(-7..=22)
    };
    let (position, trace) = lower_bound(&data, key);
    let mut text = format!("Lower bound of {key} in {data:?}\nRange      Probe");
    for (lo, hi, mid) in trace {
        write!(text, "\n[{lo:>2}, {hi:>2})   [{mid}] = {}", data[mid]).unwrap();
    }
    let present = data.get(position) == Some(&key);
    let result = if present { "Found" } else { "Insert" };
    write!(text, "\n{result} at index {position}").unwrap();
    text
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn agrees_with_linear_scan_and_interval_invariant() {
        let mut rng = StdRng::seed_from_u64(284);
        for _ in 0..1000 {
            let mut data: Vec<_> = (0..rng.random_range(0..=20))
                .map(|_| rng.random_range(-4..=4))
                .collect();
            data.sort();
            for key in -5..=5 {
                let (position, trace) = lower_bound(&data, key);
                assert_eq!(
                    position,
                    data.iter().position(|&x| x >= key).unwrap_or(data.len())
                );
                for (lo, hi, mid) in trace {
                    assert!(lo <= mid && mid < hi && hi <= data.len());
                    assert!(data[..lo].iter().all(|&x| x < key));
                    assert!(data[hi..].iter().all(|&x| x >= key));
                }
            }
        }
    }
}
