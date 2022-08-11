use std::fmt::Display;

use crate::RelativeDuration;

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

impl Display for RelativeDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let build = vec![
            pluralize("month", self.num_months()),
            pluralize("week", self.num_weeks()),
            pluralize("day", self.num_days()),
        ];

        let mut result = String::new();
        let mut iter = build.iter().flatten();

        if let Some(arg) = iter.next() {
            result.push_str(&arg);

            for arg in iter {
                result.push(' ');
                result.push_str(&arg);
            }
        }

        Ok(f.write_str(&result)?)
    }
}
