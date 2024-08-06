use jiff::civil;

use super::{compute, today};
use crate::error::Error;

pub fn month() -> Result<f64, Error> {
    month_ratio(today())
}

fn month_ratio(today: civil::Date) -> Result<f64, Error> {
    let start = civil::date(today.year(), today.month(), 1);
    let end = civil::date(today.year(), today.month(), today.days_in_month());
    compute(today, start, end)
}

#[cfg(test)]
mod tests {
    use jiff::civil;

    use super::*;
    use anyhow::Result;

    #[test]
    fn month_should_be_0() -> Result<()> {
        // first day of the month
        let current = civil::date(2021, 1, 1);
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        // first day of the month
        let current = civil::date(2021, 2, 1);
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        // first day of the month
        let current = civil::date(2020, 1, 1);
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        Ok(())
    }
    #[test]
    fn month_should_be_50() -> Result<()> {
        // middle day of the month
        let current = civil::date(2021, 1, 16);
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5);
        assert_eq!(ratio_int, 50);

        // middle day of the month
        let current = civil::date(2021, 2, 15);
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        // it's okay for february.
        assert_eq!(ratio, 0.5185185185185185);
        assert_eq!(ratio_int, 51);

        // middle of the month
        let current = civil::date(2021, 3, 16);
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5);
        assert_eq!(ratio_int, 50);

        Ok(())
    }
    #[test]
    fn month_should_be_100() -> Result<()> {
        // last day of the month
        let current = civil::date(2021, 1, 31);
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 1.0);
        assert_eq!(ratio_int, 100);

        // last day of the month
        let current = civil::date(2021, 2, 28);
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 1.0);
        assert_eq!(ratio_int, 100);

        // last day of the month
        let current = civil::date(2020, 1, 31);
        let ratio = month_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 1.0);
        assert_eq!(ratio_int, 100);

        Ok(())
    }
}
