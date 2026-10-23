use rand::{Rng, rngs::StdRng};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Q(i64, i64);

impl Q {
    fn new(mut n: i64, mut d: i64) -> Self {
        assert_ne!(d, 0);
        if d < 0 {
            n = -n;
            d = -d;
        }
        let (mut a, mut b) = (n.abs(), d);
        while b != 0 {
            (a, b) = (b, a % b);
        }
        Self(n / a, d / a)
    }
    fn add(self, b: Self) -> Self {
        Self::new(self.0 * b.1 + b.0 * self.1, self.1 * b.1)
    }
    fn sub(self, b: Self) -> Self {
        Self::new(self.0 * b.1 - b.0 * self.1, self.1 * b.1)
    }
    fn mul(self, b: Self) -> Self {
        Self::new(self.0 * b.0, self.1 * b.1)
    }
    fn div(self, b: Self) -> Self {
        Self::new(self.0 * b.1, self.1 * b.0)
    }
}

impl fmt::Display for Q {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.1 == 1 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "{}/{}", self.0, self.1)
        }
    }
}

fn q(n: i64) -> Q {
    Q(n, 1)
}

fn sum(a: &[Q]) -> Q {
    a.iter().copied().fold(q(0), Q::add)
}

fn dot(a: &[Q], b: &[Q]) -> Q {
    a.iter().zip(b).map(|(&x, &y)| x.mul(y)).fold(q(0), Q::add)
}

fn reflect(values: &[Q], normal: &[Q]) -> Vec<Q> {
    let multiple = q(2).mul(dot(values, normal)).div(dot(normal, normal));
    values
        .iter()
        .zip(normal)
        .map(|(&value, &w)| value.sub(multiple.mul(w)))
        .collect()
}

#[derive(Clone)]
struct Dataset {
    x: Vec<Q>,
    y: Vec<Q>,
}

struct Summary {
    mx: Q,
    my: Q,
    xx: Q,
    xy: Q,
    yy: Q,
    slope: Q,
    intercept: Q,
    sse: Q,
    r2: Q,
    leverage: Q,
}

fn summary(data: &Dataset) -> Summary {
    let n = q(data.x.len() as i64);
    let mx = sum(&data.x).div(n);
    let my = sum(&data.y).div(n);
    let x = data.x.iter().map(|&v| v.sub(mx)).collect::<Vec<_>>();
    let y = data.y.iter().map(|&v| v.sub(my)).collect::<Vec<_>>();
    let xx = dot(&x, &x);
    let xy = dot(&x, &y);
    let yy = dot(&y, &y);
    let slope = xy.div(xx);
    let intercept = my.sub(slope.mul(mx));
    let sse = yy.sub(xy.mul(xy).div(xx));
    let r2 = xy.mul(xy).div(xx.mul(yy));
    let leverage = x
        .iter()
        .map(|&v| q(1).div(n).add(v.mul(v).div(xx)))
        .max_by_key(|q| QOrder(*q))
        .unwrap();
    Summary {
        mx,
        my,
        xx,
        xy,
        yy,
        slope,
        intercept,
        sse,
        r2,
        leverage,
    }
}

#[derive(PartialEq, Eq)]
struct QOrder(Q);

impl PartialOrd for QOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0.0 * other.0.1).cmp(&(other.0.0 * self.0.1))
    }
}

fn from_residual(x: Vec<Q>, residual: Vec<Q>, intercept: i64, slope: i64) -> Dataset {
    let y = x
        .iter()
        .zip(residual)
        .map(|(&x, e)| q(intercept).add(q(slope).mul(x)).add(e))
        .collect();
    Dataset { x, y }
}

fn sample(rng: &mut StdRng, regime: usize) -> (Dataset, Dataset) {
    let intercept = rng.random_range(-5..=8);
    let slope = [-3, -2, -1, 1, 2, 3][rng.random_range(0..6)];
    let scale = rng.random_range(1..=3);
    let (x, residual, other_x, other_residual) = match regime {
        0 => {
            let m = rng.random_range(2..=4);
            let x = (-m..=m).map(q).collect::<Vec<_>>();
            let mean_square = dot(&x, &x).div(q(x.len() as i64));
            let e = x
                .iter()
                .map(|&t| t.mul(t).sub(mean_square))
                .collect::<Vec<_>>();
            let mut normal = vec![q(0); x.len()];
            let i = rng.random_range(0..x.len() - 2);
            normal[i] = q(1);
            normal[i + 1] = q(-2);
            normal[i + 2] = q(1);
            let other_e = reflect(&e, &normal);
            (x.clone(), e, x, other_e)
        }
        1 => {
            let n = [6, 8, 10][rng.random_range(0..3)];
            let mut x = vec![q(-1); n];
            x[n - 1] = q(n as i64 - 1);
            let mut e = (0..n - 1)
                .map(|i| q(i as i64 - (n as i64 - 2) / 2))
                .collect::<Vec<_>>();
            e.push(q(0));
            let normal = (0..n)
                .map(|i| q(if i < n / 2 { 1 } else { -1 }))
                .collect::<Vec<_>>();
            let other_x = reflect(&x, &normal);
            let other_e = reflect(&e, &normal);
            (x, e, other_x, other_e)
        }
        _ => {
            let m = rng.random_range(2..=4);
            let x = (-m..=m).map(q).collect::<Vec<_>>();
            let mut e = vec![q(0); x.len()];
            // A local second difference is perpendicular to constants and x.
            let i = rng.random_range(0..x.len() - 2);
            e[i] = q(1);
            e[i + 1] = q(-2);
            e[i + 2] = q(1);
            let normal = x
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    q(if i == 0 {
                        3
                    } else if i == 1 {
                        -1
                    } else if i == 2 {
                        -2
                    } else {
                        0
                    })
                })
                .collect::<Vec<_>>();
            let other_x = reflect(&x, &normal);
            let other_e = reflect(&e, &normal);
            (x, e, other_x, other_e)
        }
    };
    let residual = residual.into_iter().map(|v| v.mul(q(scale))).collect();
    let other_residual = other_residual
        .into_iter()
        .map(|v| v.mul(q(scale)))
        .collect();
    (
        from_residual(x, residual, intercept, slope),
        from_residual(other_x, other_residual, intercept, slope),
    )
}

