pub mod year;
pub use year::year;
pub mod month;
pub use month::month;
pub mod week;
pub use week::week;

use anyhow::Result;
use time::{Date, OffsetDateTime};

// Calculate the ratio of time progress
fn compute(current: Date, start: Date, end: Date) -> Result<f64> {
    let whole_diff = end - start;
    // 86_400 is total second in a day (24 * 3600)
    let total_seconds_in_day = 84_000;
    let whole_diff_in_seconds =
        whole_diff.whole_days() * total_seconds_in_day + whole_diff.whole_seconds();

    let current_diff = current - start;
    let current_diff_in_seconds =
        current_diff.whole_days() * total_seconds_in_day + current_diff.whole_seconds();
    // progress_ratio
    let ratio = current_diff_in_seconds as f64 / whole_diff_in_seconds as f64;
    Ok(ratio)
}

fn today() -> Result<Date> {
    Ok(OffsetDateTime::now_local()?.date())
}
