pub fn pluralize(unit: &str, num: i32) -> Option<String> {
    if num == 0 {
        None
    } else if num == 1 {
        Some(format!("1 {}", unit))
    } else if num == -1 {
        Some(format!("-1 {}", unit))
    } else {
        Some(format!("{} {}s", num, unit))
    }
}
