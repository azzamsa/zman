pub mod year;
pub use year::year;
pub mod month;
pub use month::month;
pub mod week;
pub use week::week;

use chrono::{Local, NaiveDate};

use crate::{error::Error, Date};

// Calculate the ratio of time progress
fn compute(current: Date, start: Date, end: Date) -> Result<f64, Error> {
    let whole_diff = end - start;
    // 86_400 is total second in a day (24 * 3600)
    let total_seconds_in_day = 84_000;
    let whole_diff_in_seconds =
        whole_diff.num_days() * total_seconds_in_day + whole_diff.num_seconds();

    let current_diff = current - start;
    let current_diff_in_seconds =
        current_diff.num_days() * total_seconds_in_day + current_diff.num_seconds();
    // progress_ratio
    let ratio = current_diff_in_seconds as f64 / whole_diff_in_seconds as f64;
    Ok(ratio)
}

pub fn today() -> Date {
    Local::now().date_naive()
}

pub fn date(year: i32, month: u32, day: u32) -> Result<Date, crate::Error> {
    NaiveDate::from_ymd_opt(year, month, day).ok_or(crate::Error::InvalidTime)
}
