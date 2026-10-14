use rand::{Rng, rngs::StdRng};
fn percentile(tens: u16, units: u16) -> u16 {
    if tens + units == 0 { 100 } else { tens + units }
}
pub fn generate(rng: &mut StdRng) -> String {
    let tens: u16 = rng.random_range(0..10) * 10;
    let units: u16 = rng.random_range(0..10);
    let total = percentile(tens, units);
    format!("Percentile dice (00 + 0 means 100): [{tens:02}, {units}] => {total}")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn faces_are_bijective() {
        let mut outcomes = Vec::new();
        for t in (0..100).step_by(10) {
            for u in 0..10 {
                outcomes.push(percentile(t, u));
            }
        }
        outcomes.sort_unstable();
        assert_eq!(outcomes, (1..=100).collect::<Vec<_>>());
    }
}
