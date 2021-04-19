use chrono::prelude::*;
use chrono::Duration;

use crate::util;

// Calculate the ratio of time progress
fn get_progress(current: Date<Local>, start: Date<Local>, end: Date<Local>) -> f64 {
    let whole_diff = end - start;
    let whole_diff_in_seconds = whole_diff.num_days() * 86400 + whole_diff.num_seconds();

    let current_diff = current - start;
    let current_diff_in_seconds = current_diff.num_days() * 86400 + current_diff.num_seconds();
    let progress_ratio = current_diff_in_seconds as f64 / whole_diff_in_seconds as f64;
    return progress_ratio;
}

pub fn year_progress_ratio() -> f64 {
    let current = Local::today();
    let start = Local.ymd(current.year(), 1, 1);
    let end = Local.ymd(current.year() + 1, 1, 1);
    return get_progress(current, start, end);
}

pub fn month_progress_ratio() -> f64 {
    let current = Local::today();
    let start = Local.ymd(current.year(), current.month(), 1);
    let end = {
        if current.month() == 12 {
            let days_num = util::count_days_of_month(current.year(), current.month());
            Local.ymd(current.year(), current.month(), days_num as u32)
        } else {
            Local.ymd(current.year(), current.month() + 1, 1)
        }
    };
    return get_progress(current, start, end);
}

pub fn week_progress_ratio() -> f64 {
    let current = Local::today();
    let start = current - Duration::days(current.weekday().num_days_from_monday().into());
    let end = start + Duration::days(7);
    return get_progress(current, start, end);
}
