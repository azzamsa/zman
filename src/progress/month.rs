use chrono::Datelike;

use super::{compute, today};
use crate::{error::Error, progress::date, Date};

pub fn month() -> Result<f64, Error> {
    month_ratio(today())
}

fn month_ratio(today: Date) -> Result<f64, Error> {
    let start = date(today.year(), today.month(), 1)?;
    let end_date = end_date_in_current_month(today)?;
    let end = date(today.year(), today.month(), end_date as u32)?;
    compute(today, start, end)
}

/// Count the end date in current month
fn end_date_in_current_month(today: Date) -> Result<i64, Error> {
    let (year, month) = (today.year(), today.month());
    let start = date(year, month, 1)?;

    // `12` is December
    let end_year = if month == 12 { year + 1 } else { year };

    // `12` is December
    let end_month = if month == 12 {
        1 // January
    } else {
        month + 1
    };
    let end = date(end_year, end_month, 1)?;

    Ok((end - start).num_days())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::progress::date;
    use anyhow::Result;

    #[test]
    fn month_should_be_0() -> Result<()> {
        // first day of the month
        let current = date(2021, 1, 1)?;
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        // first day of the month
        let current = date(2021, 2, 1)?;
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        // first day of the month
        let current = date(2020, 1, 1)?;
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        Ok(())
    }
    #[test]
    fn month_should_be_50() -> Result<()> {
        // middle day of the month
        let current = date(2021, 1, 16)?;
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5);
        assert_eq!(ratio_int, 50);

        // middle day of the month
        let current = date(2021, 2, 15)?;
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        // it's okay for february.
        assert_eq!(ratio, 0.5185185185185185);
        assert_eq!(ratio_int, 51);

        // middle of the month
        let current = date(2021, 3, 16)?;
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5);
        assert_eq!(ratio_int, 50);

        Ok(())
    }
    #[test]
    fn month_should_be_100() -> Result<()> {
        // last day of the month
        let current = date(2021, 1, 31)?;
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 1.0);
        assert_eq!(ratio_int, 100);

        // last day of the month
        let current = date(2021, 2, 28)?;
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 1.0);
        assert_eq!(ratio_int, 100);

        // last day of the month
        let current = date(2020, 1, 31)?;
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 1.0);
        assert_eq!(ratio_int, 100);

        Ok(())
    }
}
