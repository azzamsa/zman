use chrono::prelude::*;
use chrono::Duration;

// Count the amount of days in specified month
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

// Calculate the ratio of time progress
fn get(current: Date<Local>, start: Date<Local>, end: Date<Local>) -> f64 {
    let whole_diff = end - start;
    let whole_diff_in_seconds = whole_diff.num_days() * 86400 + whole_diff.num_seconds();

    let current_diff = current - start;
    let current_diff_in_seconds = current_diff.num_days() * 86400 + current_diff.num_seconds();
    // progress_ratio
    current_diff_in_seconds as f64 / whole_diff_in_seconds as f64
}

pub fn year_ratio() -> f64 {
    let current = Local::today();
    let start = Local.ymd(current.year(), 1, 1);
    let end = Local.ymd(current.year() + 1, 1, 1);
    get(current, start, end)
}

pub fn month_ratio() -> f64 {
    let current = Local::today();
    let start = Local.ymd(current.year(), current.month(), 1);
    let end = {
        if current.month() == 12 {
            let days_num = count_days_of_month(current.year(), current.month());
            Local.ymd(current.year(), current.month(), days_num as u32)
        } else {
            Local.ymd(current.year(), current.month() + 1, 1)
        }
    };
    get(current, start, end)
}

pub fn week_ratio() -> f64 {
    let current = Local::today();
    let start = current - Duration::days(current.weekday().num_days_from_monday().into());
    let end = start + Duration::days(7);
    get(current, start, end)
}
