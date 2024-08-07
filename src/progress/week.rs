use jiff::{civil, ToSpan};

use super::compute;
use crate::error::Error;

pub fn week(today: civil::Date) -> Result<f64, Error> {
    week_ratio(today)
}

fn week_ratio(current: civil::Date) -> Result<f64, Error> {
    let start = current.checked_sub(current.weekday().to_monday_zero_offset().days())?;
    let end = start + 6.days();
    compute(current, start, end)
}

#[cfg(test)]
mod tests {
    use jiff::civil;

    use super::*;
    use anyhow::Result;

    #[test]
    fn week_should_be_0() -> Result<()> {
        // first day of the week
        let current = civil::date(2021, 1, 4);
        let ratio = week_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;
        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        // first day of the week
        let current = civil::date(2021, 1, 11);
        let ratio = week_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;
        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        // first day of the week
        let current = civil::date(2020, 1, 6);
        let ratio = week_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;
        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        Ok(())
    }
    #[test]
    fn week_should_be_50() -> Result<()> {
        // middle day of the week
        let current = civil::date(2021, 1, 7);
        let ratio = week_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5);
        assert_eq!(ratio_int, 50);

        // middle day of the week
        let current = civil::date(2021, 1, 14);
        let ratio = week_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5);
        assert_eq!(ratio_int, 50);

        // middle day of the week
        let current = civil::date(2020, 1, 9);
        let ratio = week_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5);
        assert_eq!(ratio_int, 50);

        Ok(())
    }
    #[test]
    fn week_should_be_100() -> Result<()> {
        // last day of the week
        let current = civil::date(2021, 1, 10);
        let ratio = week_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 1.0);
        assert_eq!(ratio_int, 100);

        // last day of the week
        let current = civil::date(2021, 1, 17);
        let ratio = week_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 1.0);
        assert_eq!(ratio_int, 100);

        // last day of the week
        let current = civil::date(2020, 1, 12);
        let ratio = week_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 1.0);
        assert_eq!(ratio_int, 100);

        Ok(())
    }
}
