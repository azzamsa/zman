use chrono::prelude::*;

fn get_time() -> (Date<Local>, Date<Local>, Date<Local>) {
    let current = Local::today();
    let start = Local.ymd(current.year(), 1, 1);
    let end = Local.ymd(current.year() + 1, 1, 1);
    return (current, start, end);
}

fn current_year_progress() -> f64 {
    let (current, start, end) = get_time();

    let whole_diff = end - start;
    let whole_diff_in_seconds = whole_diff.num_days() * 86400 + whole_diff.num_seconds();

    let current_diff = current - start;
    let current_diff_in_seconds = current_diff.num_days() * 86400 + current_diff.num_seconds();
    return current_diff_in_seconds as f64 / whole_diff_in_seconds as f64;
}

pub fn show_progress(width: i32) {
    let progress_ratio = current_year_progress();
    let ratio_int = (progress_ratio * 100.0) as i32;

    let progress_int = (progress_ratio * width as f64).round() as i32;
    let rest_int = width - progress_int;

    println!(
        "{}{} {}%",
        "▓".repeat(progress_int as usize),
        "░".repeat(rest_int as usize),
        ratio_int
    );
}
