use rand::{Rng, rngs::StdRng};
use std::fmt::Write;

#[derive(Clone, Copy, Debug)]
enum Scheme {
    Lomuto,
    ThreeWay,
}

#[derive(Debug)]
struct Partition {
    lo: usize,
    hi: usize,
    equal: (usize, usize),
    pivot: i32,
    #[cfg(test)]
    before: Vec<i32>,
    after: Vec<i32>,
}

fn sort(data: &mut [i32], scheme: Scheme) -> (Vec<Partition>, usize) {
    fn recurse(
        data: &mut [i32],
        lo: usize,
        hi: usize,
        scheme: Scheme,
        trace: &mut Vec<Partition>,
        comparisons: &mut usize,
    ) {
        if hi - lo < 2 {
            return;
        }
        #[cfg(test)]
        let before = data.to_vec();
        let pivot = data[hi - 1];
        let (left, right) = match scheme {
            Scheme::Lomuto => {
                let mut next = lo;
                for i in lo..hi - 1 {
                    *comparisons += 1;
                    if data[i] <= pivot {
                        data.swap(i, next);
                        next += 1;
                    }
                }
                data.swap(next, hi - 1);
                (next, next + 1)
            }
            Scheme::ThreeWay => {
                let (mut left, mut scan, mut right) = (lo, lo, hi);
                while scan < right {
                    *comparisons += 1;
                    match data[scan].cmp(&pivot) {
                        std::cmp::Ordering::Less => {
                            data.swap(left, scan);
                            left += 1;
                            scan += 1;
                        }
                        std::cmp::Ordering::Equal => scan += 1,
                        std::cmp::Ordering::Greater => {
                            right -= 1;
                            data.swap(scan, right);
                        }
                    }
                }
                (left, right)
            }
        };
        trace.push(Partition {
            lo,
            hi,
            equal: (left, right),
            pivot,
            #[cfg(test)]
            before,
            after: data.to_vec(),
        });
        recurse(data, lo, left, scheme, trace, comparisons);
        recurse(data, right, hi, scheme, trace, comparisons);
    }
    let mut trace = Vec::new();
    let mut comparisons = 0;
    recurse(data, 0, data.len(), scheme, &mut trace, &mut comparisons);
    (trace, comparisons)
}

fn input(rng: &mut StdRng, regime: usize) -> Vec<i32> {
    let n = rng.random_range(7..=11);
    let start = rng.random_range(-8..=4);
    match regime {
        0 => (0..n).map(|_| rng.random_range(-12..=18)).collect(),
        1 => (0..n).map(|i| start + i as i32).collect(),
        2 => (0..n).rev().map(|i| start + i as i32).collect(),
        3 => vec![start; n],
        4 => (0..n).map(|_| start + rng.random_range(0..=2)).collect(),
        _ => (0..n).map(|i| start + i.min(n - 1 - i) as i32).collect(),
    }
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..6);
    let mut data = input(rng, regime);
    let scheme = if rng.random_bool(0.5) {
        Scheme::Lomuto
    } else {
        Scheme::ThreeWay
    };
    let name = match scheme {
        Scheme::Lomuto => "Lomuto, <= partition",
        Scheme::ThreeWay => "three-way, < = > partition",
    };
    let mut text =
        format!("Quicksort {data:?}\n{name}, last-element pivot\nPartition ranges [lo, hi)");
    let (trace, comparisons) = sort(&mut data, scheme);
    for step in &trace {
        let (left, right) = step.equal;
        write!(
            text,
            "\n[{}, {}) pivot {} → {:?} | {:?} | {:?}",
            step.lo,
            step.hi,
            step.pivot,
            &step.after[step.lo..left],
            &step.after[left..right],
            &step.after[right..step.hi]
        )
        .unwrap();
    }
    write!(
        text,
        "\nSorted {data:?}\n{} partitions, {comparisons} key comparisons",
        trace.len()
    )
    .unwrap();
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn check(mut data: Vec<i32>, scheme: Scheme) {
        let mut expected = data.clone();
        expected.sort();
        let (trace, _) = sort(&mut data, scheme);
        assert_eq!(data, expected);
        for step in trace {
            let (left, right) = step.equal;
            assert!(step.lo <= left && left < right && right <= step.hi);
            let mut before = step.before[step.lo..step.hi].to_vec();
            let mut after = step.after[step.lo..step.hi].to_vec();
            before.sort();
            after.sort();
            assert_eq!(before, after);
            assert_eq!(step.before[..step.lo], step.after[..step.lo]);
            assert_eq!(step.before[step.hi..], step.after[step.hi..]);
            assert!(step.after[left..right].iter().all(|&x| x == step.pivot));
            assert!(step.after[right..step.hi].iter().all(|&x| x > step.pivot));
            assert!(step.after[step.lo..left].iter().all(|&x| match scheme {
                Scheme::Lomuto => x <= step.pivot,
                Scheme::ThreeWay => x < step.pivot,
            }));
        }
    }

    #[test]
    fn exhaustive_ternary_arrays_and_constructed_shapes() {
        for n in 0..=7_u32 {
            for mut code in 0..3_usize.pow(n) {
                let data: Vec<_> = (0..n)
                    .map(|_| {
                        let x = (code % 3) as i32 - 1;
                        code /= 3;
                        x
                    })
                    .collect();
                for scheme in [Scheme::Lomuto, Scheme::ThreeWay] {
                    check(data.clone(), scheme);
                }
            }
        }
        let mut rng = StdRng::seed_from_u64(260423);
        for regime in 0..6 {
            for _ in 0..100 {
                let data = input(&mut rng, regime);
                for scheme in [Scheme::Lomuto, Scheme::ThreeWay] {
                    check(data.clone(), scheme);
                }
            }
        }
    }

    #[test]
    fn equal_keys_expose_partition_cost_boundary() {
        let mut data = vec![4; 10];
        assert_eq!(sort(&mut data, Scheme::Lomuto).1, 45);
        let (trace, comparisons) = sort(&mut data, Scheme::ThreeWay);
        assert_eq!((trace.len(), comparisons), (1, 10));
    }
}