pub fn generate(rng: &mut StdRng) -> String {
    let regime = rng.random_range(0..3);
    let (a, b) = sample(rng, regime);
    let sa = summary(&a);
    let sb = summary(&b);
    let mut output = "       A                 B\n     x       y         x       y".to_string();
    for i in 0..a.x.len() {
        output.push_str(&format!(
            "\n{:>6} {:>7}    {:>6} {:>7}",
            a.x[i].to_string(),
            a.y[i].to_string(),
            b.x[i].to_string(),
            b.y[i].to_string()
        ));
    }
    output.push_str(&format!(
        "\n\nBoth datasets\nx̄={}   ȳ={}\nOLS: ŷ = {} {} {}x\nR²={}   SSE={}",
        sa.mx,
        sa.my,
        sa.intercept,
        if sa.slope.0 < 0 { "-" } else { "+" },
        Q::new(sa.slope.0.abs(), sa.slope.1),
        sa.r2,
        sa.sse
    ));
    match rng.random_range(0..2) {
        0 => {
            let divisor = q(a.x.len() as i64 - 1);
            output.push_str(&format!(
                "\ns²(x)={}   s²(y)={}\ns(x,y)={}",
                sa.xx.div(divisor),
                sa.yy.div(divisor),
                sa.xy.div(divisor)
            ));
        }
        _ => output.push_str(&format!(
            "\nMaximum leverage\nA: {}\nB: {}",
            sa.leverage, sb.leverage
        )),
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    fn pairwise_centered(a: &[Q], b: &[Q]) -> Q {
        let mut value = q(0);
        for i in 0..a.len() {
            for j in i + 1..a.len() {
                value = value.add(a[i].sub(a[j]).mul(b[i].sub(b[j])));
            }
        }
        value.div(q(a.len() as i64))
    }

    #[test]
    fn pairs_have_identical_five_sums_and_independently_checked_fits() {
        let mut rng = StdRng::seed_from_u64(91);
        for regime in 0..3 {
            for _ in 0..150 {
                let (a, b) = sample(&mut rng, regime);
                assert_eq!(sum(&a.x), sum(&b.x));
                assert_eq!(sum(&a.y), sum(&b.y));
                assert_eq!(dot(&a.x, &a.x), dot(&b.x, &b.x));
                assert_eq!(dot(&a.x, &a.y), dot(&b.x, &b.y));
                assert_eq!(dot(&a.y, &a.y), dot(&b.y, &b.y));
                assert!(a.x != b.x || a.y != b.y);
                let points = |data: &Dataset| {
                    let mut points = data
                        .x
                        .iter()
                        .zip(&data.y)
                        .map(|(&x, &y)| (QOrder(x), QOrder(y)))
                        .collect::<Vec<_>>();
                    points.sort();
                    points
                };
                assert!(points(&a) != points(&b));
                for data in [a, b] {
                    let s = summary(&data);
                    assert_eq!(s.xx, pairwise_centered(&data.x, &data.x));
                    assert_eq!(s.xy, pairwise_centered(&data.x, &data.y));
                    assert_eq!(s.yy, pairwise_centered(&data.y, &data.y));
                    let residual = data
                        .x
                        .iter()
                        .zip(&data.y)
                        .map(|(&x, &y)| y.sub(s.intercept).sub(s.slope.mul(x)))
                        .collect::<Vec<_>>();
                    assert_eq!(sum(&residual), q(0));
                    assert_eq!(dot(&residual, &data.x), q(0));
                    assert_eq!(dot(&residual, &residual), s.sse);
                    assert!(QOrder(s.r2) < QOrder(q(1)));
                    let leverage_sum = data
                        .x
                        .iter()
                        .map(|&x| {
                            q(1).div(q(data.x.len() as i64))
                                .add(x.sub(s.mx).mul(x.sub(s.mx)).div(s.xx))
                        })
                        .fold(q(0), Q::add);
                    assert_eq!(leverage_sum, q(2));
                }
            }
        }
    }

    #[test]
    fn clustered_design_loses_unit_leverage_under_reflection() {
        let mut rng = StdRng::seed_from_u64(718);
        for _ in 0..30 {
            let (a, b) = sample(&mut rng, 1);
            assert_eq!(summary(&a).leverage, q(1));
            assert!(QOrder(summary(&b).leverage) < QOrder(q(1)));
            let mut distinct = b.x;
            distinct.sort_by_key(|q| QOrder(*q));
            distinct.dedup();
            assert_eq!(distinct.len(), 3);
        }
    }
}
