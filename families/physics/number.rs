fn n(value: f64) -> String {
    if value.abs() < 1e-9 {
        return "0".into();
    }
    let decimals = (3 - value.abs().log10().floor() as i32).clamp(0, 8) as usize;
    let text = format!("{value:.decimals$}");
    if text.contains('.') {
        text.trim_end_matches('0').trim_end_matches('.').to_owned()
    } else {
        text
    }
}
