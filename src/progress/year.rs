use anyhow::Result;
use time::{Date, Month};

use super::{compute, today};

pub fn year() -> Result<f64> {
    year_ratio(today()?)
}

fn year_ratio(today: Date) -> Result<f64> {
    let start = Date::from_calendar_date(today.year(), Month::January, 1)?;
    let end = Date::from_calendar_date(today.year() + 1, Month::January, 1)?;
    compute(today, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    #[test]
    fn year_should_be_0() -> Result<()> {
        // first day of the year
        let current = date!(2021 - 1 - 1);
        let ratio = year_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        // first day of the year
        let current = date!(2022 - 1 - 1);
        let ratio = year_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        // first day of the year
        let current = date!(2023 - 1 - 1);
        let ratio = year_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.0);
        assert_eq!(ratio_int, 0);

        Ok(())
    }
    #[test]
    fn year_should_be_50() -> Result<()> {
        // middle day of the year
        let current = date!(2021 - 7 - 4);
        let ratio = year_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5041095890410959);
        assert_eq!(ratio_int, 50);

        // middle day of the year
        let current = date!(2022 - 7 - 3);
        let ratio = year_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5013698630136987);
        assert_eq!(ratio_int, 50);

        // middle day of the year
        let current = date!(2023 - 7 - 3);
        let ratio = year_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.5013698630136987);
        assert_eq!(ratio_int, 50);

        Ok(())
    }
    #[test]
    fn year_should_be_100() -> Result<()> {
        // last day of the year
        let current = date!(2021 - 12 - 31);
        let ratio = year_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.9972602739726028);
        assert_eq!(ratio_int, 99);

        // last day of the year
        let current = date!(2023 - 12 - 31);
        let ratio = year_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.9972602739726028);
        assert_eq!(ratio_int, 99);

        // last day of the year
        let current = date!(2023 - 12 - 31);
        let ratio = year_ratio(current)?;
        let ratio_int = (ratio * 100.0) as i32;

        assert_eq!(ratio, 0.9972602739726028);
        assert_eq!(ratio_int, 99);

        Ok(())
    }
}
