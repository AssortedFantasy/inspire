fn n(value: f64) -> String {
    if value.abs() < 0.000_000_01 {
        return "0".into();
    }
    if value.abs() < 0.0001 || value.abs() >= 1_000_000.0 {
        return format!("{value:.5e}");
    }
    let places = (5 - value.abs().log10().floor() as i32).clamp(0, 9) as usize;
    if places == 0 {
        return format!("{value:.0}");
    }
    format!("{value:.places$}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_owned()
}
