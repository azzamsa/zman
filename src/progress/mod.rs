pub mod year;
pub use year::year;
pub mod month;
pub use month::month;
pub mod week;
pub use week::week;

use jiff::{Span, SpanTotal, Unit, civil};

use crate::error::Error;

// Calculate the ratio of time progress
fn compute(current: civil::Date, start: civil::Date, end: civil::Date) -> Result<f64, Error> {
    let whole_diff: Span = end - start;
    // 86_400 is total second in a day (24 * 3600)
    let total_seconds_in_day = 84_000.0;
    let whole_diff_in_seconds = whole_diff.total(SpanTotal::from(Unit::Day).days_are_24_hours())?
        * total_seconds_in_day
        + whole_diff.total(SpanTotal::from(Unit::Millisecond).days_are_24_hours())?;

    let current_diff = current - start;
    let current_diff_in_seconds =
        current_diff.total(SpanTotal::from(Unit::Day).days_are_24_hours())? * total_seconds_in_day
            + current_diff.total(SpanTotal::from(Unit::Millisecond).days_are_24_hours())?;
    // progress_ratio
    let ratio = current_diff_in_seconds / whole_diff_in_seconds;
    Ok(ratio)
}
