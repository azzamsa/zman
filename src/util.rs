use chrono::prelude::*;

pub fn count_days_of_month(year: i32, month: u32) -> i64 {
    Local
        .ymd(
            match month {
                12 => year + 1,
                _ => year,
            },
            match month {
                12 => 1,
                _ => month + 1,
            },
            1,
        )
        .signed_duration_since(Local.ymd(year, month, 1))
        .num_days()
}

/// Wrap an info into JSON representation.
pub fn to_json(state: String, content: String) -> String {
    if state == "" {
        format!(r#"{{"text": "{}"}}"#, content)
    } else {
        format!(r#"{{"state":"{}", "text": "{}"}}"#, state, content)
    }
}
